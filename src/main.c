int fib(int n) {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

// Tell GCC to put this in a special ".init" section
__attribute__((section(".init"))) void _start() {
    volatile int result = fib(10);
    while(1) {} 
}