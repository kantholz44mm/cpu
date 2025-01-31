@macro MOV %0, %1
ADDNF %0, RZ, %1
@endmacro

@macro MOVI %0, %1
ADDINF %0, RZ, %1
@endmacro

; 0:1 = 2:3 + 4:5
@macro ADD16 %0, %1, %2, %3, %4, %5
ADD %1, %3, %5
ADC %0, %2, %4
@endmacro

;  0:1 = 2:3 - 4:5
@macro SUB16 %0, %1, %2, %3, %4, %5
SUB %1, %3, %5
SBB %0, %2, %4
@endmacro

@macro ADC16 %0, %1, %2, %3, %4, %5
ADC %1, %3, %5
ADC %0, %2, %4
@endmacro

@macro SBB16 %0, %1, %2, %3, %4, %5
SBB %1, %3, %5
SBB %0, %2, %4
@endmacro

@macro INC %0
ADDI %0, %0, 1
@endmacro

@macro DEC %0
SUBI %0, %0, 1
@endmacro

@macro INCNF %0
ADDINF %0, %0, 1
@endmacro

@macro DECNF %0
SUBINF %0, %0, 1
@endmacro

@macro LDAI address
MOVI RL, {address >> 0 & 0xFF}
MOVI RH, {address >> 8 & 0xFF}
@endmacro
