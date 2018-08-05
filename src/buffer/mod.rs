use core::{mem, str};

use pretty;

#[cfg(feature = "no-panic")]
use no_panic::no_panic;

/// Safe API for formatting floating point numbers to text.
///
/// ## Example
///
/// ```rust
/// let mut buffer = ryu::Buffer::new();
/// let printed = buffer.format(1.234);
/// assert_eq!(printed, "1.234");
/// ```
#[derive(Copy, Clone)]
pub struct Buffer {
    bytes: [u8; 24],
}

impl Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn new() -> Self {
        Buffer {
            bytes: unsafe { mem::uninitialized() },
        }
    }

    #[cfg_attr(feature = "no-panic", no_panic)]
    pub fn format<F: Float>(&mut self, f: F) -> &str {
        f.write_to_ryu_buffer(self)
    }
}

impl Default for Buffer {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
    fn default() -> Self {
        Buffer::new()
    }
}

/// A floating point number, f32 or f64, that can be written into a
/// [`ryu::Buffer`][Buffer].
///
/// This trait is sealed and cannot be implemented for types outside of the
/// `ryu` crate.
pub trait Float: Sealed {
    // Not public API.
    #[doc(hidden)]
    fn write_to_ryu_buffer(self, buffer: &mut Buffer) -> &str;
}

impl Float for f32 {
    #[inline]
    #[cfg_attr(feature = "no-panic", no_panic)]
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
    #[cfg_attr(feature = "no-panic", no_panic)]
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
