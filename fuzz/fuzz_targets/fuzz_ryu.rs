#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use std::mem;

#[derive(Arbitrary, Debug)]
enum FloatInput {
    F32(f32),
    F64(f64),
}

fuzz_target!(|inputs: (FloatInput, bool)| {
    let (input, finite) = inputs;
    let mut buffer = ryu::Buffer::new();
    let string = match (input, finite) {
        (FloatInput::F32(val), false) => buffer.format(val),
        (FloatInput::F32(val), true) => buffer.format_finite(val),
        (FloatInput::F64(val), false) => buffer.format(val),
        (FloatInput::F64(val), true) => buffer.format_finite(val),
    };
    assert!(string.len() <= mem::size_of::<ryu::Buffer>());
});
