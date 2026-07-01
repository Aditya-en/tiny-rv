ENTRY(_start)

SECTIONS
{
    . = 0x0;

    .text : { 
        /* Force the .init section to be the very first bytes */
        *(.init)
        /* Then the rest of the normal code (like fib) */
        *(.text) 
    }
    
    .rodata : { *(.rodata) }
    .data : { *(.data) }
    .bss : { *(.bss) }
}