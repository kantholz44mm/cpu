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
4 registers: r0 - r3
r3 = rp
r2 = GP
r1 = GP
r0 = GP
PC

PC cannot be directly accessed and is used as the 16 bit program counter. NOTE: not in bytes, but in instructions. Each instruction is 32 bits wide.

## ALU ops:
0x0 | 0b0000 ADD
0x1 | 0b0001 ADC
0x2 | 0b0010 SUB
0x3 | 0b0011 SBB
0x4 | 0b0100 OR
0x5 | 0b0101 NOR
0x6 | 0b0110 XOR
0x7 | 0b0111 AND
0x8 | 0b1000 SHL
0x9 | 0b1001 SHR
0xA | 0b1010 /
0xB | 0b1011 /
0xC | 0b1100 /
0xD | 0b1101 /
0xE | 0b1110 /
0xF | 0b1111 /

## flags
0: Carry/Borrow
1: Equal
2: Zero

## conditionals:
0: Always
1: Carry/Borrow
2: Equal
3: Zero

## instruction encoding

```
| Byte                 |           0            |           1           |           2           |           4            |
| Bit                  | 1F 1E 1D 1C 1B 1A 19 18 17 16 15 14 13 12 11 10 0F 0E 0D 0C 0B 0A 09 08 07 06 05 04 03 02 01 00 |
| Instruction          |   opcode   |cond |    alu    | rd  | ro1 | ro2 |                      imm16                     |
```

## instructions:
```
0x0 NOP
0x1 LW    rd, [rp:ro1]      : rd = memory[rp:ro1]
0x2 LWI   rd, [imm16]       : rd = memory[imm16]
0x3 SW    [rp:ro1], ro2     : memory[rp:ro1] = ro2
0x4 SWI   [imm16], ro2      : memory[imm16] = ro2
0x5 JP    [rp:ro1]          : PC = rp:ro1
0x6 JPI   [imm16]           : PC = imm16
0x7 ALU   rd, ro1, ro2      : rd = ALU(ro1, ro2, aluop)
0x8 ALUI  rd, ro1, imm8     : rd = ALU(ro1, imm8, aluop)
0x9
0xA
0xB
0xC
0xD
0xE
0xF HCF                     : Halt and Catch Fire.
```

## control lines
RWEN:1   Register Write Enable            -> Whether the register file shall be updated with a new value
FWEN:1   Flag Write Enable                -> Whether the ALU flag register shall be updated
IOREN:1   Memory Read Enable               -> Should the memory output onto the data bus?
IOWEN:1   Memory Write Enable              -> Should the memory input from the data bus?
BSSEL:1  ALU Operand B source Selection   -> Selects if the B operand in the ALU is imm8 or ro2.
ASSEL:1 Memory Address Selection         -> Selects the source for the memory address: [rp:ro1] or [imm16]
RSSEL:1  Register Source Selection        -> Selects the source for register writes: ALU result or memory bus input
PCSSEL:1 Program Counter Source Selection -> Selects either PC+1 or, depending on MASSEL, one of [rp:ro1, imm16] for next PC value



## parts list
Quad NAND Gates:
https://www.mouser.de/ProductDetail/Toshiba/TC74HC00APF?qs=W%252B8xM3gmGj%2F0AXEac47eSQ%3D%3D
32k x 8 Parallel SRAM
https://www.mouser.de/ProductDetail/Alliance-Memory/AS6C62256-55PCN?qs=LD2UibpCYJqgbIupMJnGTQ%3D%3D

