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
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2AddMulShl,
    ModPowerOf2AddMulShlAssign, ModPowerOf2Assign, ModPowerOf2MulAssign,
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
fn mod_power_of_2_add_mul_shl_limb(
    x: &Natural,
    y: &Natural,
    z: &Natural,
    bits: u64,
    pow: u64,
) -> Natural {
    Natural::from(Limb::exact_from(x).mod_power_of_2_add_mul_shl(
        Limb::exact_from(y),
        Limb::exact_from(z),
        bits,
        pow,
    ))
}

// Returns `y * z * 2^bits` modulo `2^pow`, for `bits < pow`. Only the low `pow - bits` bits of the
// product survive the shift, so the factors are reduced and multiplied modulo `2^(pow - bits)`.
fn product_shl(mut y: Natural, mut z: Natural, bits: u64, pow: u64) -> Natural {
    let low = pow - bits;
    y.mod_power_of_2_assign(low);
    z.mod_power_of_2_assign(low);
    y.mod_power_of_2_mul_assign(z, low);
    y << bits
}

impl ModPowerOf2AddMulShl<Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// $2^k$. All three inputs must be already reduced modulo $2^k$. All three [`Natural`]s are
    /// taken by value.
    ///
    /// $f(x, y, z, b, k) = w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_2_add_mul_shl(Natural::TWO, Natural::from(5u32), 1, 5),
    ///     23
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_2_add_mul_shl(
    ///         Natural::from(14u32),
    ///         Natural::from(3u32),
    ///         2,
    ///         6
    ///     ),
    ///     50
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_power_of_2_add_mul_shl(
    ///             Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             5,
    ///             100
    ///         )
    ///         .to_string(),
    ///     "449885024048990622321109630976"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_shl(mut self, y: Self, z: Self, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_shl_assign(y, z, bits, pow);
        self
    }
}

impl<'b> ModPowerOf2AddMulShl<Self, &'b Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// $2^k$. All three inputs must be already reduced modulo $2^k$. The first [`Natural`] is taken
    /// by value, the second by value, and the third by reference.
    ///
    /// $f(x, y, z, b, k) = w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_2_add_mul_shl(
    ///         Natural::TWO,
    ///         &Natural::from(5u32),
    ///         1,
    ///         5
    ///     ),
    ///     23
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_2_add_mul_shl(
    ///         Natural::from(14u32),
    ///         &Natural::from(3u32),
    ///         2,
    ///         6
    ///     ),
    ///     50
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_power_of_2_add_mul_shl(
    ///             Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             5,
    ///             100
    ///         )
    ///         .to_string(),
    ///     "449885024048990622321109630976"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_shl(mut self, y: Self, z: &'b Self, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_shl_assign(y, z, bits, pow);
        self
    }
}

impl<'a> ModPowerOf2AddMulShl<&'a Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// $2^k$. All three inputs must be already reduced modulo $2^k$. The first [`Natural`] is taken
    /// by value, the second by reference, and the third by value.
    ///
    /// $f(x, y, z, b, k) = w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_2_add_mul_shl(
    ///         &Natural::TWO,
    ///         Natural::from(5u32),
    ///         1,
    ///         5
    ///     ),
    ///     23
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_2_add_mul_shl(
    ///         &Natural::from(14u32),
    ///         Natural::from(3u32),
    ///         2,
    ///         6
    ///     ),
    ///     50
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_power_of_2_add_mul_shl(
    ///             &Natural::from(10u32).pow(20),
    ///             Natural::from(10u32).pow(12),
    ///             5,
    ///             100
    ///         )
    ///         .to_string(),
    ///     "449885024048990622321109630976"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_shl(mut self, y: &'a Self, z: Self, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_shl_assign(y, z, bits, pow);
        self
    }
}

