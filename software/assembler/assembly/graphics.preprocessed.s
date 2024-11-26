















































ADC* RZ, RZ, RZ
XOR* R1, RZ, 50
XOR* R2, RZ, 42
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
SW [RH:RL], R1
XOR* RF, RZ, 0
ADC RL, RL, 1
SW [0xFEFF], RL
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
SW [RH:RL], R2
XOR* RF, RZ, 0
ADC RL, RL, 1
SW [0xFEFF], RL
LA $(1)
BZ my_cool_function, RZ

HCF








































my_cool_function:
XOR* R3, RZ, RH
XOR* R4, RZ, RL
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
XOR* RF, RZ, 0
SBB RL, RL, 1
SW [0xFEFF], RL
LW R1, [RH:RL]
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
XOR* RF, RZ, 0
SBB RL, RL, 1
SW [0xFEFF], RL
LW R2, [RH:RL]
XOR* RF, RZ, 0
ADC R1, R1, R2
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
SW [RH:RL], R1
XOR* RF, RZ, 0
ADC RL, RL, 1
SW [0xFEFF], RL
XOR* RH, RZ, R3
XOR* RL, RZ, R4
BZ RH:RL, RZ
