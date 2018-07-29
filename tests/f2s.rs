extern crate rand;
extern crate ryu;

use std::{f32, str};

#[test]
fn test_ryu() {
    let cases = vec![
        (1.234e20f32, "1.234E20"),
        (1.234e21f32, "1.234E21"),
        (2.71828f32, "2.71828E0"),
        (0.0f32, "0E0"),
        (-0.0f32, "-0E0"),
        (1.1e32f32, "1.1E32"),
        (1.1e-32f32, "1.1E-32"),
        (2.7182817f32, "2.7182817E0"),
        (1e-45f32, "1E-45"),
        (f32::MAX, "3.4028235E38"),
    ];
    for (f, expected) in cases {
        let mut bytes = [0u8; 24];
        let n = unsafe { ryu::f2s_buffered_n(f, &mut bytes[0]) };
        let s = str::from_utf8(&bytes[..n]).unwrap();
        assert_eq!(s, expected);
    }
}

#[test]
fn test_random() {
    let mut bytes = [0u8; 24];
    for _ in 0..1000000 {
        let f = rand::random();
        let n = unsafe { ryu::f2s_buffered_n(f, &mut bytes[0]) };
        let s = str::from_utf8(&bytes[..n]).unwrap();
        assert_eq!(f, s.parse().unwrap());
    }
}

#[cfg(exhaustive)]
#[test]
fn test_exhaustive() {
    let mut bytes = [0u8; 24];
    for u in 0..=u32::max_value() {
        if u % 1000000 == 0 {
            println!("{}", u);
        }
        let f = f32::from_bits(u);
        if !f.is_finite() {
            continue;
        }
        let n = unsafe { ryu::f2s_buffered_n(f, &mut bytes[0]) };
        let s = str::from_utf8(&bytes[..n]).unwrap();
        assert_eq!(f, s.parse().unwrap());
    }
}
