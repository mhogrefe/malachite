// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use core::mem::take;
use core::ops::{Mul, MulAssign};
use malachite_base::num::arithmetic::traits::{MulAddMul, MulSubMul, Square};

// Each part of each operand appears in exactly two products, so an owned part is borrowed by its
// first use and consumed by its last, letting the products reuse the operands' storage.
fn mul_val_val(x: GaussianRational, y: GaussianRational) -> GaussianRational {
    let real = (&x.real).mul_sub_mul(&y.real, &x.imaginary, &y.imaginary);
    GaussianRational {
        real,
        imaginary: x.real.mul_add_mul(y.imaginary, x.imaginary, y.real),
    }
}

fn mul_val_ref(x: GaussianRational, y: &GaussianRational) -> GaussianRational {
    let real = (&x.real).mul_sub_mul(&y.real, &x.imaginary, &y.imaginary);
    GaussianRational {
        real,
        imaginary: x.real.mul_add_mul(&y.imaginary, x.imaginary, &y.real),
    }
}

impl Mul<Self> for GaussianRational {
    type Output = Self;

    /// Multiplies two [`GaussianRational`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, NegativeOne};
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianRational::I * GaussianRational::I,
    ///     GaussianRational::NEGATIVE_ONE
    /// );
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// assert_eq!((x * y).to_string(), "1/3");
    /// ```
    #[inline]
    fn mul(self, other: Self) -> Self {
        mul_val_val(self, other)
    }
}

impl Mul<&Self> for GaussianRational {
    type Output = Self;

    /// Multiplies two [`GaussianRational`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, NegativeOne};
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// assert_eq!((x * &y).to_string(), "1/3");
    /// ```
    #[inline]
    fn mul(self, other: &Self) -> Self {
        mul_val_ref(self, other)
    }
}

impl Mul<GaussianRational> for &GaussianRational {
    type Output = GaussianRational;

    /// Multiplies two [`GaussianRational`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, NegativeOne};
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// assert_eq!((&x * y).to_string(), "1/3");
    /// ```
    #[inline]
    fn mul(self, other: GaussianRational) -> GaussianRational {
        // Multiplication is commutative, so the operands can be swapped to consume `other`.
        mul_val_ref(other, self)
    }
}

impl Mul<&GaussianRational> for &GaussianRational {
    type Output = GaussianRational;

    /// Multiplies two [`GaussianRational`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, NegativeOne};
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// assert_eq!((&x * &y).to_string(), "1/3");
    /// ```
    fn mul(self, other: &GaussianRational) -> GaussianRational {
        // As in fmpzi_mul, aliased operands are detected by address and routed to the squaring
        // algorithm, whose squarings avoid the GCD reductions of general multiplication. Only this
        // variant checks: two owned operands are always distinct objects.
        if core::ptr::eq(self, other) {
            return self.square();
        }
        GaussianRational {
            real: (&self.real).mul_sub_mul(&other.real, &self.imaginary, &other.imaginary),
            imaginary: (&self.real).mul_add_mul(&other.imaginary, &self.imaginary, &other.real),
        }
    }
}

impl MulAssign<Self> for GaussianRational {
    /// Multiplies a [`GaussianRational`] by a [`GaussianRational`] in place, taking the
    /// [`GaussianRational`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, NegativeOne};
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// let mut product = x;
    /// product *= y;
    /// assert_eq!(product.to_string(), "1/3");
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = mul_val_val(take(self), other);
    }
}

impl MulAssign<&Self> for GaussianRational {
    /// Multiplies a [`GaussianRational`] by a [`GaussianRational`] in place, taking the
    /// [`GaussianRational`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, NegativeOne};
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1/2+i/2").unwrap();
    /// let y = GaussianRational::from_str("1/3-i/3").unwrap();
    /// let mut product = x;
    /// product *= &y;
    /// assert_eq!(product.to_string(), "1/3");
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        *self = mul_val_ref(take(self), other);
    }
}
