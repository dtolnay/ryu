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

mod common;
mod d2s_full_table;
mod digit_table;
mod mulshift128;

use std::{mem, ptr};

use common::*;
use d2s_full_table::*;
use digit_table::*;
use mulshift128::*;

fn pow5_factor(mut value: u64) -> i32 {
    let mut count = 0i32;
    loop {
        if value == 0 {
            return 0;
        }
        if value % 5 != 0 {
            return count;
        }
        value /= 5;
        count += 1;
    }
}

// Returns true if value is divisible by 5^p.
fn multiple_of_power_of_5(value: u64, p: i32) -> bool {
    // I tried a case distinction on p, but there was no performance difference.
    pow5_factor(value) >= p
}

fn mul_shift_all(
    mut m: u64,
    mul: &[u64; 2],
    j: u32,
    vp: &mut u64,
    vm: &mut u64,
    mm_shift: u32,
) -> u64 {
    m <<= 1;
    // m is maximum 55 bits
    let (lo, tmp) = umul128(m, mul[0]);
    let (mut mid, mut hi) = umul128(m, mul[1]);
    mid = mid.wrapping_add(tmp);
    hi = hi.wrapping_add((mid < tmp) as u64); // overflow into hi

    let lo2 = lo.wrapping_add(mul[0]);
    let mid2 = mid.wrapping_add(mul[1]).wrapping_add((lo2 < lo) as u64);
    let hi2 = hi.wrapping_add((mid2 < mid) as u64);
    *vp = shiftright128(mid2, hi2, j - 64 - 1);

    if mm_shift == 1 {
        let lo3 = lo.wrapping_sub(mul[0]);
        let mid3 = mid.wrapping_sub(mul[1]).wrapping_sub((lo3 > lo) as u64);
        let hi3 = hi - (mid3 > mid) as u64;
        *vm = shiftright128(mid3, hi3, j - 64 - 1);
    } else {
        let lo3 = lo + lo;
        let mid3 = mid + mid + (lo3 < lo) as u64;
        let hi3 = hi + hi + (mid3 < mid) as u64;
        let lo4 = lo3 - mul[0];
        let mid4 = mid3 - mul[1] - (lo4 > lo3) as u64;
        let hi4 = hi3 - (mid4 > mid3) as u64;
        *vm = shiftright128(mid4, hi4, j - 64);
    }

    shiftright128(mid, hi, j - 64 - 1)
}

