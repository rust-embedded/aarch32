/*
Basic AArch32 linker script.

You must supply a file called `memory.x` which defines the memory regions
'VECTORS', 'CODE', 'DATA', 'STACKS'.

The stacks will be at the top of the STACKS region by default, use `_pack_stacks`
to overwrite default behaviour.

Based upon the linker script from https://github.com/rust-embedded/cortex-m
*/

INCLUDE memory.x

ENTRY(_start);
EXTERN(_vector_table);
EXTERN(_start);
EXTERN(_default_handler);

SECTIONS {
    .vector_table ORIGIN(VECTORS) : {
        /* The vector table must come first */
        *(.vector_table)
    } > VECTORS

    .text : {
        /* Now the rest of the code */
        *(.text .text*)
    } > CODE

    .rodata : {
        *(.rodata .rodata*)
    } > CODE

    .data : ALIGN(4) {
        . = ALIGN(4);
        __sdata = .;
        *(.data .data.*);
        . = ALIGN(4);
    } > DATA AT>CODE
    /*
     * Allow sections from user `memory.x` injected using `INSERT AFTER .data` to
     * use the .data loading mechanism by pushing __edata. Note: do not change
     * output region or load region in those user sections!
     */
    . = ALIGN(4);
    __edata = .;

    /* LMA of .data */
    __sidata = LOADADDR(.data);

    .bss (NOLOAD) : ALIGN(4) {
        . = ALIGN(4);
        __sbss = .;
        *(.bss .bss* COMMON)
        . = ALIGN(4);
    } > DATA
    /*
     * Allow sections from user `memory.x` injected using `INSERT AFTER .bss` to
     * use the .bss zeroing mechanism by pushing __ebss. Note: do not change
     * output region or load region in those user sections!
     */
    __ebss = .;

    .uninit (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);
        __suninit = .;
        *(.uninit .uninit.*);
        . = ALIGN(4);
        __euninit = .;
    } > DATA

    .filler (NOLOAD) : {
        /* Move the .stacks section to the end of the STACKS memory region */
        _next_region = ORIGIN(STACKS) + LENGTH(STACKS);
        _start_moved_stacks = _next_region - SIZEOF(.stacks);
        _start_stacks = _pack_stacks ? . : _start_moved_stacks;
        FILL(0x00)
        . = _start_stacks;
    } > STACKS

    .stacks (NOLOAD) : ALIGN(8)
    {
        . = ALIGN(8);
        _stacks_low_end = .;
        _sys_stack_end = .;
        . += _sys_stack_size;
        . = ALIGN(8);
        _sys_stack = .;
        _fiq_stack_end = .;
        . += _fiq_stack_size;
        . = ALIGN(8);
        _fiq_stack = .;
        _irq_stack_end = .;
        . += _irq_stack_size;
        . = ALIGN(8);
        _irq_stack = .;
        _abt_stack_end = .;
        . += _abt_stack_size;
        . = ALIGN(8);
        _abt_stack = .;
        _svc_stack_end = .;
        . += _svc_stack_size;
        . = ALIGN(8);
        _svc_stack = .;
        _und_stack_end = .;
        . += _und_stack_size;
        . = ALIGN(8);
        _und_stack = .;
        _hyp_stack_end = .;
        . += _hyp_stack_size;
        . = ALIGN(8);
        _hyp_stack = .;
        _stacks_high_end = .;
    } > STACKS

    /DISCARD/ : {
        *(.note .note*)
    }
}

/* We provide default sizes for the stacks to be overwritten in memory.x */
PROVIDE(_stack_top = _stacks_high_end); /* deprecated, use _xxx_stack labels as defined in .stacks section */
PROVIDE(_hyp_stack_size = 0x400);
PROVIDE(_und_stack_size = 0x400);
PROVIDE(_svc_stack_size = 0x400);
PROVIDE(_abt_stack_size = 0x400);
PROVIDE(_irq_stack_size = 0x400);
PROVIDE(_fiq_stack_size = 0x400);
PROVIDE(_sys_stack_size = 0x2000);
PROVIDE(_pack_stacks = 0); /* set this to 1 to remove the filler section pushing the stacks to the end of STACKS. */


/* Weak aliases for ASM default handlers */
PROVIDE(_start                      = _default_start);
PROVIDE(_asm_undefined_handler      = _asm_default_undefined_handler);
PROVIDE(_asm_svc_handler            = _asm_default_svc_handler);
PROVIDE(_asm_prefetch_abort_handler = _asm_default_prefetch_abort_handler);
PROVIDE(_asm_data_abort_handler     = _asm_default_data_abort_handler);
PROVIDE(_asm_irq_handler            = _asm_default_irq_handler);
PROVIDE(_asm_fiq_handler            = _asm_default_fiq_handler);

/* Weak aliases for C default handlers */
PROVIDE(_undefined_handler      = _default_handler);
PROVIDE(_svc_handler            = _default_handler);
PROVIDE(_prefetch_abort_handler = _default_handler);
PROVIDE(_data_abort_handler     = _default_handler);
PROVIDE(_irq_handler            = _default_handler);
/* There is no default C-language FIQ handler */
