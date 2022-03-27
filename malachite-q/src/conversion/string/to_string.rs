use std::fmt::{Debug, Display, Formatter, Result, Write};
use Rational;

impl Display for Rational {
    /// Converts a `Rational` to a `String`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::ZERO.to_string(), "0");
    /// assert_eq!(Rational::from(123).to_string(), "123");
    /// assert_eq!(Rational::from_str("22/7").unwrap().to_string(), "22/7");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if !self.sign {
            f.write_char('-')?;
        }
        let result = Display::fmt(&self.numerator, f);
        if self.denominator != 1 {
            f.write_char('/')?;
            Display::fmt(&self.denominator, f)
        } else {
            result
        }
    }
}

impl Debug for Rational {
    /// Converts a `Rational` to a `String`.
    ///
    /// This is the same implementation as for `Display`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Rational::ZERO.to_debug_string(), "0");
    /// assert_eq!(Rational::from(123).to_debug_string(), "123");
    /// assert_eq!(Rational::from_str("22/7").unwrap().to_debug_string(), "22/7");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
