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
use core::mem::take;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;

use crate::gaussian_integer::arithmetic::SIZE_BALANCE_BITS;

// This threshold is from `fmpzi_sqr` in FLINT 3.6.0, where it is a limb count (16 limbs, with
// 64-bit limbs); it is expressed here in bits so that it does not shift when Malachite is built
// with 32-bit limbs.
const THREE_SQUARES_THRESHOLD_BITS: u64 = 16 * 64;

enum SquareAlgorithm {
    DoubleWord(i64, i64),
    PurelyReal,
    PurelyImaginary,
    ThreeSquares,
    General,
}

// The algorithm selection of fmpzi_sqr from fmpzi/sqr.c, FLINT 3.6.0. Every path uses three
// multiplications, but arranges for as many of them as possible to be squarings, which are cheaper
// than general multiplications.
fn choose_algorithm(x: &GaussianInteger) -> SquareAlgorithm {
    // If both parts fit in a signed word, three double-word products suffice.
    if let (Ok(a), Ok(b)) = (i64::try_from(&x.real), i64::try_from(&x.imaginary)) {
        return SquareAlgorithm::DoubleWord(a, b);
    }
    if x.imaginary == 0u32 {
        return SquareAlgorithm::PurelyReal;
    }
    if x.real == 0u32 {
        return SquareAlgorithm::PurelyImaginary;
    }
    // For large, balanced operands, three squarings: with $t = a^2$ and $v = b^2$, the real part is
    // $t - v$ and the imaginary part is $(a + b)^2 - t - v$.
    let a_bits = x.real.significant_bits();
    if a_bits >= THREE_SQUARES_THRESHOLD_BITS {
        let b_bits = x.imaginary.significant_bits();
        if a_bits.abs_diff(b_bits) <= SIZE_BALANCE_BITS {
            return SquareAlgorithm::ThreeSquares;
        }
    }
    // Otherwise, two squarings and one general multiplication: $a^2 - b^2$ and $2ab$.
    SquareAlgorithm::General
}

// The squares of two `i64`s and their sums and differences cannot overflow an `i128`.
fn square_double_word(a: i64, b: i64) -> GaussianInteger {
    let (a, b) = (i128::from(a), i128::from(b));
    GaussianInteger {
        real: Integer::from(a * a - b * b),
        imaginary: Integer::from((a * b) << 1u64),
    }
}

// Each part appears in exactly two products, so an owned part is borrowed by its first use and
// consumed by its last, letting the products reuse the operand's storage.
fn square_val(x: GaussianInteger) -> GaussianInteger {
    match choose_algorithm(&x) {
        SquareAlgorithm::DoubleWord(a, b) => square_double_word(a, b),
        SquareAlgorithm::PurelyReal => GaussianInteger {
            real: x.real.square(),
            imaginary: Integer::ZERO,
        },
        SquareAlgorithm::PurelyImaginary => GaussianInteger {
            real: -x.imaginary.square(),
            imaginary: Integer::ZERO,
        },
        SquareAlgorithm::ThreeSquares => {
            let mut u = (&x.real + &x.imaginary).square();
            let t = x.real.square();
            let v = x.imaginary.square();
            u -= &t;
            u -= &v;
            GaussianInteger {
                real: t - v,
                imaginary: u,
            }
        }
        SquareAlgorithm::General => {
            let real = (&x.real).square() - (&x.imaginary).square();
            GaussianInteger {
                real,
                imaginary: (x.real * x.imaginary) << 1u64,
            }
        }
    }
}

fn square_ref(x: &GaussianInteger) -> GaussianInteger {
    match choose_algorithm(x) {
        SquareAlgorithm::DoubleWord(a, b) => square_double_word(a, b),
        SquareAlgorithm::PurelyReal => GaussianInteger {
            real: (&x.real).square(),
            imaginary: Integer::ZERO,
        },
        SquareAlgorithm::PurelyImaginary => GaussianInteger {
            real: -(&x.imaginary).square(),
            imaginary: Integer::ZERO,
        },
        SquareAlgorithm::ThreeSquares => {
            let mut u = (&x.real + &x.imaginary).square();
            let t = (&x.real).square();
            let v = (&x.imaginary).square();
            u -= &t;
            u -= &v;
            GaussianInteger {
                real: t - v,
                imaginary: u,
            }
        }
        SquareAlgorithm::General => GaussianInteger {
            real: (&x.real).square() - (&x.imaginary).square(),
            imaginary: (&x.real * &x.imaginary) << 1u64,
        },
    }
}

impl Square for GaussianInteger {
    type Output = Self;

    /// Squares a [`GaussianInteger`], taking it by value.
    ///
    /// $$
    /// f(x) = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::I;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::I.square().to_string(), "-1");
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!(x.square().to_string(), "-5-12i");
    /// ```
    #[inline]
    fn square(self) -> Self {
        square_val(self)
    }
}

impl Square for &GaussianInteger {
    type Output = GaussianInteger;

    /// Squares a [`GaussianInteger`], taking it by reference.
    ///
    /// $$
    /// f(x) = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!((&x).square().to_string(), "-5-12i");
    /// ```
    #[inline]
    fn square(self) -> GaussianInteger {
        square_ref(self)
    }
}

impl SquareAssign for GaussianInteger {
    /// Squares a [`GaussianInteger`] in place.
    ///
    /// $$
    /// x \gets x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// let mut x = GaussianInteger::from_str("2-3i").unwrap();
    /// x.square_assign();
    /// assert_eq!(x.to_string(), "-5-12i");
    /// ```
    #[inline]
    fn square_assign(&mut self) {
        *self = square_val(take(self));
    }
}
