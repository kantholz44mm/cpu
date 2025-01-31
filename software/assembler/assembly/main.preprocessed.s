






































































ADDINF R1, RZ, 0
ADDINF R2, RZ, 0
ADDINF R3, RZ, 0
ADDINF R4, RZ, 1


fib_loop:
ADD RL, R2, R4
ADC RH, R1, R3

    
    
    ANDINF RF, RF, 0x4
    BNZI [done], RF
ADDNF R1, RZ, R3
ADDNF R2, RZ, R4
ADDNF R3, RZ, RH
ADDNF R4, RZ, RL
ADDINF RH, RZ, 0xFF

LWI  RL, [0xFEFF]
SW [RH:RL], R3
ADDINF RL, RL, 1

SW [RH:RL], R4
ADDINF RL, RL, 1

SWI [0xFEFF], RL


    BZI [fib_loop], RZ

done:
    HCF

not_quite_done:
ADDINF R4, RZ, {(($ + 11) >> 0) & 0xFF}
ADDINF R5, RZ, {(($ + 10) >> 8) & 0xFF}
ADDINF RH, RZ, 0xFF

LWI  RL, [0xFEFF]
SW [RH:RL], R5
ADDINF RL, RL, 1

SW [RH:RL], R4
ADDINF RL, RL, 1

SWI [0xFEFF], RL
ADDINF RL, RZ, {(still_not_done >> 0) & 0xFF}
ADDINF RH, RZ, {(still_not_done >> 8) & 0xFF}

BZ [RH:RL], RZ
ADDINF RH, RZ, 0xFF

LWI  RL, [0xFEFF]
SUBINF RL, RL, 1

LW R4, [RH:RL]
SUBINF RL, RL, 1

LW R5, [RH:RL]
SWI [0xFEFF], RL
ADDNF RH, RZ, R5
ADDNF RL, RZ, R4

BZ [RH:RL], RZ


still_not_done:
ADDINF RH, RZ, 0xFF

LWI  RL, [0xFEFF]
SUBINF RL, RL, 1

LW R4, [RH:RL]
SUBINF RL, RL, 1

LW R5, [RH:RL]
SWI [0xFEFF], RL
ADDNF RH, RZ, R5
ADDNF RL, RZ, R4

BZ [RH:RL], RZ






fibonacci:
LWI R1, [{0xAA00 + 2}]
LWI R2, [{0xAA00 + 3}]
LWI R3, [{0xAA04 + 2}]
LWI R4, [{0xAA04 + 3}]


    
ADD RL, R2, R4
ADC RH, R1, R3

SWI [{0xAA20 + 2}], RH
SWI [{0xAA20 + 3}], RL
ADDINF RH, RZ, 0xFF

LWI  RL, [0xFEFF]
SUBINF RL, RL, 1

LW R4, [RH:RL]
SUBINF RL, RL, 1

LW R5, [RH:RL]
SWI [0xFEFF], RL
ADDNF RH, RZ, R5
ADDNF RL, RZ, R4

BZ [RH:RL], RZ

