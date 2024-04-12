// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{CheckedSquare, DivisibleBy, FloorSqrt, Parity};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::factorization::prime_sieve::{id_to_n, limbs_count_ones, n_to_bit};
use crate::num::logic::traits::{NotAssign, TrailingZeros};
use crate::slices::slice_leading_zeros;

// Replace with good primality test once we have one
fn is_prime_simple(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        n if n.even() => false,
        n => {
            for f in 3..=n.floor_sqrt() {
                if n.divisible_by(f) {
                    return false;
                }
            }
            true
        }
    }
}

pub fn limbs_prime_sieve_naive_1<T: PrimitiveUnsigned>(bit_array: &mut [T], n: u64) -> u64 {
    assert!(n > 4);
    let mut f = 5;
    let mut b = false;
    'outer: for x in &mut *bit_array {
        *x = T::MAX;
        for i in 0..T::WIDTH {
            if is_prime_simple(f) {
                x.clear_bit(i);
            }
            f += if b { 4 } else { 2 };
            if f > n {
                break 'outer;
            }
            b.not_assign();
        }
    }
    (u64::exact_from(bit_array.len()) << T::LOG_WIDTH) - limbs_count_ones(bit_array)
}

fn limbs_index_of_next_true_bit<T: PrimitiveUnsigned>(xs: &[T], start: u64) -> Option<u64> {
    let starting_index = usize::exact_from(start >> T::LOG_WIDTH);
    if starting_index >= xs.len() {
        None
    } else if let Some(result) = xs[starting_index].index_of_next_true_bit(start & T::WIDTH_MASK) {
        Some((u64::wrapping_from(starting_index) << T::LOG_WIDTH) + result)
    } else if starting_index == xs.len() - 1 {
        None
    } else {
        let true_index = starting_index + 1 + slice_leading_zeros(&xs[starting_index + 1..]);
        if true_index == xs.len() {
            None
        } else {
            let result_offset = u64::wrapping_from(true_index) << T::LOG_WIDTH;
            Some(
                result_offset
                    .checked_add(TrailingZeros::trailing_zeros(xs[true_index]))
                    .unwrap(),
            )
        }
    }
}

fn limbs_set_bit_helper<T: PrimitiveUnsigned>(xs: &mut [T], index: u64, limb_index: usize) {
    xs[limb_index].set_bit(index & T::WIDTH_MASK);
}

fn limbs_slice_set_bit<T: PrimitiveUnsigned>(xs: &mut [T], index: u64) {
    limbs_set_bit_helper(xs, index, usize::exact_from(index >> T::LOG_WIDTH));
}

fn limbs_clear_bit<T: PrimitiveUnsigned>(xs: &mut [T], index: u64) {
    let small_index = usize::exact_from(index >> T::LOG_WIDTH);
    if small_index < xs.len() {
        xs[small_index].clear_bit(index & T::WIDTH_MASK);
    }
}

pub fn limbs_prime_sieve_naive_2<T: PrimitiveUnsigned>(bit_array: &mut [T], n: u64) -> u64 {
    assert!(n > 4);
    for x in &mut *bit_array {
        *x = T::MAX;
    }
    let mut p = 0;
    loop {
        let id = limbs_index_of_next_true_bit(bit_array, if p == 0 { 0 } else { n_to_bit(p) + 1 });
        if let Some(id) = id {
            p = id_to_n(id + 1);
        } else {
            break;
        }
        let m = p.checked_square();
        if m.is_none() {
            break;
        }
        let mut m = m.unwrap();
        if m > n {
            break;
        }
        let two_p = p << 1;
        while m <= n {
            if m.odd() && !m.divisible_by(3) {
                limbs_clear_bit(bit_array, n_to_bit(m));
            }
            m += two_p;
        }
    }
    for x in &mut *bit_array {
        x.not_assign();
    }
    let bit_len = u64::exact_from(bit_array.len()) << T::LOG_WIDTH;
    for i in (0..bit_len).rev() {
        if id_to_n(i + 1) <= n {
            break;
        }
        limbs_slice_set_bit(bit_array, i);
    }
    bit_len - limbs_count_ones(bit_array)
}
