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

PROVIDE(_hyp_stack_size = 1M);
PROVIDE(_und_stack_size = 1M);
PROVIDE(_svc_stack_size = 1M);
PROVIDE(_abt_stack_size = 1M);
PROVIDE(_irq_stack_size = 1M);
PROVIDE(_fiq_stack_size = 1M);
PROVIDE(_sys_stack_size = 1M);
