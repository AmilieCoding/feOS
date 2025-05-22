use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

// -> Creates a static writer instance, like in VGA buffer. Init is called EXACTLY ONCE. No more.
lazy_static!{
    pub static ref SERIAL1: Mutex<SerialPort>={
        // -> Pass port 0x3F8 which is the standard port for the first serial interface.
        let mut serial_port = unsafe {SerialPort::new(0x3F8)};
        serial_port.init();
        Mutex::new(serial_port)
    };
}

// -> In case the serial fucker fails. In that case I guess I can just CRY.
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

// -> For printing to the host via the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

// -> While appending new line, it prints to host through serial.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}