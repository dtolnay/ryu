#![no_main]
use libfuzzer_sys::arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(arbitrary::Arbitrary, Debug, Clone)]
enum FloatInput {
    F32(f32),
    F64(f64),
}

#[derive(arbitrary::Arbitrary, Debug, Clone)]
struct Inputs {
    inputs: Vec<(FloatInput, bool)>,
}

fuzz_target!(|input: Inputs| {
    let mut buffer = ryu::Buffer::new();
    for input_step in input.inputs {
        match input_step {
            (FloatInput::F32(val), false) => buffer.format(val),
            (FloatInput::F32(val), true) => buffer.format_finite(val),
            (FloatInput::F64(val), false) => buffer.format(val),
            (FloatInput::F64(val), true) => buffer.format_finite(val),
        };
    }
});