impl<'a, 'b> ModPowerOf2AddMulShl<&'a Self, &'b Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// $2^k$. All three inputs must be already reduced modulo $2^k$. The first [`Natural`] is taken
    /// by value, the second by reference, and the third by reference.
    ///
    /// $f(x, y, z, b, k) = w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_2_add_mul_shl(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         1,
    ///         5
    ///     ),
    ///     23
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_2_add_mul_shl(
    ///         &Natural::from(14u32),
    ///         &Natural::from(3u32),
    ///         2,
    ///         6
    ///     ),
    ///     50
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(20)
    ///         .mod_power_of_2_add_mul_shl(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             5,
    ///             100
    ///         )
    ///         .to_string(),
    ///     "449885024048990622321109630976"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_mul_shl(mut self, y: &'a Self, z: &'b Self, bits: u64, pow: u64) -> Self {
        self.mod_power_of_2_add_mul_shl_assign(y, z, bits, pow);
        self
    }
}

impl ModPowerOf2AddMulShl<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, modulo
    /// $2^k$. All three inputs must be already reduced modulo $2^k$. All three [`Natural`]s are
    /// taken by reference.
    ///
    /// $f(x, y, z, b, k) = w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShl, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_power_of_2_add_mul_shl(
    ///         &Natural::TWO,
    ///         &Natural::from(5u32),
    ///         1,
    ///         5
    ///     ),
    ///     23
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_2_add_mul_shl(
    ///         &Natural::from(14u32),
    ///         &Natural::from(3u32),
    ///         2,
    ///         6
    ///     ),
    ///     50
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(20))
    ///         .mod_power_of_2_add_mul_shl(
    ///             &Natural::from(10u32).pow(20),
    ///             &Natural::from(10u32).pow(12),
    ///             5,
    ///             100
    ///         )
    ///         .to_string(),
    ///     "449885024048990622321109630976"
    /// );
    /// ```
    fn mod_power_of_2_add_mul_shl(self, y: &Natural, z: &Natural, bits: u64, pow: u64) -> Natural {
        assert_reduced(self, y, z, pow);
        if pow <= Limb::WIDTH {
            mod_power_of_2_add_mul_shl_limb(self, y, z, bits, pow)
        } else if bits >= pow {
            self.clone()
        } else {
            let low = pow - bits;
            self.mod_power_of_2_add(
                product_shl(y.mod_power_of_2(low), z.mod_power_of_2(low), bits, pow),
                pow,
            )
        }
    }
}

impl ModPowerOf2AddMulShlAssign<Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo $2^k$,
    /// in place. All three inputs must be already reduced modulo $2^k$. Both [`Natural`]s on the
    /// right-hand side are taken by value.
    ///
    /// $x \gets w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_2_add_mul_shl_assign(Natural::TWO, Natural::from(5u32), 1, 5);
    /// assert_eq!(x, 23);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_add_mul_shl_assign(Natural::from(14u32), Natural::from(3u32), 2, 6);
    /// assert_eq!(x, 50);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_power_of_2_add_mul_shl_assign(
    ///     Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     5,
    ///     100,
    /// );
    /// assert_eq!(x.to_string(), "449885024048990622321109630976");
    /// ```
    fn mod_power_of_2_add_mul_shl_assign(&mut self, y: Self, z: Self, bits: u64, pow: u64) {
        assert_reduced(self, &y, &z, pow);
        if pow <= Limb::WIDTH {
            *self = mod_power_of_2_add_mul_shl_limb(self, &y, &z, bits, pow);
        } else if bits < pow {
            self.mod_power_of_2_add_assign(product_shl(y, z, bits, pow), pow);
        }
    }
}

