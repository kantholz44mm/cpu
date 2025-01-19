@include? <pseudo_operations.s>
@include? <stack.s>

; first instruction is not guaranteed to be executed,
; depending on the initial state of the clock.
NOP

MOVI R1, 0
MOVI R2, 0
MOVI R3, 0
MOVI R4, 1
; start at 0 and 1
BZI [0xFFFF], R0
;
;;ADDI R0, R0, 1
;BNZI [branched_hcf], R0
;
;regular_hcf:
;    HCF
;
;branched_hcf:
;    NOP
;    NOP
;    NOP
;    NOP
;    HCF

; initial two numbers
;PUSH16 R1, R2
;PUSH16 R3, R4
;
;fib_loop:
;    ; do addition and overflow check
;    ADD16 RH, RL, R1, R2, R3, R4
;    MOV R1, R3
;    MOV R2, R4
;    MOV R3, RH
;    MOV R4, RL
;
;    ; check overflow
;    ANDINF RF, RF, 0x4
;    BZI [done], RF
;
;    PUSH16 R3, R4
;
;    BNZI [fib_loop], RZ
;
;done:
;    HCF

