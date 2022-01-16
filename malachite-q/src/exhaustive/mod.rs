use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use std::iter::{once, Chain, Once};
use std::mem::swap;
use Rational;

/// Generates all positive `Rational`s.
///
/// This `struct` is created by the `exhaustive_positive_rationals` function. See its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveRationals {
    pred_pred: Natural,
    pred: Natural,
}

impl Iterator for ExhaustivePositiveRationals {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        let mut anm1 = Natural::ZERO;
        swap(&mut self.pred_pred, &mut anm1);
        swap(&mut self.pred, &mut self.pred_pred);
        let k = &anm1 / &self.pred_pred; // floor(a(n - 1) / a(n))
        self.pred = ((k << 1u32) | Natural::ONE) * &self.pred_pred - anm1;
        Some(Rational {
            sign: true,
            numerator: self.pred_pred.clone(),
            denominator: self.pred.clone(),
        })
    }
}

/// Generates all positive `Rational`s.
///
/// The `Rational`s are ordered as in the Calkin-Wilf sequence. Their numerators and denominators
/// are given by the Stern-Brocot sequence. To generate the latter sequence, this iterator uses the
/// formula
/// $$
/// a_{n+1} = \left ( 2 \left \lfloor \frac{a_{n-1}}{a_n} \right \rfloor +1 \right ) a_n - a_{n-1},
/// $$
/// attributed to David S. Newman at <https://oeis.org/A002487>.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// TODO
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::strings::ExtraToString;
/// use malachite_q::exhaustive::exhaustive_positive_rationals;
///
/// assert_eq!(
///     exhaustive_positive_rationals().take(20).collect_vec().to_string(),
///     "[1, 1/2, 2, 1/3, 3/2, 2/3, 3, 1/4, 4/3, 3/5, 5/2, 2/5, 5/3, 3/4, 4, 1/5, 5/4, 4/7, 7/3, \
///     3/8]"
/// )
/// ```
pub fn exhaustive_positive_rationals() -> ExhaustivePositiveRationals {
    ExhaustivePositiveRationals {
        pred_pred: Natural::ZERO,
        pred: Natural::ONE,
    }
}

/// Generates all non-positive `Rational`s.
///
/// Zero is generated first, followed by all the positive rationals. See
/// `exhaustive_positive_rationals` for details.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// TODO
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::strings::ExtraToString;
/// use malachite_q::exhaustive::exhaustive_non_negative_rationals;
///
/// assert_eq!(
///     exhaustive_non_negative_rationals().take(20).collect_vec().to_string(),
///     "[0, 1, 1/2, 2, 1/3, 3/2, 2/3, 3, 1/4, 4/3, 3/5, 5/2, 2/5, 5/3, 3/4, 4, 1/5, 5/4, 4/7, \
///     7/3]"
/// )
/// ```
pub fn exhaustive_non_negative_rationals() -> Chain<Once<Rational>, ExhaustivePositiveRationals> {
    once(Rational::ZERO).chain(exhaustive_positive_rationals())
}

/// Generates all negative `Rational`s.
///
/// This `struct` is created by the `exhaustive_negative_rationals` function. See its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNegativeRationals {
    xs: ExhaustivePositiveRationals,
}

impl Iterator for ExhaustiveNegativeRationals {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        self.xs.next().map(|mut q| {
            q.sign = false;
            q
        })
    }
}

/// Generates all negative `Rational`s.
///
/// The sequence is the same as the sequence of positive `Rational`s, but negated. See
/// `exhaustive_positive_rationals` for details.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// TODO
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::strings::ExtraToString;
/// use malachite_q::exhaustive::exhaustive_negative_rationals;
///
/// assert_eq!(
///     exhaustive_negative_rationals().take(20).collect_vec().to_string(),
///     "[-1, -1/2, -2, -1/3, -3/2, -2/3, -3, -1/4, -4/3, -3/5, -5/2, -2/5, -5/3, -3/4, -4, -1/5, \
///     -5/4, -4/7, -7/3, -3/8]"
/// )
/// ```
pub fn exhaustive_negative_rationals() -> ExhaustiveNegativeRationals {
    ExhaustiveNegativeRationals {
        xs: exhaustive_positive_rationals(),
    }
}

/// Generates all nonzero `Rational`s.
///
/// This `struct` is created by the `exhaustive_nonzero_rationals` function. See its documentation
/// for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNonzeroRationals {
    xs: ExhaustivePositiveRationals,
    x: Option<Rational>,
    sign: bool,
}

impl Iterator for ExhaustiveNonzeroRationals {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        if self.sign {
            self.sign = false;
            let mut x = None;
            swap(&mut self.x, &mut x);
            let mut x = x.unwrap();
            x.sign = false;
            Some(x)
        } else {
            self.sign = true;
            self.x = self.xs.next();
            Some(self.x.clone().unwrap())
        }
    }
}

/// Generates all nonzero `Rational`s.
///
/// The sequence is the same the sequence of positive `Rational`s, interleaved with its negative.
/// See `exhaustive_positive_rationals` for details.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// TODO
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::strings::ExtraToString;
/// use malachite_q::exhaustive::exhaustive_nonzero_rationals;
///
/// assert_eq!(
///     exhaustive_nonzero_rationals().take(20).collect_vec().to_string(),
///     "[1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, -3/2, 2/3, -2/3, 3, -3, 1/4, -1/4, 4/3, -4/3, \
///     3/5, -3/5]"
/// )
/// ```
pub fn exhaustive_nonzero_rationals() -> ExhaustiveNonzeroRationals {
    ExhaustiveNonzeroRationals {
        xs: exhaustive_positive_rationals(),
        x: None,
        sign: false,
    }
}

/// Generates all `Rational`s.
///
/// The sequence begins with zero and is followed by the sequence of positive `Rational`s,
/// interleaved with its negative. See `exhaustive_rationals` for details.
///
/// The output length is infinite.
///
/// # Worst-case complexity per iteration
/// TODO
///
/// # Examples
/// ```
/// extern crate itertools;
/// extern crate malachite_base;
///
/// use itertools::Itertools;
/// use malachite_base::strings::ExtraToString;
/// use malachite_q::exhaustive::exhaustive_rationals;
///
/// assert_eq!(
///     exhaustive_rationals().take(20).collect_vec().to_string(),
///     "[0, 1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, -3/2, 2/3, -2/3, 3, -3, 1/4, -1/4, 4/3, \
///     -4/3, 3/5]"
/// )
/// ```
pub fn exhaustive_rationals() -> Chain<Once<Rational>, ExhaustiveNonzeroRationals> {
    once(Rational::ZERO).chain(exhaustive_nonzero_rationals())
}
