// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `mpn_bc_get_str`, `mpn_dc_get_str`, `mpn_get_str`, `mpn_bc_set_str`, `mpn_dc_set_str`,
//      `mpn_set_str`, `powtab_decide`, `mpn_compute_powtab_mul`, `mpn_compute_powtab_div`, and
//      `mpn_compute_powtab` contributed to the GNU project by Torbjörn Granlund.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::div_exact::limbs_div_exact_limb_in_place;
use crate::natural::arithmetic::div_mod::{
    limbs_div_limb_in_place_mod, limbs_div_mod_extra_in_place, limbs_div_mod_qs_to_out_rs_to_ns,
};
use crate::natural::arithmetic::mul::limb::{limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place};
use crate::natural::arithmetic::mul::toom::TUNE_PROGRAM_BUILD;
use crate::natural::arithmetic::mul::{limbs_mul_to_out, limbs_mul_to_out_scratch_len};
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{
    Limb, BASES, FROM_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD, MP_BASES_BIG_BASE_10,
    MP_BASES_BIG_BASE_INVERTED_10, MP_BASES_CHARS_PER_LIMB_10, MP_BASES_NORMALIZATION_STEPS_10,
};
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use itertools::Itertools;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, CheckedMul, DivAssignMod, DivMod, DivisibleByPowerOf2, ModPowerOf2Assign,
    Parity, PowerOf2, ShrRound, ShrRoundAssign, SquareAssign, XMulYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, Digits, ExactFrom, ExactInto, PowerOf2Digits, WrappingFrom, WrappingInto,
};
use malachite_base::num::logic::traits::{LeadingZeros, SignificantBits, TrailingZeros};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::{slice_set_zero, slice_test_zero, slice_trailing_zeros};

// TODO tune
const GET_STR_THRESHOLD_LIMIT: usize = 150;

// TODO tune
#[cfg(feature = "test_build")]
pub const GET_STR_PRECOMPUTE_THRESHOLD: usize = 29;

#[cfg(not(feature = "test_build"))]
const GET_STR_PRECOMPUTE_THRESHOLD: usize = 29;

// # Worst-case complexity
// Constant time and additional memory.
pub_const_test! {get_chars_per_limb(base: u64) -> usize {
    BASES[base as usize].0
}}

// # Worst-case complexity
// Constant time and additional memory.
const fn get_log_base_of_2(base: u64) -> Limb {
    BASES[base as usize].1
}

// # Worst-case complexity
// Constant time and additional memory.
const fn get_log_2_of_base(base: u64) -> Limb {
    BASES[base as usize].2
}

// # Worst-case complexity
// Constant time and additional memory.
const fn get_big_base(base: u64) -> Limb {
    BASES[base as usize].3
}

// # Worst-case complexity
// Constant time and additional memory.
const fn get_big_base_inverted(base: u64) -> Limb {
    BASES[base as usize].4
}

// Compute the number of base-`base` digits corresponding to `limb_count` limbs, rounding down.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `DIGITS_IN_BASE_PER_LIMB` from `gmp-impl.h`, where `res` is returned.
fn digits_in_base_per_limb(limb_count: usize, base: u64) -> u64 {
    u64::exact_from(
        Limb::x_mul_y_to_zz(
            get_log_base_of_2(base),
            Limb::exact_from(limb_count) << Limb::LOG_WIDTH,
        )
        .0,
    )
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `DIGITS_IN_BASEGT2_FROM_BITS` from `gmp-impl.h`, GMP 6.2.1, where `res` is
// returned and `base` is not a power of 2.
fn limbs_digit_count_helper(bit_count: u64, base: u64) -> u64 {
    u64::exact_from(Limb::x_mul_y_to_zz(get_log_base_of_2(base) + 1, Limb::exact_from(bit_count)).0)
        .checked_add(1)
        .unwrap()
}

// The result is either exact or one too big.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `MPN_SIZEINBASE` from `gmp-impl.h`, GMP 6.2.1, where result is returned and
// base is not a power of 2.
pub_crate_test! {limbs_digit_count(xs: &[Limb], base: u64) -> u64 {
    assert!(base > 2);
    assert!(base < u64::wrapping_from(BASES.len()));
    assert!(!base.is_power_of_two());
    let size = xs.len();
    if size == 0 {
        1
    } else {
        limbs_digit_count_helper(
            (u64::exact_from(size) << Limb::LOG_WIDTH)
                - LeadingZeros::leading_zeros(*xs.last().unwrap()),
            base,
        )
    }
}}

macro_rules! base_10_normalization_step {
    ($j: expr, $buffer: ident, $i: ident, $frac: ident) => {
        if MP_BASES_NORMALIZATION_STEPS_10 <= $j {
            let digit;
            (digit, $frac) = Limb::x_mul_y_to_zz($frac, 10);
            $buffer[$i] = T::wrapping_from(digit);
            $i += 1;
        }
    };
}

// TODO tune
const RP_LEN: usize = if TUNE_PROGRAM_BUILD {
    GET_STR_THRESHOLD_LIMIT
} else {
    GET_STR_PRECOMPUTE_THRESHOLD
};

// Convert `xs` to digits in base `base`, and put the result in `out`. Generate `len` digits,
// possibly padding with zeros to the left. If `len` is zero, generate as many characters as
// required. Return the number of significant digits. Complexity is quadratic; intended for small
// conversions.
//
// - `base` must not be a power of 2, and 2 < `base` < 256.
// - `xs.len()` < `GET_STR_PRECOMPUTE_THRESHOLD`.
// - `len` must be at least as large as the actual number of digits.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_bc_get_str` from `mpn/generic/get_str.c`, GMP 6.2.1.
pub_crate_test! {limbs_to_digits_small_base_basecase<T: PrimitiveUnsigned>(
    out: &mut [T],
    len: usize,
    xs: &[Limb],
    base: u64,
) -> usize {
    assert!(base > 2);
    assert!(base < 256);
    assert!(out.len() >= len);
    let mut xs_len = xs.len();
    assert!(xs_len < GET_STR_PRECOMPUTE_THRESHOLD);
    // Allocate memory for largest possible string, given that we only get here for operands with
    // `xs_len` < GET_STR_PRECOMPUTE_THRESHOLD and that the smallest base is 3. 7 / 11 is an
    // approximation to 1 / log_2(3).
    const BUFFER_LEN: usize = (RP_LEN << Limb::LOG_WIDTH) * 7 / 11;
    let mut buffer = [T::ZERO; BUFFER_LEN];
    let mut rs = [0; RP_LEN];
    let mut i = BUFFER_LEN;
    if base == 10 {
        // Special case code for base 10 so that the compiler has a chance to optimize things.
        const DIGIT_SHIFT: u64 = Limb::WIDTH - 4;
        const LIMIT: usize = MP_BASES_CHARS_PER_LIMB_10
            - 4usize.wrapping_sub(MP_BASES_NORMALIZATION_STEPS_10 as usize);
        rs[1..=xs_len].copy_from_slice(xs);
        while xs_len > 1 {
            limbs_div_mod_extra_in_place(
                &mut rs[..=xs_len],
                1,
                MP_BASES_BIG_BASE_10,
                MP_BASES_BIG_BASE_INVERTED_10,
                MP_BASES_NORMALIZATION_STEPS_10,
            );
            if rs[xs_len] == 0 {
                xs_len -= 1;
            }
            let mut frac = rs[0].wrapping_add(1);
            i -= MP_BASES_CHARS_PER_LIMB_10;
            // Use the fact that 10 in binary is 1010, with the lowest bit 0. After a few
            // `x_mul_y_to_zz`s, we will have accumulated enough low zeros to use a plain multiply.
            base_10_normalization_step!(0, buffer, i, frac);
            base_10_normalization_step!(1, buffer, i, frac);
            base_10_normalization_step!(2, buffer, i, frac);
            base_10_normalization_step!(3, buffer, i, frac);
            frac.shr_round_assign(4, Ceiling);
            for _ in 0..LIMIT {
                frac *= 10;
                let digit = frac >> DIGIT_SHIFT;
                buffer[i] = T::wrapping_from(digit);
                i += 1;
                frac.mod_power_of_2_assign(DIGIT_SHIFT);
            }
            i -= MP_BASES_CHARS_PER_LIMB_10;
        }
        let mut r = rs[1];
        while r != 0 {
            let d = r.div_assign_mod(10);
            i -= 1;
            buffer[i] = T::wrapping_from(d);
        }
    } else {
        // not base 10
        let digits_per_limb = get_chars_per_limb(base);
        let big_base = get_big_base(base);
        let big_base_inverted = get_big_base_inverted(base);
        let normalization_steps = LeadingZeros::leading_zeros(big_base);
        let limb_base = Limb::wrapping_from(base);
        rs[1..=xs_len].copy_from_slice(&xs[..xs_len]);
        while xs_len > 1 {
            limbs_div_mod_extra_in_place(
                &mut rs[..=xs_len],
                1,
                big_base,
                big_base_inverted,
                normalization_steps,
            );
            if rs[xs_len] == 0 {
                xs_len -= 1;
            }
            let mut frac = rs[0].wrapping_add(1);
            let old_i = i;
            i -= digits_per_limb;
            for d in &mut buffer[i..old_i] {
                let digit;
                (digit, frac) = Limb::x_mul_y_to_zz(frac, limb_base);
                *d = T::wrapping_from(digit);
            }
        }
        let mut r = rs[1];
        while r != 0 {
            let digit = r.div_assign_mod(limb_base);
            i -= 1;
            buffer[i] = T::wrapping_from(digit);
        }
    }
    let nonzero_len = BUFFER_LEN - i;
    let zero_len = len.saturating_sub(nonzero_len); // Accounts for len == 0 case
    let (out_zero, out_nonzero) = out.split_at_mut(zero_len);
    slice_set_zero(out_zero);
    out_nonzero[..nonzero_len].copy_from_slice(&buffer[i..]);
    zero_len + nonzero_len
}}

// This is equivalent to `powers` from `gmp-impl.h`, GMP 6.2.1.
struct PowerTableIndicesRow {
    start: usize, // actual power value
    len: usize,
    shift: usize,          // weight of lowest limb, in limb base B
    digits_in_base: usize, // number of corresponding digits
}

#[cfg(feature = "test_build")]
pub struct PowerTableRow<'a> {
    power: &'a [Limb],
    shift: usize,          // weight of lowest limb, in limb base B
    digits_in_base: usize, // number of corresponding digits
}

