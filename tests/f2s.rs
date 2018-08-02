// Translated from C to Rust. The original C code can be found at
// https://github.com/ulfjack/ryu and carries the following license:
//
// Copyright 2018 Ulf Adams
//
// The contents of this file may be used under the terms of the Apache License,
// Version 2.0.
//
//    (See accompanying file LICENSE-Apache or copy at
//     http://www.apache.org/licenses/LICENSE-2.0)
//
// Alternatively, the contents of this file may be used under the terms of
// the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE-Boost or copy at
//     https://www.boost.org/LICENSE_1_0.txt)
//
// Unless required by applicable law or agreed to in writing, this software
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.

extern crate rand;
extern crate ryu;

use std::{f32, str};

fn f2s(f: f32) -> String {
    let mut bytes = [0u8; 24];
    let n = unsafe { ryu::f2s_buffered_n(f, &mut bytes[0]) };
    let s = str::from_utf8(&bytes[..n]).unwrap();
    s.to_owned()
}

#[test]
fn test_ryu() {
    assert_eq!("1.234E20", f2s(1.234e20f32));
    assert_eq!("1.234E21", f2s(1.234e21f32));
    assert_eq!("2.71828E0", f2s(2.71828f32));
    assert_eq!("1.1E32", f2s(1.1e32f32));
    assert_eq!("1.1E-32", f2s(1.1e-32f32));
    assert_eq!("2.7182817E0", f2s(2.7182817f32));
    assert_eq!("1E-45", f2s(1e-45f32));
    assert_eq!("3.4028235E38", f2s(f32::MAX));
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

#[test]
fn test_basic() {
    assert_eq!("0E0", f2s(0.0));
    assert_eq!("-0E0", f2s(-0.0));
    assert_eq!("1E0", f2s(1.0));
    assert_eq!("-1E0", f2s(-1.0));
}

#[test]
fn test_switch_to_subnormal() {
    assert_eq!("1.1754944E-38", f2s(1.1754944E-38));
}

#[test]
fn test_min_and_max() {
    assert_eq!("3.4028235E38", f2s(f32::from_bits(0x7f7fffff)));
    assert_eq!("1E-45", f2s(f32::from_bits(1)));
}

// Check that we return the exact boundary if it is the shortest
// representation, but only if the original floating point number is even.
#[test]
fn test_boundary_round_even() {
    assert_eq!("3.355445E7", f2s(3.355445E7));
    assert_eq!("9E9", f2s(8.999999E9));
    assert_eq!("3.436672E10", f2s(3.4366717E10));
}

// If the exact value is exactly halfway between two shortest representations,
// then we round to even. It seems like this only makes a difference if the
// last two digits are ...2|5 or ...7|5, and we cut off the 5.
#[test]
fn test_exact_value_round_even() {
    assert_eq!("3.0540412E5", f2s(3.0540412E5));
    assert_eq!("8.0990312E3", f2s(8.0990312E3));
}

#[test]
fn test_lots_of_trailing_zeros() {
    // Pattern for the first test: 00111001100000000000000000000000
    assert_eq!("2.4414062E-4", f2s(2.4414062E-4));
    assert_eq!("2.4414062E-3", f2s(2.4414062E-3));
    assert_eq!("4.3945312E-3", f2s(4.3945312E-3));
    assert_eq!("6.3476562E-3", f2s(6.3476562E-3));
}

#[test]
fn test_regression() {
    assert_eq!("4.7223665E21", f2s(4.7223665E21));
    assert_eq!("8.388608E6", f2s(8388608.0));
    assert_eq!("1.6777216E7", f2s(1.6777216E7));
    assert_eq!("3.3554436E7", f2s(3.3554436E7));
    assert_eq!("6.7131496E7", f2s(6.7131496E7));
    assert_eq!("1.9310392E-38", f2s(1.9310392E-38));
    assert_eq!("-2.47E-43", f2s(-2.47E-43));
    assert_eq!("1.993244E-38", f2s(1.993244E-38));
    assert_eq!("4.1039004E3", f2s(4103.9003));
    assert_eq!("5.3399997E9", f2s(5.3399997E9));
    assert_eq!("6.0898E-39", f2s(6.0898E-39));
    assert_eq!("1.0310042E-3", f2s(0.0010310042));
    assert_eq!("2.882326E17", f2s(2.8823261E17));
    assert_eq!("7.038531E-26", f2s(7.0385309E-26));
    assert_eq!("9.223404E17", f2s(9.2234038E17));
    assert_eq!("6.710887E7", f2s(6.7108872E7));
    assert_eq!("1E-44", f2s(1.0E-44));
    assert_eq!("2.816025E14", f2s(2.816025E14));
    assert_eq!("9.223372E18", f2s(9.223372E18));
    assert_eq!("1.5846086E29", f2s(1.5846085E29));
    assert_eq!("1.1811161E19", f2s(1.1811161E19));
    assert_eq!("5.368709E18", f2s(5.368709E18));
    assert_eq!("4.6143166E18", f2s(4.6143165E18));
    assert_eq!("7.812537E-3", f2s(0.007812537));
    assert_eq!("1E-45", f2s(1.4E-45));
    assert_eq!("1.18697725E20", f2s(1.18697724E20));
    assert_eq!("1.00014165E-36", f2s(1.00014165E-36));
    assert_eq!("2E2", f2s(200.0));
    assert_eq!("3.3554432E7", f2s(3.3554432E7));
}

#[test]
fn test_output_length() {
    assert_eq!("1E0", f2s(1.0)); // already tested in Basic
    assert_eq!("1.2E0", f2s(1.2));
    assert_eq!("1.23E0", f2s(1.23));
    assert_eq!("1.234E0", f2s(1.234));
    assert_eq!("1.2345E0", f2s(1.2345));
    assert_eq!("1.23456E0", f2s(1.23456));
    assert_eq!("1.234567E0", f2s(1.234567));
    assert_eq!("1.2345678E0", f2s(1.2345678));
    assert_eq!("1.23456735E-36", f2s(1.23456735E-36));
}
