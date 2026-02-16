/*
Memory configuration for the MPS3-AN536 machine.

See https://github.com/qemu/qemu/blob/master/hw/arm/mps3r.c
*/

MEMORY {
    QSPI : ORIGIN = 0x08000000, LENGTH = 8M
    BRAM : ORIGIN = 0x10000000, LENGTH = 512K
    DDR  : ORIGIN = 0x20000000, LENGTH = 1536M
}

REGION_ALIAS("VECTORS", QSPI);
REGION_ALIAS("CODE", QSPI);
REGION_ALIAS("DATA", BRAM);
REGION_ALIAS("STACKS", BRAM);

SECTIONS {
    /* ### Interrupt Handler Entries
     *
     * The IRQ handler walks this section to find registered
     * interrupt handlers
     */
    .irq_entries : ALIGN(4)
    {
        /* We put this in the header */
        __irq_entries_start = .;
        /* Here are the entries */
        KEEP(*(.irq_entries));
        /* Keep this block a nice round size */
        . = ALIGN(4);
        /* We put this in the header */
        __irq_entries_end = .;
    } > CODE
} INSERT AFTER .text;

SECTIONS {
    .core1_stacks (NOLOAD) : ALIGN(8)
    {
        . = ALIGN(8);
        _core1_stacks_low_end = .;
        _core1_sys_stack_end = .;
        . += _sys_stack_size;
        . = ALIGN(8);
        _core1_sys_stack = .;
        _core1_fiq_stack_end = .;
        . += _fiq_stack_size;
        . = ALIGN(8);
        _core1_fiq_stack = .;
        _core1_irq_stack_end = .;
        . += _irq_stack_size;
        . = ALIGN(8);
        _core1_irq_stack = .;
        _core1_abt_stack_end = .;
        . += _abt_stack_size;
        . = ALIGN(8);
        _core1_abt_stack = .;
        _core1_svc_stack_end = .;
        . += _svc_stack_size;
        . = ALIGN(8);
        _core1_svc_stack = .;
        _core1_und_stack_end = .;
        . += _und_stack_size;
        . = ALIGN(8);
        _core1_und_stack = .;
        _core1_hyp_stack_end = .;
        . += _hyp_stack_size;
        . = ALIGN(8);
        _core1_hyp_stack = .;
        _core1_stacks_high_end = .;
    } > STACKS
} INSERT BEFORE .filler;

PROVIDE(kmain2 = default_kmain2);

PROVIDE(_hyp_stack_size = 16K);
PROVIDE(_und_stack_size = 16K);
PROVIDE(_svc_stack_size = 16K);
PROVIDE(_abt_stack_size = 16K);
PROVIDE(_irq_stack_size = 64);
PROVIDE(_fiq_stack_size = 64);
PROVIDE(_sys_stack_size = 16K);
