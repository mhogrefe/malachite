// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, FloorLogBase2, ModPowerOf2, NegModPowerOf2, PowerOf2,
    RoundToMultipleOfPowerOf2Assign, SaturatingSubAssign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, Zero};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, SaturatingFrom};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_q::conversion::primitive_float_from_rational::FloatConversionError;

fn from_natural_prec_round_helper(
    x: &Natural,
    prec: u64,
    rm: RoundingMode,
    bits: u64,
) -> (Float, Ordering) {
    if *x == 0 {
        return (Float::ZERO, Equal);
    }
    let mut exponent = i32::saturating_from(bits);
    if exponent > Float::MAX_EXPONENT {
        return match rm {
            Up | Ceiling | Nearest => (Float::INFINITY, Greater),
            Floor | Down => (Float::max_finite_value_with_prec(prec), Less),
            Exact => panic!("Inexact conversion from Natural to Float"),
        };
    }
    let mut needed_bits = prec;
    let sig_bits_in_highest_limb = bits.mod_power_of_2(Limb::LOG_WIDTH);
    let mut needed_limbs = 1;
    needed_bits.saturating_sub_assign(sig_bits_in_highest_limb);
    if needed_bits != 0 {
        needed_limbs += needed_bits.shr_round(Limb::LOG_WIDTH, Ceiling).0;
    }
    let mut rev_limbs = x.limbs().rev();
    let mut significand = Natural::from_owned_limbs_desc(
        (&mut rev_limbs)
            .take(usize::exact_from(needed_limbs))
            .collect(),
    );
    significand <<= significand
        .significant_bits()
        .neg_mod_power_of_2(Limb::LOG_WIDTH);
    let mut mask_width = significand.significant_bits() - prec;
    let mut erased_limb = 0;
    if mask_width >= Limb::WIDTH {
        erased_limb = significand.limbs()[0];
        significand >>= Limb::WIDTH;
        mask_width -= Limb::WIDTH;
    }
    let o = match rm {
        Exact => {
            let inexact = erased_limb != 0
                || !significand.divisible_by_power_of_2(mask_width)
                || rev_limbs.any(|y| y != 0);
            assert!(!inexact, "Inexact conversion from Natural");
            Equal
        }
        Floor | Down => {
            let inexact = erased_limb != 0
                || !significand.divisible_by_power_of_2(mask_width)
                || rev_limbs.any(|y| y != 0);
            if inexact {
                significand.round_to_multiple_of_power_of_2_assign(mask_width, Floor);
                Less
            } else {
                Equal
            }
        }
        Ceiling | Up => {
            let inexact = erased_limb != 0
                || !significand.divisible_by_power_of_2(mask_width)
                || rev_limbs.any(|y| y != 0);
            if inexact {
                let original_limb_count = significand.limb_count();
                significand.round_to_multiple_of_power_of_2_assign(mask_width, Floor);
                significand += Natural::power_of_2(mask_width);
                if significand.limb_count() > original_limb_count {
                    if exponent == Float::MAX_EXPONENT {
                        return (Float::INFINITY, Greater);
                    }
                    significand >>= 1;
                    exponent += 1;
                }
                Greater
            } else {
                Equal
            }
        }
        Nearest => {
            let half_bit = x.get_bit(bits - prec - 1);
            let inexact_after_half = !x.divisible_by_power_of_2(bits - prec - 1);
            let inexact = half_bit || inexact_after_half;
            if half_bit && (inexact_after_half || x.get_bit(bits - prec)) {
                let original_limb_count = significand.limb_count();
                significand.round_to_multiple_of_power_of_2_assign(mask_width, Floor);
                significand += Natural::power_of_2(mask_width);
                if significand.limb_count() > original_limb_count {
                    if exponent == Float::MAX_EXPONENT {
                        return (Float::INFINITY, Greater);
                    }
                    significand >>= 1;
                    exponent += 1;
                }
                Greater
            } else if inexact {
                significand.round_to_multiple_of_power_of_2_assign(mask_width, Floor);
                Less
            } else {
                Equal
            }
        }
    };
    (
        Float(Finite {
            sign: true,
            exponent,
            precision: prec,
            significand,
        }),
        o,
    )
}

