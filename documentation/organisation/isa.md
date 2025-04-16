# AVA ISA

Die "Arithemtic Visualisation Architecture" ist eine minimalistische Rechnerarchitektur, die zur Veranschaulichung der internen Prozesse einer CPU bestimmt ist.

## Generelle Information
- Harvard Architektur (Getrennter Programm- und Arbeitsspeicher)
- 8 bit Wortbreite
- 16 bit Addressbusbreite
- 32 bit Instruktionskodierung

## Registers
| ID | Name                       | Kurzform | Breite | Zugriff             | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                   |
|----|----------------------------|----------|--------|---------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0  | Zero register              | R0/RZ    | 8      | Read only           | Beschreiben dieses Registers hat keinen Effekt. Lesen liefert immer die Konstante 0. Kann verwendet werden, um unnötige Ergebnisse zu verwerfen oder um die Konstante 0 als Operand zu erhalten.                                                                                                                                                                                               |
| 1  | General purpose register 1 | R1       | 8      | Read/Write          | Kann als Zwischenspeicher für Benutzer-definierte Berechnungen verwendet werden.                                                                                                                                                                                                                                                                                                               |
| 2  | General purpose register 2 | R2       | 8      | Read/Write          | Kann als Zwischenspeicher für Benutzer-definierte Berechnungen verwendet werden.                                                                                                                                                                                                                                                                                                               |
| 3  | General purpose register 3 | R3       | 8      | Read/Write          | Kann als Zwischenspeicher für Benutzer-definierte Berechnungen verwendet werden.                                                                                                                                                                                                                                                                                                               |
| 4  | General purpose register 4 | R4       | 8      | Read/Write          | Kann als Zwischenspeicher für Benutzer-definierte Berechnungen verwendet werden.                                                                                                                                                                                                                                                                                                               |
| 5  | Flags register             | R5/RF    | 8      | Read/Write/Indirect | Kann als Zwischenspeicher für Benutzer-definierte Berechnungen verwendet werden. Bei ALU-Operationen kann die Flags, die aus einer Berechnung resultieren, in dieses Register schreiben. Wird in der gleichen Anweisung das Schreiben von Flags und das Beschreiben des Registers mit einem Resultat angefordert, so wird das Resultat der ALU verworfen und nur die Flags werden geschrieben. |
| 6  | Low address register       | R6/RL    | 8      | Read/Write          | Kann als Zwischenspeicher für Benutzer-definierte Berechnungen verwendet werden. Operationen, die mit dem Hauptspeicher interagieren, können eine Kombination aus RL und RH als Address-Operanden verwenden.                                                                                                                                                                                   |
| 7  | High address register      | R7/RH    | 8      | Read/Write          | Kann als Zwischenspeicher für Benutzer-definierte Berechnungen verwendet werden. Operationen, die mit dem Hauptspeicher interagieren, können eine Kombination aus RL und RH als Address-Operanden verwenden.                                                                                                                                                                                   |
| 8  | Program counter register   | PC       | 16     | Indirect            | Beinhaltet die Addresse der aktuell ausgeführten Instruktion. Kann weder direkt gelesen oder beschrieben werden. Jump- und Branchinstruktionen überschreiben das Register indirekt, wenn die ausgewählte Bedingung erfüllt ist.                                                                                                                                                                |

## Flags

| ID | Name         | Kurzform | Beschreibung                                                                                                                                 |
|----|--------------|----------|----------------------------------------------------------------------------------------------------------------------------------------------|
| 0  | Zero         | Z        | Z = 1 zeigt an, dass keines der Bits eines Wertes gesetzt sind. In anderen Worten, der Ergebniswert is 0.                                    |
| 1  | Carry/Borrow | C        | C = 1 zeigt an, dass bei einer Berechnung ein Überlauf/Unterlauf des Ergebnisses auftritt, wenn die Operanden nicht vorzeichenbehaftet sind. |
| 2  | Overflow     | V        | V = 1 zeigt an, dass bei einer Berechnung ein Überlauf/Unterlauf des Ergebnisses auftritt, wenn die Operanden vorzeichenbehaftet sind.       |
| 3  | Negative     | N        | N = 1 zeigt an, dass das Ergebnis negativ ist, wenn es als vorzeichenbehaftete Zahl interpretiert wird.                                      |

