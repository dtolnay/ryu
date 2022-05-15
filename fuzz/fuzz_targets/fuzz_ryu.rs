#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use std::mem;

#[derive(Arbitrary, Debug)]
enum FloatInput {
    F32(f32),
    F64(f64),
}

macro_rules! ryu_test {
    ($val:expr, $method:ident) => {
        match $val {
            val => {
                let mut buffer = ryu::Buffer::new();
                let string = buffer.$method(val);
                assert!(string.len() <= mem::size_of::<ryu::Buffer>());
                if val.is_finite() {
                    assert_eq!(val, string.parse().unwrap());
                }
            }
        }
    };
}

fuzz_target!(|inputs: (FloatInput, bool)| {
    let (input, finite) = inputs;
    match (input, finite) {
        (FloatInput::F32(val), false) => ryu_test!(val, format),
        (FloatInput::F32(val), true) => ryu_test!(val, format_finite),
        (FloatInput::F64(val), false) => ryu_test!(val, format),
        (FloatInput::F64(val), true) => ryu_test!(val, format_finite),
    }
});
