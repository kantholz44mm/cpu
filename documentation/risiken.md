# Komplexität des Designs
- Schaltungskomplexität mit Featureset der ISA gekoppelt -> Berücksichtigung bei Entwurf der ISA
- Schaltungskomplexität <-> Fehleranfälligkeit <-> Featureset der ISA

Das Design der Schaltung sollte möglichst simpel sein. Darunter zählen:
- Anzahl der elektrischen Verbindungen (innerhalb & zwischen Modulen)
- Anzahl der elektrischen Komponenten
- Modularität & Wiederverwendung von Schaltungsabschnitten

# Timing
- Signalverzögerungen: Analyse des "kritischen Pfades" und Ermitteln der maximalen Frequenz jedes Moduls
- Synchronisation: Zugriffe auf Register oder Speicher müssen präzise getaktet seinisterzugriffen oder Speicheroperationen auftreten.

# Physische Umsetzung
- Platzbedarf: Einzelne Gatter verbrauchen viel Platz -> Module in Schichten, nach Wichtigkeit für die Veranschaulichung sortiert montieren
- Stromverbrauch: Mehr Gatter -> Mehr Strom -> Belastung der Stromversorgung
- Spannungseinbrüche: Digitale Schaltungen ändern ihren Zustand oft, daher kann die Versorgungsspannung sehr volatil sein -> Viele Kondensatoren

# Schaltungsfehler
- Fehler in der logischen Struktur
- Fehler in der Transformation von logische in elektrische Struktur
- Fehler in der Umsetzung des Leiterkartenentwurfs

Schwerwiegende Fehler in der Schaltung können zur Zerstörung von elektrischen Bauteilen führen. Daher muss immer darauf geachtet werden, welche Komponenten wann und wie verbunden werden.

# Debugging
- Fehlersuche in Hardware schwierig, da viele Komponenten verbaut sind. Daher möglichst viele/alle Fehler in den Entwurfs- und Testphasen in der Simulation beheben.
- Schnittstellen für einfaches Debugging an der Hardware (Programmierinterface, Auslesen des Speichers, Visuelle Indikatoren, etc.)

# Limitierte Funktionalität
Möglicherweise können nicht alle Programme umgesetzt werden. Daher muss im Vorraus bekannt sein, welche Programme auf der CPU mindestens unterstützt werden sollen. Diese Programme müssen vor der Implementierung in Hardware simuliert & verifiziert werden.