## Kodierung von Anweisungen

Alle Anweisungen sind mit einer festen Länge von 32 bits kodiert. Der Addressraum des Anweisungsspeichers wird über diese Wortbreite indiziert. Das bedeutet, dass jedes Inkrement von PC einen separaten 32 bit Wert indiziert.

```
| Byte                 |           0            |           1           |           2           |           4            |
| Bit                  | 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00 |
| Field                |        OPCODE       |   RD   |   RO1  |   RO2  |                      IMM                       |
```

| Name   | Langform             | Breite | Start | Ende | Beschreibung                                                                                                                                                                          |
|--------|----------------------|--------|-------|------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| OPCODE | Opcode               | 4      | 28    | 31   | Identifiziert die Operation, die ausgeführt werden soll.                                                                                                                              |
| OS     | Operand Select       | 1      | 27    | 27   | Wählt die Quelle eines Operanden. Wenn OS = 1, dann ist der Operand in IMM enthalten, ansonsten werden Register verwendet.                                                            |
| WF     | Write Flag           | 1      | 26    | 26   | Wenn WF = 1 und Opcode < 8, dann wird der Wert in RF mit dem Flag-Output der ALU überschrieben.                                                                                       |
| RD     | Register Destination | 3      | 22    | 24   | Index des Zielregisters für das Ergebnis einer Operation                                                                                                                              |
| RO1    | Register Operand 1   | 3      | 19    | 21   | Index des Quellregisters für den ersten Operanden einer Operation                                                                                                                     |
| RO2    | Register Operand 2   | 3      | 16    | 18   | Index des Quellregisters für den zweiten Operanden einer Operation                                                                                                                    |
| IMM    | Immediate Operand    | 16     | 0     | 15   | Konstanter Wert, der als Operand verwendet werden kann. Gespeichert im Little-Endian Format. Je nach Operation werden alle 16 Bits verwendet oder der Wert wird auf 8 Bits trunkiert. |

## Anweisungen

### Operationen
| Opcode | Mnemonic | Name                 | Operanden          | Register | Beschreibung                                                                                                                                                       |
|--------|----------|----------------------|--------------------|----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 0      | ADC      | Add with Carry       | RD, RO1, RO2/IMM8  | RD, RF   | Addition von RO1 und RO2/IMM8 und dem aktuellen Carry/Borrow Bit in RF. Ergebnis wird in RD gespeichert                                                            |
| 1      | SBB      | Subtract with Borrow | RD, RO1, RO2/IMM8  | RD, RF   | Subtraktion von RO1 und RO2/IMM8 und dem aktuellen Carry/Borrow Bit in RF. Ergebnis wird in RD gespeichert                                                         |
| 2      | SHL      | Shift Left           | RD, RO1, RO2/IMM8  | RD, RF   | Verschiebt den Inhalt von RO1 um RO2/IMM8 Bits nach links und speichert das Ergebnis in RD                                                                         |
| 3      | SHR      | Shift Right          | RD, RO1, RO2/IMM8  | RD, RF   | Verschiebt den Inhalt von RO1 um RO2/IMM8 Bits nach rechts und speichert das Ergebnis in RD                                                                        |
| 4      | OR       | Bitwise OR           | RD, RO1, RO2/IMM8  | RD, RF   | Wendet die logische "OR" Funktion auf die Bits der gleichen Position von RO1 und RO2/IMM8 an und speichert das Ergebnis in RD                                      |
| 5      | AND      | Bitwise AND          | RD, RO1, RO2/IMM8  | RD, RF   | Wendet die logische "AND" Funktion auf die Bits der gleichen Position von RO1 und RO2/IMM8 an und speichert das Ergebnis in RD                                     |
| 6      | XOR      | Bitwise XOR          | RD, RO1, RO2/IMM8  | RD, RF   | Wendet die logische "XOR" Funktion auf die Bits der gleichen Position von RO1 und RO2/IMM8 an und speichert das Ergebnis in RD                                     |
| 7      | NAND     | Bitwise NAND         | RD, RO1, RO2/IMM8  | RD, RF   | Wendet die logische "NAND" Funktion auf die Bits der gleichen Position von RO1 und RO2/IMM8 an und speichert das Ergebnis in RD                                    |
| 8      | LW       | Load Word            | RD, [RH:RL/IMM16]  | RD       | Das Wort an Position RH:RL/IMM16 wird aus dem Hauptspeicher gelesen und in RD gespeichert                                                                          |
| 9      | SW       | Store Word           | [RH:RL/IMM16], RO2 |          | Das Wort in RO2 wird in den Hauptspeicher an die Position RH:RL/IMM16 geschrieben                                                                                  |
| 10     | BZ       | Branch if Zero       | RH:RL/IMM16 RO2    | PC       | Der Program Counter wird mit RH:RL überschrieben, wenn RO2 = 0                                                                                                     |
| 11     | BNZ      | Branch if not Zero   | RH:RL/IMM16 RO2    | PC       | Der Program Counter wird mit RH:RL überschrieben, wenn RO2 ≠ 0                                                                                                     |
| 12     | RES      | Reserved             |                    |          |                                                                                                                                                                    |
| 13     | RES      | Reserved             |                    |          |                                                                                                                                                                    |
| 14     | LA       | Load Address         | IMM16              | RH, RL   | RH:RL wird mit IMM16 überschrieben                                                                                                                                 |
| 15     | HCF      | Halt and Catch Fire  |                    |          | Die Ausführung wird gestoppt und die CPU wird angehalten. Um wieder in den Operationsmodus zu wechseln, muss der Benutzer das physische Bedieninterface verwenden. |

