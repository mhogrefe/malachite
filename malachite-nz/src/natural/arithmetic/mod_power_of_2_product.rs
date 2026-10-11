// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use core::borrow::Borrow;
use malachite_base::num::arithmetic::traits::{ModPowerOf2MulAssign, ModPowerOf2Product};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

fn assert_reduced(x: &Natural, pow: u64) {
    assert!(
        x.significant_bits() <= pow,
        "x must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
}

// When `pow` is at most `Limb::WIDTH`, every reduced factor fits in a `Limb`, and the `Limb`
// implementation multiplies them with wrapping multiplications. Otherwise the product is reduced
// after every multiplication: an exact product would grow with every factor, while
// `mod_power_of_2_mul` keeps it at `pow` bits and computes only the low half of each product. Since
// `pow` exceeds `Limb::WIDTH` here, the product of no factors is 1. Once the product is zero it
// stays zero, so the factors after the one that made it zero are not examined.
fn mod_power_of_2_product_helper<B: Borrow<Natural>, I: Iterator<Item = B>>(
    xs: I,
    pow: u64,
) -> Natural {
    if pow <= Limb::WIDTH {
        Natural::from(Limb::mod_power_of_2_product(
            xs.map(|x| {
                let x = x.borrow();
                assert_reduced(x, pow);
                Limb::exact_from(x)
            }),
            pow,
        ))
    } else {
        let mut product = Natural::ONE;
        for x in xs {
            let x = x.borrow();
            assert_reduced(x, pow);
            product.mod_power_of_2_mul_assign(x, pow);
            if product == 0u32 {
                break;
            }
        }
        product
    }
}

impl ModPowerOf2Product for Natural {
    /// Multiplies together all the [`Natural`]s in an iterator, modulo $2^k$. Every [`Natural`]
    /// must be already reduced modulo $2^k$.
    ///
    /// The product of no [`Natural`]s is 1 reduced modulo $2^k$, which is 0 when $k = 0$.
    ///
    /// $f((x_i)_ {i=0}^{n-1}, k) = y$, where $y < 2^k$ and $\prod_ {i=0}^{n-1} x_i \equiv y \mod
    /// 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any [`Natural`] is greater than or equal to $2^k$. As soon as the partial product
    /// is zero, the remaining [`Natural`]s are not examined, so an unreduced one after that point
    /// is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Product;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::mod_power_of_2_product(
    ///         vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().into_iter(),
    ///         4
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::mod_power_of_2_product(
    ///         vec_from_str::<Natural>("[18446744073709551615, 18446744073709551617, 3]")
    ///             .unwrap()
    ///             .into_iter(),
    ///         65
    ///     ),
    ///     Natural::from_str("36893488147419103229").unwrap()
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_product<I: Iterator<Item = Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_product_helper(xs, pow)
    }
}

impl<'a> ModPowerOf2Product<&'a Self> for Natural {
    /// Multiplies together all the [`Natural`]s in an iterator of [`Natural`] references, modulo
    /// $2^k$. Every [`Natural`] must be already reduced modulo $2^k$.
    ///
    /// The product of no [`Natural`]s is 1 reduced modulo $2^k$, which is 0 when $k = 0$.
    ///
    /// $f((x_i)_ {i=0}^{n-1}, k) = y$, where $y < 2^k$ and $\prod_ {i=0}^{n-1} x_i \equiv y \mod
    /// 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.count()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if any [`Natural`] is greater than or equal to $2^k$. As soon as the partial product
    /// is zero, the remaining [`Natural`]s are not examined, so an unreduced one after that point
    /// is not detected.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Product;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::mod_power_of_2_product(
    ///         vec_from_str::<Natural>("[2, 3, 5, 7]").unwrap().iter(),
    ///         4
    ///     ),
    ///     2
    /// );
    /// assert_eq!(
    ///     Natural::mod_power_of_2_product(
    ///         vec_from_str::<Natural>("[18446744073709551615, 18446744073709551617, 3]")
    ///             .unwrap()
    ///             .iter(),
    ///         65
    ///     ),
    ///     Natural::from_str("36893488147419103229").unwrap()
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_product<I: Iterator<Item = &'a Self>>(xs: I, pow: u64) -> Self {
        mod_power_of_2_product_helper(xs, pow)
    }
}
