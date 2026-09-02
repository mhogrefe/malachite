// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use crate::gaussian_rational::arithmetic::mul::{mul_val_ref, mul_val_val};
use core::mem::take;
use core::ops::{Div, DivAssign};
use malachite_base::num::arithmetic::traits::{CheckedDiv, Reciprocal};

// A purely real divisor divides both parts; a purely imaginary divisor $di$ satisfies $(a +
// bi)/(di) = (b - ai)/d$, a division of both parts followed by a clockwise quarter turn. Otherwise
// the quotient is the product with the divisor's reciprocal, computed with the fused multiplication
// kernels.
fn div_val_val(x: GaussianRational, y: GaussianRational) -> GaussianRational {
    if y.imaginary == 0u32 {
        assert!(y.real != 0u32, "division by zero");
        GaussianRational {
            real: x.real / &y.real,
            imaginary: x.imaginary / y.real,
        }
    } else if y.real == 0u32 {
        GaussianRational {
            real: x.imaginary / &y.imaginary,
            imaginary: -(x.real / y.imaginary),
        }
    } else {
        mul_val_val(x, y.reciprocal())
    }
}

fn div_val_ref(x: GaussianRational, y: &GaussianRational) -> GaussianRational {
    if y.imaginary == 0u32 {
        assert!(y.real != 0u32, "division by zero");
        GaussianRational {
            real: x.real / &y.real,
            imaginary: x.imaginary / &y.real,
        }
    } else if y.real == 0u32 {
        GaussianRational {
            real: x.imaginary / &y.imaginary,
            imaginary: -(x.real / &y.imaginary),
        }
    } else {
        mul_val_val(x, y.reciprocal())
    }
}

fn div_ref_val(x: &GaussianRational, y: GaussianRational) -> GaussianRational {
    if y.imaginary == 0u32 {
        assert!(y.real != 0u32, "division by zero");
        GaussianRational {
            real: &x.real / &y.real,
            imaginary: &x.imaginary / y.real,
        }
    } else if y.real == 0u32 {
        GaussianRational {
            real: &x.imaginary / &y.imaginary,
            imaginary: -(&x.real / y.imaginary),
        }
    } else {
        // Multiplication is commutative, so the owned reciprocal can take the consuming slot.
        mul_val_ref(y.reciprocal(), x)
    }
}

fn div_ref_ref(x: &GaussianRational, y: &GaussianRational) -> GaussianRational {
    if y.imaginary == 0u32 {
        assert!(y.real != 0u32, "division by zero");
        GaussianRational {
            real: &x.real / &y.real,
            imaginary: &x.imaginary / &y.real,
        }
    } else if y.real == 0u32 {
        GaussianRational {
            real: &x.imaginary / &y.imaginary,
            imaginary: -(&x.real / &y.imaginary),
        }
    } else {
        mul_val_ref(y.reciprocal(), x)
    }
}

impl Div<Self> for GaussianRational {
    type Output = Self;

    /// Divides a [`GaussianRational`] by another [`GaussianRational`], taking both by value.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
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
    /// # Panics
    /// Panics if the second [`GaussianRational`] is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// let y = GaussianRational::from_str("1-i").unwrap();
    /// assert_eq!((x / y).to_string(), "i");
    /// let x = GaussianRational::from_str("22/7+i").unwrap();
    /// let y = GaussianRational::from_str("1/2+i/3").unwrap();
    /// assert_eq!((x / y).to_string(), "480/91-138i/91");
    /// ```
    #[inline]
    fn div(self, other: Self) -> Self {
        div_val_val(self, other)
    }
}

impl Div<&Self> for GaussianRational {
    type Output = Self;

    /// Divides a [`GaussianRational`] by another [`GaussianRational`], taking the first by value
    /// and the second by reference.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
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
    /// # Panics
    /// Panics if the second [`GaussianRational`] is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// let y = GaussianRational::from_str("1-i").unwrap();
    /// assert_eq!((x / &y).to_string(), "i");
    /// let x = GaussianRational::from_str("22/7+i").unwrap();
    /// let y = GaussianRational::from_str("1/2+i/3").unwrap();
    /// assert_eq!((x / &y).to_string(), "480/91-138i/91");
    /// ```
    #[inline]
    fn div(self, other: &Self) -> Self {
        div_val_ref(self, other)
    }
}

impl Div<GaussianRational> for &GaussianRational {
    type Output = GaussianRational;

    /// Divides a [`GaussianRational`] by another [`GaussianRational`], taking the first by
    /// reference and the second by value.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
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
    /// # Panics
    /// Panics if the second [`GaussianRational`] is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// let y = GaussianRational::from_str("1-i").unwrap();
    /// assert_eq!((&x / y).to_string(), "i");
    /// let x = GaussianRational::from_str("22/7+i").unwrap();
    /// let y = GaussianRational::from_str("1/2+i/3").unwrap();
    /// assert_eq!((&x / y).to_string(), "480/91-138i/91");
    /// ```
    #[inline]
    fn div(self, other: GaussianRational) -> GaussianRational {
        div_ref_val(self, other)
    }
}

impl Div<&GaussianRational> for &GaussianRational {
    type Output = GaussianRational;