#[cfg(not(feature = "test_build"))]
struct PowerTableRow<'a> {
    power: &'a [Limb],
    shift: usize,          // weight of lowest limb, in limb base B
    digits_in_base: usize, // number of corresponding digits
}

// TODO tune
const DIV_1_VS_MUL_1_PERCENT: usize = 150;

// TODO tune
const HAVE_MPN_COMPUTE_POWTAB_MUL: bool = DIV_1_VS_MUL_1_PERCENT > 120;

// TODO tune
const HAVE_MPN_COMPUTE_POWTAB_DIV: bool = DIV_1_VS_MUL_1_PERCENT < 275;

#[cfg(feature = "test_build")]
pub enum PowerTableAlgorithm {
    Mul,
    Div,
}

#[cfg(not(feature = "test_build"))]
enum PowerTableAlgorithm {
    Mul,
    Div,
}

// # Worst-case complexity
// $T(n) = O(\log n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs_len`.
//
// This is equivalent to `powtab_decide` from `mpn/compute_powtab.c`, GMP 6.2.1.
pub_test! {limbs_choose_power_table_algorithm(
    exptab: &mut [usize],
    xs_len: usize,
    base: u64,
) -> (usize, PowerTableAlgorithm) {
    let digits_per_limb = get_chars_per_limb(base);
    let mut number_of_powers = 0;
    let mut power: usize = xs_len.shr_round(1, Ceiling).0;
    while power != 1 {
        exptab[number_of_powers] = power * digits_per_limb;
        number_of_powers += 1;
        power = (power + 1) >> 1;
    }
    exptab[number_of_powers] = digits_per_limb;
    if HAVE_MPN_COMPUTE_POWTAB_MUL && HAVE_MPN_COMPUTE_POWTAB_DIV {
        let power = xs_len - 1;
        let n = xs_len.shr_round(1, Ceiling).0;
        let mut mul_cost = 1;
        let mut div_cost = 1;
        for i in (1..number_of_powers).rev() {
            let pow = (power >> i) + 1;
            if n != pow << (i - 1) {
                if pow.odd() {
                    div_cost += pow;
                }
                mul_cost += if pow > 2 && pow.even() { pow << 1 } else { pow };
            } else if pow.odd() {
                mul_cost += pow;
                div_cost += pow;
            }
        }
        div_cost = div_cost * DIV_1_VS_MUL_1_PERCENT / 100;
        (
            number_of_powers,
            if mul_cost <= div_cost {
                PowerTableAlgorithm::Mul
            } else {
                PowerTableAlgorithm::Div
            },
        )
    } else if HAVE_MPN_COMPUTE_POWTAB_MUL {
        (number_of_powers, PowerTableAlgorithm::Mul)
    } else if HAVE_MPN_COMPUTE_POWTAB_DIV {
        (number_of_powers, PowerTableAlgorithm::Div)
    } else {
        panic!("no powtab function available");
    }
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_str_powtab_alloc` from `gmp-impl.h`, GMP 6.2.1.
const fn limbs_digits_power_table_scratch_len(xs_len: usize) -> usize {
    xs_len + ((Limb::WIDTH as usize) << 1)
}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_dc_get_str_itch` from `gmp-impl.h`, GMP 6.2.1.
const fn limbs_to_digits_small_base_divide_and_conquer_scratch_len(xs_len: usize) -> usize {
    xs_len + (Limb::WIDTH as usize)
}

// # Worst-case complexity
// $T(n) = O(2^n n \log n)$
//
// $M(n) = O(2^n n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `power_len`.
//
// This is equivalent to `mpn_compute_powtab_mul` from `mpn/compute_powtab.c`, GMP 6.2.1.
pub_test! {limbs_compute_power_table_using_mul<'a>(
    power_table_memory: &'a mut [Limb],
    base: u64,
    exponents: &[usize],
    power_len: usize,
) -> Vec<PowerTableRow<'a>> {
    let mut power_indices = Vec::new();
    let big_base = get_big_base(base);
    let digits_per_limb = get_chars_per_limb(base);
    let mut digits_in_base = digits_per_limb;
    let (head, mut remainder) = power_table_memory.split_first_mut().unwrap();
    *head = big_base;
    (remainder[1], remainder[0]) = Limb::x_mul_y_to_zz(big_base, big_base);
    power_indices.push(PowerTableIndicesRow {
        start: 0,
        len: 1,
        digits_in_base,
        shift: 0,
    });
    // `a` and `n` are the start index and length of a power subslice.
    let (mut start, mut len, mut shift) = if remainder[0] == 0 {
        (2, 1, 1)
    } else {
        (1, 2, 0)
    };
    digits_in_base <<= 1;
    power_indices.push(PowerTableIndicesRow {
        start,
        len,
        shift,
        digits_in_base,
    });
    let start_index = if exponents[0] == digits_per_limb << power_len {
        let power;
        (power, remainder) = remainder[shift..].split_at_mut(len);
        let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(len)];
        limbs_square_to_out(remainder, power, &mut square_scratch);
        start = 3;
        isize::exact_from(power_len) - 2
    } else {
        if (digits_in_base + digits_per_limb) << (power_len - 2) <= exponents[0] {
            // a = 3, sometimes adjusted to 4.
            let power;
            (power, remainder) = remainder[shift..].split_at_mut(len);
            let carry = limbs_mul_limb_to_out(remainder, power, big_base);
            remainder[len] = carry;
            if carry != 0 {
                len += 1;
            }
            start = 3;
            digits_in_base += digits_per_limb;
            if remainder[1] == 0 {
                start = 4;
                len -= 1;
                shift += 1;
            }
            power_indices.push(PowerTableIndicesRow {
                start,
                len,
                shift,
                digits_in_base,
            });
            let power;
            (power, remainder) = remainder[start - 3..].split_at_mut(7 - start);
            let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(len)];
            limbs_square_to_out(remainder, &power[..len], &mut square_scratch);
            start = 7;
        } else {
            remainder[2] = remainder[start - 1];
            remainder[3] = remainder[start];
            remainder = &mut remainder[2..];
            power_indices.push(PowerTableIndicesRow {
                start: 3,
                len,
                digits_in_base,
                shift,
            });
            let power;
            (power, remainder) = remainder.split_at_mut(3);
            let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(len)];
            limbs_square_to_out(remainder, &power[..len], &mut square_scratch);
            start = 6;
        }
        isize::exact_from(power_len) - 3
    };
    if start_index >= 0 {
        for i in (0..=start_index).rev() {
            let increment = (len + 1) << 1;
            digits_in_base <<= 1;
            len <<= 1;
            if remainder[len - 1] == 0 {
                len -= 1;
            }
            shift <<= 1;
            let mut adjust = 0;
            if remainder[0] == 0 {
                len -= 1;
                shift += 1;
                remainder = &mut remainder[1..];
                adjust += 1;
            }
            // Adjust new value if it is too small as input to the next squaring.
            if (digits_in_base + digits_per_limb) << i <= exponents[0] {
                let carry = limbs_slice_mul_limb_in_place(&mut remainder[..len], big_base);
                remainder[len] = carry;
                if carry != 0 {
                    len += 1;
                }
                digits_in_base += digits_per_limb;
                if remainder[0] == 0 {
                    len -= 1;
                    shift += 1;
                    adjust += 1;
                    remainder = &mut remainder[1..];
                }
            }
            power_indices.push(PowerTableIndicesRow {
                start: start + adjust,
                len,
                digits_in_base,
                shift,
            });
            start += increment;
            let power;
            (power, remainder) = remainder.split_at_mut(increment - adjust);
            if i != 0 {
                let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(len)];
                limbs_square_to_out(remainder, &power[..len], &mut square_scratch);
            }
        }
        for (&exponent, row) in exponents[1..usize::exact_from(start_index + 2)]
            .iter()
            .rev()
            .zip(power_indices[power_len - usize::exact_from(start_index + 1)..].iter_mut())
        {
            if row.digits_in_base < exponent {
                let start = row.start;
                let end = start + row.len;
                let carry =
                    limbs_slice_mul_limb_in_place(&mut power_table_memory[start..end], big_base);
                power_table_memory[end] = carry;
                if carry != 0 {
                    row.len += 1;
                }
                assert!(row.digits_in_base + digits_per_limb == exponent);
                row.digits_in_base = exponent;
                if power_table_memory[start] == 0 {
                    row.start += 1;
                    row.len -= 1;
                    row.shift += 1;
                }
            }
        }
    }
    let mut powers = Vec::with_capacity(power_indices.len());
    let mut remainder: &mut [Limb] = power_table_memory;
    let mut consumed_len = 0;
    for row in power_indices {
        remainder = &mut remainder[row.start - consumed_len..];
        let power;
        (power, remainder) = remainder.split_at_mut(row.len);
        consumed_len = row.start + power.len();
        powers.push(PowerTableRow {
            power,
            digits_in_base: row.digits_in_base,
            shift: row.shift,
        });
    }
    powers
}}

