# Aktuelle Gegebenheiten
- Unvollständige Spezifikation einer ISA
- Implementierung in Logiksimulationssoftware
- Assembler-Software

# Das Ziel
## Zentrale Forschungsfrage
Wie baue ich eine simple CPU aus logischen Gattern?
("Vom Transistor zum Rechner")

## Muss (Evaluation des Projekterfolgs)
- Vollständige Spezifikation der ISA
- Vollständige Emulation/Simulation der ISA
- Funktionsfähige Programmierwerkzeuge
  - Minimum: Assembler
  - Nice-To-Have: C, C++, etc.
- Funktionsfähige Hardware-Module (mit Eingabe/Ausgabe):
  - ALU
  - Register
- Dokumentation
  - Test Cases & Testprotokoll
  - Anwenderanleitung

## Soll/Kann
- Zusätzliche Hardware-Module
  - Arbeitsspeicher
  - Taktgeber
  - Eingabe-/Ausgabeeinheit
  - Steuereinheit
- Funktionsfähige physikalische CPU aus Einzelmodulen
- Beispielprogramme (z.B 32-bit Fibonacci-Folge, Primalitätstest, "Standard"-Benchmarks)

# Metriken mit Gewichtung:
- (1) Kosten
- (3) Komplexität
- (2) Bauform (Modularität, Erweiterbarkeit, Größe)
- (1) Performanz
- (1) Kompatibilität
- (2) Veranschaulichungsgüte

# Plan
## Recherche (2 Wochen)
- Literatur
  - ISA Entwurf
  - Existierende ISAs
  - Elektrotechnische Grundlagen
- Dozenten (Rentschler, Wendel, Gnegel)
- Ähnliche Projekte

## Vervollständigung ISA (1 Woche)
- Zielsetzung & Bewertungskriterien (worauf lege ich wert? Wo wird Komplexität eingespart?)
- Vollständige Spezifikation
  - Register
  - Operationen & Encoding
  - Flags
  - Verhalten (besonders in Edge-Cases)

## Logischer Entwurf (4 Wochen)
- Bottom-Up: Gatter -> Funktionen -> Module -> Gesamtstruktur
- Simulation
- Emulation
- Software-Tools (Assembler/Compiler/Debugger)

## Machbarkeit prüfen (1 Woche)
- Testen verschiedener Programme in Simulation/Emulation
- Feedback von Dozenten einholen
- Abschätzung der Ressourcen für physikalische Umsetzung

## Elektrischer Entwurf (4 Wochen)
- Implementierungsdetails festlegen
  - Größe der Module
  - Art der elektischen Verbindung zwischen Modulen
  - Routing-Strategien
  - Komponentenwahl (Bezug auf logischen Entwurf)
  - Trägerstruktur
- Layout der Module
- Getrennter Entwurf jedes Moduls (möglichst direkte Umsetzung der logischen Strukturen)
- Bauteile für Veranschaulichung (LEDs, Buzzer, Knöpfe, etc.)

## Inbetriebnahme (3 Wochen)
- Aufbau der einzelnen Module
- Testen jedes Moduls
- Montieren auf Trägerstruktur & Verbinden mit anderen Modulen
- Testen von Teilstrukturen wo möglich (z.B. ALU + Register)
- Testen des gesamten Aufbaus

## Puffer (ca. 4 Wochen)
