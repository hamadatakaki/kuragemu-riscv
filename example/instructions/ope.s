initialize:
    addi    a3, x0, 0x1c    # 00
    addi    a1, x0, 1       # 04
    sw      a1, 0(a3)       # 08
    addi    a1, x0, 2       # 0c
    sw      a1, -4(a3)      # 10
    addi    a1, x0, 5       # 14
    sw      a1, -8(a3)      # 18
    addi    a1, x0, 6       # 1c
    sw      a1, -c(a3)      # 20
    addi    a1, x0, 7       # 24
    sw      a1, -10(a3)     # 28
    addi    a1, x0, 3       # 2c
    sw      a1, -14(a3)     # 30
    addi    a1, x0, 4       # 34
    sw      a1, -18(a3)     # 38
    addi    a1, x0, 8       # 3c
    sw      a1, -1c(a3)     # 40
    addi    a3, x0, 0       # 44
insert_sort:
    addi    a4, x0, 1       # 48
outer_loop:
    bltu    a4, a1, outer_loop_2   # 4c (+8)
exit_loop:
    j       end             # 50 (+40)
outer_loop_2:
    lw      a6, 0(a3)       # 54
    addi    a2, a3, 0       # 58
    addi    a5, a4, 0       # 5c
inner_loop:
    lw      a7, -4(a2)      # 60
    bge     a6, a7, exit_inner_loop # 64 (+14)
    sw      a7, 0(a2)       # 68
    addi    a5, a5, -1      # 6c
    addi    a2, a2, -4      # 70
    bne     a5, x0, inner_loop      # 74 (-14)
exit_inner_loop:
    slli    a5, a5, 2       # 78
    add     a5, a0, a5      # 7c
    sw      a6, 0(a5)       # 80
    addi    a4, a4, 1       # 84
    addi    a3, a3, 4       # 88
    j       outer_loop      # 8c (-40)
end:
    jal     x0, 0           # 90
