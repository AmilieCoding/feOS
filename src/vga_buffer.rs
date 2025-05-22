// -> We gotta use this to stop future compilers omitting our shit!
use volatile::Volatile;
// -> For the writing utility :3
use core::fmt;
// -> Makes the global WRITER actually work!
use lazy_static::lazy_static;
// -> We need inferior mutability thats safe for... you guessed it! The WRITER!
use spin::Mutex;

// -> Stops the compiler whining about unused enums.
#[allow(dead_code)]
// -> This makes it printable and comparable - Something called "Copy Semantics".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// -> We represent the colors using an enum (kinda like C!).
// -> Storing as "u4" would be sufficient, but Rust doesn't have a "u4", only a "u8".
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// -> Ensures ColorCode has the exact same layout as u8.
#[repr(transparent)]
struct ColorCode(u8);
// -> Contains the full code for ColorCode.
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// -> This is going to represent the screen character and the buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_character: u8,
    color_code: ColorCode,
}

// -> This sets our buffer constraints. (Good to know!)
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer{
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

// -> This type means we can actually write to the screen.
// -> We tell the program this is valid for the entire time it is running.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer{
    // -> For writing full strings, also has "unprintable byte management".
    pub fn write_string(&mut self, s: &str){
        for byte in s.bytes(){
            match byte{
                // -> Ln1 for a printable byte or new line. Ln2 for a non printable byte.
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }

        }
    }

    // -> This writes a **single** byte to the screen buffer.
    pub fn write_byte(&mut self, byte: u8){
        match byte{
            // -> Moves to a new line if the byte is a newline character.
            b'\n' => self.new_line(),
            // -> If it's a normal character (here), and at the end of a line, go to a new one.
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                // -> Always writing to the screen bottom!
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                // -> Actually write the character and colorcode to the buffer.
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    // -> Waow! This lets us have new lines. !!Very important!!
    fn new_line(&mut self){
        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // -> This is the clear row utility. If you don't understand this, why are you looking at this code?
    fn clear_row(&mut self, row: usize){
        let blank = ScreenChar{
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// -> This lets us support Rusts formatting macros!! Very good very nice :)
impl fmt::Write for Writer{
    fn write_str(&mut self, s: &str) -> fmt::Result{
        self.write_string(s);
        Ok(())
    }
}

// -> This is for the global writer so we don't need to carry a Writer instance around.
lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    });
}

// -> We want macros right? WE LOVE MACROS!!! WOOOOOOOOOOO!!!!!!!!!
// -> Macro export makes them avaliable everywhere in our crate.
// -> Use crate::println (and similar) if we wanna use them from now on.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