fn from_natural_prec_round_helper_zero_exponent(
    x: &Natural,
    prec: u64,
    rm: RoundingMode,
    bits: u64,
) -> (Float, Ordering) {
    let mut needed_bits = prec;
    let sig_bits_in_highest_limb = bits.mod_power_of_2(Limb::LOG_WIDTH);
    let mut needed_limbs = 1;
    needed_bits.saturating_sub_assign(sig_bits_in_highest_limb);
    if needed_bits != 0 {
        needed_limbs += needed_bits.shr_round(Limb::LOG_WIDTH, Ceiling).0;
    }
    let mut rev_limbs = x.limbs().rev();
    let mut significand = Natural::from_owned_limbs_desc(
        (&mut rev_limbs)
            .take(usize::exact_from(needed_limbs))
            .collect(),
    );
    significand <<= significand
        .significant_bits()
        .neg_mod_power_of_2(Limb::LOG_WIDTH);
    let mut mask_width = significand.significant_bits() - prec;
    let mut erased_limb = 0;
    if mask_width >= Limb::WIDTH {
        erased_limb = significand.limbs()[0];
        significand >>= Limb::WIDTH;
        mask_width -= Limb::WIDTH;
    }
    let mut exponent = 0;
    let o = match rm {
        Exact => {
            let inexact = erased_limb != 0
                || !significand.divisible_by_power_of_2(mask_width)
                || rev_limbs.any(|y| y != 0);
            assert!(!inexact, "Inexact conversion from Natural");
            Equal
        }
        Floor | Down => {
            let inexact = erased_limb != 0
                || !significand.divisible_by_power_of_2(mask_width)
                || rev_limbs.any(|y| y != 0);
            if inexact {
                significand.round_to_multiple_of_power_of_2_assign(mask_width, Floor);
                Less
            } else {
                Equal
            }
        }
        Ceiling | Up => {
            let inexact = erased_limb != 0
                || !significand.divisible_by_power_of_2(mask_width)
                || rev_limbs.any(|y| y != 0);
            if inexact {
                let original_limb_count = significand.limb_count();
                significand.round_to_multiple_of_power_of_2_assign(mask_width, Floor);
                significand += Natural::power_of_2(mask_width);
                if significand.limb_count() > original_limb_count {
                    significand >>= 1;
                    exponent += 1;
                }
                Greater
            } else {
                Equal
            }
        }
        Nearest => {
            let half_bit = x.get_bit(bits - prec - 1);
            let inexact_after_half = !x.divisible_by_power_of_2(bits - prec - 1);
            let inexact = half_bit || inexact_after_half;
            if half_bit && (inexact_after_half || x.get_bit(bits - prec)) {
                let original_limb_count = significand.limb_count();
                significand.round_to_multiple_of_power_of_2_assign(mask_width, Floor);
                significand += Natural::power_of_2(mask_width);
                if significand.limb_count() > original_limb_count {
                    significand >>= 1;
                    exponent += 1;
                }
                Greater
            } else if inexact {
                significand.round_to_multiple_of_power_of_2_assign(mask_width, Floor);
                Less
            } else {
                Equal
            }
        }
    };
    (
        Float(Finite {
            sign: true,
            exponent,
            precision: prec,
            significand,
        }),
        o,
    )
}

fn from_natural_prec_round_helper_no_round_zero_exponent(
    x: &Natural,
    prec: u64,
    bits: u64,
) -> Float {
    let mut needed_bits = prec;
    let sig_bits_in_highest_limb = bits.mod_power_of_2(Limb::LOG_WIDTH);
    let mut needed_limbs = 1;
    needed_bits.saturating_sub_assign(sig_bits_in_highest_limb);
    if needed_bits != 0 {
        needed_limbs += needed_bits.shr_round(Limb::LOG_WIDTH, Ceiling).0;
    }
    let mut rev_limbs = x.limbs().rev();
    let mut significand = Natural::from_owned_limbs_desc(
        (&mut rev_limbs)
            .take(usize::exact_from(needed_limbs))
            .collect(),
    );
    significand <<= significand
        .significant_bits()
        .neg_mod_power_of_2(Limb::LOG_WIDTH);
    if significand.significant_bits() - prec >= Limb::WIDTH {
        significand >>= Limb::WIDTH;
    }
    Float(Finite {
        sign: true,
        exponent: 0,
        precision: prec,
        significand,
    })
}

pub(crate) fn from_natural_prec_round_zero_exponent(
    x: Natural,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if x == 0u32 {
        return (Float::ZERO, Equal);
    }
    let bits = x.significant_bits();
    let mut f = Float(Finite {
        sign: true,
        exponent: 0,
        precision: bits,
        significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
    });
    let o = f.set_prec_round(prec, rm);
    (f, o)
}

