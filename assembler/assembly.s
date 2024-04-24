NOP
ADDI r0 r0 0x12
SWI [0x10] r0
LWI r1 [0x10]
ADDI r1 r1 0
HCF

loop:
    SW [r1] r0
    ADDI r0 r0 2
    ADDI r1 r1 1
    CSUBI r1 0x10
Z>  JPI [continue]
    JPI [loop]

; otherwise, we're done initializing
continue:
    XOR r0 r0 r0
    XOR r1 r1 r1
    ADDI r2 r2 0x10
    HCF
    CADDI r2 0

loop2:
    LW r0 [r1]
    ADD r0 r0 r0

    ADDI r2 r2 1
    ADDI r1 r1 1
    SW [r2] r0

    CSUBI r1 0x10
Z>  JPI [continue2]
    JPI [loop2]

continue2:
HCF