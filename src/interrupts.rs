use spin::Once;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::eprintln;

static IDT: Once<InterruptDescriptorTable> = Once::new();

pub fn init_idt() {
    IDT.call_once(|| {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    })
    .load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    eprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

