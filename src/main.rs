// -> This has been made with the help of <https://os.phil-opp.com/>
// -> Really cool guides, that explain the process very well.

// -> Disable linking to the standard linker and removes the main (makes it "freestanding"!)
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// -> Tell tests to use OUR test main. We disabled main remember!
#![reexport_test_harness_main = "test_main"]

// -> All imports go here. Please see their relevant files.
mod vga_buffer;
mod serial;

// -> Handles panics in the case of a panic. (When we aren't testing.)
use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

// -> This is for when we ARE testing!
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// -> Can't use main, so here's where we start instead. (This is the entry point)
#[unsafe(no_mangle)] // -> We really shouldn't mangle this. That'd be silly as fuck.
pub extern "C" fn _start() -> ! {

    // We can use println now! How cool!
    println!("This is the feOS VGA buffer!");
    println!("It supports multi line output too{}", "!");

    // -> This invokes the tests. Pretty simple really.
    #[cfg(test)]
    test_main();

    // -> In case we want a user invoked panic for testing.
    //panic!("[PANIC] - Invoked by user.");

    // -> If this isn't infinitely running that's BAD. It will return/run into undefined memory.
    // -> That's a fuckin deathwish if it does.
    loop{}

}

// -> Custom testing framework.
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} tests", tests.len());
    for test in tests{
        test.run();
    }
    // -> Exits QEMU with a success code!
    exit_qemu(QemuExitCode::Success);
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

// -> Prints test statements automatically.
pub trait Testable{
    fn run(&self) -> ();
}

// -> Exitting QEMU cleanly and easily bc fuck APM and ACPI x3
// -> THis just specifies the exit code if it's successful or not.
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

// -> Example test case named "Trivial Assertion"
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}