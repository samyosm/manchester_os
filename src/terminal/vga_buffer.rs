use super::color::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct VGABuffer(pub &'static mut [[volatile::Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]);

impl VGABuffer {
    pub fn new() -> Self {
        VGABuffer(unsafe { &mut *(0xb8000 as *mut [[volatile::Volatile<VGAChar>; 80]; 25]) })
    }

    pub fn set_char(&mut self, x: usize, y: usize, char: VGAChar) {
        if x > BUFFER_WIDTH || y > BUFFER_HEIGHT {
            panic!("can't set character: out of bounds")
        }

        self.0[y][x].write(char);
    }

    pub fn get_byte(&self, x: usize, y: usize) -> Result<VGAChar, ()> {
        if x > BUFFER_WIDTH || y > BUFFER_HEIGHT {
            return Err(());
        }

        Ok(self.0[y][x].read())
    }
}
