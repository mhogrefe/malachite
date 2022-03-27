use malachite_base::num::arithmetic::traits::{AddMulAssign, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::mem::swap;
use Rational;

impl Rational {
    /// Converts a finite continued fraction to a `Rational`, taking the inputs by value.
    ///
    /// The input has two components. The first is the first value of the continued fraction, which
    /// may be any `Integer` and is equal to the floor of the `Rational`. The second is a `Vec` of
    /// the remaining values, which must all be positive. Using the standard notation for continued
    /// fractions, the first value is the number before the semicolon, and the second value
    /// contains the remaining numbers.
    ///
    /// Each rational number has two continued fraction representations. Either one is a valid
    /// input.
    ///
    /// $f(a_0, (a_1, a_2, a_3, \ldots)) = [a_0; a_1, a_2, a_3, \ldots]$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if any `Natural` in `xs` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// let xs = vec_from_str("[1, 2]").unwrap();
    /// assert_eq!(Rational::from_continued_fraction(Integer::ZERO, xs).to_string(), "2/3");
    ///
    /// let xs = vec_from_str("[7, 16]").unwrap();
    /// assert_eq!(Rational::from_continued_fraction(Integer::from(3), xs).to_string(), "355/113");
    /// ```
    pub fn from_continued_fraction(floor: Integer, xs: Vec<Natural>) -> Rational {
        let mut previous_numerator = Integer::ONE;
        let mut previous_denominator = Natural::ZERO;
        let mut numerator = floor;
        let mut denominator = Natural::ONE;
        for n in xs.into_iter() {
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

    /// Converts a finite continued fraction to a `Rational`, taking the inputs by reference.
    ///
    /// The input has two components. The first is the first value of the continued fraction, which
    /// may be any `Integer` and is equal to the floor of the `Rational`. The second is a `Vec` of
    /// the remaining values, which must all be positive. Using the standard notation for continued
    /// fractions, the first value is the number before the semicolon, and the second value
    /// contains the remaining numbers.
    ///
    /// Each rational number has two continued fraction representations. Either one is a valid
    /// input.
    ///
    /// $f(a_0, (a_1, a_2, a_3, \ldots)) = [a_0; a_1, a_2, a_3, \ldots]$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if any `Natural` in `xs` is zero.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    /// extern crate malachite_q;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::vecs::vec_from_str;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// let xs = vec_from_str("[1, 2]").unwrap();
    /// assert_eq!(Rational::from_continued_fraction_ref(&Integer::ZERO, &xs).to_string(), "2/3");
    ///
    /// let xs = vec_from_str("[7, 16]").unwrap();
    /// assert_eq!(
    ///     Rational::from_continued_fraction_ref(&Integer::from(3), &xs).to_string(),
    ///     "355/113"
    /// );
    /// ```
    pub fn from_continued_fraction_ref(floor: &Integer, xs: &[Natural]) -> Rational {
        let mut previous_numerator = Integer::ONE;
        let mut previous_denominator = Natural::ZERO;
        let mut numerator = floor.clone();
        let mut denominator = Natural::ONE;
        for n in xs.iter() {
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
