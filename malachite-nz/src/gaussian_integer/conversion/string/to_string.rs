// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::{
    ComparableGaussianInteger, ComparableGaussianIntegerRef, GaussianInteger,
};
use core::fmt::{Debug, Display, Formatter, Result, Write};

impl Display for GaussianInteger {
    /// Converts a [`GaussianInteger`] to a [`String`].
    ///
    /// A value with a zero imaginary part is written as its real part alone; in particular, zero is
    /// `"0"`. A purely imaginary value is written as a coefficient directly followed by `'i'`, with
    /// coefficients of 1 and -1 elided, giving `"i"` and `"-i"`. Otherwise, the real term is
    /// written first and the imaginary term follows with a joining sign, as in `"1+i"` and
    /// `"2-3i"`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::traits::ImaginaryFrom;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(GaussianInteger::default().to_string(), "0");
    /// assert_eq!(GaussianInteger::from(2).to_string(), "2");
    /// assert_eq!(GaussianInteger::from(-2).to_string(), "-2");
    /// assert_eq!(GaussianInteger::imaginary_from(1).to_string(), "i");
    /// assert_eq!(GaussianInteger::imaginary_from(-1).to_string(), "-i");
    /// assert_eq!(GaussianInteger::imaginary_from(2).to_string(), "2i");
    /// assert_eq!(GaussianInteger::imaginary_from(-2).to_string(), "-2i");
    ///
    /// let g = GaussianInteger {
    ///     real: Integer::from(1),
    ///     imaginary: Integer::from(1),
    /// };
    /// assert_eq!(g.to_string(), "1+i");
    /// let g = GaussianInteger {
    ///     real: Integer::from(1),
    ///     imaginary: Integer::from(-1),
    /// };
    /// assert_eq!(g.to_string(), "1-i");
    /// let g = GaussianInteger {
    ///     real: Integer::from(2),
    ///     imaginary: Integer::from(3),
    /// };
    /// assert_eq!(g.to_string(), "2+3i");
    /// let g = GaussianInteger {
    ///     real: Integer::from(2),
    ///     imaginary: Integer::from(-3),
    /// };
    /// assert_eq!(g.to_string(), "2-3i");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.imaginary == 0u32 {
            return Display::fmt(&self.real, f);
        }
        if self.real != 0u32 {
            Display::fmt(&self.real, f)?;
            if self.imaginary > 0u32 {
                f.write_char('+')?;
            }
        }
        if self.imaginary == 1u32 {
            f.write_char('i')
        } else if self.imaginary == -1i32 {
            f.write_str("-i")
        } else {
            Display::fmt(&self.imaginary, f)?;
            f.write_char('i')
        }
    }
}

impl Debug for GaussianInteger {
    /// Converts a [`GaussianInteger`] to a [`String`].
    ///
    /// This is the same as the [`Display::fmt`] implementation, so that a collection of
    /// [`GaussianInteger`]s is written the same way its elements are displayed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// let xs = vec![
    ///     GaussianInteger::from_str("2-3i").unwrap(),
    ///     GaussianInteger::from_str("i").unwrap(),
    ///     GaussianInteger::from_str("-5").unwrap(),
    /// ];
    /// assert_eq!(xs[0].to_debug_string(), "2-3i");
    /// assert_eq!(xs[1].to_debug_string(), "i");
    /// assert_eq!(xs[2].to_debug_string(), "-5");
    /// assert_eq!(xs.to_debug_string(), "[2-3i, i, -5]");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl Display for ComparableGaussianInteger {
    /// Converts a [`ComparableGaussianInteger`] to a [`String`], writing the wrapped
    /// [`GaussianInteger`] exactly as its own [`Display`] implementation does.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_nz::gaussian_integer::{ComparableGaussianInteger, GaussianInteger};
    ///
    /// assert_eq!(
    ///     ComparableGaussianInteger(GaussianInteger::I).to_string(),
    ///     "i"
    /// );
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl Display for ComparableGaussianIntegerRef<'_> {
    /// Converts a [`ComparableGaussianIntegerRef`] to a [`String`], writing the wrapped
    /// [`GaussianInteger`] exactly as its own [`Display`] implementation does.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_nz::gaussian_integer::{ComparableGaussianIntegerRef, GaussianInteger};
    ///
    /// let x = GaussianInteger::I;
    /// assert_eq!(ComparableGaussianIntegerRef(&x).to_string(), "i");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self.0, f)
    }
}
