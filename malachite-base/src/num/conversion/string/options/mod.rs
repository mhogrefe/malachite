// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rounding_modes::RoundingMode::{self, *};

/// A `struct` determining how much "detail" should be used when creating a scientific-notation
/// string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SciSizeOptions {
    /// Indicates that the number should be rendered in its full precision.
    Complete,
    /// Indicates how many significant figures should be shown. The precision cannot be zero.
    Precision(u64),
    /// Indicates how many digits after the decimal (or other-base) point should be shown. For
    /// example, if the base is 10 and the scale is 2, then up to two digits after the decimal point
    /// should be shown.
    Scale(u64),
}

impl Default for SciSizeOptions {
    fn default() -> SciSizeOptions {
        SciSizeOptions::Precision(16) // Similar to f64 string output
    }
}

#[cfg(feature = "test_build")]
impl SciSizeOptions {
    pub const fn is_valid(&self) -> bool {
        if let SciSizeOptions::Precision(p) = *self {
            p != 0
        } else {
            true
        }
    }
}

/// A `struct` determining how a number should be formatted as a "scientific" string.
///
/// - The base must be between 2 and 36, inclusive. The characters representing the digits are `'0'`
///   through `'9'` and either `'a'` through `'z'` or `'A'` through `'Z'`, depending on whether the
///   `lowercase` field is set. The default base is 10.
///
/// - The rounding mode determines how the output should be rounded, in case the `size_options`
///   field is such that the number can't be fully represented. The default rounding mode is
///   `Nearest`.
///
/// - The size options determine the precision or scale that the number should be displayed with.
///   The default is `Precision(16)`, which is about as much precision as an `f64` is usually
///   displayed with.
///
/// - The negative exponent threshold determines when small numbers switch to scientific notation.
///   The default is $-6$, meaning that the numbers $1/10, 1/100, 1/1000, \ldots$. would be
///   displayed as `0.1, 0.01, 0.001, 0.0001, 0.00001, 1e-6, 1e-7...`. The threshold must be
/// negative.
///
/// - The lowercase setting determines whether digits in bases greater than 10 are lowercase or
///   uppercase. The default is `true`.
///
/// - The exponent lowercase setting determines whether the exponent indicator is lowercase (`'e'`)
///   or uppercase (`'E'`). The default is `true`.
///
/// - The "force exponent plus sign" setting determines whether positive exponents should be
///   rendered with an explicit plus sign. If the base is 15 or greater, then the explicit plus sign
///   is used regardless, in order to distinguish the exponent indicator from the digit `'e'`. The
///   default is `false`.
///
/// - The "include trailing zeros" setting determines whether trailing zeros after the decimal (or
///   other-base) point should be included. The default is `false`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToSciOptions {
    pub(crate) base: u8,
    pub(crate) rounding_mode: RoundingMode,
    pub(crate) size_options: SciSizeOptions,
    neg_exp_threshold: i64,
    pub(crate) lowercase: bool,
    pub(crate) e_lowercase: bool,
    pub(crate) force_exponent_plus_sign: bool,
    pub(crate) include_trailing_zeros: bool,
}

impl Default for ToSciOptions {
    fn default() -> ToSciOptions {
        ToSciOptions {
            base: 10,
            rounding_mode: Nearest,
            size_options: SciSizeOptions::default(),
            neg_exp_threshold: -6,
            lowercase: true,
            e_lowercase: true,
            force_exponent_plus_sign: false,
            include_trailing_zeros: false,
        }
    }
}

impl ToSciOptions {
    /// Returns the base to be used in the conversion. It is always between 2 and 36, inclusive.
    #[inline]
    pub const fn get_base(&self) -> u8 {
        self.base
    }

    /// Returns the rounding mode to be used in the conversion.
    #[inline]
    pub const fn get_rounding_mode(&self) -> RoundingMode {
        self.rounding_mode
    }

    /// Returns the size options to be used in the conversion.
    #[inline]
    pub const fn get_size_options(&self) -> SciSizeOptions {
        self.size_options
    }

    /// Returns the exponent low threshold to be used in the conversion. It is always negative.
    #[inline]
    pub const fn get_neg_exp_threshold(&self) -> i64 {
        self.neg_exp_threshold
    }

    /// Returns whether the digits should be lowercase.
    #[inline]
    pub const fn get_lowercase(&self) -> bool {
        self.lowercase
    }

    /// Returns whether the exponent indicator should be lowercase (`'e'` rather than `'E'`).
    #[inline]
    pub const fn get_e_lowercase(&self) -> bool {
        self.e_lowercase
    }

    /// Returns whether positive exponents should always be preceded by an explicit plus sign.
    #[inline]
    pub const fn get_force_exponent_plus_sign(&self) -> bool {
        self.force_exponent_plus_sign
    }

    /// Returns whether trailing zeros should be included after the decimal (or other-base) point.
    #[inline]
    pub const fn get_include_trailing_zeros(&self) -> bool {
        self.include_trailing_zeros
    }

