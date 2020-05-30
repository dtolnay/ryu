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

// Returns the number of decimal digits in v, which must not contain more than 9
// digits.
#[cfg_attr(feature = "no-panic", inline)]
pub fn decimal_length9(v: u32) -> u32 {
    // Function precondition: v is not a 10-digit number.
    // (f2s: 9 digits are sufficient for round-tripping.)
    debug_assert!(v < 1000000000);

    if v >= 100000000 {
        9
    } else if v >= 10000000 {
        8
    } else if v >= 1000000 {
        7
    } else if v >= 100000 {
        6
    } else if v >= 10000 {
        5
    } else if v >= 1000 {
        4
    } else if v >= 100 {
        3
    } else if v >= 10 {
        2
    } else {
        1
    }
}

// Returns e == 0 ? 1 : ceil(log_2(5^e)); requires 0 <= e <= 3528.
#[cfg_attr(feature = "no-panic", inline)]
pub fn pow5bits(e: i32) -> i32 {
    // This approximation works up to the point that the multiplication overflows at e = 3529.
    // If the multiplication were done in 64 bits, it would fail at 5^4004 which is just greater
    // than 2^9297.
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    (((e as u32 * 1217359) >> 19) + 1) as i32
}

// Returns floor(log_10(2^e)); requires 0 <= e <= 1650.
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow2(e: i32) -> u32 {
    // The first value this approximation fails for is 2^1651 which is just greater than 10^297.
    debug_assert!(e >= 0);
    debug_assert!(e <= 1650);
    (e as u32 * 78913) >> 18
}

// Returns floor(log_10(5^e)); requires 0 <= e <= 2620.
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow5(e: i32) -> u32 {
    // The first value this approximation fails for is 5^2621 which is just greater than 10^1832.
    debug_assert!(e >= 0);
    debug_assert!(e <= 2620);
    (e as u32 * 732923) >> 20
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ceil_log2_pow5(e: i32) -> i32 {
        pow5bits(e)
    }

    #[test]
    fn test_decimal_length9() {
        assert_eq!(1, decimal_length9(0));
        assert_eq!(1, decimal_length9(1));
        assert_eq!(1, decimal_length9(9));
        assert_eq!(2, decimal_length9(10));
        assert_eq!(2, decimal_length9(99));
        assert_eq!(3, decimal_length9(100));
        assert_eq!(3, decimal_length9(999));
        assert_eq!(9, decimal_length9(999999999));
    }

    #[test]
    fn test_ceil_log2_pow5() {
        assert_eq!(1, ceil_log2_pow5(0));
        assert_eq!(3, ceil_log2_pow5(1));
        assert_eq!(5, ceil_log2_pow5(2));
        assert_eq!(7, ceil_log2_pow5(3));
        assert_eq!(10, ceil_log2_pow5(4));
        assert_eq!(8192, ceil_log2_pow5(3528));
    }

    #[test]
    fn test_log10_pow2() {
        assert_eq!(0, log10_pow2(0));
        assert_eq!(0, log10_pow2(1));
        assert_eq!(0, log10_pow2(2));
        assert_eq!(0, log10_pow2(3));
        assert_eq!(1, log10_pow2(4));
        assert_eq!(496, log10_pow2(1650));
    }

    #[test]
    fn test_log10_pow5() {
        assert_eq!(0, log10_pow5(0));
        assert_eq!(0, log10_pow5(1));
        assert_eq!(1, log10_pow5(2));
        assert_eq!(2, log10_pow5(3));
        assert_eq!(2, log10_pow5(4));
        assert_eq!(1831, log10_pow5(2620));
    }

    #[test]
    fn test_float_to_bits() {
        assert_eq!(0, 0.0_f32.to_bits());
        assert_eq!(0x40490fda, 3.1415926_f32.to_bits());
    }

    #[test]
    fn test_double_to_bits() {
        assert_eq!(0, 0.0_f64.to_bits());
        assert_eq!(0x400921FB54442D18, 3.1415926535897932384626433_f64.to_bits());
    }
}
