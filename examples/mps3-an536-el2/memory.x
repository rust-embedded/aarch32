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

PROVIDE(_hyp_stack_size = 16K);
PROVIDE(_und_stack_size = 8);
PROVIDE(_svc_stack_size = 8);
PROVIDE(_abt_stack_size = 8);
PROVIDE(_irq_stack_size = 8);
PROVIDE(_fiq_stack_size = 8);
PROVIDE(_sys_stack_size = 8);

PROVIDE(_num_cores = 2);

EXTERN(_unexpected_undefined_exception_handler);
EXTERN(_unexpected_svc_exception_handler);
EXTERN(_unexpected_hvc_exception_handler);
EXTERN(_unexpected_prefetch_abort_exception_handler);
EXTERN(_unexpected_data_abort_exception_handler);
EXTERN(_unexpected_irq_exception_handler);

PROVIDE(_undefined_handler      = _unexpected_undefined_exception_handler);
PROVIDE(_svc_handler            = _unexpected_svc_exception_handler);
PROVIDE(_hvc_handler            = _unexpected_hvc_exception_handler);
PROVIDE(_prefetch_abort_handler = _unexpected_prefetch_abort_exception_handler);
PROVIDE(_data_abort_handler     = _unexpected_data_abort_exception_handler);
PROVIDE(_irq_handler            = _unexpected_irq_exception_handler);
