#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum FloatInput {
    F32(f32),
    F64(f64),
}

fuzz_target!(|inputs: (FloatInput, bool)| {
    let (input, finite) = inputs;
    let mut buffer = ryu::Buffer::new();
    match (input, finite) {
        (FloatInput::F32(val), false) => buffer.format(val),
        (FloatInput::F32(val), true) => buffer.format_finite(val),
        (FloatInput::F64(val), false) => buffer.format(val),
        (FloatInput::F64(val), true) => buffer.format_finite(val),
    };
});