pub(crate) fn from_natural_prec_round_zero_exponent_ref(
    x: &Natural,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    if *x == 0u32 {
        return (Float::ZERO, Equal);
    }
    let bits = x.significant_bits();
    if bits <= prec {
        let mut f = Float(Finite {
            sign: true,
            exponent: 0,
            precision: bits,
            significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
        });
        let o = f.set_prec_round(prec, rm);
        (f, o)
    } else {
        from_natural_prec_round_helper_zero_exponent(x, prec, rm, bits)
    }
}

pub(crate) fn from_natural_zero_exponent(x: Natural) -> Float {
    if x == 0 {
        Float::ZERO
    } else {
        let bits = x.significant_bits();
        let prec = bits - x.trailing_zeros().unwrap();
        from_natural_prec_round_zero_exponent(x, prec, Floor).0
    }
}

pub(crate) fn from_natural_zero_exponent_ref(x: &Natural) -> Float {
    if *x == 0 {
        Float::ZERO
    } else {
        let bits = x.significant_bits();
        let prec = bits - x.trailing_zeros().unwrap();
        from_natural_prec_round_helper_no_round_zero_exponent(x, prec, bits)
    }
}

impl Float {
    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. If rounding is needed, the specified rounding mode
    /// is used. An [`Ordering`] is also returned, indicating whether the returned value is less
    /// than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_natural_prec`] instead.
    ///
    /// - If the [`Natural`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds down
    ///   to $(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the `Natural` cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::ZERO, 10, Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::from(123u32), 20, Exact);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::from(123u32), 4, Floor);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_natural_prec_round(Natural::from(123u32), 4, Ceiling);
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_natural_prec_round(x: Natural, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if x == 0u32 {
            return (Float::ZERO, Equal);
        }
        let bits = x.significant_bits();
        if let Ok(bits_i32) = i32::try_from(bits) {
            if bits_i32 <= Float::MAX_EXPONENT {
                let mut f = Float(Finite {
                    sign: true,
                    exponent: bits_i32,
                    precision: bits,
                    significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
                });
                let o = f.set_prec_round(prec, rm);
                return (f, o);
            }
        }
        match rm {
            Up | Ceiling | Nearest => (Float::INFINITY, Greater),
            Floor | Down => (Float::max_finite_value_with_prec(prec), Less),
            Exact => panic!("Inexact conversion from Natural to Float"),
        }
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference. If the [`Float`]
    /// is nonzero, it has the specified precision. If rounding is needed, the specified rounding
    /// mode is used. An [`Ordering`] is also returned, indicating whether the returned value is
    /// less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_natural_prec_ref`] instead.
    ///
    /// - If the [`Natural`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds down
    ///   to $(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the `Natural` cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::ZERO, 10, Exact);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::from(123u32), 20, Exact);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::from(123u32), 4, Floor);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_natural_prec_round_ref(&Natural::from(123u32), 4, Ceiling);
    /// assert_eq!(x.to_string(), "1.3e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_natural_prec_round_ref(
        x: &Natural,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            return (Float::ZERO, Equal);
        }
        let bits = x.significant_bits();
        if bits <= prec {
            if let Ok(bits_i32) = i32::try_from(bits) {
                if bits_i32 <= Float::MAX_EXPONENT {
                    let mut f = Float(Finite {
                        sign: true,
                        exponent: bits_i32,
                        precision: bits,
                        significand: x << bits.neg_mod_power_of_2(Limb::LOG_WIDTH),
                    });
                    let o = f.set_prec_round(prec, rm);
                    return (f, o);
                }
            }
            match rm {
                Up | Ceiling | Nearest => (Float::INFINITY, Greater),
                Floor | Down => (Float::max_finite_value_with_prec(prec), Less),
                Exact => panic!("Inexact conversion from Natural to Float"),
            }
        } else {
            from_natural_prec_round_helper(x, prec, rm, bits)
        }
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value. If the [`Float`] is
    /// nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Natural`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_natural_prec_round`].
    ///
    /// - If the [`Natural`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::from(123u32), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec(Natural::from(123u32), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_natural_prec(x: Natural, prec: u64) -> (Float, Ordering) {
        Float::from_natural_prec_round(x, prec, Nearest)
    }

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference. If the [`Float`]
    /// is nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the [`Natural`]'s number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_natural_prec_round_ref`].
    ///
    /// - If the [`Natural`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$.
    ///
    /// # Worst-case complexity
    /// $T(m,n) = O(\max(m,n))$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `n.significant_bits()`, and $n$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::from(123u32), 20);
    /// assert_eq!(x.to_string(), "123.0");
    /// assert_eq!(x.get_prec(), Some(20));
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_natural_prec_ref(&Natural::from(123u32), 4);
    /// assert_eq!(x.to_string(), "1.2e2");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_natural_prec_ref(x: &Natural, prec: u64) -> (Float, Ordering) {
        Float::from_natural_prec_round_ref(x, prec, Nearest)
    }
}

