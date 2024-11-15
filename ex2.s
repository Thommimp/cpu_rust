main:
        addi    s0, zero, 64
        addi    s1, zero, 256
        addi    sp, sp, -16
        sw      s0, 12(sp)
        sw      s1, 8(sp)
        mv      t6, zero
        add     t3, a2, a1
        slli    a6, a1, 2
        lui     a3, 16
        addi    a7, a3, -256
        lui     t0, 4080
        ecall
