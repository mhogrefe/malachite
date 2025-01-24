// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// [`FromSciString`](super::traits::FromSciString), a trait for converting strings, possibly using
/// scientific notation, to numbers.
///
/// # from_sci_string
/// ```
/// use malachite_base::num::conversion::traits::FromSciString;
///
/// assert_eq!(u8::from_sci_string("123"), Some(123));
/// assert_eq!(u8::from_sci_string("123.5"), Some(124));
/// assert_eq!(u8::from_sci_string("256"), None);
/// assert_eq!(u64::from_sci_string("1.23e10"), Some(12300000000));
/// ```
///
/// # from_sci_string_with_options
/// ```
/// use malachite_base::num::conversion::string::options::FromSciStringOptions;
/// use malachite_base::num::conversion::traits::FromSciString;
/// use malachite_base::rounding_modes::RoundingMode::*;
///
/// let mut options = FromSciStringOptions::default();
/// assert_eq!(
///     u8::from_sci_string_with_options("123.5", options),
///     Some(124)
/// );
///
/// options.set_rounding_mode(Floor);
/// assert_eq!(
///     u8::from_sci_string_with_options("123.5", options),
///     Some(123)
/// );
///
/// options = FromSciStringOptions::default();
/// options.set_base(16);
/// assert_eq!(u8::from_sci_string_with_options("ff", options), Some(255));
/// ```
pub mod from_sci_string;
/// [`FromStringBase`](super::traits::FromStringBase), a trait for converting strings in a specified
/// base to numbers.
pub mod from_string;
/// [`ToSciOptions`](options::ToSciOptions) and
/// [`FromSciSringOptions`](options::FromSciStringOptions), `struct`s for specifying parameters when
/// using the [`FromSciString`](super::traits::FromSciString) and [`ToSci`](super::traits::ToSci)
/// traits.
pub mod options;
/// [`ToSci`](super::traits::ToSci), a trait for converting a number to string, possibly using
/// scientific notation.
///
/// # to_sci
/// ```
/// use malachite_base::num::conversion::traits::ToSci;
///
/// // If the value can fit in a `u32`, the result is the same as with `to_string`
/// assert_eq!(123u8.to_sci().to_string(), "123");
///
/// assert_eq!(u128::MAX.to_sci().to_string(), "3.402823669209385e38");
/// assert_eq!(i128::MIN.to_sci().to_string(), "-1.701411834604692e38");
/// ```
///
/// # to_sci_with_options
/// ```
/// use malachite_base::num::conversion::string::options::ToSciOptions;
/// use malachite_base::num::conversion::traits::ToSci;
/// use malachite_base::rounding_modes::RoundingMode::*;
///
/// let mut options = ToSciOptions::default();
/// assert_eq!(123456u32.to_sci_with_options(options).to_string(), "123456");
///
/// options.set_precision(3);
/// assert_eq!(123456u32.to_sci_with_options(options).to_string(), "1.23e5");
///
/// options.set_rounding_mode(Ceiling);
/// assert_eq!(123456u32.to_sci_with_options(options).to_string(), "1.24e5");
///
/// options.set_e_uppercase();
/// assert_eq!(123456u32.to_sci_with_options(options).to_string(), "1.24E5");
///
/// options.set_force_exponent_plus_sign(true);
/// assert_eq!(
///     123456u32.to_sci_with_options(options).to_string(),
///     "1.24E+5"
/// );
///
/// options = ToSciOptions::default();
/// options.set_base(36);
/// assert_eq!(123456u32.to_sci_with_options(options).to_string(), "2n9c");
///
/// options.set_uppercase();
/// assert_eq!(123456u32.to_sci_with_options(options).to_string(), "2N9C");
///
/// options.set_base(2);
/// options.set_precision(10);
/// assert_eq!(
///     123456u32.to_sci_with_options(options).to_string(),
///     "1.1110001e16"
/// );
///
/// options.set_include_trailing_zeros(true);
/// assert_eq!(
///     123456u32.to_sci_with_options(options).to_string(),
///     "1.111000100e16"
/// );
/// ```
///
/// # fmt_sci_valid
/// ```
/// use malachite_base::num::conversion::string::options::ToSciOptions;
/// use malachite_base::num::conversion::traits::ToSci;
/// use malachite_base::rounding_modes::RoundingMode::*;
///
/// let mut options = ToSciOptions::default();
/// assert!(123u8.fmt_sci_valid(options));
/// assert!(u128::MAX.fmt_sci_valid(options));
/// options.set_rounding_mode(Exact);
/// assert!(!u128::MAX.fmt_sci_valid(options)); // u128::MAX has more than 16 significant digits
/// options.set_precision(50);
/// assert!(u128::MAX.fmt_sci_valid(options));
/// ```
pub mod to_sci;
/// The [`BaseFmtWrapper`](to_string::BaseFmtWrapper) struct and
/// [`ToStringBase`](super::traits::ToStringBase) trait, used for converting numbers to strings.
///
/// # Display::fmt for BaseFmtWrapper
/// ```
/// use malachite_base::num::conversion::string::to_string::BaseFmtWrapper;
///
/// let x = BaseFmtWrapper::new(1000000000u32, 36);
/// assert_eq!(format!("{}", x), "gjdgxs");
/// assert_eq!(format!("{:#}", x), "GJDGXS");
/// assert_eq!(format!("{:010}", x), "0000gjdgxs");
/// assert_eq!(format!("{:#010}", x), "0000GJDGXS");
///
/// let x = BaseFmtWrapper::new(-1000000000i32, 36);
/// assert_eq!(format!("{}", x), "-gjdgxs");
/// assert_eq!(format!("{:#}", x), "-GJDGXS");
/// assert_eq!(format!("{:010}", x), "-000gjdgxs");
/// assert_eq!(format!("{:#010}", x), "-000GJDGXS");
/// ```
///
/// # Debug::fmt for BaseFmtWrapper
/// ```
/// use malachite_base::num::conversion::string::to_string::BaseFmtWrapper;
///
/// let x = BaseFmtWrapper::new(1000000000u32, 36);
/// assert_eq!(format!("{:?}", x), "gjdgxs");
/// assert_eq!(format!("{:#?}", x), "GJDGXS");
/// assert_eq!(format!("{:010?}", x), "0000gjdgxs");
/// assert_eq!(format!("{:#010?}", x), "0000GJDGXS");
///
/// let x = BaseFmtWrapper::new(-1000000000i32, 36);
/// assert_eq!(format!("{:?}", x), "-gjdgxs");
/// assert_eq!(format!("{:#?}", x), "-GJDGXS");
/// assert_eq!(format!("{:010?}", x), "-000gjdgxs");
/// assert_eq!(format!("{:#010?}", x), "-000GJDGXS");
/// ```
///
/// # to_string_base
/// ```
/// use malachite_base::num::conversion::traits::ToStringBase;
///
/// assert_eq!(1000u16.to_string_base(2), "1111101000");
/// assert_eq!(1000u16.to_string_base(10), "1000");
/// assert_eq!(1000u16.to_string_base(36), "rs");
///
/// assert_eq!(1000i16.to_string_base(2), "1111101000");
/// assert_eq!(1000i16.to_string_base(10), "1000");
/// assert_eq!(1000i16.to_string_base(36), "rs");
///
/// assert_eq!((-1000i16).to_string_base(2), "-1111101000");
/// assert_eq!((-1000i16).to_string_base(10), "-1000");
/// assert_eq!((-1000i16).to_string_base(36), "-rs");
/// ```
///
/// # to_string_base_upper
/// ```
/// use malachite_base::num::conversion::traits::ToStringBase;
///
/// assert_eq!(1000u16.to_string_base_upper(2), "1111101000");
/// assert_eq!(1000u16.to_string_base_upper(10), "1000");
/// assert_eq!(1000u16.to_string_base_upper(36), "RS");
///
/// assert_eq!(1000i16.to_string_base_upper(2), "1111101000");
/// assert_eq!(1000i16.to_string_base_upper(10), "1000");
/// assert_eq!(1000i16.to_string_base_upper(36), "RS");
///
/// assert_eq!((-1000i16).to_string_base_upper(2), "-1111101000");
/// assert_eq!((-1000i16).to_string_base_upper(10), "-1000");
/// assert_eq!((-1000i16).to_string_base_upper(36), "-RS");
/// ```
pub mod to_string;
