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
