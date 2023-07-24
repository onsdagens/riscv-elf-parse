.section .init, "ax"
.global _start

_start:
    lw t0, 0(t0)
    sw t1, 0(t1)

.section .some_section , "ax"
.global _some_symbol

_some_symbol:
    lw t0, 0(t0)
    sw t1, 0(t1)