// # Worst-case complexity
// $T(n) = O(2^n n \log n)$
//
// $M(n) = O(2^n n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `power_len`.
//
// This is equivalent to `mpn_compute_powtab_div` from `mpn/compute_powtab.c`, GMP 6.2.1.
pub_test! {limbs_compute_power_table_using_div<'a>(
    power_table_memory: &'a mut [Limb],
    base: u64,
    exponents: &[usize],
    power_len: usize,
) -> Vec<PowerTableRow<'a>> {
    let big_base = get_big_base(base);
    let digits_per_limb = get_chars_per_limb(base);
    let big_base_trailing_zeros = TrailingZeros::trailing_zeros(big_base);
    power_table_memory[0] = big_base;
    let mut powers = Vec::with_capacity(power_len + 1);
    let (mut power, mut remainder) = power_table_memory.split_at_mut(1);
    powers.push(PowerTableRow {
        power: &*power,
        digits_in_base: digits_per_limb,
        shift: 0,
    });
    let mut digits_in_base = digits_per_limb;
    let mut len = 1;
    let mut shift = 0;
    for &exp in exponents[..power_len].iter().rev() {
        let two_n = len << 1;
        let mut square_scratch = vec![0; limbs_square_to_out_scratch_len(power.len())];
        limbs_square_to_out(remainder, power, &mut square_scratch);
        len = two_n - 1;
        if remainder[len] != 0 {
            len += 1;
        }
        digits_in_base <<= 1;
        if digits_in_base != exp {
            limbs_div_exact_limb_in_place(&mut remainder[..len], big_base);
            if remainder[len - 1] == 0 {
                len -= 1;
            }
            digits_in_base -= digits_per_limb;
        }
        shift <<= 1;
        // Strip low zero limbs, but be careful to keep the result divisible by big_base.
        let mut adjust = 0;
        while remainder[adjust] == 0
            && remainder[adjust + 1].divisible_by_power_of_2(big_base_trailing_zeros)
        {
            adjust += 1;
        }
        len -= adjust;
        shift += adjust;
        remainder = &mut remainder[adjust..];
        let next_power;
        (next_power, remainder) = remainder.split_at_mut(two_n);
        power = &mut next_power[..len];
        powers.push(if power[0] == 0 {
            PowerTableRow {
                power: &power[1..],
                digits_in_base,
                shift: shift + 1,
            }
        } else {
            PowerTableRow {
                power,
                shift,
                digits_in_base,
            }
        });
    }
    powers
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs_len`.
//
// This is equivalent to `mpn_compute_powtab` from `mpn/compute_powtab.c`, GMP 6.2.1.
pub_test! {limbs_compute_power_table(
    power_table_memory: &mut [Limb],
    xs_len: usize,
    base: u64,
    forced_algorithm: Option<PowerTableAlgorithm>,
) -> (usize, Vec<PowerTableRow>) {
    let mut exponents = [0; Limb::WIDTH as usize];
    let (power_len, auto_algorithm) =
        limbs_choose_power_table_algorithm(&mut exponents, xs_len, base);
    let algorithm = forced_algorithm.unwrap_or(auto_algorithm);
    let powers = match algorithm {
        PowerTableAlgorithm::Mul => {
            limbs_compute_power_table_using_mul(power_table_memory, base, &exponents, power_len)
        }
        PowerTableAlgorithm::Div => {
            limbs_compute_power_table_using_div(power_table_memory, base, &exponents, power_len)
        }
    };
    (power_len, powers)
}}

// TODO tune
const GET_STR_DC_THRESHOLD: usize = 15;

// Convert `xs` to a string with a base as represented in `powers`, and put the string in `out`.
// Generate `len` characters, possibly padding with zeros to the left. If `len` is zero, generate as
// many characters as required. Return a pointer immediately after the last digit of the result
// string. This uses divide-and-conquer and is intended for large conversions.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_dc_get_str` from `mpn/generic/get_str.c`, GMP 6.2.1.
fn limbs_to_digits_small_base_divide_and_conquer<T: PrimitiveUnsigned>(
    out: &mut [T],
    mut len: usize,
    xs: &mut [Limb],
    base: u64,
    powers: &[PowerTableRow],
    i: usize,
    scratch: &mut [Limb],
) -> usize {
    let xs_len = xs.len();
    if xs_len < GET_STR_DC_THRESHOLD {
        if xs_len != 0 {
            limbs_to_digits_small_base_basecase(out, len, xs, base)
        } else {
            fail_on_untested_path("limbs_to_digits_small_base_divide_and_conquer, xs_len == 0");
            slice_set_zero(&mut out[..len]);
            len
        }
    } else {
        let power = &powers[i];
        let power_len = power.power.len();
        let shift = power.shift;
        let total_len = power_len + shift;
        if xs_len < total_len
            || xs_len == total_len && limbs_cmp_same_length(&xs[shift..], power.power) == Less
        {
            fail_on_untested_path(
                "limbs_to_digits_small_base_divide_and_conquer, \
                xs_len < total_len || \
                xs_len == total_len && \
                limbs_cmp_same_length(&xs[shift..], power.power) == Less",
            );
            limbs_to_digits_small_base_divide_and_conquer(
                out,
                len,
                xs,
                base,
                powers,
                i - 1,
                scratch,
            )
        } else {
            let power = &powers[i];
            limbs_div_mod_qs_to_out_rs_to_ns(scratch, &mut xs[shift..], power.power);
            let mut q_len = xs_len - total_len;
            if scratch[q_len] != 0 {
                q_len += 1;
            }
            assert!(
                q_len < total_len
                    || q_len == total_len
                        && limbs_cmp_same_length(&scratch[shift..total_len], power.power) == Less
            );
            if len != 0 {
                len -= powers[i].digits_in_base;
            }
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(q_len);
            let next_index = limbs_to_digits_small_base_divide_and_conquer(
                out,
                len,
                scratch_lo,
                base,
                powers,
                i - 1,
                scratch_hi,
            );
            limbs_to_digits_small_base_divide_and_conquer(
                &mut out[next_index..],
                power.digits_in_base,
                &mut xs[..total_len],
                base,
                powers,
                i - 1,
                scratch,
            ) + next_index
        }
    }
}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_crate_test! {limbs_to_digits_small_base_no_alg_specified<T: PrimitiveUnsigned>(
    out: &mut [T],
    base: u64,
    xs: &mut [Limb],
) -> usize {
    limbs_to_digits_small_base(out, base, xs, None)
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_get_str` from `mpn/generic/get_str.c`, GMP 6.2.1, where `un != 0` and
// base is not a power of 2.
pub_test! {limbs_to_digits_small_base<T: PrimitiveUnsigned>(
    out: &mut [T],
    base: u64,
    xs: &mut [Limb],
    forced_algorithm: Option<PowerTableAlgorithm>,
) -> usize {
    let xs_len = xs.len();
    if xs_len == 0 {
        0
    } else if xs_len < GET_STR_PRECOMPUTE_THRESHOLD {
        limbs_to_digits_small_base_basecase(out, 0, xs, base)
    } else {
        // Allocate one large block for the powers of big_base.
        let mut power_table_memory = vec![0; limbs_digits_power_table_scratch_len(xs_len)];
        // Compute a table of powers, were the largest power is >= sqrt(U).
        let digits_len = digits_in_base_per_limb(xs_len, base);
        let len = 1 + usize::exact_from(digits_len) / get_chars_per_limb(base);
        let (power_len, powers) =
            limbs_compute_power_table(&mut power_table_memory, len, base, forced_algorithm);
        // Using our precomputed powers, convert our number.
        let mut scratch =
            vec![0; limbs_to_digits_small_base_divide_and_conquer_scratch_len(xs_len)];
        limbs_to_digits_small_base_divide_and_conquer(
            out,
            0,
            xs,
            base,
            &powers,
            power_len,
            &mut scratch,
        )
    }
}}

// Returns digits in ascending order.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `digits.len()`.
pub_test! {limbs_to_digits_basecase<T: ConvertibleFrom<Limb> + PrimitiveUnsigned>(
    digits: &mut Vec<T>,
    xs: &mut [Limb],
    base: Limb,
) {
    assert!(base >= 2);
    assert!(xs.len() > 1);
    assert!(T::convertible_from(base));
    let mut digits_per_limb = 0;
    let mut big_base = 1;
    while let Some(next) = big_base.checked_mul(base) {
        big_base = next;
        digits_per_limb += 1;
    }
    while !slice_test_zero(xs) {
        let mut big_digit = limbs_div_limb_in_place_mod(xs, big_base);
        for _ in 0..digits_per_limb - 1 {
            digits.push(T::wrapping_from(big_digit.div_assign_mod(base)));
        }
        digits.push(T::wrapping_from(big_digit));
    }
    let trailing_zeros = slice_trailing_zeros(digits);
    digits.truncate(digits.len() - trailing_zeros);
}}

// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `digits.len()`.
pub_test! {to_digits_asc_naive_primitive<T: for<'a> ExactFrom<&'a Natural> + PrimitiveUnsigned>(
    digits: &mut Vec<T>,
    x: &Natural,
    base: T,
) where
    Natural: From<T>,
{
    assert!(base > T::ONE);
    let mut remainder = x.clone();
    let nat_base = Natural::from(base);
    while remainder != 0 {
        digits.push(T::exact_from(&remainder.div_assign_mod(&nat_base)));
    }
}}

// # Worst-case complexity
// $T(n) = O((n^2/m) \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `x.significant_bits()`, and $m$ is
// `base.significant_bits()`.
pub_test! {to_digits_asc_naive(digits: &mut Vec<Natural>, x: &Natural, base: &Natural) {
    assert!(*base > 1);
    let mut remainder = x.clone();
    while remainder != 0 {
        digits.push(remainder.div_assign_mod(base));
    }
}}

// TODO tune
const TO_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD: u64 = 50;

const SQRT_MAX_LIMB: Limb = (1 << (Limb::WIDTH >> 1)) - 1;

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
fn compute_powers_for_to_digits(base: &Natural, bits: u64) -> Vec<Natural> {
    if bits / base.significant_bits() < TO_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD {
        return Vec::new();
    }
    let limit = (bits + 3).shr_round(1, Ceiling).0;
    let mut powers = Vec::new();
    let mut power = base.clone();
    loop {
        powers.push(power.clone());
        power.square_assign();
        if power.significant_bits() >= limit {
            break;
        }
    }
    powers.push(power);
    powers
}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
fn to_digits_asc_divide_and_conquer_limb<
    T: ConvertibleFrom<Limb> + for<'a> ExactFrom<&'a Natural> + PrimitiveUnsigned,
>(
    digits: &mut Vec<T>,
    mut x: Natural,
    base: Limb,
    powers: &[Natural],
    power_index: usize,
) where
    Limb: Digits<T>,
    Natural: From<T>,
{
    let bits = x.significant_bits();
    if bits / base.significant_bits() < TO_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD {
        if base <= SQRT_MAX_LIMB {
            match x {
                Natural(Small(x)) => {
                    digits.extend_from_slice(&x.to_digits_asc(&T::wrapping_from(base)));
                }
                Natural(Large(ref mut xs)) => limbs_to_digits_basecase(digits, xs, base),
            }
        } else {
            to_digits_asc_naive_primitive(digits, &x, T::exact_from(base));
        }
    } else {
        let (q, r) = x.div_mod(&powers[power_index]);
        let start_len = digits.len();
        to_digits_asc_divide_and_conquer_limb(digits, r, base, powers, power_index - 1);
        if q != 0 {
            for _ in digits.len() - start_len..1 << power_index {
                digits.push(T::ZERO);
            }
            to_digits_asc_divide_and_conquer_limb(digits, q, base, powers, power_index - 1);
        }
    }
}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
fn to_digits_asc_divide_and_conquer(
    digits: &mut Vec<Natural>,
    x: &Natural,
    base: &Natural,
    powers: &[Natural],
    power_index: usize,
) {
    let bits = x.significant_bits();
    if bits / base.significant_bits() < TO_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD {
        to_digits_asc_naive(digits, x, base);
    } else {
        let (q, r) = x.div_mod(&powers[power_index]);
        let start_len = digits.len();
        to_digits_asc_divide_and_conquer(digits, &r, base, powers, power_index - 1);
        if q != 0 {
            for _ in digits.len() - start_len..1 << power_index {
                digits.push(Natural::ZERO);
            }
            to_digits_asc_divide_and_conquer(digits, &q, base, powers, power_index - 1);
        }
    }
}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
pub_test! {to_digits_asc_limb<
    T: ConvertibleFrom<Limb> + for<'a> ExactFrom<&'a Natural> + PrimitiveUnsigned,
>(
    x: &Natural,
    base: Limb,
) -> Vec<T>
where
    Limb: Digits<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    assert!(base >= 2);
    if let Some(log_base) = base.checked_log_base_2() {
        x.to_power_of_2_digits_asc(log_base)
    } else {
        let t_base = T::exact_from(base);
        match x {
            Natural(Small(x)) => x.to_digits_asc(&t_base),
            Natural(Large(xs)) => {
                if base < 256 {
                    let mut digits =
                        vec![
                            T::ZERO;
                            usize::exact_from(limbs_digit_count(xs, u64::wrapping_from(base)))
                        ];
                    let mut xs = xs.clone();
                    let len = limbs_to_digits_small_base(
                        &mut digits,
                        u64::wrapping_from(base),
                        &mut xs,
                        None,
                    );
                    digits.truncate(len);
                    digits.reverse();
                    digits
                } else {
                    let powers = compute_powers_for_to_digits(
                        &From::<Limb>::from(base),
                        x.significant_bits(),
                    );
                    let mut digits = Vec::new();
                    to_digits_asc_divide_and_conquer_limb(
                        &mut digits,
                        x.clone(),
                        base,
                        &powers,
                        powers.len().saturating_sub(1),
                    );
                    digits
                }
            }
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
pub_test! {to_digits_desc_limb<
    T: ConvertibleFrom<Limb> + for<'a> ExactFrom<&'a Natural> + PrimitiveUnsigned,
>(
    x: &Natural,
    base: Limb,
) -> Vec<T>
where
    Limb: Digits<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    assert!(base >= 2);
    if let Some(log_base) = base.checked_log_base_2() {
        x.to_power_of_2_digits_desc(log_base)
    } else {
        let t_base = T::exact_from(base);
        match x {
            Natural(Small(x)) => x.to_digits_desc(&t_base),
            Natural(Large(xs)) => {
                if base < 256 {
                    let mut digits =
                        vec![
                            T::ZERO;
                            usize::exact_from(limbs_digit_count(xs, u64::wrapping_from(base)))
                        ];
                    let mut xs = xs.clone();
                    let len = limbs_to_digits_small_base(
                        &mut digits,
                        u64::wrapping_from(base),
                        &mut xs,
                        None,
                    );
                    digits.truncate(len);
                    digits
                } else {
                    let powers = compute_powers_for_to_digits(
                        &From::<Limb>::from(base),
                        x.significant_bits(),
                    );
                    let mut digits = Vec::new();
                    to_digits_asc_divide_and_conquer_limb(
                        &mut digits,
                        x.clone(),
                        base,
                        &powers,
                        powers.len().saturating_sub(1),
                    );
                    digits.reverse();
                    digits
                }
            }
        }
    }
}}

// optimized for large base
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
pub_test! {to_digits_asc_large(x: &Natural, base: &Natural) -> Vec<Natural> {
    if *x == 0 {
        Vec::new()
    } else if x < base {
        vec![x.clone()]
    } else if let Some(log_base) = base.checked_log_base_2() {
        x.to_power_of_2_digits_asc(log_base)
    } else {
        match x {
            Natural(Large(_)) => {
                let powers = compute_powers_for_to_digits(base, x.significant_bits());
                let mut digits = Vec::new();
                to_digits_asc_divide_and_conquer(
                    &mut digits,
                    x,
                    base,
                    &powers,
                    powers.len().saturating_sub(1),
                );
                digits
            }
            _ => panic!("x must be large"),
        }
    }
}}

// optimized for large base
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
pub_test! {to_digits_desc_large(x: &Natural, base: &Natural) -> Vec<Natural> {
    if *x == 0 {
        Vec::new()
    } else if x < base {
        vec![x.clone()]
    } else if let Some(log_base) = base.checked_log_base_2() {
        x.to_power_of_2_digits_desc(log_base)
    } else {
        match x {
            Natural(Large(_)) => {
                let powers = compute_powers_for_to_digits(base, x.significant_bits());
                let mut digits = Vec::new();
                to_digits_asc_divide_and_conquer(
                    &mut digits,
                    x,
                    base,
                    &powers,
                    powers.len().saturating_sub(1),
                );
                digits.reverse();
                digits
            }
            _ => panic!("x must be large"),
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {from_digits_desc_naive_primitive<T: PrimitiveUnsigned>(
    xs: &[T],
    base: T
) -> Option<Natural>
where
    Natural: From<T>,
{
    assert!(base > T::ONE);
    let mut n = Natural::ZERO;
    let n_base = Natural::from(base);
    for &x in xs {
        if x >= base {
            return None;
        }
        n *= &n_base;
        n += Natural::from(x);
    }
    Some(n)
}}

// # Worst-case complexity
// $T(n) = O(m^2 n \log (m n) \log\log (m n))$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `base.significant_bits()`, and $m$ is
// `xs.len()`.
pub_test! {from_digits_desc_naive(xs: &[Natural], base: &Natural) -> Option<Natural> {
    assert!(*base > 1);
    let mut n = Natural::ZERO;
    for x in xs {
        if x >= base {
            return None;
        }
        n *= base;
        n += x;
    }
    Some(n)
}}

// Compute the number of limbs corresponding to `digit_count` base-`base` digits, rounding up.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `LIMBS_PER_DIGIT_IN_BASE` from `gmp-impl.h`, where `res` is returned and
// `base` is not a power of 2.
pub_test! {limbs_per_digit_in_base(digit_count: usize, base: u64) -> u64 {
    (u64::exact_from(Limb::x_mul_y_to_zz(get_log_2_of_base(base), Limb::exact_from(digit_count)).0)
        >> (Limb::LOG_WIDTH - 3))
        + 2
}}

// The input digits are in descending order.
//
// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_bc_set_str` from `mpn/generic/set_str.c`, GMP 6.2.1, where `base` is
// not a power of 2.
pub_test! {limbs_from_digits_small_base_basecase<T: PrimitiveUnsigned>(
    out: &mut [Limb],
    xs: &[T],
    base: u64,
) -> Option<usize>
where
    Limb: WrappingFrom<T>,
{
    let xs_len = xs.len();
    assert!(base > 2);
    assert!(base < 256);
    assert_ne!(xs_len, 0);
    let big_base = get_big_base(base);
    let digits_per_limb = get_chars_per_limb(base);
    let limb_base: Limb = base.wrapping_into();
    let t_base = T::wrapping_from(base);
    let mut size = 0;
    let mut i = 0;
    for chunk in xs[..xs_len - 1].chunks_exact(digits_per_limb) {
        let (&chunk_head, chunk_tail) = chunk.split_first().unwrap();
        if chunk_head >= t_base {
            return None;
        }
        let mut y = Limb::wrapping_from(chunk_head);
        if limb_base == 10 {
            // This is a common case. Help the compiler avoid multiplication.
            for &x in chunk_tail {
                if x >= t_base {
                    return None;
                }
                let x = Limb::wrapping_from(x);
                assert!(x < 10);
                y = y * 10 + x;
            }
        } else {
            for &x in chunk_tail {
                if x >= t_base {
                    return None;
                }
                let x = Limb::wrapping_from(x);
                assert!(x < limb_base);
                y = y * limb_base + x;
            }
        }
        if size == 0 {
            if y != 0 {
                out[0] = y;
                size = 1;
            }
        } else {
            let (out_last, out_init) = out[..=size].split_last_mut().unwrap();
            let mut carry = limbs_slice_mul_limb_in_place(out_init, big_base);
            if limbs_slice_add_limb_in_place(out_init, y) {
                carry += 1;
            }
            if carry != 0 {
                *out_last = carry;
                size += 1;
            }
        }
        i += digits_per_limb;
    }
    let mut big_base = limb_base;
    let (&remainder_head, remainder_tail) = xs[i..].split_first().unwrap();
    if remainder_head >= t_base {
        return None;
    }
    let mut y = Limb::wrapping_from(remainder_head);
    if limb_base == 10 {
        // This is a common case. Help the compiler avoid multiplication.
        for &x in remainder_tail {
            if x >= t_base {
                return None;
            }
            let x = Limb::wrapping_from(x);
            assert!(x < 10);
            y = y * 10 + x;
            big_base *= 10;
        }
    } else {
        for &x in remainder_tail {
            if x >= t_base {
                return None;
            }
            let x = Limb::wrapping_from(x);
            assert!(x < limb_base);
            y = y * limb_base + x;
            big_base *= limb_base;
        }
    }
    if size == 0 {
        if y != 0 {
            out[0] = y;
            size = 1;
        }
    } else {
        let (out_last, out_init) = out[..=size].split_last_mut().unwrap();
        let mut carry = limbs_slice_mul_limb_in_place(out_init, big_base);
        if limbs_slice_add_limb_in_place(out_init, y) {
            carry += 1;
        }
        if carry != 0 {
            *out_last = carry;
            size += 1;
        }
    }
    Some(size)
}}

// TODO tune

// must be greater than get_chars_per_limb(3), which is 40 for 64-bit build
const SET_STR_DC_THRESHOLD: usize = 7100;

// The input digits are in descending order.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_dc_set_str` from `mpn/generic/set_str.c`, GMP 6.2.1, where `base` is
// not a power of 2.
pub_test! {limbs_from_digits_small_base_divide_and_conquer<T: PrimitiveUnsigned>(
    out: &mut [Limb],
    xs: &[T],
    base: u64,
    powers: &[PowerTableRow],
    i: usize,
    scratch: &mut [Limb],
) -> Option<usize>
where
    Limb: WrappingFrom<T>,
{
    if i == 0 {
        return limbs_from_digits_small_base_basecase(out, xs, base);
    }
    let xs_len = xs.len();
    let power = &powers[i];
    let len_lo = power.digits_in_base;
    if xs_len <= len_lo {
        return if xs_len < SET_STR_DC_THRESHOLD {
            fail_on_untested_path(
                "limbs_from_digits_small_base_divide_and_conquer, xs_len < SET_STR_DC_THRESHOLD",
            );
            limbs_from_digits_small_base_basecase(out, xs, base)
        } else {
            limbs_from_digits_small_base_divide_and_conquer(out, xs, base, powers, i - 1, scratch)
        };
    }
    let len_hi = xs_len - len_lo;
    let (xs_lo, xs_hi) = xs.split_at(len_hi);
    assert!(len_lo >= len_hi);
    let out_len_hi = if len_hi < SET_STR_DC_THRESHOLD {
        limbs_from_digits_small_base_basecase(scratch, xs_lo, base)
    } else {
        limbs_from_digits_small_base_divide_and_conquer(scratch, xs_lo, base, powers, i - 1, out)
    }?;
    let shift = power.shift;
    let adjusted_power_len = power.power.len() + shift;
    if out_len_hi == 0 {
        // Zero +1 limb here, to avoid reading an allocated but uninitialized limb in
        // limbs_slice_add_limb_in_place below.
        slice_set_zero(&mut out[..=adjusted_power_len]);
    } else {
        let (out_lo, out_hi) = out.split_at_mut(shift);
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(power.power.len(), out_len_hi)];
        limbs_mul_to_out(out_hi, power.power, &scratch[..out_len_hi], &mut mul_scratch);
        slice_set_zero(out_lo);
    }
    let out_len_lo = if len_lo < SET_STR_DC_THRESHOLD {
        limbs_from_digits_small_base_basecase(scratch, xs_hi, base)
    } else {
        let (scratch_lo, scratch_hi) = scratch.split_at_mut(adjusted_power_len + 1);
        limbs_from_digits_small_base_divide_and_conquer(
            scratch_lo,
            xs_hi,
            base,
            powers,
            i - 1,
            scratch_hi,
        )
    }?;
    if out_len_lo != 0 {
        let (out_lo, out_hi) = out.split_at_mut(out_len_lo);
        if limbs_slice_add_same_length_in_place_left(out_lo, &scratch[..out_len_lo]) {
            assert!(!limbs_slice_add_limb_in_place(out_hi, 1));
        }
    }
    let mut n = out_len_hi + adjusted_power_len;
    if out[n - 1] == 0 {
        n -= 1;
    }
    Some(n)
}}

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpn_dc_set_str_itch` from `gmp-impl.h`, GMP 6.2.1.
const fn limbs_from_digits_small_base_divide_and_conquer_scratch_len(xs_len: usize) -> usize {
    xs_len + (Limb::WIDTH as usize)
}

// must be greater than get_chars_per_limb(3), which is 40 for 64-bit build
const SET_STR_PRECOMPUTE_THRESHOLD: usize = 7100;

// The input digits are in descending order.
//
// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_set_str` from `mpn/generic/set_str.c`, GMP 6.2.1, where `base` is not
// a power of 2.
pub_test! {limbs_from_digits_small_base<T: PrimitiveUnsigned>(
    out: &mut [Limb],
    xs: &[T],
    base: u64,
) -> Option<usize>
where
    Limb: WrappingFrom<T>,
{
    let xs_len = xs.len();
    if xs_len < SET_STR_PRECOMPUTE_THRESHOLD {
        limbs_from_digits_small_base_basecase(out, xs, base)
    } else {
        let chars_per_limb = get_chars_per_limb(base);
        let len = xs_len / chars_per_limb + 1;
        // Allocate one large block for the powers of big_base.
        let mut power_table_memory = vec![0; limbs_digits_power_table_scratch_len(len)];
        let (power_len, powers) =
            limbs_compute_power_table(&mut power_table_memory, len, base, None);
        let mut scratch = vec![0; limbs_from_digits_small_base_divide_and_conquer_scratch_len(len)];
        limbs_from_digits_small_base_divide_and_conquer(
            out,
            xs,
            base,
            &powers,
            power_len,
            &mut scratch,
        )
    }
}}

// # Worst-case complexity
// $T(n) = O(n^2)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {from_digits_desc_basecase<T: PrimitiveUnsigned>(xs: &[T], base: Limb) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
{
    assert!(base >= 2);
    let t_base = T::try_from(base).ok()?;
    let mut digits_per_limb = 0;
    let mut big_base = 1;
    while let Some(next) = big_base.checked_mul(base) {
        big_base = next;
        digits_per_limb += 1;
    }
    let big_big_base = Natural::from(big_base);
    let mut x = Natural::ZERO;
    for chunk in xs.rchunks(digits_per_limb).rev() {
        for &y in chunk {
            if y >= t_base {
                return None;
            }
        }
        let big_digit =
            Limb::from_digits_desc(&base, chunk.iter().map(|&x| Limb::wrapping_from(x)))?;
        x *= &big_big_base;
        x += Natural::from(big_digit);
    }
    Some(x)
}}

// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `digits`.
fn compute_powers_for_from_digits(base: &Natural, digits: usize) -> Vec<Natural> {
    if u64::exact_from(digits) * base.significant_bits() < FROM_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD
    {
        return Vec::new();
    }
    let limit = digits.shr_round(1u64, Ceiling).0;
    let mut powers = Vec::new();
    let mut power = base.clone();
    let mut p = 1;
    loop {
        powers.push(power.clone());
        if p >= limit {
            break;
        }
        power.square_assign();
        p <<= 1;
    }
    powers
}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
fn from_digits_desc_divide_and_conquer_limb<T: PrimitiveUnsigned>(
    xs: &[T],
    base: Limb,
    powers: &[Natural],
    power_index: usize,
) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
    Natural: From<T>,
{
    let xs_len = xs.len();
    let b = u64::exact_from(xs_len) * base.significant_bits();
    if power_index == 0 || b < FROM_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD {
        if base <= SQRT_MAX_LIMB {
            from_digits_desc_basecase(xs, base)
        } else {
            from_digits_desc_naive_primitive(xs, T::exact_from(base))
        }
    } else {
        let p = usize::power_of_2(power_index.exact_into());
        if xs_len <= p {
            from_digits_desc_divide_and_conquer_limb(xs, base, powers, power_index - 1)
        } else {
            let (xs_hi, xs_lo) = xs.split_at(xs_len - p);
            let out_hi =
                from_digits_desc_divide_and_conquer_limb(xs_hi, base, powers, power_index - 1)?;
            let out_lo =
                from_digits_desc_divide_and_conquer_limb(xs_lo, base, powers, power_index - 1)?;
            Some(out_hi * &powers[power_index] + out_lo)
        }
    }
}

