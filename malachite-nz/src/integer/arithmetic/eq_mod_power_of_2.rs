// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2001, 2002, 2013 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::EqModPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns whether
// the negative of the `Natural` is equivalent to a limb mod two to the power of `pow`; that is,
// whether the `pow` least-significant bits of the negative of the `Natural` and the limb are equal.
//
// This function assumes that `limbs` has length at least 2 and the last (most significant) limb is
// nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_test! {limbs_eq_mod_power_of_2_neg_limb(xs: &[Limb], y: Limb, pow: u64) -> bool {
    if y == 0 {
        return limbs_divisible_by_power_of_2(xs, pow);
    }
    let i = usize::exact_from(pow >> Limb::LOG_WIDTH);
    match i.cmp(&xs.len()) {
        Greater => false,
        Equal => {
            if pow & Limb::WIDTH_MASK == 0 {
                // Check whether the sum of X and y is 0 mod B ^ xs.len().
                let mut carry = y;
                for &x in xs {
                    let sum = x.wrapping_add(carry);
                    if sum != 0 {
                        return false;
                    }
                    carry = 1;
                }
                true
            } else {
                false
            }
        }
        Less => {
            if i == 0 {
                xs[0].eq_mod_power_of_2(y.wrapping_neg(), pow)
            } else {
                xs[0] == y.wrapping_neg()
                    && xs[1..i].iter().all(|&x| x == Limb::MAX)
                    && xs[i].eq_mod_power_of_2(Limb::MAX, pow & Limb::WIDTH_MASK)
            }
        }
    }
}}

fn limbs_eq_mod_power_of_2_neg_pos_greater(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    let xs_len = xs.len();
    let i = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let small_pow = pow & Limb::WIDTH_MASK;
    if i > xs_len || i == xs_len && small_pow != 0 {
        false
    } else {
        let ys_len = ys.len();
        let mut y_nonzero_seen = false;
        for j in 0..i {
            let y = if j >= ys_len {
                Limb::MAX
            } else if y_nonzero_seen {
                !ys[j]
            } else if ys[j] == 0 {
                0
            } else {
                y_nonzero_seen = true;
                ys[j].wrapping_neg()
            };
            if xs[j] != y {
                return false;
            }
        }
        if small_pow == 0 {
            true
        } else {
            // i < xs_len
            let y = if i >= ys_len {
                Limb::MAX
            } else if y_nonzero_seen {
                !ys[i]
            } else {
                ys[i].wrapping_neg()
            };
            xs[i].eq_mod_power_of_2(y, small_pow)
        }
    }
}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
// whether the first `Natural` and the negative of the second natural (equivalently, the negative of
// the first `Natural` and the second `Natural`) are equivalent mod two to the power of `pow`; that
// is, whether their `pow` least-significant bits are equal.
//
// This function assumes that neither slice is empty and their last elements are nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_congruent_2exp_p` from `mpz/cong_2exp.c`, GMP 6.2.1, where `a` is
// negative and `c` is positive.
pub_test! {limbs_eq_mod_power_of_2_neg_pos(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_power_of_2_neg_pos_greater(xs, ys, pow)
    } else {
        limbs_eq_mod_power_of_2_neg_pos_greater(ys, xs, pow)
    }
}}

impl Natural {
    fn eq_mod_power_of_2_neg_limb(&self, other: Limb, pow: u64) -> bool {
        match *self {
            Natural(Small(ref small)) => {
                pow <= Limb::WIDTH && small.wrapping_neg().eq_mod_power_of_2(other, pow)
            }
            Natural(Large(ref limbs)) => limbs_eq_mod_power_of_2_neg_limb(limbs, other, pow),
        }
    }

    fn eq_mod_power_of_2_neg_pos(&self, other: &Natural, pow: u64) -> bool {
        match (self, other) {
            (_, &Natural(Small(y))) => self.eq_mod_power_of_2_neg_limb(y, pow),
            (&Natural(Small(x)), _) => other.eq_mod_power_of_2_neg_limb(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                limbs_eq_mod_power_of_2_neg_pos(xs, ys, pow)
            }
        }
    }
}

impl EqModPowerOf2<&Integer> for &Integer {
    /// Returns whether one [`Integer`] is equal to another modulo $2^k$; that is, whether their $k$
    /// least-significant bits (in two's complement) are equal.
    ///
    /// $f(x, y, k) = (x \equiv y \mod 2^k)$.
    ///
    /// $f(x, y, k) = (\exists n \in \Z : x - y = n2^k)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(pow, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::EqModPowerOf2;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::ZERO.eq_mod_power_of_2(&Integer::from(-256), 8),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from(-0b1101).eq_mod_power_of_2(&Integer::from(0b11011), 3),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from(-0b1101).eq_mod_power_of_2(&Integer::from(0b11011), 4),
    ///     false
    /// );
    /// ```
    fn eq_mod_power_of_2(self, other: &Integer, pow: u64) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod_power_of_2(&other.abs, pow)
        } else {
            self.abs.eq_mod_power_of_2_neg_pos(&other.abs, pow)
        }
    }
}