    /// Divides a [`GaussianRational`] by another [`GaussianRational`], taking both by reference.
    ///
    /// $$
    /// f(x, y) = \frac{x}{y}.
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
    /// # Panics
    /// Panics if the second [`GaussianRational`] is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// let y = GaussianRational::from_str("1-i").unwrap();
    /// assert_eq!((&x / &y).to_string(), "i");
    /// let x = GaussianRational::from_str("22/7+i").unwrap();
    /// let y = GaussianRational::from_str("1/2+i/3").unwrap();
    /// assert_eq!((&x / &y).to_string(), "480/91-138i/91");
    /// ```
    #[inline]
    fn div(self, other: &GaussianRational) -> GaussianRational {
        div_ref_ref(self, other)
    }
}

impl DivAssign<Self> for GaussianRational {
    /// Divides a [`GaussianRational`] by another [`GaussianRational`] in place, taking the
    /// [`GaussianRational`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets \frac{x}{y}.
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
    /// # Panics
    /// Panics if the second [`GaussianRational`] is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("22/7+i").unwrap();
    /// x /= GaussianRational::from_str("1/2+i/3").unwrap();
    /// assert_eq!(x.to_string(), "480/91-138i/91");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = div_val_val(take(self), other);
    }
}

impl DivAssign<&Self> for GaussianRational {
    /// Divides a [`GaussianRational`] by another [`GaussianRational`] in place, taking the
    /// [`GaussianRational`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets \frac{x}{y}.
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
    /// # Panics
    /// Panics if the second [`GaussianRational`] is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianRational::from_str("22/7+i").unwrap();
    /// x /= &GaussianRational::from_str("1/2+i/3").unwrap();
    /// assert_eq!(x.to_string(), "480/91-138i/91");
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &Self) {
        *self = div_val_ref(take(self), other);
    }
}

impl CheckedDiv<Self> for GaussianRational {
    type Output = Self;

    /// Divides a [`GaussianRational`] by another [`GaussianRational`], taking both by value.
    /// Returns `None` when the second [`GaussianRational`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \frac{x}{y} \right ) & \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
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
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// let y = GaussianRational::from_str("1-i").unwrap();
    /// assert_eq!((x.checked_div(y)).unwrap().to_string(), "i");
    /// let x = GaussianRational::from_str("22/7+i").unwrap();
    /// let y = GaussianRational::ZERO;
    /// assert_eq!(x.checked_div(y), None);
    /// ```
    #[inline]
    fn checked_div(self, other: Self) -> Option<Self> {
        if other.real == 0u32 && other.imaginary == 0u32 {
            None
        } else {
            Some(div_val_val(self, other))
        }
    }
}

impl CheckedDiv<&Self> for GaussianRational {
    type Output = Self;

    /// Divides a [`GaussianRational`] by another [`GaussianRational`], taking the first by value
    /// and the second by reference. Returns `None` when the second [`GaussianRational`] is zero,
    /// `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \frac{x}{y} \right ) & \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
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
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// let y = GaussianRational::from_str("1-i").unwrap();
    /// assert_eq!((x.checked_div(&y)).unwrap().to_string(), "i");
    /// let x = GaussianRational::from_str("22/7+i").unwrap();
    /// let y = GaussianRational::ZERO;
    /// assert_eq!(x.checked_div(&y), None);
    /// ```
    #[inline]
    fn checked_div(self, other: &Self) -> Option<Self> {
        if other.real == 0u32 && other.imaginary == 0u32 {
            None
        } else {
            Some(div_val_ref(self, other))
        }
    }
}

impl CheckedDiv<GaussianRational> for &GaussianRational {
    type Output = GaussianRational;

    /// Divides a [`GaussianRational`] by another [`GaussianRational`], taking the first by
    /// reference and the second by value. Returns `None` when the second [`GaussianRational`] is
    /// zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \frac{x}{y} \right ) & \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
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
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// let y = GaussianRational::from_str("1-i").unwrap();
    /// assert_eq!(((&x).checked_div(y)).unwrap().to_string(), "i");
    /// let x = GaussianRational::from_str("22/7+i").unwrap();
    /// let y = GaussianRational::ZERO;
    /// assert_eq!((&x).checked_div(y), None);
    /// ```
    #[inline]
    fn checked_div(self, other: GaussianRational) -> Option<GaussianRational> {
        if other.real == 0u32 && other.imaginary == 0u32 {
            None
        } else {
            Some(div_ref_val(self, other))
        }
    }
}

impl CheckedDiv<&GaussianRational> for &GaussianRational {
    type Output = GaussianRational;

    /// Divides a [`GaussianRational`] by another [`GaussianRational`], taking both by reference.
    /// Returns `None` when the second [`GaussianRational`] is zero, `Some` otherwise.
    ///
    /// $$
    /// f(x, y) = \begin{cases}
    ///     \operatorname{Some}\left ( \frac{x}{y} \right ) & \text{if} \\quad y \neq 0 \\\\
    ///     \text{None} & \text{otherwise}
    /// \end{cases}
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
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianRational::from_str("1+i").unwrap();
    /// let y = GaussianRational::from_str("1-i").unwrap();
    /// assert_eq!(((&x).checked_div(&y)).unwrap().to_string(), "i");
    /// let x = GaussianRational::from_str("22/7+i").unwrap();
    /// let y = GaussianRational::ZERO;
    /// assert_eq!((&x).checked_div(&y), None);
    /// ```
    #[inline]
    fn checked_div(self, other: &GaussianRational) -> Option<GaussianRational> {
        if other.real == 0u32 && other.imaginary == 0u32 {
            None
        } else {
            Some(div_ref_ref(self, other))
        }
    }
}
