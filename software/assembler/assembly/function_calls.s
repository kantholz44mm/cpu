@include? <pseudo_operations.s>
@include? <stack.s>

my_cool_function:
    MOV R3, RH
    MOV R4, RL
    POP R1
    POP R2
    
    ADD R1, R1, R2
    PUSH R1

    MOV RH, R3
    MOV RL, R4
    RET
