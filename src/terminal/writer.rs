use core::fmt;

use spin::{Mutex, Once};

use super::{
    color::{Color, ColorCode},
    vga_buffer::{VGABuffer, VGAChar, BUFFER_HEIGHT, BUFFER_WIDTH},
};

static STD_WRITER: Once<Mutex<Writer>> = Once::new();
static ERR_WRITER: Once<Mutex<Writer>> = Once::new();

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: VGABuffer,
}

impl Writer {
    pub fn std_global() -> spin::MutexGuard<'static, Writer> {
        STD_WRITER
            .call_once(|| Mutex::new(Writer::new(ColorCode::default(), VGABuffer::new())))
            .lock()
    }

    pub fn err_global() -> spin::MutexGuard<'static, Writer> {
        ERR_WRITER
            .call_once(|| {
                Mutex::new(Writer::new(
                    ColorCode::new(Color::Red, Color::Black),
                    VGABuffer::new(),
                ))
            })
            .lock()
    }

    fn new(color_code: ColorCode, buffer: VGABuffer) -> Self {
        Writer {
            column_position: 0,
            color_code,
            buffer,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.set_char(
                    col,
                    row,
                    VGAChar {
                        ascii_character: byte,
                        color_code: self.color_code,
                    },
                );
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.get_byte(col, row).expect("couldn't get byte");
                self.buffer.set_char(col, row - 1, char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = VGAChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.set_char(col, row, blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::terminal::writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Writer::std_global().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::terminal::writer::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    use core::fmt::Write;
    Writer::err_global().write_fmt(args).unwrap();
}
