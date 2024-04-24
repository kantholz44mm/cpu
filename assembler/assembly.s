NOP
loop:
    SW [r1] r0
    ADDI r0 r0 2
    ADDI r1 r1 1
    CSUBI r1 0x40
Z>  JPI [continue]
    JPI [loop]

; otherwise, we're done initializing
continue:
    XOR r1 r1 r1
loop2:
    LW r0 [r1]
    ADDI r0 r0 8

    ADDI r1 r1 0x40
    SW [r1] r0

    SUBI r1 r1 0x3F
    CSUBI r1 0x40
Z>  JPI [continue2]
    JPI [loop2]

continue2:
HCF