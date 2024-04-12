// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

/// Returns a number's continued fraction.
///
/// The form of a continued fraction is $[a_0; a_1, a_2, a_3\ldots]$. The first component of the
/// output pair is $a_0$, and the second is an iterator that produces the $a_i$ for $i > 0$.
pub trait ContinuedFraction {
    type CF: Iterator<Item = Natural>;

    fn continued_fraction(self) -> (Integer, Self::CF);
}

/// Returns a number's convergents, as an iterator of [`Rational`]s.
///
/// The convergents of a real number are the rational numbers whose continued fractions are the
/// prefixes of the original number's continued fraction.
pub trait Convergents {
    type C: Iterator<Item = Rational>;

    fn convergents(self) -> Self::C;
}
