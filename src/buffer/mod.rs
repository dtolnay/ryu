use core::{mem, str};

use pretty;

#[derive(Copy, Clone)]
pub struct Buffer {
    bytes: [u8; 24],
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            bytes: unsafe { mem::uninitialized() },
        }
    }

    pub fn format<F: Float>(&mut self, f: F) -> &str {
        f.write_to_ryu_buffer(self)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer::new()
    }
}

pub trait Float: Sealed {
    #[doc(hidden)]
    fn write_to_ryu_buffer(self, buffer: &mut Buffer) -> &str;
}

impl Float for f32 {
    fn write_to_ryu_buffer(self, buffer: &mut Buffer) -> &str {
        unsafe {
            let n = pretty::f2s_buffered_n(self, &mut buffer.bytes[0]);
            str::from_utf8_unchecked(&buffer.bytes[..n])
        }
    }
}

impl Float for f64 {
    fn write_to_ryu_buffer(self, buffer: &mut Buffer) -> &str {
        unsafe {
            let n = pretty::d2s_buffered_n(self, &mut buffer.bytes[0]);
            str::from_utf8_unchecked(&buffer.bytes[..n])
        }
    }
}

pub trait Sealed {}
impl Sealed for f32 {}
impl Sealed for f64 {}
