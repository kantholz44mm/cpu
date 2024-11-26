








































ADC* RZ, RZ, RZ


XOR* R1, RZ, 0
XOR* R2, RZ, 0
XOR* R3, RZ, 0
XOR* R4, RZ, 1


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
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
SW [RH:RL], R3
XOR* RF, RZ, 0
ADC RL, RL, 1
SW [0xFEFF], RL
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
SW [RH:RL], R4
XOR* RF, RZ, 0
ADC RL, RL, 1
SW [0xFEFF], RL

fib_loop:
    
XOR* RF, RZ, 0
ADC RL, R2, R4
ADC RH, R1, R3
XOR* R1, RZ, R3
XOR* R2, RZ, R4
XOR* R3, RZ, RH
XOR* R4, RZ, RL

    
    AND* RF, RF, 0x4
    BNZ done, RF
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
SW [RH:RL], R3
XOR* RF, RZ, 0
ADC RL, RL, 1
SW [0xFEFF], RL
XOR* RH, RZ, 0xFF
LW  RL, [0xFEFF]
SW [RH:RL], R4
XOR* RF, RZ, 0
ADC RL, RL, 1
SW [0xFEFF], RL

    BZ fib_loop, RZ

done:
    HCF

