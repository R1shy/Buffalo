use lazy_static::lazy_static;
use x86_64::{registers::rflags::RFlags, structures::idt::{InterruptDescriptorTable, InterruptStackFrame}};

use crate::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(faultfault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{

    let flags: RFlags = stack_frame.cpu_flags;
    log::warn!("EXCEPTION: BREAKPOINT\nFLAGS: {:#?}", flags);


}

extern "x86-interrupt" fn faultfault_handler(
    stack_frame: InterruptStackFrame,
    code: u64
    ) -> !
{
    panic!("DOUBLE FAULT!")
}
