@include? <pseudo_operations.s>
@include? <stack.s>

@define X_START 50
@define Y_START 50
@define X_END   150
@define Y_END   100
@define COLOR   0xFF

NOP
MOV R1, X_START
MOV R2, Y_START

start_line:
    MOV R3, X_END
    MOV R1, X_START
line_loop:
    SUB RZ, R3, R1

    ; check if zero, if yes, go to next line
    AND* RF, RF, 0x1
    BNZ next_line, RF

    ; draw pixel
    MOV RH, R2
    MOV RL, R1
    MOV RF, COLOR
    SW [RH:RL], RF
    INC R1

    BZ line_loop, RZ

next_line:
    MOV R3, Y_END
    SUB RZ, R3, R2
    AND* RF, RF, 0x1
    BNZ done, RF

    INC R2
    BZ start_line, RZ

done:
    HCF