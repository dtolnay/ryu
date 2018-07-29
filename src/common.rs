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

use std::ptr;

// Returns e == 0 ? 1 : ceil(log_2(5^e)).
pub fn pow5bits(e: i32) -> u32 {
    // This function has only been tested for 0 <= e <= 1500.
    debug_assert!(e >= 0);
    debug_assert!(e <= 1500);
    ((e as u32 * 1217359) >> 19) + 1
}

// Returns floor(log_10(2^e)).
pub fn log10_pow2(e: i32) -> i32 {
    // This function has only been tested for 0 <= e <= 1500.
    debug_assert!(e >= 0);
    debug_assert!(e <= 1500);
    ((e as u32 * 78913) >> 18) as i32
}

// Returns floor(log_10(5^e)).
pub fn log10_pow5(e: i32) -> i32 {
    // This function has only been tested for 0 <= e <= 1500.
    debug_assert!(e >= 0);
    debug_assert!(e <= 1500);
    ((e as u32 * 732923) >> 20) as i32
}

pub unsafe fn copy_special_str(result: *mut u8, sign: bool) -> usize {
    if sign {
        ptr::write(result, b'-');
    }
    ptr::copy_nonoverlapping(b"0E0".as_ptr(), result.offset(sign as isize), 3);
    sign as usize + 3
}
