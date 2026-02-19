target extended-remote :1234
break kmain
break _asm_undefined_handler
break _asm_svc_handler
break _asm_prefetch_abort_handler
break _asm_data_abort_handler
break _asm_irq_handler
break _asm_fiq_handler
layout asm
layout regs
set logging file ./target/debug.log
set logging enabled on
stepi
