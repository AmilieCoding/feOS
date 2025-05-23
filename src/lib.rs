#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;

use core::panic::PanicInfo;

// -> Handles tests when we are testing.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// -> Prints test statements automatically.
pub trait Testable{
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[PASSED]");
    }
}

// -> Custom testing framework.
// -> Cannot lock behind "#[cfg(test)]" -> Rust has a panic attack.
// #[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} tests", tests.len());
    for test in tests{
        test.run();
    }
    // -> Exits QEMU with a success code!
    exit_qemu(QemuExitCode::Success);
}

// -> Handles panics in the case of a test.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// -> Actually where the test begin as we can't use main
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

// -> Exitting QEMU cleanly and easily bc fuck APM and ACPI x3
// -> This just specifies the exit code if it's successful or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// -> Creates a port at 0xf4 then passes the exit code to said port.
pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn init(){
    interrupts::init_idt();
}

#[test_case]
fn test_breakpoint_exception(){
    x86_64::instructions::interrupts::int3();
}