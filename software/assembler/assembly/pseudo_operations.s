@macro NOP
ADC* RZ, RZ, RZ
@endmacro

@macro MOV %0, %1
XOR* %0, RZ, %1
@endmacro

@macro ADD %0, %1, %2
MOV RF, 0
ADC %0, %1, %2
@endmacro

@macro SUB %0, %1, %2
MOV RF, 0
SBB %0, %1, %2
@endmacro

; 0:1 = 2:3 + 4:5 + C
@macro ADC16 %0, %1, %2, %3, %4, %5
ADC %1, %3, %5
ADC %0, %2, %4
@endmacro

;  0:1 = 2:3 - 4:5 - C
@macro SBB16 %0, %1, %2, %3, %4, %5
SBB %1, %3, %5
SBB %0, %2, %4
@endmacro

@macro ADD16 %0, %1, %2, %3, %4, %5
MOV RF, 0
ADC16 %0, %1, %2, %3, %4, %5
@endmacro

@macro SUB16 %0, %1, %2, %3, %4, %5
MOV RF, 0
SBC16 %0, %1, %2, %3, %4, %5
@endmacro

@macro INC %0
MOV RF, 0
ADC %0, %0, 1
@endmacro

@macro DEC %0
MOV RF, 0
SBB %0, %0, 1
@endmacro

@macro CALL %0
LA $(1)
BZ %0, RZ
@endmacro

@macro RET
BZ RH:RL, RZ
@endmacro