    /// Sets the base to be used in the conversion.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    #[inline]
    pub fn set_base(&mut self, base: u8) {
        assert!(base >= 2);
        assert!(base <= 36);
        self.base = base;
    }

    /// Sets the rounding mode to be used in the conversion.
    #[inline]
    pub fn set_rounding_mode(&mut self, rm: RoundingMode) {
        self.rounding_mode = rm;
    }

    /// Sets the size options to the "Complete" mode, indicating that the number should be converted
    /// using its full precision.
    #[inline]
    pub fn set_size_complete(&mut self) {
        self.size_options = SciSizeOptions::Complete;
    }

    /// Sets the size options to some precision, or number of significant digits.
    ///
    /// # Panics
    /// Panics if `precision` is zero.
    #[inline]
    pub fn set_precision(&mut self, precision: u64) {
        assert_ne!(precision, 0);
        self.size_options = SciSizeOptions::Precision(precision);
    }

    /// Sets the size options to some scale, or number of digits after the decimal (or other-base)
    /// point.
    #[inline]
    pub fn set_scale(&mut self, scale: u64) {
        self.size_options = SciSizeOptions::Scale(scale);
    }

    /// Sets the threshold at which nonzero numbers with a small absolute value start being
    /// represented using negative exponents.
    #[inline]
    pub fn set_neg_exp_threshold(&mut self, neg_exp_threshold: i64) {
        assert!(neg_exp_threshold < 0);
        self.neg_exp_threshold = neg_exp_threshold;
    }

    /// Specifies that digits in bases greater than ten should be output with lowercase letters.
    #[inline]
    pub fn set_lowercase(&mut self) {
        self.lowercase = true;
    }

    /// Specifies that digits in bases greater than ten should be output with uppercase letters.
    #[inline]
    pub fn set_uppercase(&mut self) {
        self.lowercase = false;
    }

    /// Specifies that the exponent-indicating character should be `'e'`.
    #[inline]
    pub fn set_e_lowercase(&mut self) {
        self.e_lowercase = true;
    }

    /// Specifies that the exponent-indicating character should be `'E'`.
    #[inline]
    pub fn set_e_uppercase(&mut self) {
        self.e_lowercase = false;
    }

    /// Sets whether a positive exponent should be preceded by an explicit plus sign.
    ///
    /// If the base is 15 or greater, an explicit plus sign will be used regardless, in order to
    /// differentiate the exponent-indicating character from the digit `'e'`.
    #[inline]
    pub fn set_force_exponent_plus_sign(&mut self, force_exponent_plus_sign: bool) {
        self.force_exponent_plus_sign = force_exponent_plus_sign;
    }

    /// Sets whether trailing zeros after the decimal (or other-base) point should be included.
    #[inline]
    pub fn set_include_trailing_zeros(&mut self, include_trailing_zeros: bool) {
        self.include_trailing_zeros = include_trailing_zeros;
    }

    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        (2..=36).contains(&self.base) && self.neg_exp_threshold < 0 && self.size_options.is_valid()
    }
}

/// A `struct` determining how a number should be parsed from a "scientific" string.
///
/// - The base must be between 2 and 36, inclusive. The characters representing the digits may be
///   `'0'` through `'9'` and either `'a'` through `'z'` or `'A'` through `'Z'`. The default base is
///   10.
///
/// - The rounding mode determines how the output should be rounded, in case the output type can't
///   represent all possible input strings. The default rounding mode is `Nearest`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FromSciStringOptions {
    pub(crate) base: u8,
    pub(crate) rounding_mode: RoundingMode,
}

impl Default for FromSciStringOptions {
    fn default() -> FromSciStringOptions {
        FromSciStringOptions {
            base: 10,
            rounding_mode: Nearest,
        }
    }
}

impl FromSciStringOptions {
    /// Returns the base to be used in the conversion. It is always between 2 and 36, inclusive.
    #[inline]
    pub const fn get_base(&self) -> u8 {
        self.base
    }

    /// Returns the rounding mode to be used in the conversion.
    #[inline]
    pub const fn get_rounding_mode(&self) -> RoundingMode {
        self.rounding_mode
    }

    /// Sets the base to be used in the conversion.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    #[inline]
    pub fn set_base(&mut self, base: u8) {
        assert!(base >= 2);
        assert!(base <= 36);
        self.base = base;
    }

    /// Sets the rounding mode to be used in the conversion.
    #[inline]
    pub fn set_rounding_mode(&mut self, rm: RoundingMode) {
        self.rounding_mode = rm;
    }

    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        (2..=36).contains(&self.base)
    }
}

/// Iterators that generate [`SciSizeOptions`], [`ToSciOptions`], and [`FromSciStringOptions`]
/// without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`SciSizeOptions`], [`ToSciOptions`], and [`FromSciStringOptions`]
/// randomly.
pub mod random;
