// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::XXAddYYToZZ;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::SignificantBits;

// Returns the number of significant bits of the sum of the absolute values of the elements of `xs`,
// and of the largest absolute value. While every element fits in one limb, their absolute values
// are added in two limbs and or-ed together, so nothing is allocated; at the first element that
// does not, the sum is redone exactly as a `Natural`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements.
//
// This is equivalent to `_fmpz_vec_sum_max_bits` from `fmpz_vec/sum_max_bits.c`, FLINT 3.6.0, whose
// fast path takes the elements that fit in a signed word; this one takes every one-limb element.
crate_test_fn! {vec_sum_max_bits<C: PolynomialCoefficient>(xs: &[C]) -> (u64, u64) {
    let mut sum_hi: Limb = 0;
    let mut sum_lo: Limb = 0;
    let mut or: Limb = 0;
    for x in xs {
        match *x.unsigned_abs_ref() {
            Natural(Small(small)) => {
                (sum_hi, sum_lo) = Limb::xx_add_yy_to_zz(sum_hi, sum_lo, 0, small);
                or |= small;
            }
            Natural(Large(_)) => {
                let sum: Natural = xs.iter().map(PolynomialCoefficient::unsigned_abs_ref).sum();
                return (sum.significant_bits(), vec_max_bits(xs).0);
            }
        }
    }
    let sum_bits = if sum_hi == 0 {
        sum_lo.significant_bits()
    } else {
        Limb::WIDTH + sum_hi.significant_bits()
    };
    (sum_bits, or.significant_bits())
}}

// Returns the number of significant bits of the sum of the absolute values of the elements of `xs`.
// While every element fits in one limb, their absolute values are added in two limbs, so nothing is
// allocated; at the first element that does not, the sum is redone exactly as a `Natural`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements.
//
// This is the first result of `_fmpz_vec_sum_max_bits` from `fmpz_vec/sum_max_bits.c`, FLINT 3.6.0.
crate_test_fn! {vec_sum_abs_significant_bits<C: PolynomialCoefficient>(xs: &[C]) -> u64 {
    let mut sum_hi: Limb = 0;
    let mut sum_lo: Limb = 0;
    for x in xs {
        match *x.unsigned_abs_ref() {
            Natural(Small(small)) => {
                (sum_hi, sum_lo) = Limb::xx_add_yy_to_zz(sum_hi, sum_lo, 0, small);
            }
            Natural(Large(_)) => {
                return xs
                    .iter()
                    .map(PolynomialCoefficient::unsigned_abs_ref)
                    .sum::<Natural>()
                    .significant_bits();
            }
        }
    }
    if sum_hi == 0 {
        sum_lo.significant_bits()
    } else {
        Limb::WIDTH + sum_hi.significant_bits()
    }
}}
