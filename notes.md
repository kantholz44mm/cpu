# 8-Bit CPU Architecture _"Arch"_

## general
- 8 bit word size
- 16 bit address bus
- little endian

## sources:
- [SingleCycleCPU.pdf](https://ee.usc.edu/~redekopp/ee457/slides/EE457Unit5_SingleCycleCPU.pdf)
- [Berkeley Lecture](https://inst.eecs.berkeley.edu/~cs61c/fa14/lec/27/2014Fa-CS61C-L27-sc-CPU-1up.pdf)
- [Adder/Subtractor 4bit ADC,ADD,SBB,SUB](https://electronics.stackexchange.com/questions/555425/are-carry-flags-set-even-when-we-discard-the-carry-in-an-arithmetic-logic-unit)


## registers
8 registers: r0 - r7
r7 = rp
r6 = GP
r5 = GP
r4 = GP
r3 = GP
r2 = GP
r1 = GP
r0 = GP
PC

r6 used as page register in store instructions.
PC cannot be directly accessed and is used as the 16 bit program counter. NOTE: not in bytes, but in instructions. Each instruction is 32 bits wide.

## ALU ops:
0b000 ADD
0b001 ADC
0b010 SUB
0b011 SBB
0b100 OR
0b101 NOR
0b110 XOR
0b111 AND

## flags
0: Carry/Borrow
1: Overflow
2: Equal
3: Negative
4: Zero

## instruction encoding
- fixed size, 4 bytes
- "cond" are the conditional execution bits (see conditional flags above.)
- "rd", "ro1" and "ro2" are registers, encoded 0-7 with 3 bits each.
- "aluop" are the last 3 bits of the instruction and specify which operation is executed by the ALU.

```
| Byte                 |           0            |           1           |           2           |           4            |
| Bit                  | 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F |
| Instruction          |   opcode   |  cond  |  rd    |  ro1   |  ro2   |                      imm16                     |
| Instruction          |   opcode   |  cond  |  rd    |  ro1   |  ro2   |         imm8          | aluop  |               |
```

## instructions:
all instructions marked with ^(i.e the ALU operations) modify the flags register r7/rf.
all instruction taking either imm16 or imm8 are 4 bytes long, otherwise they are 2.
all instructions can be conditionally executed based on the state of r7/rf.
```
0x0 NOP
0x1 LW    rd, [ro1:rp]      : rd = memory[ro1:rp]
0x2 LWI   rd, [imm16]       : rd = memory[imm16]
0x3 SW    [ro1:rp], ro2     : memory[ro1:rp] = ro2
0x4 SWI   [imm16], ro2      : memory[imm16] = ro2
0x5 MW    rd, ro1           : rd = ro1
0x6 MWI   rd, imm8          : rd = imm8
0x7 JP    ro1:ro2           : PC = ro1:ro2
0x8 JPI   imm16             : PC = imm16
0x9 ALU   rd, ro1, ro2      : rd = ALU(ro1, ro2, aluop)
0xA ALUI  rd, ro1, imm8     : rd = ALU(ro1, imm8, aluop)
0xB ALUF  rd, ro1, ro2      : same as ALU, but updates Flags register
0xC ALUFI rd, ro1, imm8     : same as ALUI, but updates Flags register
0xD CMP^  ro1, ro2          : rf = compare(ro1, ro2)
0xE
0xF
```

# control lines
RWEN:1   Register Write Enable            -> Whether the register file shall be updated with a new value
FWEN:1   Flag Write Enable                -> Should the Flag register be updated?
MWEN:1   Memory Read/Write Selection      -> Should the memory read or write? (if none required, just read. no harm.)
BSSEL:1  ALU Operand B source Selection   -> Selects if the B operand in the ALU is imm8 or ro2.
MASSEL:1  Memory Address Selection        -> Selects the source for the memory address: [ro1:rp] or [imm16]
RSSEL:2  Register Source Selection        -> Selects the source for register writes: [imm8, ALU result, memory read]
PCSSEL:2 Program Counter Source Selection -> Selects whether the PC shall be loaded from imm16, the adder or ro1:ro2 or 0