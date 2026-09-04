/*
Memory configuration for the Xilinx Zynq-7000 (Arm Cortex-A9), as emulated by
the QEMU `xilinx-zynq-a9` machine.

DDR SDRAM lives at 0x0000_0000. QEMU maps 128 MiB there by default (memory
region `zynq.ext_ram`, 0x0000_0000 .. 0x07FF_FFFF).

See https://github.com/qemu/qemu/blob/master/hw/arm/xilinx_zynq.c
*/

MEMORY {
    DDR : ORIGIN = 0x00000000, LENGTH = 128M
}

REGION_ALIAS("VECTORS", DDR);
REGION_ALIAS("CODE", DDR);
REGION_ALIAS("DATA", DDR);
REGION_ALIAS("STACKS", DDR);

PROVIDE(_hyp_stack_size = 16K);
PROVIDE(_und_stack_size = 16K);
PROVIDE(_svc_stack_size = 16K);
PROVIDE(_abt_stack_size = 16K);
PROVIDE(_irq_stack_size = 64);
PROVIDE(_fiq_stack_size = 64);
PROVIDE(_sys_stack_size = 16K);

/* This is a dual-core Cortex-A9, so reserve a set of stacks per core. */
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
