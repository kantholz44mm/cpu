
; the stack grows upwards, starting at 0xFF00.
; [SP] contains the lower byte of the stackpointer.
@define SP 0xFEFF

@macro PUSH %0
MOVI RH, 0xFF
LWI  RL, [SP]
SW [RH:RL], %0
INCNF RL
SWI [SP], RL
@endmacro

@macro POP %0
MOVI RH, 0xFF
LWI  RL, [SP]
DECNF RL
SWI [SP], RL
LW %0, [RH:RL]
@endmacro

@macro PEEK %0
MOVI RH, 0xFF
LWI  RL, [SP]
LW %0, [RH:RL]
@endmacro

@macro PUSH16 %0, %1
MOVI RH, 0xFF
LWI  RL, [SP]
SW [RH:RL], %0
INCNF RL
SW [RH:RL], %1
INCNF RL
SWI [SP], RL
@endmacro

@macro POP16 %0, %1
MOVI RH, 0xFF
LWI  RL, [SP]
DECNF RL
LW %1, [RH:RL]
DECNF RL
LW %0, [RH:RL]
SWI [SP], RL
@endmacro

@macro PEEK16 %0, %1
MOVI RH, 0xFF
LWI  RL, [SP]
DECNF RL
LW %1, [RH:RL]
DECNF RL
LW %0, [RH:RL]
@endmacro

@macro PEEKAT %dest, %offset
MOVI RH, 0xFF
LWI  RL, [SP]
SUBINF RL, RL, %offset
LW %dest, [RH:RL]
@endmacro
