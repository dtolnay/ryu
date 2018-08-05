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

#[macro_use]
mod macros;

use std::{f32, str};

fn print(f: f32) -> String {
    let mut bytes = [0u8; 24];
    let n = unsafe { ryu::raw::f2s_buffered_n(f, &mut bytes[0]) };
    let s = str::from_utf8(&bytes[..n]).unwrap();
    s.to_owned()
}

fn pretty(f: f32) -> String {
    ryu::Buffer::new().format(f).to_owned()
}

#[test]
fn test_ryu() {
    check!(1.234E20, 123400000000000000000.0);
    check!(1.234E21, 1.234e21);
    check!(2.71828E0, 2.71828);
    check!(1.1E32, 1.1e32);
    check!(1.1E-32, 1.1e-32);
    check!(2.7182817E0, 2.7182817);
    check!(1E-45, 1e-45);
    check!(3.4028235E38, 3.4028235e38);
    check!(-1.234E-3, -0.001234);
}

#[test]
fn test_random() {
    let mut bytes = [0u8; 24];
    let mut buffer = ryu::Buffer::new();
    for _ in 0..1000000 {
        let f = rand::random();
        let n = unsafe { ryu::raw::f2s_buffered_n(f, &mut bytes[0]) };
        assert_eq!(f, str::from_utf8(&bytes[..n]).unwrap().parse().unwrap());
        assert_eq!(f, buffer.format(f).parse().unwrap());
    }
}

#[test]
fn test_basic() {
    check!(0E0, 0.0);
    check!(-0E0, -0.0);
    check!(1E0, 1.0);
    check!(-1E0, -1.0);
    assert_eq!(print(f32::NAN), "NaN");
    assert_eq!(print(f32::INFINITY), "Infinity");
    assert_eq!(print(f32::NEG_INFINITY), "-Infinity");
}

#[test]
fn test_switch_to_subnormal() {
    check!(1.1754944E-38, 1.1754944e-38);
}

#[test]
fn test_min_and_max() {
    assert_eq!(f32::from_bits(0x7f7fffff), 3.4028235e38);
    check!(3.4028235E38, 3.4028235e38);
    assert_eq!(f32::from_bits(1), 1e-45);
    check!(1E-45, 1e-45);
}

// Check that we return the exact boundary if it is the shortest
// representation, but only if the original floating point number is even.
#[test]
fn test_boundary_round_even() {
    check!(3.355445E7, 33554450.0);
    check!(9E9, 9000000000.0);
    check!(3.436672E10, 34366720000.0);
}

// If the exact value is exactly halfway between two shortest representations,
// then we round to even. It seems like this only makes a difference if the
// last two digits are ...2|5 or ...7|5, and we cut off the 5.
#[test]
fn test_exact_value_round_even() {
    check!(3.0540412E5, 305404.12);
    check!(8.0990312E3, 8099.0312);
}

#[test]
fn test_lots_of_trailing_zeros() {
    // Pattern for the first test: 00111001100000000000000000000000
    check!(2.4414062E-4, 0.00024414062);
    check!(2.4414062E-3, 0.0024414062);
    check!(4.3945312E-3, 0.0043945312);
    check!(6.3476562E-3, 0.0063476562);
}

#[test]
fn test_regression() {
    check!(4.7223665E21, 4.7223665e21);
    check!(8.388608E6, 8388608.0);
    check!(1.6777216E7, 16777216.0);
    check!(3.3554436E7, 33554436.0);
    check!(6.7131496E7, 67131496.0);
    check!(1.9310392E-38, 1.9310392e-38);
    check!(-2.47E-43, -2.47e-43);
    check!(1.993244E-38, 1.993244e-38);
    check!(4.1039004E3, 4103.9004);
    check!(5.3399997E9, 5339999700.0);
    check!(6.0898E-39, 6.0898e-39);
    check!(1.0310042E-3, 0.0010310042);
    check!(2.882326E17, 288232600000000000.0);
    check!(7.038531E-26, 7.038531e-26);
    check!(9.223404E17, 922340400000000000.0);
    check!(6.710887E7, 67108870.0);
    check!(1E-44, 1e-44);
    check!(2.816025E14, 281602500000000.0);
    check!(9.223372E18, 9223372000000000000.0);
    check!(1.5846086E29, 1.5846086e29);
    check!(1.1811161E19, 11811161000000000000.0);
    check!(5.368709E18, 5368709000000000000.0);
    check!(4.6143166E18, 4614316600000000000.0);
    check!(7.812537E-3, 0.007812537);
    check!(1E-45, 1e-45);
    check!(1.18697725E20, 118697725000000000000.0);
    check!(1.00014165E-36, 1.00014165e-36);
    check!(2E2, 200.0);
    check!(3.3554432E7, 33554432.0);
}

#[test]
fn test_output_length() {
    check!(1E0, 1.0); // already tested in Basic
    check!(1.2E0, 1.2);
    check!(1.23E0, 1.23);
    check!(1.234E0, 1.234);
    check!(1.2345E0, 1.2345);
    check!(1.23456E0, 1.23456);
    check!(1.234567E0, 1.234567);
    check!(1.2345678E0, 1.2345678);
    check!(1.23456735E-36, 1.23456735e-36);
}
