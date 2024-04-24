NOP
ADDI r0, r0, 0x12
ADDI r1, r1, 0x34
; now we 
ADDI r2, r0, 0x56
ADCI r3, r1, 0x78
SWI [0], r2
SWI [1], r3
HCF