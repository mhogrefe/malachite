use arithmetic::traits::SimplestRationalInInterval;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::conversion::string::from_sci_string::preprocess_sci_string;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::FromSciString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::conversion::string::from_sci_string::FromSciStringHelper;
use Rational;

impl FromSciString for Rational {
    /// Converts a string, possibly in scientfic notation, to a `Rational`.
    ///
    /// Use `FromSciStringOptions` to specify the base (from 2 to 36, inclusive). The rounding mode
    /// option is ignored.
    ///
    /// If the base is greater than 10, the higher digits are represented by the letters `'a'`
    /// through `'z'` or `'A'` through `'Z'`; the case doesn't matter and doesn't need to be
    /// consistent.
    ///
    /// Exponents are allowed, and are indicated using the character `'e'` or `'E'`. If the base is
    /// 15 or greater, and ambiguity arises where it may not be clear whether `'e'` is a digit or
    /// an exponent indicator. To resolve this ambiguity, always use a `'+'` or `'-'` sign after
    /// the exponent indicator when the base is 15 or greater.
    ///
    /// The exponent itself is always parsed using base 10.
    ///
    /// Decimal (or other-base) points are allowed.
    ///
    /// If the string is unparseable, `None` is returned.
    ///
    /// This function is very literal; given "0.333", it will return $333/1000$ rather than $1/3$.
    /// If you'd prefer that it return $1/3$, consider `from_sci_string_simplest`. However, that
    /// function has its quirks too: given "0.1", it will not return $1/10$ (see its documentation
    /// for an explanation of this behavior). This function _does_ return $1/10$.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::conversion::string::options::FromSciStringOptions;
    /// use malachite_base::num::conversion::traits::FromSciString;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_sci_string("123").unwrap(), 123);
    /// assert_eq!(Rational::from_sci_string("0.1").unwrap().to_string(), "1/10");
    /// assert_eq!(Rational::from_sci_string("0.333").unwrap().to_string(), "333/1000");
    /// assert_eq!(Rational::from_sci_string("1.2e5").unwrap(), 120000);
    /// assert_eq!(Rational::from_sci_string("1.2e-5").unwrap().to_string(), "3/250000");
    ///
    /// let mut options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(Rational::from_sci_string_with_options("ff", options).unwrap(), 255);
    /// assert_eq!(Rational::from_sci_string_with_options("ffE+5", options).unwrap(), 267386880);
    /// assert_eq!(
    ///     Rational::from_sci_string_with_options("ffE-5", options).unwrap().to_string(),
    ///     "255/1048576"
    /// );
    /// ```
    fn from_sci_string_with_options(s: &str, options: FromSciStringOptions) -> Option<Rational> {
        let (s, exponent) = preprocess_sci_string(s, options)?;
        let x = Rational::from(Integer::parse_int(&s, options.get_base())?);
        Some(x * Rational::from(options.get_base()).pow(exponent))
    }
}

impl Rational {
    /// Converts a string, possibly in scientfic notation, to a `Natural`. This function finds the
    /// simplest `Rational` which rounds to the target string according to the precision implied by
    /// the string.
    ///
    /// Use `FromSciStringOptions` to specify the base (from 2 to 36, inclusive). The rounding mode
    /// option is ignored.
    ///
    /// If the base is greater than 10, the higher digits are represented by the letters `'a'`
    /// through `'z'` or `'A'` through `'Z'`; the case doesn't matter and doesn't need to be
    /// consistent.
    ///
    /// Exponents are allowed, and are indicated using the character `'e'` or `'E'`. If the base is
    /// 15 or greater, and ambiguity arises where it may not be clear whether `'e'` is a digit or
    /// an exponent indicator. To resolve this ambiguity, always use a `'+'` or `'-'` sign after
    /// the exponent indicator when the base is 15 or greater.
    ///
    /// The exponent itself is always parsed using base 10.
    ///
    /// Decimal (or other-base) points are allowed.
    ///
    /// If the string is unparseable, `None` is returned.
    ///
    /// Here's a more precise description of the function's behavior. Suppose we are using base
    /// $b$, and the literal value of the string (as parsed by `from_sci_string`) is q, and the
    /// implied scale is $s$ (meaning $s$ digits are provided after the point; if the string is
    /// "123.456", then $s$ is 3). Then this function computes $\epsilon = b^{-s}/2$ and finds the
    /// simplest `Rational` in the closed interval $[q - \epsilon, q + \epsilon]$. The simplest
    /// `Rational` is the one with minimal denominator; if there are multiple such `Rational`s, the
    /// one with the smallest absolute numerator is chosen.
    ///
    /// This method allows the function to convert "0.333" to $1/3$, since $1/3$ is the simplest
    /// `Rational` in the interval $[0.3325, 0.3335]$. But note that if the scale of the input is
    /// low, some unexpected behavior may occur. For example, "0.1" will be converted into $1/7$
    /// rather than $1/10$, since $1/7$ is the simplest `Rational` in $[0.05, 0.15]$. If you'd
    /// prefer that result be $1/10$, you have a few options:
    /// - Use `from_sci_string` instead. This function interprets its input literally; it converts
    ///   "0.333" to $333/1000$.
    /// - Increase the scale of the input; "0.10" is converted to $1/10$.
    /// - Use `from_sci_string`, and round the result manually using functions like
    ///   `round_to_multiple` and `simplest_rational_in_closed_interval`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::conversion::string::options::FromSciStringOptions;
    /// use malachite_base::num::conversion::traits::FromSciString;
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from_sci_string("123").unwrap(), 123);
    /// assert_eq!(Rational::from_sci_string("0.1").unwrap().to_string(), "1/10");
    /// assert_eq!(Rational::from_sci_string("0.333").unwrap().to_string(), "333/1000");
    /// assert_eq!(Rational::from_sci_string("1.2e5").unwrap(), 120000);
    /// assert_eq!(Rational::from_sci_string("1.2e-5").unwrap().to_string(), "3/250000");
    ///
    /// let mut options = FromSciStringOptions::default();
    /// options.set_base(16);
    /// assert_eq!(Rational::from_sci_string_with_options("ff", options).unwrap(), 255);
    /// assert_eq!(Rational::from_sci_string_with_options("ffE+5", options).unwrap(), 267386880);
    /// assert_eq!(
    ///     Rational::from_sci_string_with_options("ffE-5", options).unwrap().to_string(),
    ///     "255/1048576"
    /// );
    /// ```
    pub fn from_sci_string_simplest_with_options(
        s: &str,
        options: FromSciStringOptions,
    ) -> Option<Rational> {
        let (s, exponent) = preprocess_sci_string(s, options)?;
        let x = Rational::from(Integer::parse_int(&s, options.get_base())?);
        let p = Rational::from(options.get_base()).pow(exponent);
        let q = x * &p;
        if exponent >= 0 {
            Some(q)
        } else {
            let epsilon = p >> 1;
            Some(Rational::simplest_rational_in_closed_interval(
                &(&q - &epsilon),
                &(q + epsilon),
            ))
        }
    }

    #[inline]
    pub fn from_sci_string_simplest(s: &str) -> Option<Rational> {
        Rational::from_sci_string_simplest_with_options(s, FromSciStringOptions::default())
    }
}
