/*
Memory configuration for the Arm Versatile Peripheral Board.

See https://github.com/qemu/qemu/blob/master/hw/arm/versatilepb.c
*/

MEMORY {
    SDRAM : ORIGIN = 0, LENGTH = 128M
}

REGION_ALIAS("VECTORS", SDRAM);
REGION_ALIAS("CODE", SDRAM);
REGION_ALIAS("DATA", SDRAM);
REGION_ALIAS("STACKS", SDRAM);

PROVIDE(_hyp_stack_size = 16K);
PROVIDE(_und_stack_size = 16K);
PROVIDE(_svc_stack_size = 16K);
PROVIDE(_abt_stack_size = 16K);
PROVIDE(_irq_stack_size = 64);
PROVIDE(_fiq_stack_size = 64);
PROVIDE(_sys_stack_size = 16K);

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

