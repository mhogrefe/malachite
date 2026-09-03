// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use core::iter::Product;
use core::mem::take;
use core::ops::{Mul, MulAssign};
use malachite_base::iterators::balanced_fold;
use malachite_base::num::arithmetic::traits::{MulAddMul, MulSubMul, Square};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;

use crate::gaussian_integer::arithmetic::SIZE_BALANCE_BITS;

// This threshold is from `fmpzi_mul` in FLINT 3.6.0, where it is a limb count (13 limbs, with
// 64-bit limbs); it is expressed here in bits so that it does not shift when Malachite is built
// with 32-bit limbs.
const KARATSUBA_THRESHOLD_BITS: u64 = 13 * 64;

enum MulAlgorithm {
    DoubleWord(i64, i64, i64, i64),
    Karatsuba,
    Fused,
}

// The algorithm selection of fmpzi_mul from fmpzi/mul.c, FLINT 3.6.0, except that the squaring
// fallback for aliased operands is omitted: Rust's ownership rules make the aliasing detectable at
// the call site, and a dedicated squaring implementation may be added later.
fn choose_algorithm(x: &GaussianInteger, y: &GaussianInteger) -> MulAlgorithm {
    // If all four parts fit in a signed word, two double-word products per output part suffice.
    if let (Ok(a), Ok(b), Ok(c), Ok(d)) = (
        i64::try_from(&x.real),
        i64::try_from(&x.imaginary),
        i64::try_from(&y.real),
        i64::try_from(&y.imaginary),
    ) {
        return MulAlgorithm::DoubleWord(a, b, c, d);
    }
    // For large, balanced operands, a Karatsuba-style scheme computes the product with three
    // multiplications instead of four: with $t = ac$ and $v = bd$, the real part is $t - v$ and the
    // imaginary part is $(a + b)(c + d) - t - v$.
    let a_bits = x.real.significant_bits();
    if a_bits >= KARATSUBA_THRESHOLD_BITS {
        let b_bits = x.imaginary.significant_bits();
        let c_bits = y.real.significant_bits();
        let d_bits = y.imaginary.significant_bits();
        if c_bits >= KARATSUBA_THRESHOLD_BITS
            && a_bits.abs_diff(b_bits) <= SIZE_BALANCE_BITS
            && c_bits.abs_diff(d_bits) <= SIZE_BALANCE_BITS
        {
            return MulAlgorithm::Karatsuba;
        }
    }
    // Otherwise, the four products are computed with the fused kernels.
    MulAlgorithm::Fused
}

// The products of two `i64`s and their sums and differences cannot overflow an `i128`.
fn mul_double_word(a: i64, b: i64, c: i64, d: i64) -> GaussianInteger {
    let (a, b, c, d) = (i128::from(a), i128::from(b), i128::from(c), i128::from(d));
    GaussianInteger {
        real: Integer::from(a * c - b * d),
        imaginary: Integer::from(a * d + b * c),
    }
}

// Each part of each operand appears in exactly two products, so an owned part is borrowed by its
// first use and consumed by its last, letting the products reuse the operands' storage.
pub(super) fn mul_val_val(x: GaussianInteger, y: GaussianInteger) -> GaussianInteger {
    match choose_algorithm(&x, &y) {
        MulAlgorithm::DoubleWord(a, b, c, d) => mul_double_word(a, b, c, d),
        MulAlgorithm::Karatsuba => {
            let mut u = (&x.real + &x.imaginary) * (&y.real + &y.imaginary);
            let t = x.real * y.real;
            let v = x.imaginary * y.imaginary;
            u -= &t;
            u -= &v;
            GaussianInteger {
                real: t - v,
                imaginary: u,
            }
        }
        MulAlgorithm::Fused => {
            let real = (&x.real).mul_sub_mul(&y.real, &x.imaginary, &y.imaginary);
            GaussianInteger {
                real,
                imaginary: x.real.mul_add_mul(y.imaginary, x.imaginary, y.real),
            }
        }
    }
}

pub(super) fn mul_val_ref(x: GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    match choose_algorithm(&x, y) {
        MulAlgorithm::DoubleWord(a, b, c, d) => mul_double_word(a, b, c, d),
        MulAlgorithm::Karatsuba => {
            let mut u = (&x.real + &x.imaginary) * (&y.real + &y.imaginary);
            let t = x.real * &y.real;
            let v = x.imaginary * &y.imaginary;
            u -= &t;
            u -= &v;
            GaussianInteger {
                real: t - v,
                imaginary: u,
            }
        }
        MulAlgorithm::Fused => {
            let real = (&x.real).mul_sub_mul(&y.real, &x.imaginary, &y.imaginary);
            GaussianInteger {
                real,
                imaginary: x.real.mul_add_mul(&y.imaginary, x.imaginary, &y.real),
            }
        }
    }
}