impl<'b> ModPowerOf2AddMulShlAssign<Self, &'b Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo $2^k$,
    /// in place. All three inputs must be already reduced modulo $2^k$. The first [`Natural`] on
    /// the right-hand side is taken by value and the second by reference.
    ///
    /// $x \gets w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_2_add_mul_shl_assign(Natural::TWO, &Natural::from(5u32), 1, 5);
    /// assert_eq!(x, 23);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_add_mul_shl_assign(Natural::from(14u32), &Natural::from(3u32), 2, 6);
    /// assert_eq!(x, 50);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_power_of_2_add_mul_shl_assign(
    ///     Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     5,
    ///     100,
    /// );
    /// assert_eq!(x.to_string(), "449885024048990622321109630976");
    /// ```
    fn mod_power_of_2_add_mul_shl_assign(&mut self, y: Self, z: &'b Self, bits: u64, pow: u64) {
        assert_reduced(self, &y, z, pow);
        if pow <= Limb::WIDTH {
            *self = mod_power_of_2_add_mul_shl_limb(self, &y, z, bits, pow);
        } else if bits < pow {
            let low = pow - bits;
            self.mod_power_of_2_add_assign(product_shl(y, z.mod_power_of_2(low), bits, pow), pow);
        }
    }
}

impl<'a> ModPowerOf2AddMulShlAssign<&'a Self, Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo $2^k$,
    /// in place. All three inputs must be already reduced modulo $2^k$. The first [`Natural`] on
    /// the right-hand side is taken by reference and the second by value.
    ///
    /// $x \gets w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_2_add_mul_shl_assign(&Natural::TWO, Natural::from(5u32), 1, 5);
    /// assert_eq!(x, 23);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_add_mul_shl_assign(&Natural::from(14u32), Natural::from(3u32), 2, 6);
    /// assert_eq!(x, 50);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_power_of_2_add_mul_shl_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     Natural::from(10u32).pow(12),
    ///     5,
    ///     100,
    /// );
    /// assert_eq!(x.to_string(), "449885024048990622321109630976");
    /// ```
    fn mod_power_of_2_add_mul_shl_assign(&mut self, y: &'a Self, z: Self, bits: u64, pow: u64) {
        assert_reduced(self, y, &z, pow);
        if pow <= Limb::WIDTH {
            *self = mod_power_of_2_add_mul_shl_limb(self, y, &z, bits, pow);
        } else if bits < pow {
            let low = pow - bits;
            self.mod_power_of_2_add_assign(product_shl(y.mod_power_of_2(low), z, bits, pow), pow);
        }
    }
}

impl<'a, 'b> ModPowerOf2AddMulShlAssign<&'a Self, &'b Self> for Natural {
    /// Adds the product of two [`Natural`]s, shifted left by `bits`, to a [`Natural`] modulo $2^k$,
    /// in place. All three inputs must be already reduced modulo $2^k$. Both [`Natural`]s on the
    /// right-hand side are taken by reference.
    ///
    /// $x \gets w$, where $x, y, z, w < 2^k$ and $x + yz2^b \equiv w \mod 2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2AddMulShlAssign, Pow};
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_2_add_mul_shl_assign(&Natural::TWO, &Natural::from(5u32), 1, 5);
    /// assert_eq!(x, 23);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_add_mul_shl_assign(&Natural::from(14u32), &Natural::from(3u32), 2, 6);
    /// assert_eq!(x, 50);
    ///
    /// let mut x = Natural::from(10u32).pow(20);
    /// x.mod_power_of_2_add_mul_shl_assign(
    ///     &Natural::from(10u32).pow(20),
    ///     &Natural::from(10u32).pow(12),
    ///     5,
    ///     100,
    /// );
    /// assert_eq!(x.to_string(), "449885024048990622321109630976");
    /// ```
    fn mod_power_of_2_add_mul_shl_assign(&mut self, y: &'a Self, z: &'b Self, bits: u64, pow: u64) {
        assert_reduced(self, y, z, pow);
        if pow <= Limb::WIDTH {
            *self = mod_power_of_2_add_mul_shl_limb(self, y, z, bits, pow);
        } else if bits < pow {
            let low = pow - bits;
            self.mod_power_of_2_add_assign(
                product_shl(y.mod_power_of_2(low), z.mod_power_of_2(low), bits, pow),
                pow,
            );
        }
    }
}
