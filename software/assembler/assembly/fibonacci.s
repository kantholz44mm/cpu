@include? <stack.s>
@include? <pseudo_operations.s>
@include? <function_calls.s>

fibonacci:
    PGET16 PARAM0, R1, R2
    PGET16 PARAM1, R3, R4

    ; add the two numbers
    ADD16 RH, RL, R1, R2, R3, R4
    PSET16 RETVAL, RH, RL

    RET
