/*
Memory configuration for the MPS3-AN536 machine.

See https://github.com/qemu/qemu/blob/master/hw/arm/mps3r.c
*/

MEMORY {
    QSPI : ORIGIN = 0x08000000, LENGTH = 8M
    DDR  : ORIGIN = 0x20000000, LENGTH = 128M
}

REGION_ALIAS("VECTORS", QSPI);
REGION_ALIAS("CODE", QSPI);
REGION_ALIAS("DATA", DDR);
REGION_ALIAS("STACKS", DDR);

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


PROVIDE(_hyp_stack_size = 1M);
PROVIDE(_und_stack_size = 1M);
PROVIDE(_svc_stack_size = 1M);
PROVIDE(_abt_stack_size = 1M);
PROVIDE(_irq_stack_size = 1M);
PROVIDE(_fiq_stack_size = 1M);
PROVIDE(_sys_stack_size = 1M);
