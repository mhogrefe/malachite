// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::fmt::{Formatter, Write};
use malachite_base::num::conversion::string::options::ToSciOptions;
use malachite_base::num::conversion::traits::ToSci;

impl ToSci for Integer {
    /// Determines whether an [`Integer`] can be converted to a string using
    /// [`to_sci`](`Self::to_sci`) and a particular set of options.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut options = ToSciOptions::default();
    /// assert!(Integer::from(123).fmt_sci_valid(options));
    /// assert!(Integer::from(u128::MAX).fmt_sci_valid(options));
    /// // u128::MAX has more than 16 significant digits
    /// options.set_rounding_mode(Exact);
    /// assert!(!Integer::from(u128::MAX).fmt_sci_valid(options));
    /// options.set_precision(50);
    /// assert!(Integer::from(u128::MAX).fmt_sci_valid(options));
    /// ```
    #[inline]
    fn fmt_sci_valid(&self, options: ToSciOptions) -> bool {
        self.unsigned_abs_ref().fmt_sci_valid(options)
    }

    /// Converts an [`Integer`] to a string using a specified base, possibly formatting the number
    /// using scientific notation.
    ///
    /// See [`ToSciOptions`] for details on the available options. Note that setting
    /// `neg_exp_threshold` has no effect, since there is never a need to use negative exponents
    /// when representing an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `options.rounding_mode` is `Exact`, but the size options are such that the input
    /// must be rounded.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::conversion::string::options::ToSciOptions;
    /// use malachite_base::num::conversion::traits::ToSci;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(u128::MAX).to_sci().to_string(),
    ///     "3.402823669209385e38"
    /// );
    /// assert_eq!(
    ///     Integer::from(i128::MIN).to_sci().to_string(),
    ///     "-1.701411834604692e38"
    /// );
    ///
    /// let n = Integer::from(123456u32);
    /// let mut options = ToSciOptions::default();
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "123456");
    ///
    /// options.set_precision(3);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.23e5");
    ///
    /// options.set_rounding_mode(Ceiling);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.24e5");
    ///
    /// options.set_e_uppercase();
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.24E5");
    ///
    /// options.set_force_exponent_plus_sign(true);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.24E+5");
    ///
    /// options = ToSciOptions::default();
    /// options.set_base(36);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "2n9c");
    ///
    /// options.set_uppercase();
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "2N9C");
    ///
    /// options.set_base(2);
    /// options.set_precision(10);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.1110001e16");
    ///
    /// options.set_include_trailing_zeros(true);
    /// assert_eq!(n.to_sci_with_options(options).to_string(), "1.111000100e16");
    /// ```
    fn fmt_sci(&self, f: &mut Formatter, mut options: ToSciOptions) -> core::fmt::Result {
        let abs = self.unsigned_abs_ref();
        if *self >= 0u32 {
            abs.fmt_sci(f, options)
        } else {
            options.set_rounding_mode(-options.get_rounding_mode());
            f.write_char('-')?;
            abs.fmt_sci(f, options)
        }
    }
}
