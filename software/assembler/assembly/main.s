@include? <pseudo_operations.s>
@include? <stack.s>
@include? <function_calls.s>

; start at 0 and 1
MOVI R1, 0
MOVI R2, 0
MOVI R3, 0
MOVI R4, 1


a_simple_loop:
    ADDI R1, R1, 1
    BZI [a_simple_loop], RZ


fib_loop:
    ADD16 RH, RL, R1, R2, R3, R4
    
    ; check overflow
    ANDINF RF, RF, 0x4
    BNZI [done], RF

    MOV R1, R3
    MOV R2, R4
    MOV R3, RH
    MOV R4, RL
    PUSH16 R3, R4

    BZI [fib_loop], RZ

done:
    HCF

not_quite_done:
    CALL still_not_done
    RET

still_not_done:
    RET

@include? <fibonacci.s>