### Verhalten von Flags-Register

| Opcode | Mnemonic | Zero                                    | Carry/Borrow                                                                                                               | Overflow                                                                                                             | Negative            |
|--------|----------|-----------------------------------------|----------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------|---------------------|
| 0      | ADC      | 1, wenn das Resultat 0 ist, ansonsten 0 | 1, wenn das Resultat einer Addition von zwei nicht vorzeichenbehafteten Operanden mehr als 8 Bits benötigt, ansonsten 0    | 1, wenn das Resultat einer Addition von zwei vorzeichenbehafteten Operanden mehr als 8 Bits benötigt, ansonsten 0    | Bit 7 des Resultats |
| 1      | SBB      | 1, wenn das Resultat 0 ist, ansonsten 0 | 1, wenn das Resultat einer Subtraktion von zwei nicht vorzeichenbehafteten Operanden mehr als 8 Bits benötigt, ansonsten 0 | 1, wenn das Resultat einer Subtraktion von zwei vorzeichenbehafteten Operanden mehr als 8 Bits benötigt, ansonsten 0 | Bit 7 des Resultats |
| 2      | SHL      | 1, wenn das Resultat 0 ist, ansonsten 0 | Bit 7 des Operanden (herausgeschobener Wert)                                                                               | 0                                                                                                                    | Bit 7 des Resultats |
| 3      | SHR      | 1, wenn das Resultat 0 ist, ansonsten 0 | Bit 0 des Operanden (herausgeschobener Wert)                                                                               | 0                                                                                                                    | Bit 7 des Resultats |
| 4      | OR       | 1, wenn das Resultat 0 ist, ansonsten 0 | 0                                                                                                                          | 0                                                                                                                    | Bit 7 des Resultats |
| 5      | AND      | 1, wenn das Resultat 0 ist, ansonsten 0 | 0                                                                                                                          | 0                                                                                                                    | Bit 7 des Resultats |
| 6      | XOR      | 1, wenn das Resultat 0 ist, ansonsten 0 | 0                                                                                                                          | 0                                                                                                                    | Bit 7 des Resultats |
| 7      | NAND     | 1, wenn das Resultat 0 ist, ansonsten 0 | 0                                                                                                                          | 0                                                                                                                    | Bit 7 des Resultats |

