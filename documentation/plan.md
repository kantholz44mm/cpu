# Aktuelle Gegebenheiten
- Unvollständige Spezifikation einer ISA
- Implementierung in Logiksimulationssoftware
- Assembler-Software

# Das Ziel
## Zentrale Forschungsfrage
Wie baue ich eine simple CPU aus logischen Gattern?
("Vom Transistor zum Rechner")

## Muss
- Vollständige Spezifikation der ISA
- Vollständige Emulation/Simulation der ISA
- Funktionsfähige Programmierwerkzeuge
- Funktionsfähige Hardware-Module:
  - ALU
  - Register

## Fest eingeplant
- Funktionsfähige physikalische CPU aus Einzelmodulen

# Metriken mit Gewichtung:
- (1) Kosten
- (1) Größe
- (3) Komplexität
- (1) Modularität & Erweiterbarkeit
- (1) Performanz
- (1) Kompatibilität
- (2) Veranschaulichungsgüte

# Plan
## Recherche
- Literatur
  - ISA Entwurf
  - Existierende ISAs
  - Elektrotechnische Grundlagen
- Dozenten (Rentschler, Wendel, Gnegel)
- Ähnliche Projekte

## Vervollständigung ISA
- Zielsetzung & Bewertungskriterien (worauf lege ich wert? Wo wird Komplexität eingespart?)
- Vollständige Spezifikation
  - Register
  - Operationen & Encoding
  - Flags
  - Verhalten (besonders in Edge-Cases)

## Logischer Entwurf
- Bottom-Up: Gatter -> Funktionen -> Module -> Gesamtstruktur
- Simulation
- Emulation
- Software-Tools (Assembler/Compiler/Debugger)

## Machbarkeit prüfen
- Testen verschiedener Programme in Simulation/Emulation
- Feedback von Dozenten einholen
- Abschätzung der Ressourcen für physikalische Umsetzung

## Elektrischer Entwurf
- Implementierungsdetails festlegen
  - Größe der Module
  - Art der elektischen Verbindung zwischen Modulen
  - Routing-Strategien
  - Komponentenwahl (Bezug auf logischen Entwurf)
  - Trägerstruktur
- Layout der Module
- Getrennter Entwurf jedes Moduls (möglichst direkte Umsetzung der logischen Strukturen)
- Bauteile für Veranschaulichung (LEDs, Buzzer, Knöpfe, etc.)

## Inbetriebnahme
- Aufbau der einzelnen Module
- Testen jedes Moduls
- Montieren auf Trägerstruktur & Verbinden mit anderen Modulen
- Testen von Teilstrukturen wo möglich (z.B. ALU + Register)
- Testen des gesamten Aufbaus