fn decimal_length(v: u64) -> u32 {
    // This is slightly faster than a loop.
    // The average output length is 16.38 digits, so we check high-to-low.
    // Function precondition: v is not an 18, 19, or 20-digit number.
    // (17 digits are sufficient for round-tripping.)
    debug_assert!(v < 100000000000000000);

    if v >= 10000000000000000 {
        17
    } else if v >= 1000000000000000 {
        16
    } else if v >= 100000000000000 {
        15
    } else if v >= 10000000000000 {
        14
    } else if v >= 1000000000000 {
        13
    } else if v >= 100000000000 {
        12
    } else if v >= 10000000000 {
        11
    } else if v >= 1000000000 {
        10
    } else if v >= 100000000 {
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

#[must_use]
pub unsafe fn d2s_buffered_n(f: f64, result: *mut u8) -> usize {
    const DOUBLE_MANTISSA_BITS: u32 = 52;
    const DOUBLE_EXPONENT_BITS: u32 = 11;
    const DOUBLE_POW5_INV_BITCOUNT: i32 = 122;
    const DOUBLE_POW5_BITCOUNT: i32 = 121;

    // Step 1: Decode the floating-point number, and unify normalized and subnormal cases.
    let offset = (1u32 << (DOUBLE_EXPONENT_BITS - 1)) - 1;
    let bits = f.to_bits().to_le();

    // Decode bits into sign, mantissa, and exponent.
    let sign = ((bits >> (DOUBLE_MANTISSA_BITS + DOUBLE_EXPONENT_BITS)) & 1) != 0;
    let ieee_mantissa = bits & ((1u64 << DOUBLE_MANTISSA_BITS) - 1);
    let ieee_exponent =
        (bits >> DOUBLE_MANTISSA_BITS) as u32 & ((1u32 << DOUBLE_EXPONENT_BITS) - 1);

    let e2: i32;
    let m2: u64;
    // Case distinction; exit early for the easy cases.
    if ieee_exponent == ((1u32 << DOUBLE_EXPONENT_BITS) - 1)
        || (ieee_exponent == 0 && ieee_mantissa == 0)
    {
        return copy_special_str(result, sign);
    } else if ieee_exponent == 0 {
        // We subtract 2 so that the bounds computation has 2 additional bits.
        e2 = 1 - offset as i32 - DOUBLE_MANTISSA_BITS as i32 - 2;
        m2 = ieee_mantissa;
    } else {
        e2 = ieee_exponent as i32 - offset as i32 - DOUBLE_MANTISSA_BITS as i32 - 2;
        m2 = (1u64 << DOUBLE_MANTISSA_BITS) | ieee_mantissa;
    }
    let even = (m2 & 1) == 0;
    let accept_bounds = even;

    // Step 2: Determine the interval of legal decimal representations.
    let mv = 4 * m2;
    // Implicit bool -> int conversion. True is 1, false is 0.
    let mm_shift = ((m2 != (1u64 << DOUBLE_MANTISSA_BITS)) || (ieee_exponent <= 1)) as u32;
    // We would compute mp and mm like this:
    //     uint64_t mp = 4 * m2 + 2;
    //     uint64_t mm = mv - 1 - mm_shift;

    // Step 3: Convert to a decimal power base using 128-bit arithmetic.
    let mut vr: u64;
    let mut vp: u64 = mem::uninitialized();
    let mut vm: u64 = mem::uninitialized();
    let e10: i32;
    let mut vm_is_trailing_zeros = false;
    let mut vr_is_trailing_zeros = false;
    if e2 >= 0 {
        // I tried special-casing q == 0, but there was no effect on performance.
        // This expression is slightly faster than max(0, log10_pow2(e2) - 1).
        let q = log10_pow2(e2) - (e2 > 3) as i32;
        e10 = q;
        let k = DOUBLE_POW5_INV_BITCOUNT + pow5bits(q) as i32 - 1;
        let i = -e2 + q + k;
        vr = mul_shift_all(
            m2,
            &DOUBLE_POW5_INV_SPLIT[q as usize],
            i as u32,
            &mut vp,
            &mut vm,
            mm_shift,
        );
        if q <= 21 {
            // Only one of mp, mv, and mm can be a multiple of 5, if any.
            if mv % 5 == 0 {
                vr_is_trailing_zeros = multiple_of_power_of_5(mv, q);
            } else {
                if accept_bounds {
                    // Same as min(e2 + (~mm & 1), pow5_factor(mm)) >= q
                    // <=> e2 + (~mm & 1) >= q && pow5_factor(mm) >= q
                    // <=> true && pow5_factor(mm) >= q, since e2 >= q.
                    vm_is_trailing_zeros = multiple_of_power_of_5(mv - 1 - mm_shift as u64, q);
                } else {
                    // Same as min(e2 + 1, pow5_factor(mp)) >= q.
                    vp -= multiple_of_power_of_5(mv + 2, q) as u64;
                }
            }
        }
    } else {
        // This expression is slightly faster than max(0, log10_pow5(-e2) - 1).
        let q = log10_pow5(-e2) - (-e2 > 1) as i32;
        e10 = q + e2;
        let i = -e2 - q;
        let k = pow5bits(i) as i32 - DOUBLE_POW5_BITCOUNT;
        let j = q - k;
        vr = mul_shift_all(
            m2,
            &DOUBLE_POW5_SPLIT[i as usize],
            j as u32,
            &mut vp,
            &mut vm,
            mm_shift,
        );
        if q <= 1 {
            vr_is_trailing_zeros = (!(mv as u32) & 1) >= q as u32;
            if accept_bounds {
                vm_is_trailing_zeros = (!((mv - 1 - mm_shift as u64) as u32) & 1) >= q as u32;
            } else {
                vp -= 1;
            }
        } else if q < 63 {
            // TODO(ulfjack): Use a tighter bound here.
            // We need to compute min(ntz(mv), pow5_factor(mv) - e2) >= q-1
            // <=> ntz(mv) >= q-1  &&  pow5_factor(mv) - e2 >= q-1
            // <=> ntz(mv) >= q-1
            // <=> (mv & ((1 << (q-1)) - 1)) == 0
            // We also need to make sure that the left shift does not overflow.
            vr_is_trailing_zeros = (mv & ((1u64 << (q - 1)) - 1)) == 0;
        }
    }

    // Step 4: Find the shortest decimal representation in the interval of legal representations.
    let mut removed = 0u32;
    let mut last_removed_digit = 0u8;
    let output: u64;
    // On average, we remove ~2 digits.
    if vm_is_trailing_zeros || vr_is_trailing_zeros {
        // General case, which happens rarely (<1%).
        while vp / 10 > vm / 10 {
            vm_is_trailing_zeros &= vm - (vm / 10) * 10 == 0;
            vr_is_trailing_zeros &= last_removed_digit == 0;
            last_removed_digit = (vr % 10) as u8;
            vr /= 10;
            vp /= 10;
            vm /= 10;
            removed += 1;
        }
        if vm_is_trailing_zeros {
            while vm % 10 == 0 {
                vr_is_trailing_zeros &= last_removed_digit == 0;
                last_removed_digit = (vr % 10) as u8;
                vr /= 10;
                vp /= 10;
                vm /= 10;
                removed += 1;
            }
        }
        if vr_is_trailing_zeros && last_removed_digit == 5 && vr % 2 == 0 {
            // Round down not up if the number ends in X50000.
            last_removed_digit = 4;
        }
        // We need to take vr+1 if vr is outside bounds or we need to round up.
        output = vr + ((vr == vm && (!accept_bounds || !vm_is_trailing_zeros))
            || (last_removed_digit >= 5)) as u64;
    } else {
        // Specialized for the common case (>99%).
        while vp / 10 > vm / 10 {
            last_removed_digit = (vr % 10) as u8;
            vr /= 10;
            vp /= 10;
            vm /= 10;
            removed += 1;
        }
        // We need to take vr+1 if vr is outside bounds or we need to round up.
        output = vr + ((vr == vm) || (last_removed_digit >= 5)) as u64;
    }
    // The average output length is 16.38 digits.
    let olength = decimal_length(output);
    let vplength = olength + removed;
    let mut exp = e10 + vplength as i32 - 1;

    // Step 5: Print the decimal representation.
    let mut index = 0isize;
    if sign {
        *result.offset(index) = b'-';
        index += 1;
    }

    // Print decimal digits after the decimal point.
    let mut i = 0isize;
    // 64-bit division is efficient on 64-bit platforms.
    let mut output2 = output;
    while output2 >= 10000 {
        let c = (output2 - 10000 * (output2 / 10000)) as u32;
        output2 /= 10000;
        let c0 = (c % 100) << 1;
        let c1 = (c / 100) << 1;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c0 as isize),
            result.offset(index + olength as isize - i - 1),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c1 as isize),
            result.offset(index + olength as isize - i - 3),
            2,
        );
        i += 4;
    }
    if output2 >= 100 {
        let c = ((output2 % 100) << 1) as u32;
        output2 /= 100;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c as isize),
            result.offset(index + olength as isize - i - 1),
            2,
        );
        i += 2;
    }
    if output2 >= 10 {
        let c = (output2 << 1) as u32;
        *result.offset(index + olength as isize - i) = DIGIT_TABLE[c as usize + 1];
        *result.offset(index) = DIGIT_TABLE[c as usize];
    } else {
        // Print the leading decimal digit.
        *result.offset(index) = b'0' + output2 as u8;
    }

    // Print decimal point if needed.
    if olength > 1 {
        *result.offset(index + 1) = b'.';
        index += olength as isize + 1;
    } else {
        index += 1;
    }

    // Print the exponent.
    *result.offset(index) = b'E';
    index += 1;
    if exp < 0 {
        *result.offset(index) = b'-';
        index += 1;
        exp = -exp;
    }

    if exp >= 100 {
        let c = exp % 10;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset((2 * (exp / 10)) as isize),
            result.offset(index),
            2,
        );
        *result.offset(index + 2) = b'0' + c as u8;
        index += 3;
    } else if exp >= 10 {
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset((2 * exp) as isize),
            result.offset(index),
            2,
        );
        index += 2;
    } else {
        *result.offset(index) = b'0' + exp as u8;
        index += 1;
    }

    index as usize
}
