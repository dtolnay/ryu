use core::{mem, str};

use pretty;

#[derive(Copy, Clone)]
pub struct Buffer {
    bytes: [u8; 24],
}

impl Buffer {
    #[inline]
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
    #[inline]
    fn default() -> Self {
        Buffer::new()
    }
}

pub trait Float: Sealed {
    #[doc(hidden)]
    fn write_to_ryu_buffer(self, buffer: &mut Buffer) -> &str;
}

impl Float for f32 {
    #[inline]
    fn write_to_ryu_buffer(self, buffer: &mut Buffer) -> &str {
        unsafe {
            let n = pretty::f2s_buffered_n(self, &mut buffer.bytes[0]);
            debug_assert!(n <= buffer.bytes.len());
            let slice = buffer.bytes.get_unchecked(..n);
            str::from_utf8_unchecked(slice)
        }
    }
}

impl Float for f64 {
    #[inline]
    fn write_to_ryu_buffer(self, buffer: &mut Buffer) -> &str {
        unsafe {
            let n = pretty::d2s_buffered_n(self, &mut buffer.bytes[0]);
            debug_assert!(n <= buffer.bytes.len());
            let slice = buffer.bytes.get_unchecked(..n);
            str::from_utf8_unchecked(slice)
        }
    }
}

pub trait Sealed {}
impl Sealed for f32 {}
impl Sealed for f64 {}
