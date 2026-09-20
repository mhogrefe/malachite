// Copyright © 2026 Mikhail Hogrefe
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
/// [`ToLatex`](crate::strings::latex::ToLatex) implementations for primitive integers and primitive
/// floats, converting them to LaTeX math-mode fragments.
///
/// # fmt_latex
/// ```
/// use malachite_base::strings::latex::ToLatex;
///
/// assert_eq!(0u8.to_latex().to_string(), "0");
/// assert_eq!(123u32.to_latex().to_string(), "123");
/// assert_eq!((-45i16).to_latex().to_string(), "-45");
/// assert_eq!(i64::MIN.to_latex().to_string(), "-9223372036854775808");
/// ```
///
/// | value      | fragment               | renders as             |
/// |------------|------------------------|------------------------|
/// | `0u8`      | `0`                    | $0$                    |
/// | `123u32`   | `123`                  | $123$                  |
/// | `-45i16`   | `-45`                  | $-45$                  |
/// | `i64::MIN` | `-9223372036854775808` | $-9223372036854775808$ |
///
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::strings::latex::ToLatex;
///
/// assert_eq!(f64::NAN.to_latex().to_string(), r"\text{NaN}");
/// assert_eq!(f64::INFINITY.to_latex().to_string(), r"\infty");
/// assert_eq!(f64::NEGATIVE_INFINITY.to_latex().to_string(), r"-\infty");
/// assert_eq!(0.0f64.to_latex().to_string(), "0.0");
/// assert_eq!((-0.0f64).to_latex().to_string(), "-0.0");
///
/// assert_eq!(1.0f64.to_latex().to_string(), "1.0");
/// assert_eq!(0.00123f64.to_latex().to_string(), "0.00123");
/// assert_eq!(1.0e16f64.to_latex().to_string(), r"1.0 \times 10^{16}");
/// assert_eq!(
///     f32::MIN_POSITIVE_SUBNORMAL.to_latex().to_string(),
///     r"1.0 \times 10^{-45}"
/// );
/// ```
///
/// | value                         | fragment              | renders as            |
/// |-------------------------------|-----------------------|-----------------------|
/// | `f64::NAN`                    | `\text{NaN}`          | $\text{NaN}$          |
/// | `f64::INFINITY`               | `\infty`              | $\infty$              |
/// | `f64::NEGATIVE_INFINITY`      | `-\infty`             | $-\infty$             |
/// | `0.0`                         | `0.0`                 | $0.0$                 |
/// | `-0.0`                        | `-0.0`                | $-0.0$                |
/// | `1.0`                         | `1.0`                 | $1.0$                 |
/// | `0.00123`                     | `0.00123`             | $0.00123$             |
/// | `1.0e16`                      | `1.0 \times 10^{16}`  | $1.0 \times 10^{16}$  |
/// | `f32::MIN_POSITIVE_SUBNORMAL` | `1.0 \times 10^{-45}` | $1.0 \times 10^{-45}$ |
pub mod latex;
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
/// assert_eq!((-1000i16).to_string_base(62), "-G8");
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
/// [`ToTypst`](crate::strings::typst::ToTypst) implementations for primitive integers and primitive
/// floats, converting them to Typst math-mode fragments.
///
/// # fmt_typst
/// ```
/// use malachite_base::strings::typst::ToTypst;
///
/// assert_eq!(0u8.to_typst().to_string(), "0");
/// assert_eq!(123u32.to_typst().to_string(), "123");
/// assert_eq!((-45i16).to_typst().to_string(), "-45");
/// assert_eq!(i64::MIN.to_typst().to_string(), "-9223372036854775808");
/// ```
///
/// | value      | fragment               |
/// |------------|------------------------|
/// | `0u8`      | `0`                    |
/// | `123u32`   | `123`                  |
/// | `-45i16`   | `-45`                  |
/// | `i64::MIN` | `-9223372036854775808` |
///
/// ```
/// use malachite_base::num::basic::floats::PrimitiveFloat;
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::strings::typst::ToTypst;
///
/// assert_eq!(f64::NAN.to_typst().to_string(), r#""NaN""#);
/// assert_eq!(f64::INFINITY.to_typst().to_string(), "infinity");
/// assert_eq!(f64::NEGATIVE_INFINITY.to_typst().to_string(), "-infinity");
/// assert_eq!(0.0f64.to_typst().to_string(), "0.0");
/// assert_eq!((-0.0f64).to_typst().to_string(), "-0.0");
///
/// assert_eq!(1.0f64.to_typst().to_string(), "1.0");
/// assert_eq!(0.00123f64.to_typst().to_string(), "0.00123");
/// assert_eq!(1.0e16f64.to_typst().to_string(), "1.0 times 10^(16)");
/// assert_eq!(
///     f32::MIN_POSITIVE_SUBNORMAL.to_typst().to_string(),
///     "1.0 times 10^(-45)"
/// );
/// ```
///
/// | value                         | fragment             |
/// |-------------------------------|----------------------|
/// | `f64::NAN`                    | `"NaN"`              |
/// | `f64::INFINITY`               | `infinity`           |
/// | `f64::NEGATIVE_INFINITY`      | `-infinity`          |
/// | `0.0`                         | `0.0`                |
/// | `-0.0`                        | `-0.0`               |
/// | `1.0`                         | `1.0`                |
/// | `0.00123`                     | `0.00123`            |
/// | `1.0e16`                      | `1.0 times 10^(16)`  |
/// | `f32::MIN_POSITIVE_SUBNORMAL` | `1.0 times 10^(-45)` |
pub mod typst;
