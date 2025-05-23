// -> This has been made with the help of <https://os.phil-opp.com/>
// -> Really cool guides, that explain the process very well.

// -> Disable linking to the standard linker and removes the main (makes it "freestanding"!)
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(feos::test_runner)]

// -> Tell tests to use OUR test main. We disabled main remember!
#![reexport_test_harness_main = "test_main"]

// -> All imports go here. Please see their relevant files.
mod vga_buffer;
mod serial;

// -> Handles panics in the case of a panic. (When we aren't testing.)
use core::panic::PanicInfo;

// -> Can't use main, so here's where we start instead. (This is the entry point)
#[unsafe(no_mangle)] // -> We really shouldn't mangle this. That'd be silly as fuck.
pub extern "C" fn _start() -> ! {

    // We can use println now! How cool!

    // -> Add our interrupts handler.
    println!("[KERNEL] Initialised interrupt handler successfully.");
    feos::init();

    // -> This invokes the tests. Pretty simple really.
    #[cfg(test)]
    test_main();
    println!("[KERNEL] Test cases initialised successfully.");

    println!("This is the feOS VGA buffer!");
    println!("It supports multi line output too{}", "!");

    // -> In case we want a user invoked panic for testing.
    //panic!("[PANIC] - Invoked by user.");

    // -> If this isn't infinitely running that's BAD. It will return/run into undefined memory.
    // -> That's a fuckin deathwish if it does.
    loop{}

}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    feos::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// -> Example test case named "Trivial Assertion"
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}