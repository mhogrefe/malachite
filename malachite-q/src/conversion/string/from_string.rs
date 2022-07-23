use crate::Rational;
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;
use std::str::FromStr;

impl FromStr for Rational {
    type Err = ();

    /// Converts an string to a [`Rational`].
    ///
    /// If the string does not represent a valid [`Rational`], an `Err` is returned. The numerator
    /// and denominator do not need to be in lowest terms, but the denominator must be nonzero. A
    /// negative sign is only allowed at the 0th position of the string.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `s.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::from_str("123456").unwrap(), 123456);
    /// assert_eq!(Rational::from_str("00123456").unwrap(), 123456);
    /// assert_eq!(Rational::from_str("0").unwrap(), 0);
    /// assert_eq!(Rational::from_str("-123456").unwrap(), -123456);
    /// assert_eq!(Rational::from_str("-00123456").unwrap(), -123456);
    /// assert_eq!(Rational::from_str("-0").unwrap(), 0);
    /// assert_eq!(Rational::from_str("22/7").unwrap().to_string(), "22/7");
    /// assert_eq!(Rational::from_str("01/02").unwrap().to_string(), "1/2");
    /// assert_eq!(Rational::from_str("3/21").unwrap().to_string(), "1/7");
    /// assert_eq!(Rational::from_str("-22/7").unwrap().to_string(), "-22/7");
    /// assert_eq!(Rational::from_str("-01/02").unwrap().to_string(), "-1/2");
    /// assert_eq!(Rational::from_str("-3/21").unwrap().to_string(), "-1/7");
    ///
    /// assert!(Rational::from_str("").is_err());
    /// assert!(Rational::from_str("a").is_err());
    /// assert!(Rational::from_str("1/0").is_err());
    /// assert!(Rational::from_str("/1").is_err());
    /// assert!(Rational::from_str("1/").is_err());
    /// assert!(Rational::from_str("--1").is_err());
    /// assert!(Rational::from_str("1/-2").is_err());
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Rational, ()> {
        let (abs_string, sign) = if let Some(abs_string) = s.strip_prefix('-') {
            (abs_string, false)
        } else {
            (s, true)
        };
        let numerator;
        let denominator;
        if let Some(slash_index) = abs_string.find('/') {
            numerator = Natural::from_str(&abs_string[..slash_index])?;
            denominator = Natural::from_str(&abs_string[slash_index + 1..])?;
            if denominator == 0u32 {
                return Err(());
            }
        } else {
            numerator = Natural::from_str(abs_string)?;
            denominator = Natural::ONE;
        }
        Ok(Rational::from_sign_and_naturals(
            sign,
            numerator,
            denominator,
        ))
    }
}
