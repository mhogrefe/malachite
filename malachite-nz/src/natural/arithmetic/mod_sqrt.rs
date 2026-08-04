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
use malachite_base::num::arithmetic::traits::{
    CheckedSqrt, JacobiSymbol, ModMulPrecomputed, ModMulPrecomputedAssign, ModPow, ModPowerOf2,
    ModSqrt, Parity,
};
use malachite_base::num::basic::traits::{One, Two, Zero};

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
// The structure follows FLINT's: an exhaustive search below 600 (from `n_sqrtmod` in
// `ulong_extras/sqrtmod.c`), and above that the Jacobi test, the two fast exponentiation cases, and
// the Tonelli-Shanks loop of `_fmpz_sqrtmod`, with its iteration cap. The perfect-square test is
// load-bearing: modulo an odd square, no value has Jacobi symbol -1, so without the test the
// quadratic-nonresidue search could not terminate.
fn mod_sqrt_ref_ref(x: &Natural, m: &Natural) -> Option<Natural> {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
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
    if m.mod_power_of_2(2) == 3 {
        return Some(x.mod_pow((m + Natural::ONE) >> 2, m));
    }
    if m.mod_power_of_2(3) == 5 {
        let root: Natural = x.mod_pow((m + THREE) >> 3, m);
        let square = (&root).mod_mul_precomputed(&root, m, &data);
        if square == *x {
            return Some(root);
        }
        let g: Natural = Natural::TWO.mod_pow((m - Natural::ONE) >> 2, m);
        return Some((&g).mod_mul_precomputed(&root, m, &data));
    }
    // Tonelli-Shanks. Here m == 1 mod 8, so if m is prime, 2 is a quadratic residue and the
    // smallest nonresidue is odd.
    let mut r = 0u64;
    let mut p1 = m - Natural::ONE;
    loop {
        p1 >>= 1;
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
    let mut root: Natural = x.mod_pow((&p1 + Natural::ONE) >> 1, m);
    // the maximum number of iterations if m is prime
    let mut iter = r - 1;
    while b != 1 {
        let mut b_pow = b.clone();
        let mut new_r = 0;
        loop {
            b_pow.mod_mul_precomputed_assign(b_pow.clone(), m, &data);
            new_r += 1;
            if new_r >= r || b_pow == 1 {
                break;
            }
        }
        let mut g_pow = g;
        for _ in 1..r - new_r {
            g_pow.mod_mul_precomputed_assign(g_pow.clone(), m, &data);
        }
        root.mod_mul_precomputed_assign(&g_pow, m, &data);
        g = (&g_pow).mod_mul_precomputed(&g_pow, m, &data);
        b.mod_mul_precomputed_assign(&g, m, &data);
        r = new_r;
        if iter == 0 {
            // too many iterations; m is not prime
            root = Natural::ZERO;
            break;
        }
        iter -= 1;
    }
    if root == 0 { None } else { Some(root) }
}

macro_rules! natural_mod_sqrt_doc {
    ($f:item) => {
        /// Computes a square root of a [`Natural`] modulo another [`Natural`] $m$: a $y$ with $y^2
        /// \equiv x \pmod m$. The input must be already reduced modulo $m$.
        ///
        /// If $m$ is an odd prime, a root is returned whenever one exists, and `None` is returned
        /// exactly when $x$ is a quadratic nonresidue. For other moduli the function still
        /// terminates and is deterministic, but it may return `None` even though a root exists, and
        /// it may return a value that is not a root, so if $m$ is not known to be prime, a returned
        /// root should be verified by squaring. The behavior for such moduli matches FLINT's.
        ///
        /// $f(x, m) = y$, where $x, y < m$ and $y^2 \equiv x \mod m$, if such a $y$ is found.
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
        /// use malachite_base::num::arithmetic::traits::ModSqrt;
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!(
        ///     (&Natural::from(4u32)).mod_sqrt(&Natural::from(5u32)),
        ///     Some(Natural::from(2u32))
        /// );
        /// assert_eq!((&Natural::from(2u32)).mod_sqrt(&Natural::from(3u32)), None);
        /// assert_eq!(
        ///     (&Natural::from(12909u32)).mod_sqrt(&Natural::from(65537u32)),
        ///     Some(Natural::from(50618u32))
        /// );
        /// ```
        ///
        /// This is equivalent to `fmpz_sqrtmod` from `fmpz/sqrtmod.c`, FLINT 3.6.0, returning an
        /// `Option` where FLINT sets an output and returns a flag.
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
