use conversion::traits::ContinuedFraction;
use malachite_base::num::arithmetic::traits::{DivMod, Floor};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::mem::swap;
use Rational;

/// An iterable that produces the continued fraction of a `Rational`.
///
/// See `Rational::continued_fraction` and `Rational::continued_fraction_ref` for more information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalContinuedFraction {
    numerator: Natural,
    denominator: Natural,
}

impl RationalContinuedFraction {
    pub_crate_test! {is_done(&self) -> bool {
        self.denominator == 0u32 || self.numerator == 0u32
    }}
}

impl Iterator for RationalContinuedFraction {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.denominator == 0u32 || self.numerator == 0u32 {
            None
        } else {
            let n;
            (n, self.numerator) = (&self.numerator).div_mod(&self.denominator);
            swap(&mut self.numerator, &mut self.denominator);
            Some(n)
        }
    }
}

impl ContinuedFraction for Rational {
    type CF = RationalContinuedFraction;

    /// Returns the continued fraction of a `Rational`, taking the `Rational` by value.
    ///
    /// The output has two components. The first is the first value of the continued fraction,
    /// which may be any `Integer` and is equal to the floor of the `Rational`. The second is an
    /// iterator that produces the remaining values, which are all positive. Using the standard
    /// notation for continued fractions, the first value is the number before the semicolon, and
    /// the second value produces the remaining numbers.
    ///
    /// Each rational number has two continued fraction representations. The shorter of the two
    /// representations (the one that does not end in 1) is returned.
    ///
    /// $f(x) = (a_0, (a_1, a_2, \ldots, a_3)),$ where $x = [a_0; a_1, a_2, \ldots, a_3]$ and
    /// $a_3 \neq 1$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate itertools;
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use itertools::Itertools;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::conversion::traits::ContinuedFraction;
    /// use malachite_q::Rational;
    ///
    /// let (head, tail) = Rational::from_signeds(2, 3).continued_fraction();
    /// let tail = tail.collect_vec();
    /// assert_eq!(head, 0);
    /// assert_eq!(tail.to_debug_string(), "[1, 2]");
    ///
    /// let (head, tail) = Rational::from_signeds(355, 113).continued_fraction();
    /// let tail = tail.collect_vec();
    /// assert_eq!(head, 3);
    /// assert_eq!(tail.to_debug_string(), "[7, 16]");
    /// ```
    fn continued_fraction(mut self) -> (Integer, RationalContinuedFraction) {
        let f = (&self).floor();
        self -= Rational::from(&f);
        let (d, n) = self.into_numerator_and_denominator();
        (
            f,
            RationalContinuedFraction {
                numerator: n,
                denominator: d,
            },
        )
    }
}

impl<'a> ContinuedFraction for &'a Rational {
    type CF = RationalContinuedFraction;

    /// Returns the continued fraction of a `Rational`, taking the `Rational` by reference.
    ///
    /// The output has two components. The first is the first value of the continued fraction,
    /// which may be any `Integer` and is equal to the floor of the `Rational`. The second is an
    /// iterator that produces the remaining values, which are all positive. Using the standard
    /// notation for continued fractions, the first value is the number before the semicolon, and
    /// the second value produces the remaining numbers.
    ///
    /// Each rational number has two continued fraction representations. The shorter of the two
    /// representations (the one that does not end in 1) is returned.
    ///
    /// $f(x) = (a_0, (a_1, a_2, \ldots, a_3)),$ where $x = [a_0; a_1, a_2, \ldots, a_3]$ and
    /// $a_3 \neq 1$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate itertools;
    /// extern crate malachite_base;
    /// extern crate malachite_q;
    ///
    /// use itertools::Itertools;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::conversion::traits::ContinuedFraction;
    /// use malachite_q::Rational;
    ///
    /// let (head, tail) = (&Rational::from_signeds(2, 3)).continued_fraction();
    /// let tail = tail.collect_vec();
    /// assert_eq!(head, 0);
    /// assert_eq!(tail.to_debug_string(), "[1, 2]");
    ///
    /// let (head, tail) = (&Rational::from_signeds(355, 113)).continued_fraction();
    /// let tail = tail.collect_vec();
    /// assert_eq!(head, 3);
    /// assert_eq!(tail.to_debug_string(), "[7, 16]");
    /// ```
    fn continued_fraction(self) -> (Integer, RationalContinuedFraction) {
        let f = self.floor();
        let (d, n) = (self - Rational::from(&f)).into_numerator_and_denominator();
        (
            f,
            RationalContinuedFraction {
                numerator: n,
                denominator: d,
            },
        )
    }
}
