# 8-Bit CPU Architecture _"Arch"_

## general
- 8 bit word size
- 16 bit address bus
- 32 bit instruction size
- little endian

## sources:
- [SingleCycleCPU.pdf](https://ee.usc.edu/~redekopp/ee457/slides/EE457Unit5_SingleCycleCPU.pdf)
- [Berkeley Lecture](https://inst.eecs.berkeley.edu/~cs61c/fa14/lec/27/2014Fa-CS61C-L27-sc-CPU-1up.pdf)
- [Adder/Subtractor 4bit ADC,ADD,SBB,SUB](https://electronics.stackexchange.com/questions/555425/are-carry-flags-set-even-when-we-discard-the-carry-in-an-arithmetic-logic-unit)


## registers
R7: H (High address byte)
R6: L (Low address byte)
R5: F (Flags)
R4: General Purpose
R3: General Purpose
R2: General Purpose
R1: General Purpose
R0: Null register

PC: 16 bit program counter

PC cannot be directly accessed and is used as the 16 bit program counter. NOTE: not in bytes, but in instructions. Each instruction is 32 bits wide.
R7 / RP is implicitly used for memory operations as the "page" (i.e higher) part of the address. R0 is hardwired to be 0 when read, with writes having no effect.
R6 & R7 are used for memory address operations.
R5 / F is the flags register. It is updated by performing an ALU operation.

## flags
0: Zero
1: Carry/Borrow
2: Overflow
3: Negative

## instruction encoding

```
| Byte                 |           0            |           1           |           2           |           4            |
| Bit                  | 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00 |
| Field                |   OPCODE   |OS|WF|  |   RD   |   RO1  |   RO2  |                      IMM                       |
```

OPCODE :  4 : Identifies the operation
OS     :  1 : Selects operand mode; 0 means register, 1 means immediate
WF     :  1 : Whether to update the flag register. If 0, The register can be written to like any other register.
RD     :  3 : is the index of the destination register
RO1    :  3 : Index of first operand register
RO1    :  3 : Index of second operand register
IMM    : 16 : 16 or 8 bit immediate operand (depending on operation)

## instructions:
```
0 | ADC    RD, RO1, [RO2/IMM8]             : RD = RO1 + RO2/IMM8 + Carry/Borrow
1 | SBB    RD, RO1, [RO2/IMM8]             : RD = RO1 - RO2/IMM8 - Carry/Borrow
2 | SHL    RD, RO1, [RO2/IMM8]             : RD = RO1 << RO2/IMM8
3 | SHR    RD, RO1, [RO2/IMM8]             : RD = RO1 >> RO2/IMM8
4 | OR     RD, RO1, [RO2/IMM8]             : RD = RO1 | RO2/IMM8
5 | NOR    RD, RO1, [RO2/IMM8]             : RD = ~(RO1 | RO2/IMM8)
6 | XOR    RD, RO1, [RO2/IMM8]             : RD = RO1 ^ RO2/IMM8
7 | AND    RD, RO1, [RO2/IMM8]             : RD = RO1 & RO2/IMM8

8 | LW     RD, [RH:RL/IMM16]               : RD = Memory[RH:RL/IMM16]
9 | SW     [RH:RL/IMM16], RO2              : Memory[RH:RL/IMM16] = RO2
A | BZ     R02/IMM8                        : If RO2/IMM8 == 0 ? PC = RH:RL : NOP
B | BNZ    RO2/IMM8                        : If RO2/IMM8 != 0 ? PC = RH:RL : NOP
C | JZ     IMM16                           : If RF == 0 ? PC = IMM16
D | JNZ    IMM16                           : If RF != 0 ? PC = IMM16
E | LA     IMM16                           : RH:HL = IMM16
F | HCF
```

All ALU operations (0-7) can be suffixed with *. If the suffix is found, the flags register will not be updated.

## Flag setting behavior
                ZCVN
ADC/SBB:        FFFF
OR/NOR/XOR/AND: F00F
SHL/SHR:        FF0F
