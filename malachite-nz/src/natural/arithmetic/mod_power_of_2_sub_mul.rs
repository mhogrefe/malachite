// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Mul, ModPowerOf2MulAssign, ModPowerOf2Sub, ModPowerOf2SubAssign, ModPowerOf2SubMul,
    ModPowerOf2SubMulAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

fn assert_reduced(x: &Natural, y: &Natural, z: &Natural, pow: u64) {
    assert!(
        x.significant_bits() <= pow,
        "self must be reduced mod 2^pow, but {x} >= 2^{pow}"
    );
    assert!(
        y.significant_bits() <= pow,
        "y must be reduced mod 2^pow, but {y} >= 2^{pow}"
    );
    assert!(
        z.significant_bits() <= pow,
        "z must be reduced mod 2^pow, but {z} >= 2^{pow}"
    );
}

// When `pow` is at most `Limb::WIDTH`, every reduced argument fits in a `Limb`.
fn mod_power_of_2_sub_mul_limb(x: &Natural, y: &Natural, z: &Natural, pow: u64) -> Natural {
    Natural::from(Limb::exact_from(x).mod_power_of_2_sub_mul(
        Limb::exact_from(y),
        Limb::exact_from(z),
        pow,
    ))
}

impl ModPowerOf2SubMul<Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo $2^k$. All three inputs
    /// must be already reduced modulo $2^k$. All three [`Natural`]s are taken by value.
    ///
    /// $f(x, y, z, k) = w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(13u32).mod_power_of_2_sub_mul(Natural::TWO, Natural::from(5u32), 5),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_power_of_2_sub_mul(
    ///             Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             100
    ///         )
    ///         .to_string(),
    ///     "144397418130122718239553224704"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul(mut self, y: Self, z: Self, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_assign(y, z, pow);
        self
    }
}

impl<'b> ModPowerOf2SubMul<Self, &'b Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo $2^k$. All three inputs
    /// must be already reduced modulo $2^k$. The first [`Natural`] is taken by value, the second by
    /// value, and the third by reference.
    ///
    /// $f(x, y, z, k) = w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(13u32).mod_power_of_2_sub_mul(Natural::TWO, &Natural::from(5u32), 5),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_power_of_2_sub_mul(
    ///             Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             100
    ///         )
    ///         .to_string(),
    ///     "144397418130122718239553224704"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul(mut self, y: Self, z: &'b Self, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_assign(y, z, pow);
        self
    }
}

impl<'a> ModPowerOf2SubMul<&'a Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo $2^k$. All three inputs
    /// must be already reduced modulo $2^k$. The first [`Natural`] is taken by value, the second by
    /// reference, and the third by value.
    ///
    /// $f(x, y, z, k) = w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(13u32).mod_power_of_2_sub_mul(&Natural::TWO, Natural::from(5u32), 5),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_power_of_2_sub_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             100
    ///         )
    ///         .to_string(),
    ///     "144397418130122718239553224704"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul(mut self, y: &'a Self, z: Self, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_assign(y, z, pow);
        self
    }
}

impl<'a, 'b> ModPowerOf2SubMul<&'a Self, &'b Self> for Natural {
    type Output = Self;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo $2^k$. All three inputs
    /// must be already reduced modulo $2^k$. The first [`Natural`] is taken by value, the second by
    /// reference, and the third by reference.
    ///
    /// $f(x, y, z, k) = w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(13u32).mod_power_of_2_sub_mul(&Natural::TWO, &Natural::from(5u32), 5),
    ///     3
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_power_of_2_sub_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             100
    ///         )
    ///         .to_string(),
    ///     "144397418130122718239553224704"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_sub_mul(mut self, y: &'a Self, z: &'b Self, pow: u64) -> Self {
        self.mod_power_of_2_sub_mul_assign(y, z, pow);
        self
    }
}

impl ModPowerOf2SubMul<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts the product of two [`Natural`]s from a [`Natural`], modulo $2^k$. All three inputs
    /// must be already reduced modulo $2^k$. All three [`Natural`]s are taken by reference.
    ///
    /// $f(x, y, z, k) = w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMul, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(13u32)).mod_power_of_2_sub_mul(&Natural::TWO, &Natural::from(5u32), 5),
    ///     3
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(20))
    ///         .mod_power_of_2_sub_mul(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             100
    ///         )
    ///         .to_string(),
    ///     "144397418130122718239553224704"
    /// );
    /// ```
    fn mod_power_of_2_sub_mul(self, y: &Natural, z: &Natural, pow: u64) -> Natural {
        assert_reduced(self, y, z, pow);
        if pow <= Limb::WIDTH {
            mod_power_of_2_sub_mul_limb(self, y, z, pow)
        } else {
            self.mod_power_of_2_sub(y.mod_power_of_2_mul(z, pow), pow)
        }
    }
}

