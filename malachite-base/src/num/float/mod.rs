// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::floats::PrimitiveFloat;
use core::cmp::Ordering::{self, *};
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::str::FromStr;

/// `NiceFloat` is a wrapper around primitive float types that provides nicer [`Eq`], [`Ord`],
/// [`Hash`], [`Display`], and [`FromStr`] instances.
///
/// In most languages, floats behave weirdly due to the IEEE 754 standard. The `NiceFloat` type
/// ignores the standard in favor of more intuitive behavior.
/// * Using `NiceFloat`, `NaN`s are equal to themselves. There is a single, unique `NaN`; there's no
///   concept of signalling `NaN`s. Positive and negative zero are two distinct values, not equal to
///   each other.
/// * The `NiceFloat` hash respects this equality.
/// * `NiceFloat` has a total order. These are the classes of floats, in ascending order:
///   - Negative infinity
///   - Negative nonzero finite floats
///   - Negative zero
///   - NaN
///   - Positive zero
///   - Positive nonzero finite floats
///   - Positive infinity
/// * `NiceFloat` uses a different [`Display`] implementation than floats do by default in Rust. For
///   example, Rust will format `f32::MIN_POSITIVE_SUBNORMAL` as something with many zeros, but
///   `NiceFloat(f32::MIN_POSITIVE_SUBNORMAL)` just formats it as `"1.0e-45"`. The conversion
///   function uses David Tolnay's [`ryu`](https://docs.rs/ryu/latest/ryu/) crate, with a few
///   modifications:
///   - All finite floats have a decimal point. For example, Ryu by itself would convert
///     `f32::MIN_POSITIVE_SUBNORMAL` to `"1e-45"`.
///   - Positive infinity, negative infinity, and NaN are converted to the strings `"Infinity"`,
///     `"-Infinity"`, and "`NaN`", respectively.
/// * [`FromStr`] accepts these strings.
#[derive(Clone, Copy, Default)]
pub struct NiceFloat<T: PrimitiveFloat>(pub T);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum FloatType {
    NegativeInfinity,
    NegativeFinite,
    NegativeZero,
    NaN,
    PositiveZero,
    PositiveFinite,
    PositiveInfinity,
}

impl<T: PrimitiveFloat> NiceFloat<T> {
    fn float_type(self) -> FloatType {
        let f = self.0;
        if f.is_nan() {
            FloatType::NaN
        } else if f.sign() == Greater {
            if f == T::ZERO {
                FloatType::PositiveZero
            } else if f.is_finite() {
                FloatType::PositiveFinite
            } else {
                FloatType::PositiveInfinity
            }
        } else if f == T::ZERO {
            FloatType::NegativeZero
        } else if f.is_finite() {
            FloatType::NegativeFinite
        } else {
            FloatType::NegativeInfinity
        }
    }
}

impl<T: PrimitiveFloat> PartialEq<NiceFloat<T>> for NiceFloat<T> {
    /// Compares two `NiceFloat`s for equality.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of an equality operation that
    /// respects the expected properties of symmetry, reflexivity, and transitivity. Using
    /// `NiceFloat`, `NaN`s are equal to themselves. There is a single, unique `NaN`; there's no
    /// concept of signalling `NaN`s. Positive and negative zero are two distinct values, not equal
    /// to each other.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    ///
    /// assert_eq!(NiceFloat(0.0), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(f32::NAN), NiceFloat(f32::NAN));
    /// assert_ne!(NiceFloat(f32::NAN), NiceFloat(0.0));
    /// assert_ne!(NiceFloat(0.0), NiceFloat(-0.0));
    /// assert_eq!(NiceFloat(1.0), NiceFloat(1.0));
    /// ```
    #[inline]
    fn eq(&self, other: &NiceFloat<T>) -> bool {
        let f = self.0;
        let g = other.0;
        f.to_bits() == g.to_bits() || f.is_nan() && g.is_nan()
    }
}

impl<T: PrimitiveFloat> Eq for NiceFloat<T> {}

impl<T: PrimitiveFloat> Hash for NiceFloat<T> {
    /// Computes a hash of a `NiceFloat`.
    ///
    /// The hash is compatible with `NiceFloat` equality: all `NaN`s hash to the same value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn hash<H: Hasher>(&self, state: &mut H) {
        let f = self.0;
        if f.is_nan() {
            "NaN".hash(state);
        } else {
            f.to_bits().hash(state);
        }
    }
}

impl<T: PrimitiveFloat> Ord for NiceFloat<T> {
    /// Compares two `NiceFloat`s.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of a comparison operation that
    /// respects the expected properties of antisymmetry, reflexivity, and transitivity. `NiceFloat`
    /// has a total order. These are the classes of floats, in ascending order:
    ///   - Negative infinity
    ///   - Negative nonzero finite floats
    ///   - Negative zero
    ///   - NaN
    ///   - Positive zero
    ///   - Positive nonzero finite floats
    ///   - Positive infinity
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    ///
    /// assert!(NiceFloat(0.0) > NiceFloat(-0.0));
    /// assert!(NiceFloat(f32::NAN) < NiceFloat(0.0));
    /// assert!(NiceFloat(f32::NAN) > NiceFloat(-0.0));
    /// assert!(NiceFloat(f32::INFINITY) > NiceFloat(f32::NAN));
    /// assert!(NiceFloat(f32::NAN) < NiceFloat(1.0));
    /// ```
    fn cmp(&self, other: &NiceFloat<T>) -> Ordering {
        let self_type = self.float_type();
        let other_type = other.float_type();
        self_type.cmp(&other_type).then_with(|| {
            if self_type == FloatType::PositiveFinite || self_type == FloatType::NegativeFinite {
                self.0.partial_cmp(&other.0).unwrap()
            } else {
                Equal
            }
        })
    }
}

