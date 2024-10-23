
; the stack grows upwards, starting at 0xFF00.
; [SP] contains the lower byte of the stackpointer.
@define SP 0xFEFF

@macro PUSH %0
MOV RH, 0xFF
LW  RL, [SP]
SW [RH:RL], %0
INC RL
SW [SP], RL
@endmacro

@macro POP %0
MOV RH, 0xFF
LW  RL, [SP]
DEC RL
SW [SP], RL
LW %0, [RH:RL]
@endmacro

@macro PEEK %0
MOV RH, 0xFF
LW  RL, [SP]
LW %0, [RH:RL]
@endmacro

@macro PUSH16 %0, %1
PUSH %0
PUSH %1
@endmacro

@macro POP16 %0, %1
POP %1
POP %0
@endmacro

@macro PEEK16 %0, %1
MOV RH, 0xFF
LW  RL, [SP]
DEC RL
LW %1, [RH:RL]
DEC RL
LW %0, [RH:RL]
@endmacro
