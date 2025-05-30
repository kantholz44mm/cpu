import os

# Set the directory containing the images
image_dir = './images/pcb'

# Get all .jpg files
files = [f for f in os.listdir(image_dir) if f.endswith('.jpg')]

for filename in sorted(files):
    name_parts = filename[:-4].split('_')  # Remove '.jpg' and split

    if len(name_parts) < 3:
        print(f"Skipping unrecognized filename: {filename}")
        continue

    board_name = name_parts[0].capitalize()
    if 'not' in name_parts:
        assembled_status = 'Partially assembled'
    else:
        assembled_status = 'Assembled'
    side = name_parts[-1]

    description = f"{assembled_status} {board_name} circuit board, {side} side"
    label = f"app:{'_'.join(name_parts)}"

    latex_block = f"""\\subsection{{{description}}} \\label{{{label}}}
\\begin{{center}}
    \\includegraphics[height=\\textwidth]{{images/pcb/{filename}}}
\\end{{center}}
\\newpage
"""
    print(latex_block)
