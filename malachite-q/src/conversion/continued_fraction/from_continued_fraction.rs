// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{AddMulAssign, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

impl Rational {
    /// Converts a finite continued fraction to a [`Rational`], taking the inputs by value.
    ///
    /// The input has two components. The first is the first value of the continued fraction, which
    /// may be any [`Integer`] and is equal to the floor of the [`Rational`]. The second is an
    /// iterator of the remaining values, which must all be positive. Using the standard notation
    /// for continued fractions, the first value is the number before the semicolon, and the second
    /// value contains the remaining numbers.
    ///
    /// Each rational number has two continued fraction representations. Either one is a valid
    /// input.
    ///
    /// $f(a_0, (a_1, a_2, a_3, \ldots)) = [a_0; a_1, a_2, a_3, \ldots]$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((nm)^2 \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(floor.significant_bits(),
    /// xs.map(Natural::significant_bits).max())`, and $m$ is `xs.count()`.
    ///
    /// # Panics
    /// Panics if any [`Natural`] in `xs` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// let xs = vec_from_str("[1, 2]").unwrap().into_iter();
    /// assert_eq!(
    ///     Rational::from_continued_fraction(Integer::ZERO, xs).to_string(),
    ///     "2/3"
    /// );
    ///
    /// let xs = vec_from_str("[7, 16]").unwrap().into_iter();
    /// assert_eq!(
    ///     Rational::from_continued_fraction(Integer::from(3), xs).to_string(),
    ///     "355/113"
    /// );
    /// ```
    pub fn from_continued_fraction<I: Iterator<Item = Natural>>(floor: Integer, xs: I) -> Rational {
        let mut previous_numerator = Integer::ONE;
        let mut previous_denominator = Natural::ZERO;
        let mut numerator = floor;
        let mut denominator = Natural::ONE;
        for n in xs {
            assert_ne!(n, 0u32);
            previous_numerator.add_mul_assign(&numerator, Integer::from(&n));
            previous_denominator.add_mul_assign(&denominator, n);
            swap(&mut numerator, &mut previous_numerator);
            swap(&mut denominator, &mut previous_denominator);
        }
        Rational {
            sign: numerator >= 0,
            numerator: numerator.unsigned_abs(),
            denominator,
        }
    }

    /// Converts a finite continued fraction to a [`Rational`], taking the inputs by reference.
    ///
    /// The input has two components. The first is the first value of the continued fraction, which
    /// may be any [`Integer`] and is equal to the floor of the [`Rational`]. The second is an
    /// iterator of the remaining values, which must all be positive. Using the standard notation
    /// for continued fractions, the first value is the number before the semicolon, and the second
    /// value contains the remaining numbers.
    ///
    /// Each rational number has two continued fraction representations. Either one is a valid
    /// input.
    ///
    /// $f(a_0, (a_1, a_2, a_3, \ldots)) = [a_0; a_1, a_2, a_3, \ldots]$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((nm)^2 \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(nm \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(floor.significant_bits(),
    /// xs.map(Natural::significant_bits).max())`, and $m$ is `xs.count()`.
    ///
    /// # Panics
    /// Panics if any [`Natural`] in `xs` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// let xs = vec_from_str("[1, 2]").unwrap();
    /// assert_eq!(
    ///     Rational::from_continued_fraction_ref(&Integer::ZERO, xs.iter()).to_string(),
    ///     "2/3"
    /// );
    ///
    /// let xs = vec_from_str("[7, 16]").unwrap();
    /// assert_eq!(
    ///     Rational::from_continued_fraction_ref(&Integer::from(3), xs.iter()).to_string(),
    ///     "355/113"
    /// );
    /// ```
    pub fn from_continued_fraction_ref<'a, I: Iterator<Item = &'a Natural>>(
        floor: &Integer,
        xs: I,
    ) -> Rational {
        let mut previous_numerator = Integer::ONE;
        let mut previous_denominator = Natural::ZERO;
        let mut numerator = floor.clone();
        let mut denominator = Natural::ONE;
        for n in xs {
            assert_ne!(*n, 0u32);
            previous_numerator.add_mul_assign(&numerator, Integer::from(n));
            previous_denominator.add_mul_assign(&denominator, n);
            swap(&mut numerator, &mut previous_numerator);
            swap(&mut denominator, &mut previous_denominator);
        }
        Rational {
            sign: numerator >= 0,
            numerator: numerator.unsigned_abs(),
            denominator,
        }
    }
}
