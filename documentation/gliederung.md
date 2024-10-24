- Einleitung
   - Motivation: Warum wurde dieses Projekt gewählt? Relevanz und Zielsetzung.
   - Ziel: Überblick über die beabsichtigte CPU und deren Eigenschaften (8-Bit, Single-Cycle, etc.).
   - Aufbau der Arbeit: Kurze Beschreibung der folgenden Kapitel.

- Grundlagen (WIP)
   - CPU-Grundlagen:
     - Befehlssatz
     - ALU
     - Register
     - Steuerwerk
     - Einordnung & Metriken:
       - Single- & Multicycle
       - Von Neumann/Harvard
       - RISC/CISC
   - Instruction Set Architecture (ISA): 
     - Was ist eine ISA
     - Wofür braucht man sie?
     - Überblick über typische 8-Bit-ISAs (z.B. 6502, Z80, etc.)

- Konzept
   - Entwurf der ISA: Beschreibung des Befehlssatzes (arithmetische, logische und Speicherzugriffs-Befehle).
   - Designprinzipien: Erklärung der Designentscheidungen
     - Modularität
     - ausschließlich NAND-Gatter
     - Single-Cycle
   - Überblick über die logische Struktur der CPU (Bottom-up, Gatter -> Funktionen -> Module -> CPU)

- Implementierung
   - Logisches Design: Details zur Implementierung der CPU-Module (ALU, Steuerwerk, Register) auf Gatterebene.
   - Simulationen: Durchgeführte Simulationen zur Verifikation des Designs vor der Hardwareumsetzung.
   - Elektrisches Design: Beschreibung des elektrischen Designs & der Platinenerstellung.
   - Assembler Software: Implementierung eines Assemblers zur Übersetzung von Befehlen in Maschinencode.
   - Testprogramme: Beschreibung einfacher Programme zur Überprüfung der CPU-Funktionalität.

- Inbetriebnahme
   - Aufbau: Physische Montage der CPU-Module und deren Zusammenschaltung.
   - Testen und Debuggen: Vorgehen zur Überprüfung der CPU-Funktionalität und Beseitigung von Fehlern.

- Fazit
   - Ergebnisse: Zusammenfassung der Leistungsfähigkeit und Funktionstests.
   - Schwierigkeiten und Lösungen: Besondere Herausforderungen und wie sie gemeistert wurden.
   - Ausblick: Mögliche Verbesserungen oder Erweiterungen (z.B. Pipelining, Branch Prediction).
   - Persönliche Erkenntnisse: Reflexion über den Projektverlauf und gewonnene Erfahrungen.