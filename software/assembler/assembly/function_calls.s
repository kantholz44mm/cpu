@include? <pseudo_operations.s>
@include? <stack.s>

@macro CALL function
MOVI R4, {(($ + 11) >> 0) & 0xFF}
MOVI R5, {(($ + 10) >> 8) & 0xFF}
PUSH16 R5, R4
MOVI RL, {(function >> 0) & 0xFF}
MOVI RH, {(function >> 8) & 0xFF}
BZ [RH:RL], RZ
@endmacro

@macro RET
POP16 R5, R4
MOV RH, R5
MOV RL, R4
BZ [RH:RL], RZ
@endmacro

@define PARAM0 0xAA00
@define PARAM1 0xAA04
@define PARAM2 0xAA08
@define PARAM3 0xAA0C
@define PARAM4 0xAA10
@define PARAM5 0xAA14
@define PARAM6 0xAA18
@define PARAM7 0xAA1C

@define RETVAL 0xAA20


@macro PSET8 param, source
SWI [{param + 3}], source
@endmacro

@macro PGET8 param, target
LWI target, [{param + 3}]
@endmacro

@macro PSET16 param, sourceH, sourceL
SWI [{param + 2}], sourceH
SWI [{param + 3}], sourceL
@endmacro

@macro PGET16 param, targetH, targetL
LWI targetH, [{param + 2}]
LWI targetL, [{param + 3}]
@endmacro