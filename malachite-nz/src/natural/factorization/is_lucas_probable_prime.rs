// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use crate::natural::modular::mod_pow::LimbsRedc;
use crate::platform::Limb;
use alloc::vec;
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::factorization::is_lucas_probable_prime::selfridge_q;
use malachite_base::num::factorization::traits::{IsLucasProbablePrime, IsSquare};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::num::modular::traits::{ModAdd, ModInverse, ModSub};
use malachite_base::num::number_theory::traits::JacobiSymbol;

// x -= a mod m, for x < B^n (not necessarily reduced) and a < m, all with n limbs.
fn limbs_sub_mod_in_place(xs: &mut [Limb], ys: &[Limb], ms: &[Limb]) {
    if limbs_sub_same_length_in_place_left(xs, ys) {
        limbs_slice_add_same_length_in_place_left(xs, ms);
    }
}

// This is fmpz_is_probabprime_lucas from fmpz/is_probabprime_lucas.c and fmpz_lucas_chain from
// fmpz/lucas_chain.c, FLINT 3.7.0-dev, except that the search for D stops when D shares a proper
// factor with n (see selfridge_q), and the chain uses Montgomery arithmetic.
fn natural_is_lucas_probable_prime(n: &Natural) -> bool {
    if let Ok(x) = u128::try_from(n) {
        return x.is_lucas_probable_prime();
    }
    if n.even() || n.is_square() {
        return false;
    }
    let d_mod_n = |d: i64| {
        let r = Natural::from(d.unsigned_abs()) % n;
        if d < 0 && r != 0u32 { n - r } else { r }
    };
    let Some(q) = selfridge_q(&Natural::ZERO, &d_mod_n, &|r: &Natural| r.jacobi_symbol(n)) else {
        return false;
    };
    // A = P^2 / Q - 2 mod n, where P = 1; Q is invertible, as selfridge_q explains.
    let a = d_mod_n(q).mod_inverse(n).unwrap().mod_sub(Natural::TWO, n);
    let ms = n.as_limbs_asc();
    let len = ms.len();
    let mut redc = LimbsRedc::new(ms);
    // A = 0 only if n divides D + 1, which would take a search through about n / 2 values of D.
    let a_redc = if a == 0u32 {
        fail_on_untested_path("natural_is_lucas_probable_prime, a == 0");
        vec![0; len]
    } else {
        redc.to_redc(a.as_limbs_asc())
    };
    let two_redc = redc.to_redc(&[2]);
    // m = (n - (D / n)) / 2 = (n + 1) / 2
    let m = (n >> 1u32) + Natural::ONE;
    let mut vm = two_redc.clone();
    let mut vm1 = a_redc.clone();
    let mut t = vec![0; len];
    for bit in (0..m.significant_bits()).rev() {
        if m.get_bit(bit) {
            redc.mul(&mut t, &vm, &vm1);
            limbs_sub_mod_in_place(&mut t, &a_redc, ms);
            swap(&mut vm, &mut t);
            redc.square(&mut t, &vm1);
            limbs_sub_mod_in_place(&mut t, &two_redc, ms);
            swap(&mut vm1, &mut t);
        } else {
            redc.mul(&mut t, &vm, &vm1);
            limbs_sub_mod_in_place(&mut t, &a_redc, ms);
            swap(&mut vm1, &mut t);
            redc.square(&mut t, &vm);
            limbs_sub_mod_in_place(&mut t, &two_redc, ms);
            swap(&mut vm, &mut t);
        }
    }
    // U_{n+1} = (A V_m - 2 V_{m+1}) / D, so test whether A V_m = 2 V_{m+1} mod n.
    redc.mul(&mut t, &vm, &a_redc);
    let lhs = Natural::from_owned_limbs_asc(redc.redc_to_reduced(&t));
    let vm1 = Natural::from_owned_limbs_asc(redc.redc_to_reduced(&vm1));
    lhs == (&vm1).mod_add(&vm1, n)
}

impl IsLucasProbablePrime for Natural {
    /// Tests whether a [`Natural`] is a Lucas probable prime, with Selfridge's parameters. The
    /// [`Natural`] is taken by reference.
    ///
    /// Let $D$ be the first of $5, -7, 9, -11, \ldots$ whose Jacobi symbol $(D / n)$ is $-1$, and
    /// let $P = 1$ and $Q = (1 - D) / 4$. Then $n$ is a Lucas probable prime if $U_{n+1}(P, Q)
    /// \equiv 0 \pmod n$, where $U$ is the Lucas sequence. If an earlier $D$ shares a proper factor
    /// with $n$, or if $n$ is a perfect square, so that no such $D$ exists, $n$ is not a Lucas
    /// probable prime. Of the even numbers, only 2 is one. Every prime is a Lucas probable prime.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::factorization::traits::IsLucasProbablePrime;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(97u32).is_lucas_probable_prime());
    /// // 323 = 17 * 19 is the smallest Lucas pseudoprime.
    /// assert!(Natural::from(323u32).is_lucas_probable_prime());
    /// assert!(!Natural::from(15u32).is_lucas_probable_prime());
    ///
    /// let n = Natural::from_str("100000000000000000000000000000000000000000000000151").unwrap();
    /// assert!(n.is_lucas_probable_prime());
    ///
    /// // A strong pseudoprime to base 2, but not a Lucas probable prime
    /// let n = Natural::from_str("1971171711155257004609808164365888160584741").unwrap();
    /// assert!(!n.is_lucas_probable_prime());
    /// ```
    ///
    /// This is equivalent to `fmpz_is_probabprime_lucas` from `fmpz.h`, FLINT 3.7.0-dev, except
    /// that FLINT skips any $D$ with $(D / n) = 0$, and therefore also accepts some composites with
    /// small factors.
    #[inline]
    fn is_lucas_probable_prime(&self) -> bool {
        natural_is_lucas_probable_prime(self)
    }
}
