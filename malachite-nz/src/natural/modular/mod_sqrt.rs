// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 William Hart
//
//      Copyright © 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{CheckedSqrt, Parity};
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::comparison::traits::OrdDouble;
use malachite_base::num::modular::traits::{
    ModMulPrecomputed, ModMulPrecomputedAssign, ModPow, ModPowerOf2, ModSqrt, ModSquare,
    ModSquarePrecomputed, ModSquarePrecomputedAssign,
};
use malachite_base::num::number_theory::traits::JacobiSymbol;

const THREE: Natural = Natural::const_from(3);

// Computes a square root of `x` modulo `m`, where `x` must be reduced modulo `m`.
//
// If `m` is an odd prime, a root is returned whenever one exists. For other moduli the function
// still terminates and is deterministic, but it may return `None` when a root exists, or a value
// that is not a root.
//
// This has the same behavior as `fmpz_sqrtmod` from `fmpz/sqrtmod.c`, FLINT 3.6.0, for every input,
// except that for even moduli between 50 and 600 FLINT consults a Jacobi-symbol routine whose
// behavior for even moduli is undefined, and this function does not.
//
// FLINT 3.7 rewrote `fmpz_sqrtmod`, and its results for composite moduli differ from 3.6.0's, e.g.
// it finds no root of 4 modulo 6.
//
// The structure follows FLINT's: an exhaustive search below 600 (from `n_sqrtmod` in
// `ulong_extras/sqrtmod.c`), and above that the Jacobi test, the two fast exponentiation cases, and
// the Tonelli-Shanks loop of `_fmpz_sqrtmod`, with its iteration cap. The perfect-square test is
// load-bearing: modulo an odd square, no value has Jacobi symbol -1, so without the test the
// quadratic-nonresidue search could not terminate.
fn mod_sqrt_unnormalized(x: &Natural, m: &Natural) -> Option<Natural> {
    // Since x < m, a `Small` modulus implies a `Small` x. The `Limb` implementation is the same
    // algorithm, so this delegation does not change any output. It also handles the exhaustive
    // search for moduli below 600: a `Large` modulus is always far above that threshold.
    if let Natural(Small(m_small)) = m {
        let Natural(Small(x_small)) = x else {
            unreachable!();
        };
        return x_small.mod_sqrt(*m_small).map(Natural::from);
    }
    if *x <= 1u32 {
        return Some(x.clone());
    }
    // The evenness test must come first, since the Jacobi symbol requires an odd modulus; the
    // perfect-square test keeps the quadratic-nonresidue search below terminating.
    if m.even() || m.checked_sqrt().is_some() || x.jacobi_symbol(m) == -1 {
        return None;
    }
    let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(m);
    if m.mod_power_of_2(2) == 3u32 {
        return Some(x.mod_pow((m + Natural::ONE) >> 2u32, m));
    }
    if m.mod_power_of_2(3) == 5u32 {
        let root: Natural = x.mod_pow((m + THREE) >> 3u32, m);
        let square = (&root).mod_square_precomputed(m, &data);
        if square == *x {
            return Some(root);
        }
        let g: Natural = Natural::TWO.mod_pow((m - Natural::ONE) >> 2u32, m);
        return Some((&g).mod_mul_precomputed(&root, m, &data));
    }
    // Tonelli-Shanks. Here m == 1 mod 8, so if m is prime, 2 is a quadratic residue and the
    // smallest nonresidue is odd.
    let mut r = 0u64;
    let mut p1 = m - Natural::ONE;
    loop {
        p1 >>= 1u32;
        r += 1;
        if p1.odd() {
            break;
        }
    }
    let mut b = x.mod_pow(&p1, m);
    let mut k = THREE;
    while (&k).jacobi_symbol(m) != -1 {
        k += Natural::TWO;
    }
    let mut g = k.mod_pow(&p1, m);
    let mut root: Natural = x.mod_pow((&p1 + Natural::ONE) >> 1u32, m);
    // the maximum number of iterations if m is prime
    let mut iter = r - 1;
    while b != 1u32 {
        let mut b_pow = b.clone();
        let mut new_r = 0;
        loop {
            b_pow.mod_square_precomputed_assign(m, &data);
            new_r += 1;
            if new_r >= r || b_pow == 1u32 {
                break;
            }
        }
        let mut g_pow = g;
        for _ in 1..r - new_r {
            g_pow.mod_square_precomputed_assign(m, &data);
        }
        root.mod_mul_precomputed_assign(&g_pow, m, &data);
        g = (&g_pow).mod_square_precomputed(m, &data);
        b.mod_mul_precomputed_assign(&g, m, &data);
        r = new_r;
        if iter == 0 {
            // too many iterations; m is not prime
            root = Natural::ZERO;
            break;
        }
        iter -= 1;
    }
    if root == 0u32 { None } else { Some(root) }
}