// # Worst-case complexity
// $T(n, m) = O(nm (\log (nm))^2 \log\log (nm))$
//
// $M(n, m) = O(nm \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is
// `base.significant_bits()`.
pub_test! {from_digits_desc_divide_and_conquer(
    xs: &[Natural],
    base: &Natural,
    powers: &[Natural],
    power_index: usize,
) -> Option<Natural> {
    let xs_len = xs.len();
    if power_index == 0
        || u64::exact_from(xs_len) * base.significant_bits()
            < FROM_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD
    {
        from_digits_desc_naive(xs, base)
    } else {
        let p = usize::power_of_2(u64::exact_from(power_index));
        if xs_len <= p {
            from_digits_desc_divide_and_conquer(xs, base, powers, power_index - 1)
        } else {
            let (xs_hi, xs_lo) = xs.split_at(xs_len - p);
            let out_hi = from_digits_desc_divide_and_conquer(xs_hi, base, powers, power_index - 1)?;
            let out_lo = from_digits_desc_divide_and_conquer(xs_lo, base, powers, power_index - 1)?;
            Some(out_hi * &powers[power_index] + out_lo)
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {from_digits_asc_limb<I: Iterator<Item = T>, T: TryFrom<Limb> + PrimitiveUnsigned>(
    xs: I,
    base: Limb,
) -> Option<Natural>
where
    Limb: ExactFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    assert!(base >= 2);
    if let Some(log_base) = base.checked_log_base_2() {
        Natural::from_power_of_2_digits_asc(log_base, xs)
    } else {
        let mut xs = xs.collect_vec();
        T::try_from(base).ok()?;
        if xs.is_empty() {
            return Some(Natural::ZERO);
        }
        xs.reverse();
        if base < 256 {
            let u64_base = base.exact_into();
            let mut out = vec![0; usize::exact_from(limbs_per_digit_in_base(xs.len(), u64_base))];
            let len = limbs_from_digits_small_base(&mut out, &xs, u64_base)?;
            out.truncate(len);
            Some(Natural::from_owned_limbs_asc(out))
        } else {
            let t_base = T::wrapping_from(base);
            let powers = compute_powers_for_from_digits(&Natural::from(t_base), xs.len());
            from_digits_desc_divide_and_conquer_limb(
                &xs,
                base,
                &powers,
                powers.len().saturating_sub(1),
            )
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
#[allow(clippy::trait_duplication_in_bounds)]
fn from_digits_asc_limb_from_natural<
    I: Iterator<Item = Natural>,
    T: TryFrom<Limb> + for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
>(
    xs: I,
    base: Limb,
) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
    Natural: From<T>,
{
    assert!(base >= 2);
    if let Some(log_base) = base.checked_log_base_2() {
        Natural::from_power_of_2_digits_asc(log_base, xs)
    } else {
        let large_xs = xs;
        let mut xs = Vec::new();
        T::try_from(base).ok()?;
        for x in large_xs {
            xs.push(T::try_from(&x).ok()?);
        }
        if xs.is_empty() {
            return Some(Natural::ZERO);
        }
        xs.reverse();
        if base < 256 {
            let u64_base = base.exact_into();
            let mut out = vec![0; usize::exact_from(limbs_per_digit_in_base(xs.len(), u64_base))];
            let len = limbs_from_digits_small_base(&mut out, &xs, u64_base)?;
            out.truncate(len);
            Some(Natural::from_owned_limbs_asc(out))
        } else {
            let t_base = T::wrapping_from(base);
            let powers = compute_powers_for_from_digits(&Natural::from(t_base), xs.len());
            from_digits_desc_divide_and_conquer_limb(
                &xs,
                base,
                &powers,
                powers.len().saturating_sub(1),
            )
        }
    }
}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
pub_test! {from_digits_desc_limb<I: Iterator<Item = T>, T: PrimitiveUnsigned>(
    xs: I,
    base: Limb,
) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    assert!(base >= 2);
    if let Some(log_base) = base.checked_log_base_2() {
        Natural::from_power_of_2_digits_desc(log_base, xs)
    } else {
        let xs = xs.collect_vec();
        T::try_from(base).ok()?;
        if xs.is_empty() {
            return Some(Natural::ZERO);
        }
        if base < 256 {
            let u64_base = base.exact_into();
            let mut out = vec![0; usize::exact_from(limbs_per_digit_in_base(xs.len(), u64_base))];
            let len = limbs_from_digits_small_base(&mut out, &xs, u64_base)?;
            out.truncate(len);
            Some(Natural::from_owned_limbs_asc(out))
        } else {
            let t_base = T::wrapping_from(base);
            let powers = compute_powers_for_from_digits(&Natural::from(t_base), xs.len());
            from_digits_desc_divide_and_conquer_limb(
                &xs,
                base,
                &powers,
                powers.len().saturating_sub(1),
            )
        }
    }
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.count()`.
#[allow(clippy::trait_duplication_in_bounds)]
fn from_digits_desc_limb_from_natural<
    I: Iterator<Item = Natural>,
    T: TryFrom<Limb> + for<'a> TryFrom<&'a Natural> + PrimitiveUnsigned,
>(
    xs: I,
    base: Limb,
) -> Option<Natural>
where
    Limb: WrappingFrom<T>,
    Natural: From<T>,
{
    assert!(base >= 2);
    if let Some(log_base) = base.checked_log_base_2() {
        Natural::from_power_of_2_digits_desc(log_base, xs)
    } else {
        let large_xs = xs;
        let mut xs = Vec::new();
        T::try_from(base).ok()?;
        for x in large_xs {
            xs.push(T::try_from(&x).ok()?);
        }
        if xs.is_empty() {
            return Some(Natural::ZERO);
        }
        if base < 256 {
            let u64_base = base.exact_into();
            let mut out = vec![0; usize::exact_from(limbs_per_digit_in_base(xs.len(), u64_base))];
            let len = limbs_from_digits_small_base(&mut out, &xs, u64_base)?;
            out.truncate(len);
            Some(Natural::from_owned_limbs_asc(out))
        } else {
            let t_base = T::wrapping_from(base);
            let powers = compute_powers_for_from_digits(&Natural::from(t_base), xs.len());
            from_digits_desc_divide_and_conquer_limb(
                &xs,
                base,
                &powers,
                powers.len().saturating_sub(1),
            )
        }
    }
}

// # Worst-case complexity
// $T(n, m) = O(nm (\log (nm))^2 \log\log (nm))$
//
// $M(n, m) = O(nm \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
// `base.significant_bits()`.
//
// optimized for large base
pub_test! {from_digits_asc_large<I: Iterator<Item = Natural>>(
    xs: I,
    base: &Natural,
) -> Option<Natural> {
    if let Some(log_base) = base.checked_log_base_2() {
        Natural::from_power_of_2_digits_asc(log_base, xs)
    } else {
        let mut xs = xs.collect_vec();
        xs.reverse();
        let powers = compute_powers_for_from_digits(base, xs.len());
        from_digits_desc_divide_and_conquer(&xs, base, &powers, powers.len().saturating_sub(1))
    }
}}

// # Worst-case complexity
// $T(n, m) = O(nm (\log (nm))^2 \log\log (nm))$
//
// $M(n, m) = O(nm \log (nm))$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
// `base.significant_bits()`.
//
// optimized for large base
pub_test! {from_digits_desc_large<I: Iterator<Item = Natural>>(
    xs: I,
    base: &Natural,
) -> Option<Natural> {
    if let Some(log_base) = base.checked_log_base_2() {
        Natural::from_power_of_2_digits_desc(log_base, xs)
    } else {
        let xs = xs.collect_vec();
        let powers = compute_powers_for_from_digits(base, xs.len());
        from_digits_desc_divide_and_conquer(&xs, base, &powers, powers.len().saturating_sub(1))
    }
}}

impl Digits<u8> for Natural {
    /// Returns a [`Vec`] containing the digits of a [`Natural`] in ascending order (least- to
    /// most-significant).
    ///
    /// If the [`Natural`] is 0, the [`Vec`] is empty; otherwise, it ends with a nonzero digit.
    ///
    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or $d_{k-1} \neq
    /// 0$, and
    ///
    /// $$
    /// \sum_{i=0}^{k-1}b^i d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// See [here](super::general_digits#to_digits_asc).
    #[inline]
    fn to_digits_asc(&self, base: &u8) -> Vec<u8> {
        match self {
            Natural(Small(x)) => x.to_digits_asc(base),
            Natural(Large(xs)) => {
                if let Some(log_base) = base.checked_log_base_2() {
                    self.to_power_of_2_digits_asc(log_base)
                } else {
                    let mut digits =
                        vec![0; usize::exact_from(limbs_digit_count(xs, u64::from(*base)))];
                    let mut xs = xs.clone();
                    let len =
                        limbs_to_digits_small_base(&mut digits, u64::from(*base), &mut xs, None);
                    digits.truncate(len);
                    digits.reverse();
                    digits
                }
            }
        }
    }

    /// Returns a [`Vec`] containing the digits of a [`Natural`] in descending order (most- to
    /// least-significant).
    ///
    /// If the [`Natural`] is 0, the [`Vec`] is empty; otherwise, it begins with a nonzero digit.
    ///
    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or $d_{k-1} \neq
    /// 0$, and
    ///
    /// $$
    /// \sum_{i=0}^{k-1}b^i d_{k-i-1} = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// See [here](super::general_digits#to_digits_desc).
    #[inline]
    fn to_digits_desc(&self, base: &u8) -> Vec<u8> {
        match self {
            Natural(Small(x)) => x.to_digits_desc(base),
            Natural(Large(xs)) => {
                if let Some(log_base) = base.checked_log_base_2() {
                    self.to_power_of_2_digits_desc(log_base)
                } else {
                    let mut digits =
                        vec![0; usize::exact_from(limbs_digit_count(xs, u64::from(*base)))];
                    let mut xs = xs.clone();
                    let len =
                        limbs_to_digits_small_base(&mut digits, u64::from(*base), &mut xs, None);
                    digits.truncate(len);
                    digits
                }
            }
        }
    }

    /// Converts an iterator of digits into a [`Natural`].
    ///
    /// The input digits are in ascending order (least- to most-significant). The function returns
    /// `None` if any of the digits are greater than or equal to the base.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^id_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// See [here](super::general_digits#from_digits_asc).
    #[inline]
    fn from_digits_asc<I: Iterator<Item = u8>>(base: &u8, digits: I) -> Option<Natural> {
        if let Some(log_base) = base.checked_log_base_2() {
            Natural::from_power_of_2_digits_asc(log_base, digits)
        } else {
            let base = u64::from(*base);
            let mut xs = digits.collect_vec();
            if xs.is_empty() {
                return Some(Natural::ZERO);
            }
            xs.reverse();
            let mut out = vec![0; usize::exact_from(limbs_per_digit_in_base(xs.len(), base))];
            let _ = limbs_from_digits_small_base(&mut out, &xs, base)?;
            Some(Natural::from_owned_limbs_asc(out))
        }
    }

    /// Converts an iterator of digits into a [`Natural`].
    ///
    /// The input digits are in descending order (most- to least-significant). The function returns
    /// `None` if any of the digits are greater than or equal to the base.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^{k-i-1}d_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// See [here](super::general_digits#from_digits_desc).
    #[inline]
    fn from_digits_desc<I: Iterator<Item = u8>>(base: &u8, digits: I) -> Option<Natural> {
        if let Some(log_base) = base.checked_log_base_2() {
            Natural::from_power_of_2_digits_desc(log_base, digits)
        } else {
            let base = u64::from(*base);
            let xs = digits.collect_vec();
            if xs.is_empty() {
                return Some(Natural::ZERO);
            }
            let mut out = vec![0; usize::exact_from(limbs_per_digit_in_base(xs.len(), base))];
            let _ = limbs_from_digits_small_base(&mut out, &xs, base)?;
            Some(Natural::from_owned_limbs_asc(out))
        }
    }
}

fn to_digits_asc_unsigned<
    T: for<'a> TryFrom<&'a Natural>
        + ConvertibleFrom<Limb>
        + PrimitiveUnsigned
        + for<'a> WrappingFrom<&'a Natural>,
>(
    x: &Natural,
    base: &T,
) -> Vec<T>
where
    Limb: TryFrom<T> + Digits<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    if let Ok(base) = Limb::try_from(*base) {
        to_digits_asc_limb(x, base)
    } else {
        to_digits_asc_large(x, &Natural::from(*base))
            .into_iter()
            .map(|n| T::wrapping_from(&n))
            .collect()
    }
}

fn to_digits_desc_unsigned<
    T: for<'a> TryFrom<&'a Natural>
        + ConvertibleFrom<Limb>
        + PrimitiveUnsigned
        + for<'a> WrappingFrom<&'a Natural>,
>(
    x: &Natural,
    base: &T,
) -> Vec<T>
where
    Limb: TryFrom<T> + Digits<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    if let Ok(base) = Limb::try_from(*base) {
        to_digits_desc_limb(x, base)
    } else {
        to_digits_desc_large(x, &Natural::from(*base))
            .into_iter()
            .map(|n| T::wrapping_from(&n))
            .collect()
    }
}

fn from_digits_asc_unsigned<T: ConvertibleFrom<Limb> + PrimitiveUnsigned, I: Iterator<Item = T>>(
    base: &T,
    digits: I,
) -> Option<Natural>
where
    Limb: TryFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    if let Ok(base) = Limb::try_from(*base) {
        from_digits_asc_limb(digits, base)
    } else {
        from_digits_asc_large(digits.map(Natural::from), &Natural::from(*base))
    }
}

fn from_digits_desc_unsigned<T: ConvertibleFrom<Limb> + PrimitiveUnsigned, I: Iterator<Item = T>>(
    base: &T,
    digits: I,
) -> Option<Natural>
where
    Limb: TryFrom<T> + WrappingFrom<T>,
    Natural: From<T> + PowerOf2Digits<T>,
{
    if let Ok(base) = Limb::try_from(*base) {
        from_digits_desc_limb(digits, base)
    } else {
        from_digits_desc_large(digits.map(Natural::from), &Natural::from(*base))
    }
}

macro_rules! digits_unsigned {
    ($d: ident) => {
        impl Digits<$d> for Natural {
            /// Returns a [`Vec`] containing the digits of a [`Natural`] in ascending order (least-
            /// to most-significant).
            ///
            /// If the [`Natural`] is 0, the [`Vec`] is empty; otherwise, it ends with a nonzero
            /// digit.
            ///
            /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or
            /// $d_{k-1} \neq 0$, and
            ///
            /// $$
            /// \sum_{i=0}^{k-1}b^i d_i = x.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n (\log n)^2 \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2.
            ///
            /// # Examples
            /// See [here](super::general_digits#to_digits_asc).
            #[inline]
            fn to_digits_asc(&self, base: &$d) -> Vec<$d> {
                to_digits_asc_unsigned(self, base)
            }

            /// Returns a [`Vec`] containing the digits of a [`Natural`] in descending order (most-
            /// to least-significant).
            ///
            /// If the [`Natural`] is 0, the [`Vec`] is empty; otherwise, it begins with a nonzero
            /// digit.
            ///
            /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or
            /// $d_{k-1} \neq 0$, and
            ///
            /// $$
            /// \sum_{i=0}^{k-1}b^i d_{k-i-1} = x.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n (\log n)^2 \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2.
            ///
            /// # Examples
            /// See [here](super::general_digits#to_digits_desc).
            #[inline]
            fn to_digits_desc(&self, base: &$d) -> Vec<$d> {
                to_digits_desc_unsigned(self, base)
            }

            /// Converts an iterator of digits into a [`Natural`].
            ///
            /// The input digits are in ascending order (least- to most-significant). The function
            /// returns `None` if any of the digits are greater than or equal to the base.
            ///
            /// $$
            /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^id_i.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n (\log n)^2 \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2.
            ///
            /// # Examples
            /// See [here](super::general_digits#from_digits_asc).
            #[inline]
            fn from_digits_asc<I: Iterator<Item = $d>>(base: &$d, digits: I) -> Option<Natural> {
                from_digits_asc_unsigned(base, digits)
            }

            /// Converts an iterator of digits into a [`Natural`].
            ///
            /// The input digits are in descending order (most- to least-significant). The function
            /// returns `None` if any of the digits are greater than or equal to the base.
            ///
            /// $$
            /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^{k-i-1}d_i.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n (\log n)^2 \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `digits.count()`.
            ///
            /// # Panics
            /// Panics if `base` is less than 2.
            ///
            /// # Examples
            /// See [here](super::general_digits#to_digits_desc).
            #[inline]
            fn from_digits_desc<I: Iterator<Item = $d>>(base: &$d, digits: I) -> Option<Natural> {
                from_digits_desc_unsigned(base, digits)
            }
        }
    };
}
digits_unsigned!(u16);
digits_unsigned!(u32);
digits_unsigned!(u64);
digits_unsigned!(u128);
digits_unsigned!(usize);

impl Digits<Natural> for Natural {
    /// Returns a [`Vec`] containing the digits of a [`Natural`] in ascending order (least- to
    /// most-significant).
    ///
    /// If the [`Natural`] is 0, the [`Vec`] is empty; otherwise, it ends with a nonzero digit.
    ///
    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or $d_{k-1} \neq
    /// 0$, and
    ///
    /// $$
    /// \sum_{i=0}^{k-1}b^i d_i = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::num::conversion::traits::Digits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO
    ///         .to_digits_asc(&Natural::from(6u32))
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     Natural::TWO
    ///         .to_digits_asc(&Natural::from(6u32))
    ///         .to_debug_string(),
    ///     "[2]"
    /// );
    /// assert_eq!(
    ///     Natural::from(123456u32)
    ///         .to_digits_asc(&Natural::from(3u32))
    ///         .to_debug_string(),
    ///     "[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2]"
    /// );
    /// ```
    fn to_digits_asc(&self, base: &Natural) -> Vec<Natural> {
        match base {
            Natural(Small(b)) => self
                .to_digits_asc(b)
                .into_iter()
                .map(Natural::from)
                .collect(),
            _ => to_digits_asc_large(self, base),
        }
    }

    /// Returns a [`Vec`] containing the digits of a [`Natural`] in descending order (most- to
    /// least-significant).
    ///
    /// If the [`Natural`] is 0, the [`Vec`] is empty; otherwise, it begins with a nonzero digit.
    ///
    /// $f(x, b) = (d_i)_ {i=0}^{k-1}$, where $0 \leq d_i < b$ for all $i$, $k=0$ or $d_{k-1} \neq
    /// 0$, and
    ///
    /// $$
    /// \sum_{i=0}^{k-1}b^i d_{k-i-1} = x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Two, Zero};
    /// use malachite_base::num::conversion::traits::Digits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO
    ///         .to_digits_desc(&Natural::from(6u32))
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// assert_eq!(
    ///     Natural::TWO
    ///         .to_digits_desc(&Natural::from(6u32))
    ///         .to_debug_string(),
    ///     "[2]"
    /// );
    /// assert_eq!(
    ///     Natural::from(123456u32)
    ///         .to_digits_desc(&Natural::from(3u32))
    ///         .to_debug_string(),
    ///     "[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0]"
    /// );
    /// ```
    fn to_digits_desc(&self, base: &Natural) -> Vec<Natural> {
        match base {
            Natural(Small(b)) => self
                .to_digits_desc(b)
                .into_iter()
                .map(Natural::from)
                .collect(),
            _ => to_digits_desc_large(self, base),
        }
    }

    /// Converts an iterator of digits into a [`Natural`].
    ///
    /// The input digits are in ascending order (least- to most-significant). The function returns
    /// `None` if any of the digits are greater than or equal to the base.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^id_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm (\log (nm))^2 \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `digits.count()`, and $m$ is
    /// `base.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::Digits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from_digits_asc(
    ///         &Natural::from(64u32),
    ///         vec_from_str::<Natural>("[0, 0, 0]").unwrap().into_iter()
    ///     )
    ///     .to_debug_string(),
    ///     "Some(0)"
    /// );
    /// assert_eq!(
    ///     Natural::from_digits_asc(
    ///         &Natural::from(3u32),
    ///         vec_from_str::<Natural>("[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2]")
    ///             .unwrap()
    ///             .into_iter()
    ///     )
    ///     .to_debug_string(),
    ///     "Some(123456)"
    /// );
    /// assert_eq!(
    ///     Natural::from_digits_asc(
    ///         &Natural::from(8u32),
    ///         vec_from_str::<Natural>("[3, 7, 1]").unwrap().into_iter()
    ///     )
    ///     .to_debug_string(),
    ///     "Some(123)"
    /// );
    /// assert_eq!(
    ///     Natural::from_digits_asc(
    ///         &Natural::from(8u32),
    ///         vec_from_str::<Natural>("[1, 10, 3]").unwrap().into_iter()
    ///     )
    ///     .to_debug_string(),
    ///     "None"
    /// );
    /// ```
    #[inline]
    fn from_digits_asc<I: Iterator<Item = Natural>>(base: &Natural, digits: I) -> Option<Natural> {
        match base {
            Natural(Small(b)) => from_digits_asc_limb_from_natural::<_, Limb>(digits, *b),
            _ => from_digits_asc_large(digits, base),
        }
    }

    /// Converts an iterator of digits into a [`Natural`].
    ///
    /// The input digits are in descending order (most- to least-significant). The function returns
    /// `None` if any of the digits are greater than or equal to the base.
    ///
    /// $$
    /// f((d_i)_ {i=0}^{k-1}, b) = \sum_{i=0}^{k-1}b^{k-i-1}d_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm (\log (nm))^2 \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `digits.count()`, and $m$ is
    /// `base.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::Digits;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from_digits_desc(
    ///         &Natural::from(64u32),
    ///         vec_from_str::<Natural>("[0, 0, 0]").unwrap().into_iter()
    ///     )
    ///     .to_debug_string(),
    ///     "Some(0)"
    /// );
    /// assert_eq!(
    ///     Natural::from_digits_desc(
    ///         &Natural::from(3u32),
    ///         vec_from_str::<Natural>("[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0]")
    ///             .unwrap()
    ///             .into_iter()
    ///     )
    ///     .to_debug_string(),
    ///     "Some(123456)"
    /// );
    /// assert_eq!(
    ///     Natural::from_digits_desc(
    ///         &Natural::from(8u32),
    ///         vec_from_str::<Natural>("[1, 7, 3]").unwrap().into_iter()
    ///     )
    ///     .to_debug_string(),
    ///     "Some(123)"
    /// );
    /// assert_eq!(
    ///     Natural::from_digits_desc(
    ///         &Natural::from(8u32),
    ///         vec_from_str::<Natural>("[3, 10, 1]").unwrap().into_iter()
    ///     )
    ///     .to_debug_string(),
    ///     "None"
    /// );
    /// ```
    #[inline]
    fn from_digits_desc<I: Iterator<Item = Natural>>(base: &Natural, digits: I) -> Option<Natural> {
        match base {
            Natural(Small(b)) => from_digits_desc_limb_from_natural::<_, Limb>(digits, *b),
            _ => from_digits_desc_large(digits, base),
        }
    }
}
