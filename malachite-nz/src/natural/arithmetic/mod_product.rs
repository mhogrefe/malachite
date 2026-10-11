// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2019 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::platform::Limb;
use core::borrow::Borrow;
use malachite_base::num::arithmetic::traits::{
    ModMulPrecomputed, ModMulPrecomputedAssign, ModProduct,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;

fn assert_reduced(x: &Natural, m: &Natural) {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
}

// When `m` fits in a `Limb`, so does every reduced factor, and the `Limb` implementation multiplies
// them. Otherwise the product is reduced after every multiplication, since an exact product would
// grow with every factor; the data for multiplying modulo `m` is computed once and reused. Since
// `m` exceeds `Limb::MAX` here, the product of no factors is 1. Once the product is zero it stays
// zero, so the factors after the one that made it zero are not examined.
fn mod_product_helper<B: Borrow<Natural>, I: Iterator<Item = B>>(xs: I, m: &Natural) -> Natural {
    assert_ne!(*m, 0u32, "m cannot be zero");
    if let Natural(Small(small_m)) = *m {
        Natural::from(Limb::mod_product(
            xs.map(|x| {
                let x = x.borrow();
                assert_reduced(x, m);
                Limb::exact_from(x)
            }),
            small_m,
        ))
    } else {
        let data = <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
        let mut product = Natural::ONE;
        for x in xs {
            let x = x.borrow();
            assert_reduced(x, m);
            product.mod_mul_precomputed_assign(x, m, &data);
            if product == 0u32 {
                break;
            }
        }
        product
    }
}

impl ModProduct<Self, Self> for Natural {
    /// Multiplies together all the [`Natural`]s in an iterator, modulo another [`Natural`] $m$,
    /// taking the factors by value and the modulus by value. Every [`Natural`] in the iterator must
    /// be already reduced modulo $m$.
    ///
    /// The product of no [`Natural`]s is 1 reduced modulo $m$, which is 0 when $m = 1$.
    ///
    /// $f((x_i)_ {i=0}^{n-1}, m) = y$, where $y < m$ and $\prod_ {i=0}^{n-1} x_i \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any [`Natural`] in the iterator is greater than or equal to `m`.
    /// As soon as the partial product is zero, the remaining [`Natural`]s are not examined, so an
    /// unreduced one after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModProduct;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::mod_product(
    ///         vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().into_iter(),
    ///         Natural::from(11u32)
    ///     ),
    ///     1
    /// );
    ///
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// let x = Natural::from_str("1000000000000000000000000000055").unwrap();
    /// assert_eq!(
    ///     Natural::mod_product([x.clone(), x.clone(), x].into_iter(), m).to_string(),
    ///     "1000000000000000000000000000049"
    /// );
    /// ```
    #[inline]
    fn mod_product<I: Iterator<Item = Self>>(xs: I, m: Self) -> Self {
        mod_product_helper(xs, &m)
    }
}

impl<'b> ModProduct<Self, &'b Self> for Natural {
    /// Multiplies together all the [`Natural`]s in an iterator, modulo another [`Natural`] $m$,
    /// taking the factors by value and the modulus by reference. Every [`Natural`] in the iterator
    /// must be already reduced modulo $m$.
    ///
    /// The product of no [`Natural`]s is 1 reduced modulo $m$, which is 0 when $m = 1$.
    ///
    /// $f((x_i)_ {i=0}^{n-1}, m) = y$, where $y < m$ and $\prod_ {i=0}^{n-1} x_i \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any [`Natural`] in the iterator is greater than or equal to `m`.
    /// As soon as the partial product is zero, the remaining [`Natural`]s are not examined, so an
    /// unreduced one after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModProduct;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::mod_product(
    ///         vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().into_iter(),
    ///         &Natural::from(11u32)
    ///     ),
    ///     1
    /// );
    ///
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// let x = Natural::from_str("1000000000000000000000000000055").unwrap();
    /// assert_eq!(
    ///     Natural::mod_product([x.clone(), x.clone(), x].into_iter(), &m).to_string(),
    ///     "1000000000000000000000000000049"
    /// );
    /// ```
    #[inline]
    fn mod_product<I: Iterator<Item = Self>>(xs: I, m: &'b Self) -> Self {
        mod_product_helper(xs, m)
    }
}

impl<'a> ModProduct<&'a Self, Self> for Natural {
    /// Multiplies together all the [`Natural`]s in an iterator, modulo another [`Natural`] $m$,
    /// taking the factors by reference and the modulus by value. Every [`Natural`] in the iterator
    /// must be already reduced modulo $m$.
    ///
    /// The product of no [`Natural`]s is 1 reduced modulo $m$, which is 0 when $m = 1$.
    ///
    /// $f((x_i)_ {i=0}^{n-1}, m) = y$, where $y < m$ and $\prod_ {i=0}^{n-1} x_i \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any [`Natural`] in the iterator is greater than or equal to `m`.
    /// As soon as the partial product is zero, the remaining [`Natural`]s are not examined, so an
    /// unreduced one after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModProduct;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::mod_product(
    ///         vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().iter(),
    ///         Natural::from(11u32)
    ///     ),
    ///     1
    /// );
    ///
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// let x = Natural::from_str("1000000000000000000000000000055").unwrap();
    /// assert_eq!(
    ///     Natural::mod_product([x.clone(), x.clone(), x].iter(), m).to_string(),
    ///     "1000000000000000000000000000049"
    /// );
    /// ```
    #[inline]
    fn mod_product<I: Iterator<Item = &'a Self>>(xs: I, m: Self) -> Self {
        mod_product_helper(xs, &m)
    }
}

impl<'a, 'b> ModProduct<&'a Self, &'b Self> for Natural {
    /// Multiplies together all the [`Natural`]s in an iterator, modulo another [`Natural`] $m$,
    /// taking the factors by reference and the modulus by reference. Every [`Natural`] in the
    /// iterator must be already reduced modulo $m$.
    ///
    /// The product of no [`Natural`]s is 1 reduced modulo $m$, which is 0 when $m = 1$.
    ///
    /// $f((x_i)_ {i=0}^{n-1}, m) = y$, where $y < m$ and $\prod_ {i=0}^{n-1} x_i \equiv y \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero or if any [`Natural`] in the iterator is greater than or equal to `m`.
    /// As soon as the partial product is zero, the remaining [`Natural`]s are not examined, so an
    /// unreduced one after that point is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModProduct;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::mod_product(
    ///         vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().iter(),
    ///         &Natural::from(11u32)
    ///     ),
    ///     1
    /// );
    ///
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// let x = Natural::from_str("1000000000000000000000000000055").unwrap();
    /// assert_eq!(
    ///     Natural::mod_product([x.clone(), x.clone(), x].iter(), &m).to_string(),
    ///     "1000000000000000000000000000049"
    /// );
    /// ```
    #[inline]
    fn mod_product<I: Iterator<Item = &'a Self>>(xs: I, m: &'b Self) -> Self {
        mod_product_helper(xs, m)
    }
}
