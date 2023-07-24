.section .init, "ax"
.global _start

_start:
    lw x0, 0(x0)
    sw x1, 0(x1)

.section .some_section , "ax"
.global _some_symbol

_some_symbol:
    lw x0, 0(x0)
    sw x1, 0(x1)

