// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::ops::Deref;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};
#[cfg(feature = "test_build")]
use malachite_base::num::modular::traits::DivisibleByPowerOf2;
use malachite_base::num::modular::traits::NegModPowerOf2;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

/// A floating-point number with an arbitrarily large exponent.
///
/// A `BigExponentFloat` is either NaN, $\infty$, $-\infty$, zero, or a finite nonzero number $\pm
/// m2^e$, where $m$ is a positive integer and $e$ is an [`Integer`] of any size. Unlike a
/// [`Float`](crate::Float), it differs in three ways:
/// - Its exponent is unbounded, so no operation overflows or underflows.
/// - It has no stored precision: its significand is exactly as long as its value requires, and each
///   operation takes the precision and rounding mode of its result as arguments.
/// - It has no negative zero.
///
/// This is the counterpart of FLINT's `arf_t`.
///
/// Every value has a single representation. The significand of a finite nonzero value is a
/// [`Natural`] whose most significant bit is the high bit of its top limb, like a
/// [`Float`](crate::Float)'s, and whose lowest limb is nonzero. A finite nonzero value with
/// exponent $e$ lies in $[2^{e-1}, 2^e)$ in absolute value, as for [`Float`](crate::Float).
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeBigExponentFloat", into = "SerdeBigExponentFloat")
)]
pub struct BigExponentFloat(pub(crate) InnerBigExponentFloat);

// The serialized form of a `BigExponentFloat`: its exact hexadecimal string.
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct SerdeBigExponentFloat(alloc::string::String);

// Every value has a single representation, so structural equality and hashing are value equality
// and hashing, apart from NaN, which `PartialEq` for `BigExponentFloat` treats separately.
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum InnerBigExponentFloat {
    NaN,
    Infinity {
        sign: bool,
    },
    Zero,
    Finite {
        sign: bool,
        exponent: Integer,
        significand: Natural,
    },
}

impl BigExponentFloat {
    // The finite nonzero `BigExponentFloat` ±m * 2^(e - b), where b is the number of significant
    // bits of the positive `m`, so that `exponent` is e: the value lies in [2^(e - 1), 2^e) in
    // absolute value. Trailing zeros of `m` are removed, and what is left is aligned to the top of
    // its limbs.
    pub(crate) fn from_parts(sign: bool, exponent: Integer, mut m: Natural) -> Self {
        assert_ne!(m, 0u32);
        let zeros = m.trailing_zeros().unwrap();
        m >>= zeros;
        let bits = m.significant_bits();
        m <<= bits.neg_mod_power_of_2(Limb::LOG_WIDTH);
        Self(InnerBigExponentFloat::Finite {
            sign,
            exponent,
            significand: m,
        })
    }

    // The negative of `self`; zero, NaN, and the infinities' magnitudes are unchanged.
    pub(crate) fn negate(mut self) -> Self {
        if let Self(
            InnerBigExponentFloat::Finite { sign, .. } | InnerBigExponentFloat::Infinity { sign },
        ) = &mut self
        {
            sign.not_assign();
        }
        self
    }

    // Removes the trailing zero limbs of a significand whose top limb's high bit is set.
    pub(crate) fn trim_significand(significand: &mut Natural) {
        let zero_limbs = significand.trailing_zeros().unwrap() >> Limb::LOG_WIDTH;
        if zero_limbs != 0 {
            *significand >>= zero_limbs << Limb::LOG_WIDTH;
        }
    }

    /// Determines whether a [`BigExponentFloat`] is valid: whether its significand, if it has one,
    /// is a valid [`Natural`], aligned to the top of its limbs, with a nonzero lowest limb.
    ///
    /// This function is only available when the `test_build` feature is enabled.
    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        match self {
            Self(InnerBigExponentFloat::Finite {
                significand,
                exponent,
                ..
            }) => {
                if !significand.is_valid() || !exponent.is_valid() {
                    return false;
                }
                let bits = significand.significant_bits();
                bits != 0
                    && bits.divisible_by_power_of_2(Limb::LOG_WIDTH)
                    && !significand.divisible_by_power_of_2(Limb::WIDTH)
            }
            _ => true,
        }
    }
}

/// `ComparableBigExponentFloat` is a wrapper around a [`BigExponentFloat`], taking the
/// [`BigExponentFloat`] by value.
///
/// [`BigExponentFloat`] implements [`PartialEq`] and [`PartialOrd`] following IEEE 754, so NaN is
/// not equal to itself and is not comparable to anything. `ComparableBigExponentFloat` implements
/// [`Eq`], [`Ord`], and [`Hash`](core::hash::Hash) instead:
/// - NaN is equal to itself, and to no other value.
/// - The ordering is total: NaN is ordered just below zero, above every negative value. (This
///   agrees with [`ComparableFloat`](crate::ComparableFloat), which places NaN between the two
///   zeros.)
/// - The hash is compatible with equality.
///
/// A [`BigExponentFloat`] has no stored precision and only one zero, so, unlike
/// [`ComparableFloat`](crate::ComparableFloat), equal values are never distinguished.
///
/// `ComparableBigExponentFloat` owns its value, which is useful, for example, for keys in a hash
/// map. In other situations, [`ComparableBigExponentFloatRef`] may be better, since it only holds a
/// reference.
#[derive(Clone)]
pub struct ComparableBigExponentFloat(pub BigExponentFloat);

/// `ComparableBigExponentFloatRef` is a wrapper around a [`BigExponentFloat`], taking the
/// [`BigExponentFloat`] by reference.
///
/// See the [`ComparableBigExponentFloat`] documentation for details.
#[derive(Clone)]
pub struct ComparableBigExponentFloatRef<'a>(pub &'a BigExponentFloat);

impl ComparableBigExponentFloat {
    /// Returns a [`ComparableBigExponentFloatRef`] referring to the wrapped value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// let x = ComparableBigExponentFloat(BigExponentFloat::ONE);
    /// assert_eq!(format!("{:?}", x.as_ref().0), "0x1.0");
    /// ```
    pub const fn as_ref(&self) -> ComparableBigExponentFloatRef<'_> {
        ComparableBigExponentFloatRef(&self.0)
    }
}

impl Deref for ComparableBigExponentFloat {
    type Target = BigExponentFloat;

    /// Allows a [`ComparableBigExponentFloat`] to dereference to a [`BigExponentFloat`].
    ///
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloat;
    ///
    /// let x = ComparableBigExponentFloat(BigExponentFloat::ONE);
    /// assert_eq!(*x, BigExponentFloat::ONE);
    /// ```
    fn deref(&self) -> &BigExponentFloat {
        &self.0
    }
}

impl Deref for ComparableBigExponentFloatRef<'_> {
    type Target = BigExponentFloat;

    /// Allows a [`ComparableBigExponentFloatRef`] to dereference to a [`BigExponentFloat`].
    ///
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
    ///
    /// let x = BigExponentFloat::ONE;
    /// let y = ComparableBigExponentFloatRef(&x);
    /// assert_eq!(*y, BigExponentFloat::ONE);
    /// ```
    fn deref(&self) -> &BigExponentFloat {
        self.0
    }
}

/// Basic functions for working with [`BigExponentFloat`]s.
pub mod basic;
/// Traits for comparing [`BigExponentFloat`]s for equality or order, with each other and with other
/// numeric types.
pub mod comparison;
/// Functions for converting [`BigExponentFloat`]s to and from other types, including strings.
pub mod conversion;
/// Iterators that generate [`BigExponentFloat`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`BigExponentFloat`]s randomly.
pub mod random;
