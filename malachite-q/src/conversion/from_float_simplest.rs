use crate::arithmetic::traits::SimplestRationalInInterval;
use malachite_base::num::basic::floats::PrimitiveFloat;
use crate::Rational;

impl Rational {
    /// Converts a primitive float to the simplest [`Rational`] that rounds to that value.
    ///
    /// To be more specific: Suppose the floating-point input is $x$. If $x$ is an integer, its
    /// [`Rational`] equivalent is returned. Otherwise, this function finds $a$ and $b$, which are
    /// the floating point predecessor and successor of $x$, and finds the simplest [`Rational`] in
    /// the open interval $(\frac{x + a}{2}, \frac{x + b}{2})$. "Simplicity" refers to low
    /// complexity. See [`Rational::cmp_complexity`] for a definition of complexity.
    ///
    /// For example, `0.1f32` is converted to $1/10$ rather than to the exact value of the float,
    /// which is $13421773/134217728$. If you want the exact value, use `Rational::from` instead.
    ///
    /// The floating point value cannot be NaN or infinite.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.sci_exponent()`.
    ///
    /// # Panics
    /// Panics if `x` is NaN or infinite.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_float_simplest(0.0), 0);
    /// assert_eq!(Rational::from_float_simplest(1.5).to_string(), "3/2");
    /// assert_eq!(Rational::from_float_simplest(-1.5).to_string(), "-3/2");
    /// assert_eq!(Rational::from_float_simplest(0.1f32).to_string(), "1/10");
    /// assert_eq!(Rational::from_float_simplest(0.33333334f32).to_string(), "1/3");
    /// ```
    pub fn from_float_simplest<T: PrimitiveFloat>(x: T) -> Rational
    where
        Rational: From<T>,
    {
        let q = Rational::from(x);
        if *q.denominator_ref() <= 2u32 {
            q
        } else {
            let succ_q = Rational::from(x.next_higher());
            let pred_q = Rational::from(x.next_lower());
            let x = (pred_q + &q) >> 1;
            let y = (succ_q + q) >> 1;
            Rational::simplest_rational_in_open_interval(&x, &y)
        }
    }
}
