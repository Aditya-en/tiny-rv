.section .text
.global _start

_start:
    # fib(10) iteratively
    # a = fib(n-1), b = fib(n), counter in x5
    
    addi x10, x0, 0      # a = 0 (fib(0))
    addi x11, x0, 1      # b = 1 (fib(1))
    addi x5,  x0, 10     # counter = 10

loop:
    beq  x5, x0, done    # if counter == 0, done
    add  x12, x10, x11   # tmp = a + b
    addi x10, x11, 0     # a = b
    addi x11, x12, 0     # b = tmp
    addi x5,  x5, -1     # counter--
    beq  x0, x0, loop    # jump back (unconditional)

done:
    # x10 now holds fib(10) = 55
    beq  x0, x0, done    # halt