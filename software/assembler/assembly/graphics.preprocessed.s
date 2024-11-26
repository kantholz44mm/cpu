











































ADC* RZ, RZ, RZ
XOR* R1, RZ, 50
XOR* R2, RZ, 50

start_line:
XOR* R3, RZ, 150
XOR* R1, RZ, 50
line_loop:
XOR* RF, RZ, 0
SBB RZ, R3, R1

    
    AND* RF, RF, 0x1
    BNZ next_line, RF

    
XOR* RH, RZ, R2
XOR* RL, RZ, R1
XOR* RF, RZ, 0xFF
    SW [RH:RL], RF
XOR* RF, RZ, 0
ADC R1, R1, 1

    BZ line_loop, RZ

next_line:
XOR* R3, RZ, 100
XOR* RF, RZ, 0
SBB RZ, R3, R2
    AND* RF, RF, 0x1
    BNZ done, RF
XOR* RF, RZ, 0
ADC R2, R2, 1
    BZ start_line, RZ

done:
    HCF