impl ModPowerOf2SubMulAssign<Self, Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo $2^k$, in place. All
    /// three inputs must be already reduced modulo $2^k$. Both [`Natural`]s on the right-hand side
    /// are taken by value.
    ///
    /// $x \gets w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(13u32);
    /// x.mod_power_of_2_sub_mul_assign(Natural::TWO, Natural::from(5u32), 5);
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_power_of_2_sub_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     100,
    /// );
    /// assert_eq!(x.to_string(), "144397418130122718239553224704");
    /// ```
    fn mod_power_of_2_sub_mul_assign(&mut self, mut y: Self, z: Self, pow: u64) {
        assert_reduced(self, &y, &z, pow);
        if pow <= Limb::WIDTH {
            *self = mod_power_of_2_sub_mul_limb(self, &y, &z, pow);
        } else {
            y.mod_power_of_2_mul_assign(z, pow);
            self.mod_power_of_2_sub_assign(y, pow);
        }
    }
}

impl<'b> ModPowerOf2SubMulAssign<Self, &'b Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo $2^k$, in place. All
    /// three inputs must be already reduced modulo $2^k$. The first [`Natural`] on the right-hand
    /// side is taken by value and the second by reference.
    ///
    /// $x \gets w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(13u32);
    /// x.mod_power_of_2_sub_mul_assign(Natural::TWO, &Natural::from(5u32), 5);
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_power_of_2_sub_mul_assign(
    ///     Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     100,
    /// );
    /// assert_eq!(x.to_string(), "144397418130122718239553224704");
    /// ```
    fn mod_power_of_2_sub_mul_assign(&mut self, mut y: Self, z: &'b Self, pow: u64) {
        assert_reduced(self, &y, z, pow);
        if pow <= Limb::WIDTH {
            *self = mod_power_of_2_sub_mul_limb(self, &y, z, pow);
        } else {
            y.mod_power_of_2_mul_assign(z, pow);
            self.mod_power_of_2_sub_assign(y, pow);
        }
    }
}

impl<'a> ModPowerOf2SubMulAssign<&'a Self, Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo $2^k$, in place. All
    /// three inputs must be already reduced modulo $2^k$. The first [`Natural`] on the right-hand
    /// side is taken by reference and the second by value.
    ///
    /// $x \gets w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(13u32);
    /// x.mod_power_of_2_sub_mul_assign(&Natural::TWO, Natural::from(5u32), 5);
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_power_of_2_sub_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     100,
    /// );
    /// assert_eq!(x.to_string(), "144397418130122718239553224704");
    /// ```
    fn mod_power_of_2_sub_mul_assign(&mut self, y: &'a Self, mut z: Self, pow: u64) {
        assert_reduced(self, y, &z, pow);
        if pow <= Limb::WIDTH {
            *self = mod_power_of_2_sub_mul_limb(self, y, &z, pow);
        } else {
            z.mod_power_of_2_mul_assign(y, pow);
            self.mod_power_of_2_sub_assign(z, pow);
        }
    }
}

impl<'a, 'b> ModPowerOf2SubMulAssign<&'a Self, &'b Self> for Natural {
    /// Subtracts the product of two [`Natural`]s from a [`Natural`] modulo $2^k$, in place. All
    /// three inputs must be already reduced modulo $2^k$. Both [`Natural`]s on the right-hand side
    /// are taken by reference.
    ///
    /// $x \gets w$, where $x, y, z, w < 2^k$ and $x - yz \equiv w \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self`, `y`, or `z` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2SubMulAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(13u32);
    /// x.mod_power_of_2_sub_mul_assign(&Natural::TWO, &Natural::from(5u32), 5);
    /// assert_eq!(x, 3);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_power_of_2_sub_mul_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     100,
    /// );
    /// assert_eq!(x.to_string(), "144397418130122718239553224704");
    /// ```
    fn mod_power_of_2_sub_mul_assign(&mut self, y: &'a Self, z: &'b Self, pow: u64) {
        assert_reduced(self, y, z, pow);
        if pow <= Limb::WIDTH {
            *self = mod_power_of_2_sub_mul_limb(self, y, z, pow);
        } else {
            self.mod_power_of_2_sub_assign(y.mod_power_of_2_mul(z, pow), pow);
        }
    }
}
