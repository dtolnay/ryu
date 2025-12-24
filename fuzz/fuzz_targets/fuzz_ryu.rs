#![no_main]

use libfuzzer_sys::fuzz_target;
use std::mem;

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

fuzz_target!(|inputs: (f64, bool)| {
    let (input, finite) = inputs;
    match (input, finite) {
        (val, false) => ryu_test!(val, format),
        (val, true) => ryu_test!(val, format_finite),
    }
});
