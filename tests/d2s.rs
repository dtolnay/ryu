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

use std::{f64, str};

fn d2s(f: f64) -> String {
    let mut bytes = [0u8; 24];
    let n = unsafe { ryu::d2s_buffered_n(f, &mut bytes[0]) };
    let s = str::from_utf8(&bytes[..n]).unwrap();
    s.to_owned()
}

#[test]
fn test_ryu() {
    assert_eq!("3E-1", d2s(0.3));
    assert_eq!("1.234E20", d2s(1.234e20f64));
    assert_eq!("1.234E21", d2s(1.234e21f64));
    assert_eq!("2.71828E0", d2s(2.71828f64));
    assert_eq!("0E0", d2s(0.0f64));
    assert_eq!("-0E0", d2s(-0.0f64));
    assert_eq!("1.1E128", d2s(1.1e128f64));
    assert_eq!("1.1E-64", d2s(1.1e-64f64));
    assert_eq!("2.718281828459045E0", d2s(2.718281828459045f64));
    assert_eq!("5E-324", d2s(5e-324f64));
    assert_eq!("1.7976931348623157E308", d2s(f64::MAX));
}

#[test]
fn test_random() {
    let mut bytes = [0u8; 24];
    for _ in 0..1000000 {
        let f = rand::random();
        let n = unsafe { ryu::d2s_buffered_n(f, &mut bytes[0]) };
        let s = str::from_utf8(&bytes[..n]).unwrap();
        assert_eq!(f, s.parse().unwrap());
    }
}

#[test]
fn test_basic() {
    assert_eq!("0E0", d2s(0.0));
    assert_eq!("-0E0", d2s(-0.0));
    assert_eq!("1E0", d2s(1.0));
    assert_eq!("-1E0", d2s(-1.0));
}

#[test]
fn test_switch_to_subnormal() {
    assert_eq!("2.2250738585072014E-308", d2s(2.2250738585072014E-308));
}

#[test]
fn test_min_and_max() {
    assert_eq!(
        "1.7976931348623157E308",
        d2s(f64::from_bits(0x7fefffffffffffff))
    );
    assert_eq!("5E-324", d2s(f64::from_bits(1)));
}

#[test]
fn test_lots_of_trailing_zeros() {
    assert_eq!("2.9802322387695312E-8", d2s(2.98023223876953125E-8));
}

#[test]
fn test_regression() {
    assert_eq!("-2.109808898695963E16", d2s(-2.109808898695963E16));
    assert_eq!("4.940656E-318", d2s(4.940656E-318));
    assert_eq!("1.18575755E-316", d2s(1.18575755E-316));
    assert_eq!("2.989102097996E-312", d2s(2.989102097996E-312));
    assert_eq!("9.0608011534336E15", d2s(9.0608011534336E15));
    assert_eq!("4.708356024711512E18", d2s(4.708356024711512E18));
    assert_eq!("9.409340012568248E18", d2s(9.409340012568248E18));
    assert_eq!("1.2345678E0", d2s(1.2345678));
}

#[test]
fn test_output_length() {
    assert_eq!("1E0", d2s(1.0)); // already tested in Basic
    assert_eq!("1.2E0", d2s(1.2));
    assert_eq!("1.23E0", d2s(1.23));
    assert_eq!("1.234E0", d2s(1.234));
    assert_eq!("1.2345E0", d2s(1.2345));
    assert_eq!("1.23456E0", d2s(1.23456));
    assert_eq!("1.234567E0", d2s(1.234567));
    assert_eq!("1.2345678E0", d2s(1.2345678)); // already tested in Regression
    assert_eq!("1.23456789E0", d2s(1.23456789));
    assert_eq!("1.234567895E0", d2s(1.234567895)); // 1.234567890 would be trimmed
    assert_eq!("1.2345678901E0", d2s(1.2345678901));
    assert_eq!("1.23456789012E0", d2s(1.23456789012));
    assert_eq!("1.234567890123E0", d2s(1.234567890123));
    assert_eq!("1.2345678901234E0", d2s(1.2345678901234));
    assert_eq!("1.23456789012345E0", d2s(1.23456789012345));
    assert_eq!("1.234567890123456E0", d2s(1.234567890123456));
    assert_eq!("1.2345678901234567E0", d2s(1.2345678901234567));

    // Test 32-bit chunking
    assert_eq!("4.294967294E0", d2s(4.294967294)); // 2^32 - 2
    assert_eq!("4.294967295E0", d2s(4.294967295)); // 2^32 - 1
    assert_eq!("4.294967296E0", d2s(4.294967296)); // 2^32
    assert_eq!("4.294967297E0", d2s(4.294967297)); // 2^32 + 1
    assert_eq!("4.294967298E0", d2s(4.294967298)); // 2^32 + 2
}