impl TryFrom<Natural> for Float {
    type Error = FloatConversionError;

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by value.
    ///
    /// If the [`Natural`] is nonzero, the precision of the [`Float`] is the minimum possible
    /// precision to represent the [`Natural`] exactly. If you want to specify some other precision,
    /// try [`Float::from_natural_prec`]. This may require rounding, which uses [`Nearest`] by
    /// default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_natural_prec_round`].
    ///
    /// If the [`Natural`] is greater than or equal to $2^{2^{30}-1}$, this function returns an
    /// overflow error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Float::try_from(Natural::ZERO).unwrap().to_string(), "0.0");
    /// assert_eq!(
    ///     Float::try_from(Natural::from(123u32)).unwrap().to_string(),
    ///     "123.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(Natural::from(123u32)).unwrap().get_prec(),
    ///     Some(7)
    /// );
    /// assert_eq!(
    ///     Float::try_from(Natural::from(10u32)).unwrap().to_string(),
    ///     "10.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(Natural::from(10u32)).unwrap().get_prec(),
    ///     Some(3)
    /// );
    /// ```
    fn try_from(x: Natural) -> Result<Float, Self::Error> {
        if x == 0 {
            Ok(Float::ZERO)
        } else {
            let bits = x.significant_bits();
            let prec = bits - x.trailing_zeros().unwrap();
            let (f, o) = Float::from_natural_prec_round(x, prec, Floor);
            if o == Equal {
                Ok(f)
            } else {
                Err(FloatConversionError::Overflow)
            }
        }
    }
}

impl TryFrom<&Natural> for Float {
    type Error = FloatConversionError;

    /// Converts a [`Natural`] to a [`Float`], taking the [`Natural`] by reference.
    ///
    /// If the [`Natural`] is nonzero, the precision of the [`Float`] is the minimum possible
    /// precision to represent the [`Natural`] exactly. If you want to specify some other precision,
    /// try [`Float::from_natural_prec`]. This may require rounding, which uses [`Nearest`] by
    /// default. To specify a rounding mode as well as a precision, try
    /// [`Float::from_natural_prec_round`].
    ///
    /// If the [`Natural`] is greater than or equal to $2^{2^{30}-1}$, this function returns an
    /// overflow error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `n.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Float::try_from(&Natural::ZERO).unwrap().to_string(), "0.0");
    /// assert_eq!(
    ///     Float::try_from(&Natural::from(123u32)).unwrap().to_string(),
    ///     "123.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Natural::from(123u32)).unwrap().get_prec(),
    ///     Some(7)
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Natural::from(10u32)).unwrap().to_string(),
    ///     "10.0"
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Natural::from(10u32)).unwrap().get_prec(),
    ///     Some(3)
    /// );
    /// ```
    #[inline]
    fn try_from(x: &Natural) -> Result<Float, Self::Error> {
        if *x == 0 {
            Ok(Float::ZERO)
        } else {
            let bits = x.significant_bits();
            let prec = bits - x.trailing_zeros().unwrap();
            let (f, o) = Float::from_natural_prec_round_ref(x, prec, Floor);
            if o == Equal {
                Ok(f)
            } else {
                Err(FloatConversionError::Overflow)
            }
        }
    }
}

impl ConvertibleFrom<&Natural> for Float {
    /// Determines whether a [`Natural`] can be converted to an [`Float`], taking the [`Natural`] by
    /// reference.
    ///
    /// The [`Natural`]s that are convertible to [`Float`]s are those whose that would not overflow:
    /// that is, those that are less than $2^{2^{30}-1}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Float::convertible_from(&Natural::ZERO), true);
    /// assert_eq!(Float::convertible_from(&Natural::from(3u8)), true);
    /// ```
    #[inline]
    fn convertible_from(x: &Natural) -> bool {
        *x == 0
            || (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
                .contains(&i32::saturating_from(x.floor_log_base_2()).saturating_add(1))
    }
}