pub(super) fn mul_ref_ref(x: &GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    // As in fmpzi_mul, aliased operands are detected by address and routed to the squaring
    // algorithm, which replaces general multiplications with cheaper squarings. Only this variant
    // checks: two owned operands are always distinct objects.
    if core::ptr::eq(x, y) {
        return x.square();
    }
    match choose_algorithm(x, y) {
        MulAlgorithm::DoubleWord(a, b, c, d) => mul_double_word(a, b, c, d),
        MulAlgorithm::Karatsuba => {
            let mut u = (&x.real + &x.imaginary) * (&y.real + &y.imaginary);
            let t = &x.real * &y.real;
            let v = &x.imaginary * &y.imaginary;
            u -= &t;
            u -= &v;
            GaussianInteger {
                real: t - v,
                imaginary: u,
            }
        }
        MulAlgorithm::Fused => GaussianInteger {
            real: (&x.real).mul_sub_mul(&y.real, &x.imaginary, &y.imaginary),
            imaginary: (&x.real).mul_add_mul(&y.imaginary, &x.imaginary, &y.real),
        },
    }
}

impl Mul<Self> for GaussianInteger {
    type Output = Self;

    /// Multiplies two [`GaussianInteger`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, One};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     GaussianInteger::I * -GaussianInteger::I,
    ///     GaussianInteger::ONE
    /// );
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// assert_eq!((x * y).to_string(), "10+11i");
    /// ```
    #[inline]
    fn mul(self, other: Self) -> Self {
        mul_val_val(self, other)
    }
}

impl Mul<&Self> for GaussianInteger {
    type Output = Self;

    /// Multiplies two [`GaussianInteger`]s, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// assert_eq!((x * &y).to_string(), "10+11i");
    /// ```
    #[inline]
    fn mul(self, other: &Self) -> Self {
        mul_val_ref(self, other)
    }
}

impl Mul<GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Multiplies two [`GaussianInteger`]s, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// assert_eq!((&x * y).to_string(), "10+11i");
    /// ```
    #[inline]
    fn mul(self, other: GaussianInteger) -> GaussianInteger {
        // Multiplication is commutative, so the operands can be swapped to consume `other`.
        mul_val_ref(other, self)
    }
}

impl Mul<&GaussianInteger> for &GaussianInteger {
    type Output = GaussianInteger;

    /// Multiplies two [`GaussianInteger`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// assert_eq!((&x * &y).to_string(), "10+11i");
    /// ```
    #[inline]
    fn mul(self, other: &GaussianInteger) -> GaussianInteger {
        mul_ref_ref(self, other)
    }
}

impl MulAssign<Self> for GaussianInteger {
    /// Multiplies a [`GaussianInteger`] by a [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// let mut product = x;
    /// product *= y;
    /// assert_eq!(product.to_string(), "10+11i");
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = mul_val_val(take(self), other);
    }
}

impl MulAssign<&Self> for GaussianInteger {
    /// Multiplies a [`GaussianInteger`] by a [`GaussianInteger`] in place, taking the
    /// [`GaussianInteger`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets xy.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// let y = GaussianInteger::from_str("-1+4i").unwrap();
    /// let mut product = x;
    /// product *= &y;
    /// assert_eq!(product.to_string(), "10+11i");
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        *self = mul_val_ref(take(self), other);
    }
}

impl Product for GaussianInteger {
    /// Multiplies together all the [`GaussianInteger`]s in an iterator.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the real and imaginary parts of the [`GaussianInteger`]s.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(
    ///     GaussianInteger::product(
    ///         vec_from_str::<GaussianInteger>("[2, -3i, 5+i, 7-2i]")
    ///             .unwrap()
    ///             .into_iter()
    ///     )
    ///     .to_string(),
    ///     "-18-222i"
    /// );
    /// ```
    #[inline]
    fn product<I>(xs: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        balanced_fold(xs, |x| *x == 0u32, |a, b| *a *= b).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for GaussianInteger {
    /// Multiplies together all the [`GaussianInteger`]s in an iterator of [`GaussianInteger`]
    /// references.
    ///
    /// $$
    /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of significant bits
    /// of the real and imaginary parts of the [`GaussianInteger`]s.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(
    ///     GaussianInteger::product(
    ///         vec_from_str::<GaussianInteger>("[2, -3i, 5+i, 7-2i]")
    ///             .unwrap()
    ///             .iter()
    ///     )
    ///     .to_string(),
    ///     "-18-222i"
    /// );
    /// ```
    #[inline]
    fn product<I>(xs: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        balanced_fold(xs.cloned(), |x| *x == 0u32, |a, b| *a *= b).unwrap_or(Self::ONE)
    }
}