// Computes the square root of `x` modulo `m` in $[0, m/2]$, where `x` must be reduced modulo `m`,
// or returns `None` if none is found. The result of `mod_sqrt_unnormalized` is checked by squaring,
// since for composite moduli it may not be a root, and replaced by `m` minus it if it is larger
// than $m/2$.
//
// For prime moduli, this returns a root exactly when `fmpz_sqrtmod` from `fmpz/sqrtmod.c`, FLINT
// 3.7.0-dev, does, and the same root up to negation: FLINT returns the root in [0, m/2] for
// one-word moduli, as `n_sqrtmod` does, but may return the other one for larger moduli.
fn mod_sqrt_ref_ref(x: &Natural, m: &Natural) -> Option<Natural> {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    let root = mod_sqrt_unnormalized(x, m)?;
    if (&root).mod_square(m) != *x {
        return None;
    }
    Some(if m.cmp_double(&root) == Less {
        m - root
    } else {
        root
    })
}

macro_rules! natural_mod_sqrt_doc {
    ($f:item) => {
        /// Computes a square root of a [`Natural`] modulo another [`Natural`] $m$: a $y$ with $y^2
        /// \equiv x \pmod m$. The input must be already reduced modulo $m$.
        ///
        /// The root returned is the one in $[0, m/2]$: of the roots $y$ and $m - y$, the smaller.
        /// If $m$ is an odd prime, a root is returned whenever one exists, and `None` is returned
        /// exactly when $x$ is a quadratic nonresidue. For other moduli the function still
        /// terminates and is deterministic, and a returned value is always a root, but `None` may
        /// be returned even though a root exists, and which root is returned is unspecified.
        ///
        /// $f(x, m) = y$, where $y \leq m/2$ and $y^2 \equiv x \mod m$, if such a $y$ is found.
        ///
        /// # Worst-case complexity
        /// $T(n) = O(n^3 \log n \log\log n)$
        ///
        /// $M(n) = O(n \log n)$
        ///
        /// where $T$ is time, $M$ is additional memory, and $n$ is `m.significant_bits()`. The
        /// bound assumes that the quadratic-nonresidue search does not dominate; under the extended
        /// Riemann hypothesis the search inspects $O((\log m)^2)$ candidates.
        ///
        /// # Panics
        /// Panics if `self` is greater than or equal to `m`.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::basic::traits::Two;
        /// use malachite_base::num::modular::traits::ModSqrt;
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!(
        ///     (&Natural::from(4u32)).mod_sqrt(&Natural::from(5u32)),
        ///     Some(Natural::TWO)
        /// );
        /// assert_eq!((&Natural::TWO).mod_sqrt(&Natural::from(3u32)), None);
        /// assert_eq!(
        ///     (&Natural::from(12909u32)).mod_sqrt(&Natural::from(65537u32)),
        ///     Some(Natural::from(14919u32))
        /// );
        /// ```
        ///
        /// This is equivalent to `fmpz_sqrtmod` from `fmpz/sqrtmod.c`, FLINT 3.7.0-dev, when \$m\$
        /// is prime, up to the choice between a root and its negation, returning an `Option` where
        /// FLINT sets an output and returns a flag.
        $f
    };
}

impl ModSqrt<Self> for Natural {
    type Output = Self;

    natural_mod_sqrt_doc! {
        #[inline]
        fn mod_sqrt(self, m: Self) -> Option<Self> {
            mod_sqrt_ref_ref(&self, &m)
        }
    }
}

impl<'a> ModSqrt<&'a Self> for Natural {
    type Output = Self;

    natural_mod_sqrt_doc! {
        #[inline]
        fn mod_sqrt(self, m: &'a Self) -> Option<Self> {
            mod_sqrt_ref_ref(&self, m)
        }
    }
}

impl ModSqrt<Natural> for &Natural {
    type Output = Natural;

    natural_mod_sqrt_doc! {
        #[inline]
        fn mod_sqrt(self, m: Natural) -> Option<Natural> {
            mod_sqrt_ref_ref(self, &m)
        }
    }
}

impl ModSqrt<&Natural> for &Natural {
    type Output = Natural;

    natural_mod_sqrt_doc! {
        #[inline]
        fn mod_sqrt(self, m: &Natural) -> Option<Natural> {
            mod_sqrt_ref_ref(self, m)
        }
    }
}
