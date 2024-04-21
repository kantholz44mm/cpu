        NOP
        NOP

someshit: ; this will be hit once only
    EQ> LWI r1 [0x55]
    Z>  LWI r2 [0x44]
        JPI [someshit]
someothershit: