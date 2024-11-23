@include? <pseudo_operations.s>
@include? <stack.s>

; first instruction is not guaranteed to be executed,
; depending on the initial state of the clock.
NOP

; start at 0 and 1
MOV R1, 0
MOV R2, 0
MOV R3, 0
MOV R4, 1

; initial two numbers
PUSH16 R1, R2
PUSH16 R3, R4

fib_loop:
    ; do addition and overflow check
    ADD16 RH, RL, R1, R2, R3, R4
    MOV R1, R3
    MOV R2, R4
    MOV R3, RH
    MOV R4, RL

    ; check overflow
    AND* RF, RF, 0x2
    BNZ done, RF

    PUSH16 R3, R4

    BZ fib_loop, RZ

done:
    HCF

