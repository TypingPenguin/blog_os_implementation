use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s); // Call the write_string method
        Ok(())
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

pub fn print_something(){
    use core::fmt::Write;
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}, // A mutable reference to a static variable
    };
    writer.write_byte(b'H');
    writer.write_string("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // Ensure that the struct has the same memory layout as its single field
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8)) // Bitwise OR
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // Ensure that the struct's fields are laid out exactly like in C
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25; // Number of rows
const BUFFER_WIDTH: usize = 80; // Number of columns

use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],}

pub struct Writer {
    column_position: usize, // Current column
    color_code: ColorCode,
    buffer: &'static mut Buffer, // A mutable reference to a static variable
}

impl Writer {
    pub fn write_string(&mut self, s:&str){
        for byte in s.bytes(){
            match byte{
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // ■
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte{
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH { // If we are at the end of the row
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1; // The last row
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1; // Increment the column position
            }

        }
    }
    fn new_line(&mut self) {/* TODO */}
}