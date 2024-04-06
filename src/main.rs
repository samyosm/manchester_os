#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

mod gdt;
pub mod interrupts;
pub mod terminal;

use core::panic::PanicInfo;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{}", info);
    loop {}
}

fn init() {
    gdt::init_gdt();
    interrupts::init_idt();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome!");

    init();

    #[cfg(test)]
    test_main();

    loop {}
}
