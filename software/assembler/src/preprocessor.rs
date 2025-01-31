use std::{collections::HashSet, iter::zip, path::Path};

use regex::Regex;

#[derive(Debug)]
struct Macro {
    pub name: String,
    pub body: String,
    pub args: Vec<String>,
}

fn preprocess_include(initial_file: &Path, input: &String) -> Result<String, String> {
    let include_pattern = Regex::new(r"(?s)@include(?P<skipdup>\?)?\s*<(?P<file>[^\n]*?)>").unwrap();
    let mut included_files: HashSet<String> = HashSet::new();
    let mut text = input.clone();
    included_files.insert(initial_file.to_str().unwrap().to_string());

    while let Some(captures) = include_pattern.captures(&text) {
        let filename = captures.name("file").unwrap().as_str();
        let file = std::fs::canonicalize(filename).map_err(|error| {
            format!("Error when reading source file: '{}': {error}", filename)
        })?;

        let skipdup = captures.name("skipdup").is_some();
        
        if !file.is_file() {
            return Err(format!("No such file: '{}'", file.to_str().unwrap()));
        }

        if !included_files.insert(String::from(file.to_str().unwrap())) {
            if skipdup {
                text.replace_range(captures.get(0).unwrap().range(), "");
                continue;
            }
            return Err(format!("Recursive/Duplicate file include detected: '{}'", file.to_str().unwrap()));
        }
        
        match std::fs::read_to_string(file.to_path_buf()) {
            Ok(content) => { println!("include: {}", captures.get(0).unwrap().as_str()); text.replace_range(captures.get(0).unwrap().range(), &content); },
            Err(error) => { return Err(format!("Error when reading source file: '{}': {error}", file.to_str().unwrap())); }
        }
    }

    Ok(text)
}

fn preprocess_macros(input: &String) -> Result<String, String> {
    let macro_pattern = Regex::new(r"(?s)@macro\s+(?P<name>[a-zA-Z0-9_*]+)(?P<arguments>[^\n]*)\n(?P<body>.*?)@endmacro").unwrap();
    let mut macros: Vec<Macro> = Vec::new();
    let mut text = input.clone();

    while let Some(capture) = macro_pattern.captures(&text) {
        let name = capture.name("name").unwrap().as_str();
        let args: Vec<String> = capture.name("arguments").unwrap().as_str().split(',').filter_map(|arg| {
            match arg.trim() {
                "" => None,
                otherwise => Some(otherwise.to_string())
            }
        }).collect();
        let body = capture.name("body").unwrap().as_str();
        
        if let Some(_) = macros.iter().find(|m| (**m).name == name) {
            return Err(format!("Found duplicate macro definition: '{}'", name));
        }
        
        macros.push(Macro {
            name: name.to_string(),
            body: body.to_string(),
            args: args.clone(),
        });
        
        text.replace_range(capture.get(0).unwrap().range(), "");
    }

    // parse the text for any macro invocations and replace accordingly
    let mut invocation_found = true;
    while invocation_found {
        invocation_found = false;

        for m in macros.iter() {
            let invocation_pattern = Regex::new(&format!(r"(?m)^\s*{}\b(?P<arguments>[^\n]*)", regex::escape(&m.name))).unwrap();
            if let Some(invocation) = invocation_pattern.captures(&text) {
                let args: Vec<String> = invocation.name("arguments").unwrap().as_str().split(',').filter_map(|arg| {
                    match arg.trim() {
                        "" => None,
                        otherwise => Some(otherwise.to_string())
                    }
                }).collect();
                
                if args.len() != m.args.len() {
                    return Err(format!("Found {} arguments, but expected {} while expanding macro '{}'", args.len(), m.args.len(), m.name));
                }
        
                let mut expansion = m.body.clone();
                for (name, value) in zip(m.args.iter(), args.iter()) {
                    expansion = expansion.replace(&format!(r"{}", regex::escape(name)), value);
                }
    
                text.replace_range(invocation.get(0).unwrap().range(), &expansion);
                invocation_found = true;
            }
        }
    }

    Ok(text)
}

fn preprocess_defines(input: &String) -> Result<String, String> {
    let define_pattern = Regex::new(r"(?s)@define\s+(?P<name>[a-zA-Z0-9_]+)\s+(?P<content>[^\n]*)").unwrap();
    let mut defines: Vec<(String, String)> = Vec::new();
    let mut text = input.clone();

    while let Some(capture) = define_pattern.captures(&text) {
        let name = capture.name("name").unwrap().as_str();
        let content = capture.name("content").unwrap().as_str();
        
        if let Some(_) = defines.iter().find(|(defined_name, _)| defined_name == name) {
            return Err(format!("Found duplicate definition: '{}'", name));
        }
        
        defines.push((name.to_string(), content.to_string()));
        text.replace_range(capture.get(0).unwrap().range(), "");
    }

    // parse the text for any define invocations and replace accordingly
    for (name, content) in defines.iter() {
        let invocation_pattern = Regex::new(&format!(r"(?m)^[^;\n]*\b(?P<content>{})\b", regex::escape(&name))).unwrap();
        while let Some(invocation) = invocation_pattern.captures(&text) {
            text.replace_range(invocation.name("content").unwrap().range(), &content);
        }
    }

    Ok(text)
}

fn preprocess_strip_comments(input: &String) -> String {
    let comment_pattern = Regex::new(r"(?m);[^\n]*").unwrap();
    comment_pattern.replace_all(&input, "").to_string()
}

/// read and preprocess the given input file.
pub fn preprocess(input_file: &Path) -> Result<String, String> {
    let input_file = std::fs::canonicalize(input_file).map_err(|error| {
        format!("Error when reading source file: '{}': {error}", input_file.to_str().unwrap())
    })?;
    let text = std::fs::read_to_string(&input_file).map_err(|error| {
        format!("Error when reading source file: '{}': {error}", input_file.to_str().unwrap())
    })?;

    let text_post_include = preprocess_include(&input_file, &text)?;
    let text_post_macros = preprocess_macros(&text_post_include)?;
    let text_post_defines = preprocess_defines(&text_post_macros)?;
    let text_post_strip = preprocess_strip_comments(&text_post_defines);

    Ok(text_post_strip)
}
