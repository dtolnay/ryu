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

// Returns (lo, hi).
#[cfg_attr(feature = "no-panic", inline)]
pub fn umul128(a: u64, b: u64) -> (u64, u64) {
    let a_lo = a as u32;
    let a_hi = (a >> 32) as u32;
    let b_lo = b as u32;
    let b_hi = (b >> 32) as u32;

    let b00 = a_lo as u64 * b_lo as u64;
    let b01 = a_lo as u64 * b_hi as u64;
    let b10 = a_hi as u64 * b_lo as u64;
    let b11 = a_hi as u64 * b_hi as u64;

    let mid_sum = b01 + b10;
    let mid_carry = (mid_sum < b01) as u64;

    let product_lo = b00.wrapping_add(mid_sum << 32);
    let product_lo_carry = (product_lo < b00) as u64;

    let product_hi = b11 + (mid_sum >> 32) + (mid_carry << 32) + product_lo_carry;
    (product_lo, product_hi)
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn shiftright128(lo: u64, hi: u64, dist: u32) -> u64 {
    // We don't need to handle the case dist >= 64 here (see above).
    debug_assert!(dist > 0);
    debug_assert!(dist < 64);
    (hi << (64 - dist)) | (lo >> dist)
}