impl<T: PrimitiveFloat> PartialOrd<NiceFloat<T>> for NiceFloat<T> {
    /// Compares a `NiceFloat` to another `NiceFloat`.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &NiceFloat<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[doc(hidden)]
pub trait FmtRyuString: Copy {
    fn fmt_ryu_string(self, f: &mut Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_fmt_ryu_string {
    ($f: ident) => {
        impl FmtRyuString for $f {
            #[inline]
            fn fmt_ryu_string(self, f: &mut Formatter<'_>) -> fmt::Result {
                let mut buffer = ryu::Buffer::new();
                let printed = buffer.format_finite(self);
                // Convert e.g. "1e100" to "1.0e100". `printed` is ASCII, so we can manipulate bytes
                // rather than chars.
                let mut e_index = None;
                let mut found_dot = false;
                for (i, &b) in printed.as_bytes().iter().enumerate() {
                    match b {
                        b'.' => {
                            found_dot = true;
                            break; // If there's a '.', we don't need to do anything
                        }
                        b'e' => {
                            e_index = Some(i);
                            break; // OK to break since there won't be a '.' after an 'e'
                        }
                        _ => {}
                    }
                }
                if found_dot {
                    f.write_str(printed)
                } else {
                    if let Some(e_index) = e_index {
                        let mut out_bytes = ::alloc::vec![0; printed.len() + 2];
                        let (in_bytes_lo, in_bytes_hi) = printed.as_bytes().split_at(e_index);
                        let (out_bytes_lo, out_bytes_hi) = out_bytes.split_at_mut(e_index);
                        out_bytes_lo.copy_from_slice(in_bytes_lo);
                        out_bytes_hi[0] = b'.';
                        out_bytes_hi[1] = b'0';
                        out_bytes_hi[2..].copy_from_slice(in_bytes_hi);
                        f.write_str(core::str::from_utf8(&out_bytes).unwrap())
                    } else {
                        panic!("Unexpected Ryu string: {}", printed);
                    }
                }
            }
        }
    };
}

impl_fmt_ryu_string!(f32);

impl_fmt_ryu_string!(f64);

impl<T: PrimitiveFloat> Display for NiceFloat<T> {
    /// Formats a `NiceFloat` as a string.
    ///
    /// `NiceFloat` uses a different [`Display`] implementation than floats do by default in Rust.
    /// For example, Rust will convert `f32::MIN_POSITIVE_SUBNORMAL` to something with many zeros,
    /// but `NiceFloat(f32::MIN_POSITIVE_SUBNORMAL)` just converts to `"1.0e-45"`. The conversion
    /// function uses David Tolnay's [`ryu`](https://docs.rs/ryu/latest/ryu/) crate, with a few
    /// modifications:
    /// - All finite floats have a decimal point. For example, Ryu by itself would convert
    ///   `f32::MIN_POSITIVE_SUBNORMAL` to `"1e-45"`.
    /// - Positive infinity, negative infinity, and NaN are converted to the strings `"Infinity"`,
    ///   `"-Infinity"`, and "`NaN`", respectively.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    /// use malachite_base::num::float::NiceFloat;
    ///
    /// assert_eq!(NiceFloat(0.0).to_string(), "0.0");
    /// assert_eq!(NiceFloat(-0.0).to_string(), "-0.0");
    /// assert_eq!(NiceFloat(f32::INFINITY).to_string(), "Infinity");
    /// assert_eq!(NiceFloat(f32::NEGATIVE_INFINITY).to_string(), "-Infinity");
    /// assert_eq!(NiceFloat(f32::NAN).to_string(), "NaN");
    ///
    /// assert_eq!(NiceFloat(1.0).to_string(), "1.0");
    /// assert_eq!(NiceFloat(-1.0).to_string(), "-1.0");
    /// assert_eq!(
    ///     NiceFloat(f32::MIN_POSITIVE_SUBNORMAL).to_string(),
    ///     "1.0e-45"
    /// );
    /// assert_eq!(
    ///     NiceFloat(std::f64::consts::E).to_string(),
    ///     "2.718281828459045"
    /// );
    /// assert_eq!(
    ///     NiceFloat(std::f64::consts::PI).to_string(),
    ///     "3.141592653589793"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_nan() {
            f.write_str("NaN")
        } else if self.0.is_infinite() {
            if self.0.sign() == Greater {
                f.write_str("Infinity")
            } else {
                f.write_str("-Infinity")
            }
        } else {
            self.0.fmt_ryu_string(f)
        }
    }
}

impl<T: PrimitiveFloat> Debug for NiceFloat<T> {
    /// Formats a `NiceFloat` as a string.
    ///
    /// This is identical to the [`Display::fmt`] implementation.
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T: PrimitiveFloat> FromStr for NiceFloat<T> {
    type Err = <T as FromStr>::Err;

    /// Converts a `&str` to a `NiceFloat`.
    ///
    /// If the `&str` does not represent a valid `NiceFloat`, an `Err` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ = `src.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(NiceFloat::from_str("NaN").unwrap(), NiceFloat(f32::NAN));
    /// assert_eq!(NiceFloat::from_str("-0.00").unwrap(), NiceFloat(-0.0f64));
    /// assert_eq!(NiceFloat::from_str(".123").unwrap(), NiceFloat(0.123f32));
    /// ```
    #[inline]
    fn from_str(src: &str) -> Result<NiceFloat<T>, <T as FromStr>::Err> {
        match src {
            "NaN" => Ok(T::NAN),
            "Infinity" => Ok(T::INFINITY),
            "-Infinity" => Ok(T::NEGATIVE_INFINITY),
            "inf" | "-inf" => T::from_str("invalid"),
            src => T::from_str(src),
        }
        .map(NiceFloat)
    }
}
