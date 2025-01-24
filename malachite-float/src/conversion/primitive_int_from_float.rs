// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::is_power_of_2::float_is_signed_min;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{significand_bits, Float};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, ShrRound};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::from::{SignedFromFloatError, UnsignedFromFloatError};
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

#[allow(clippy::type_repetition_in_bounds)]
fn unsigned_rounding_from_float<T: PrimitiveUnsigned>(f: Float, rm: RoundingMode) -> (T, Ordering)
where
    for<'a> T: TryFrom<&'a Natural>,
{
    match f {
        float_nan!() => panic!("Can't convert NaN to {}", T::NAME),
        float_infinity!() => match rm {
            Floor | Down | Nearest => (T::MAX, Less),
            _ => panic!("Can't convert Infinity to {} using {}", T::NAME, rm),
        },
        float_negative_infinity!() => match rm {
            Ceiling | Down | Nearest => (T::ZERO, Greater),
            _ => panic!("Can't convert -Infinity to {} using {}", T::NAME, rm),
        },
        float_either_zero!() => (T::ZERO, Equal),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(exponent);
            if !sign {
                match rm {
                    Ceiling | Down | Nearest => (T::ZERO, Greater),
                    _ => panic!("Cannot convert negative Float to {} using {}", T::NAME, rm),
                }
            } else if exponent < 0 {
                match rm {
                    Floor | Down | Nearest => (T::ZERO, Less),
                    Ceiling | Up => (T::ONE, Greater),
                    Exact => {
                        panic!("Cannot convert Float to {} using {}", T::NAME, rm)
                    }
                }
            } else if exponent > i64::wrapping_from(T::WIDTH) {
                match rm {
                    Floor | Down | Nearest => (T::MAX, Less),
                    _ => panic!("Cannot convert large Float to {} using {}", T::NAME, rm),
                }
            } else {
                let sb = significand_bits(&significand);
                let eb = exponent.unsigned_abs();
                let (n, o) = if sb >= eb {
                    significand.shr_round(sb - eb, rm)
                } else {
                    (significand << (eb - sb), Equal)
                };
                let (n, o) = if let Ok(n) = T::try_from(&n) {
                    (n, o)
                } else {
                    match rm {
                        Floor | Down | Nearest => (T::MAX, Less),
                        _ => panic!("Cannot convert large Float to {} using {}", T::NAME, rm),
                    }
                };
                (n, o)
            }
        }
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn unsigned_rounding_from_float_ref<T: PrimitiveUnsigned>(
    f: &Float,
    rm: RoundingMode,
) -> (T, Ordering)
where
    for<'a> T: TryFrom<&'a Natural>,
{
    match f {
        float_nan!() => panic!("Can't convert NaN to {}", T::NAME),
        float_infinity!() => match rm {
            Floor | Down | Nearest => (T::MAX, Less),
            _ => panic!("Can't convert Infinity to {} using {}", T::NAME, rm),
        },
        float_negative_infinity!() => match rm {
            Ceiling | Down | Nearest => (T::ZERO, Greater),
            _ => panic!("Can't convert -Infinity to {} using {}", T::NAME, rm),
        },
        float_either_zero!() => (T::ZERO, Equal),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(*exponent);
            if !sign {
                match rm {
                    Ceiling | Down | Nearest => (T::ZERO, Greater),
                    _ => panic!("Cannot convert negative Float to {} using {}", T::NAME, rm),
                }
            } else if exponent < 0 {
                match rm {
                    Floor | Down | Nearest => (T::ZERO, Less),
                    Ceiling | Up => (T::ONE, Greater),
                    Exact => {
                        panic!("Cannot convert Float to {} using {}", T::NAME, rm)
                    }
                }
            } else if exponent > i64::wrapping_from(T::WIDTH) {
                match rm {
                    Floor | Down | Nearest => (T::MAX, Less),
                    _ => panic!("Cannot convert large Float to {} using {}", T::NAME, rm),
                }
            } else {
                let sb = significand_bits(significand);
                let eb = exponent.unsigned_abs();
                let (n, o) = if sb >= eb {
                    significand.shr_round(sb - eb, rm)
                } else {
                    (significand << (eb - sb), Equal)
                };
                let (n, o) = if let Ok(n) = T::try_from(&n) {
                    (n, o)
                } else {
                    match rm {
                        Floor | Down | Nearest => (T::MAX, Less),
                        _ => panic!("Cannot convert large Float to {} using {}", T::NAME, rm),
                    }
                };
                (n, o)
            }
        }
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn unsigned_try_from_float<T: PrimitiveUnsigned>(f: Float) -> Result<T, UnsignedFromFloatError>
where
    for<'a> T: WrappingFrom<&'a Natural>,
{
    match f {
        float_either_zero!() => Ok(T::ZERO),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(exponent);
            if !sign {
                Err(UnsignedFromFloatError::FloatNegative)
            } else if exponent <= 0 || exponent > i64::wrapping_from(T::WIDTH) {
                Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
            } else {
                let sb = significand_bits(&significand);
                let eb = exponent.unsigned_abs();
                let n = if sb >= eb {
                    let bits = sb - eb;
                    if significand.divisible_by_power_of_2(bits) {
                        Ok(significand >> bits)
                    } else {
                        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
                    }
                } else {
                    Ok(significand << (eb - sb))
                };
                n.map(|n| T::wrapping_from(&n))
            }
        }
        _ => Err(UnsignedFromFloatError::FloatInfiniteOrNan),
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn unsigned_try_from_float_ref<T: PrimitiveUnsigned>(f: &Float) -> Result<T, UnsignedFromFloatError>
where
    for<'a> T: WrappingFrom<&'a Natural>,
{
    match f {
        float_either_zero!() => Ok(T::ZERO),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(*exponent);
            if !sign {
                Err(UnsignedFromFloatError::FloatNegative)
            } else if exponent <= 0 || exponent > i64::wrapping_from(T::WIDTH) {
                Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
            } else {
                let sb = significand_bits(significand);
                let eb = exponent.unsigned_abs();
                let n = if sb >= eb {
                    let bits = sb - eb;
                    if significand.divisible_by_power_of_2(bits) {
                        Ok(significand >> bits)
                    } else {
                        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
                    }
                } else {
                    Ok(significand << (eb - sb))
                };
                n.map(|n| T::wrapping_from(&n))
            }
        }
        _ => Err(UnsignedFromFloatError::FloatInfiniteOrNan),
    }
}

fn unsigned_convertible_from_float<T: PrimitiveUnsigned>(f: &Float) -> bool {
    match f {
        float_either_zero!() => true,
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(*exponent);
            *sign && exponent > 0 && exponent <= i64::wrapping_from(T::WIDTH) && {
                let sb = significand_bits(significand);
                let eb = exponent.unsigned_abs();
                sb < eb || significand.divisible_by_power_of_2(sb - eb)
            }
        }
        _ => false,
    }
}

macro_rules! impl_unsigned_from {
    ($t: ident) => {
        impl RoundingFrom<Float> for $t {
            /// Converts a [`Float`] to an unsigned primitive integer, using a specified
            /// [`RoundingMode`] and taking the [`Float`] by value. An [`Ordering`] is also
            /// returned, indicating whether the returned value is less than, equal to, or greater
            /// than the original value.
            ///
            /// If the [`Float`] is negative (including $-\infty$), then it will be rounded to zero
            /// when the [`RoundingMode`] is `Ceiling`, `Down`, or `Nearest`. Otherwise, this
            /// function will panic.
            ///
            /// If the [`Float`] is greater than the maximum representable value of the unsigned
            /// type (including $\infty$), then it will be rounded to the maximum value when the
            /// [`RoundingMode`] is `Floor`, `Down`, or `Nearest`. Otherwise, this function will
            /// panic.
            ///
            /// If the [`Float`] is NaN, the function will panic regardless of the rounding mode.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the [`Float`] is not an integer and `rm` is `Exact`, or if the [`Float`]
            /// is less than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`, if the [`Float`]
            /// is greater than the maximum representable value of the unsigned type and `rm` is not
            /// `Down`, `Floor`, or `Nearest`, or if the [`Float`] is NaN.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#rounding_from).
            #[inline]
            fn rounding_from(f: Float, rm: RoundingMode) -> ($t, Ordering) {
                unsigned_rounding_from_float(f, rm)
            }
        }

        impl RoundingFrom<&Float> for $t {
            /// Converts a [`Float`] to an unsigned primitive integer, using a specified
            /// [`RoundingMode`] and taking the [`Float`] by reference. An [`Ordering`] is also
            /// returned, indicating whether the returned value is less than, equal to, or greater
            /// than the original value.
            ///
            /// If the [`Float`] is negative (including $-\infty$), then it will be rounded to zero
            /// when the [`RoundingMode`] is `Ceiling`, `Down`, or `Nearest`. Otherwise, this
            /// function will panic.
            ///
            /// If the [`Float`] is greater than the maximum representable value of the unsigned
            /// type (including $\infty$), then it will be rounded to the maximum value when the
            /// [`RoundingMode`] is `Floor`, `Down`, or `Nearest`. Otherwise, this function will
            /// panic.
            ///
            /// If the [`Float`] is NaN, the function will panic regardless of the rounding mode.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the [`Float`] is not an integer and `rm` is `Exact`, or if the [`Float`]
            /// is less than zero and `rm` is not `Down`, `Ceiling`, or `Nearest`, if the [`Float`]
            /// is greater than the maximum representable value of the unsigned type and `rm` is not
            /// `Down`, `Floor`, or `Nearest`, or if the [`Float`] is NaN.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#rounding_from).
            #[inline]
            fn rounding_from(f: &Float, rm: RoundingMode) -> ($t, Ordering) {
                unsigned_rounding_from_float_ref(f, rm)
            }
        }

        impl TryFrom<Float> for $t {
            type Error = UnsignedFromFloatError;

            /// Converts a [`Float`] to a primitive unsigned integer, taking the [`Float`] by value.
            /// If the [`Float`] is not equal to an unsigned primitive integer of the given type, an
            /// error is returned.
            ///
            /// Both positive and negative zero convert to a primitive unsigned integer zero.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#try_from).
            #[inline]
            fn try_from(f: Float) -> Result<$t, Self::Error> {
                unsigned_try_from_float(f)
            }
        }

        impl TryFrom<&Float> for $t {
            type Error = UnsignedFromFloatError;

            /// Converts a [`Float`] to a primitive unsigned integer, taking the [`Float`] by
            /// reference. If the [`Float`] is not equal to an unsigned primitive integer of the
            /// given type, an error is returned.
            ///
            /// Both positive and negative zero convert to a primitive unsigned integer zero.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#try_from).
            #[inline]
            fn try_from(f: &Float) -> Result<$t, Self::Error> {
                unsigned_try_from_float_ref(f)
            }
        }

        impl ConvertibleFrom<&Float> for $t {
            /// Determines whether a [`Float`] can be converted to an unsigned primitive integer,
            /// taking the [`Float`] by reference.
            ///
            /// Both positive and negative zero are convertible to any unsigned primitive integer.
            /// (Although negative zero is nominally negative, the real number it represents is
            /// zero, which is not negative.)
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#convertible_from).
            #[inline]
            fn convertible_from(f: &Float) -> bool {
                unsigned_convertible_from_float::<$t>(f)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned_from);

#[allow(clippy::trait_duplication_in_bounds, clippy::type_repetition_in_bounds)]
fn signed_rounding_from_float<T: PrimitiveSigned>(f: Float, rm: RoundingMode) -> (T, Ordering)
where
    for<'a> T: TryFrom<&'a Natural> + TryFrom<&'a Integer>,
{
    match f {
        float_nan!() => panic!("Can't convert NaN to {}", T::NAME),
        float_infinity!() => match rm {
            Floor | Down | Nearest => (T::MAX, Less),
            _ => panic!("Can't convert Infinity to {} using {}", T::NAME, rm),
        },
        float_negative_infinity!() => match rm {
            Ceiling | Down | Nearest => (T::MIN, Greater),
            _ => panic!("Can't convert -Infinity to {} using {}", T::NAME, rm),
        },
        float_either_zero!() => (T::ZERO, Equal),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(exponent);
            if sign {
                if exponent < 0 {
                    match rm {
                        Floor | Down | Nearest => (T::ZERO, Less),
                        Ceiling | Up => (T::ONE, Greater),
                        Exact => {
                            panic!("Cannot convert Float to Integer using {rm}")
                        }
                    }
                } else if exponent >= i64::wrapping_from(T::WIDTH) {
                    match rm {
                        Floor | Down | Nearest => (T::MAX, Less),
                        _ => {
                            panic!("Cannot convert Float to Integer using {rm}")
                        }
                    }
                } else {
                    let sb = significand_bits(&significand);
                    let eb = exponent.unsigned_abs();
                    let (n, o) = if sb >= eb {
                        significand.shr_round(sb - eb, rm)
                    } else {
                        (significand << (eb - sb), Equal)
                    };
                    let (n, o) = if let Ok(n) = T::try_from(&n) {
                        (n, o)
                    } else {
                        match rm {
                            Floor | Down | Nearest => (T::MAX, Less),
                            _ => {
                                panic!("Cannot convert large Float to {} using {}", T::NAME, rm)
                            }
                        }
                    };
                    (n, o)
                }
            } else if exponent < 0 {
                match rm {
                    Ceiling | Down | Nearest => (T::ZERO, Greater),
                    Floor | Up => (T::NEGATIVE_ONE, Less),
                    Exact => {
                        panic!("Cannot convert Float to Integer using {rm}")
                    }
                }
            } else if exponent > i64::wrapping_from(T::WIDTH) {
                // This doesn't catch the case where -2^(W+1) < x < -2^W, but that's ok because the
                // next else block handles it.
                match rm {
                    Ceiling | Down | Nearest => (T::MIN, Greater),
                    _ => {
                        panic!("Cannot convert Float to Integer using {rm}")
                    }
                }
            } else {
                let sb = significand_bits(&significand);
                let eb = exponent.unsigned_abs();
                let (n, o) = if sb >= eb {
                    significand.shr_round(sb - eb, -rm)
                } else {
                    (significand << (eb - sb), Equal)
                };
                let (n, o) = if let Ok(n) = T::try_from(&-n) {
                    (n, o.reverse())
                } else {
                    match rm {
                        Ceiling | Down | Nearest => (T::MIN, Greater),
                        _ => panic!(
                            "Cannot convert large negative Float to {} using {}",
                            T::NAME,
                            rm
                        ),
                    }
                };
                (n, o)
            }
        }
    }
}

#[allow(clippy::trait_duplication_in_bounds, clippy::type_repetition_in_bounds)]
fn signed_rounding_from_float_ref<T: PrimitiveSigned>(f: &Float, rm: RoundingMode) -> (T, Ordering)
where
    for<'a> T: TryFrom<&'a Natural> + TryFrom<&'a Integer>,
{
    match f {
        float_nan!() => panic!("Can't convert NaN to {}", T::NAME),
        float_infinity!() => match rm {
            Floor | Down | Nearest => (T::MAX, Less),
            _ => panic!("Can't convert Infinity to {} using {}", T::NAME, rm),
        },
        float_negative_infinity!() => match rm {
            Ceiling | Down | Nearest => (T::MIN, Greater),
            _ => panic!("Can't convert -Infinity to {} using {}", T::NAME, rm),
        },
        float_either_zero!() => (T::ZERO, Equal),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(*exponent);
            if *sign {
                if exponent < 0 {
                    match rm {
                        Floor | Down | Nearest => (T::ZERO, Less),
                        Ceiling | Up => (T::ONE, Greater),
                        Exact => {
                            panic!("Cannot convert Float to Integer using {rm}")
                        }
                    }
                } else if exponent >= i64::wrapping_from(T::WIDTH) {
                    match rm {
                        Floor | Down | Nearest => (T::MAX, Less),
                        _ => {
                            panic!("Cannot convert Float to Integer using {rm}")
                        }
                    }
                } else {
                    let sb = significand_bits(significand);
                    let eb = exponent.unsigned_abs();
                    let (n, o) = if sb >= eb {
                        significand.shr_round(sb - eb, rm)
                    } else {
                        (significand << (eb - sb), Equal)
                    };
                    let (n, o) = if let Ok(n) = T::try_from(&n) {
                        (n, o)
                    } else {
                        match rm {
                            Floor | Down | Nearest => (T::MAX, Less),
                            _ => {
                                panic!("Cannot convert large Float to {} using {}", T::NAME, rm)
                            }
                        }
                    };
                    (n, o)
                }
            } else if exponent < 0 {
                match rm {
                    Ceiling | Down | Nearest => (T::ZERO, Greater),
                    Floor | Up => (T::NEGATIVE_ONE, Less),
                    Exact => {
                        panic!("Cannot convert Float to Integer using {rm}")
                    }
                }
            } else if exponent > i64::wrapping_from(T::WIDTH) {
                // This doesn't catch the case where -2^(W+1) < x < -2^W, but that's ok because the
                // next else block handles it.
                match rm {
                    Ceiling | Down | Nearest => (T::MIN, Greater),
                    _ => {
                        panic!("Cannot convert Float to Integer using {rm}")
                    }
                }
            } else {
                let sb = significand_bits(significand);
                let eb = exponent.unsigned_abs();
                let (n, o) = if sb >= eb {
                    significand.shr_round(sb - eb, -rm)
                } else {
                    (significand << (eb - sb), Equal)
                };
                let (n, o) = if let Ok(n) = T::try_from(&-n) {
                    (n, o.reverse())
                } else {
                    match rm {
                        Ceiling | Down | Nearest => (T::MIN, Greater),
                        _ => panic!(
                            "Cannot convert large negative Float to {} using {}",
                            T::NAME,
                            rm
                        ),
                    }
                };
                (n, o)
            }
        }
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn signed_try_from_float<T: PrimitiveSigned>(f: Float) -> Result<T, SignedFromFloatError>
where
    for<'a> T: TryFrom<&'a Integer>,
{
    match f {
        float_either_zero!() => Ok(T::ZERO),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(exponent);
            if exponent <= 0
                || (sign && exponent >= i64::wrapping_from(T::WIDTH)
                    || !sign && exponent > i64::wrapping_from(T::WIDTH))
            {
                Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
            } else {
                let sb = significand_bits(&significand);
                let eb = exponent.unsigned_abs();
                let i = Integer::from_sign_and_abs(
                    sign,
                    if sb >= eb {
                        let bits = sb - eb;
                        if significand.divisible_by_power_of_2(bits) {
                            significand >> bits
                        } else {
                            return Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange);
                        }
                    } else {
                        significand << (eb - sb)
                    },
                );
                T::try_from(&i).map_err(|_| SignedFromFloatError::FloatNonIntegerOrOutOfRange)
            }
        }
        _ => Err(SignedFromFloatError::FloatInfiniteOrNan),
    }
}

#[allow(clippy::type_repetition_in_bounds)]
fn signed_try_from_float_ref<T: PrimitiveSigned>(f: &Float) -> Result<T, SignedFromFloatError>
where
    for<'a> T: TryFrom<&'a Integer>,
{
    match f {
        float_either_zero!() => Ok(T::ZERO),
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(*exponent);
            if exponent <= 0
                || (*sign && exponent >= i64::wrapping_from(T::WIDTH)
                    || !*sign && exponent > i64::wrapping_from(T::WIDTH))
            {
                Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange)
            } else {
                let sb = significand_bits(significand);
                let eb = exponent.unsigned_abs();
                let i = Integer::from_sign_and_abs(
                    *sign,
                    if sb >= eb {
                        let bits = sb - eb;
                        if significand.divisible_by_power_of_2(bits) {
                            significand >> bits
                        } else {
                            return Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange);
                        }
                    } else {
                        significand << (eb - sb)
                    },
                );
                T::try_from(&i).map_err(|_| SignedFromFloatError::FloatNonIntegerOrOutOfRange)
            }
        }
        _ => Err(SignedFromFloatError::FloatInfiniteOrNan),
    }
}

fn signed_convertible_from_float<T: PrimitiveSigned>(f: &Float) -> bool {
    match f {
        float_either_zero!() => true,
        Float(Finite {
            exponent,
            significand,
            ..
        }) => {
            let exponent = i64::from(*exponent);
            if exponent <= 0 {
                return false;
            }
            if exponent >= i64::wrapping_from(T::WIDTH) {
                float_is_signed_min::<T>(f)
            } else {
                let sb = significand_bits(significand);
                let eb = exponent.unsigned_abs();
                sb < eb || significand.divisible_by_power_of_2(sb - eb)
            }
        }
        _ => false,
    }
}

macro_rules! impl_signed_from {
    ($t: ident) => {
        impl RoundingFrom<Float> for $t {
            /// Converts a [`Float`] to a signed primitive integer, using a specified
            /// [`RoundingMode`] and taking the [`Float`] by value. An [`Ordering`] is also
            /// returned, indicating whether the returned value is less than, equal to, or greater
            /// than the original value.
            ///
            /// If the [`Float`] is less than the minimum representable value of the signed type
            /// (including $-\infty$), then it will be rounded to zero when the [`RoundingMode`] is
            /// `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will panic.
            ///
            /// If the [`Float`] is greater than the maximum representable value of the signed type
            /// (including $\infty$), then it will be rounded to the maximum value when the
            /// [`RoundingMode`] is `Floor`, `Down`, or `Nearest`. Otherwise, this function will
            /// panic.
            ///
            /// If the [`Float`] is NaN, the function will panic regardless of the rounding mode.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the [`Float`] is not an integer and `rm` is `Exact`, or if the [`Float`]
            /// is smaller than the minimum representable value of the signed type and `rm` is not
            /// `Down`, `Ceiling`, or `Nearest`, if the [`Float`] is greater than the maximum
            /// representable value of the signed type and `rm` is not `Down`, `Floor`, or
            /// `Nearest`, or if the [`Float`] is NaN.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#rounding_from).
            #[inline]
            fn rounding_from(f: Float, rm: RoundingMode) -> ($t, Ordering) {
                signed_rounding_from_float(f, rm)
            }
        }

        impl RoundingFrom<&Float> for $t {
            /// Converts a [`Float`] to a signed primitive integer, using a specified
            /// [`RoundingMode`] and taking the [`Float`] by reference. An [`Ordering`] is also
            /// returned, indicating whether the returned value is less than, equal to, or greater
            /// than the original value.
            ///
            /// If the [`Float`] is less than the minimum representable value of the signed type
            /// (including $-\infty$), then it will be rounded to zero when the [`RoundingMode`] is
            /// `Ceiling`, `Down`, or `Nearest`. Otherwise, this function will panic.
            ///
            /// If the [`Float`] is greater than the maximum representable value of the signed type
            /// (including $\infty$), then it will be rounded to the maximum value when the
            /// [`RoundingMode`] is `Floor`, `Down`, or `Nearest`. Otherwise, this function will
            /// panic.
            ///
            /// If the [`Float`] is NaN, the function will panic regardless of the rounding mode.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the [`Float`] is not an integer and `rm` is `Exact`, or if the [`Float`]
            /// is smaller than the minimum representable value of the signed type and `rm` is not
            /// `Down`, `Ceiling`, or `Nearest`, if the [`Float`] is greater than the maximum
            /// representable value of the signed type and `rm` is not `Down`, `Floor`, or
            /// `Nearest`, or if the [`Float`] is NaN.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#rounding_from).
            #[inline]
            fn rounding_from(f: &Float, rm: RoundingMode) -> ($t, Ordering) {
                signed_rounding_from_float_ref(f, rm)
            }
        }

        impl TryFrom<Float> for $t {
            type Error = SignedFromFloatError;

            /// Converts a [`Float`] to a primitive signed integer, taking the [`Float`] by value.
            /// If the [`Float`] is not equal to a signed primitive integer of the given type, an
            /// error is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#try_from).
            #[inline]
            fn try_from(f: Float) -> Result<$t, Self::Error> {
                signed_try_from_float(f)
            }
        }

        impl TryFrom<&Float> for $t {
            type Error = SignedFromFloatError;

            /// Converts a [`Float`] to a primitive signed integer, taking the [`Float`] by
            /// reference. If the [`Float`] is not equal to a signed primitive integer of the given
            /// type, an error is returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#try_from).
            #[inline]
            fn try_from(f: &Float) -> Result<$t, Self::Error> {
                signed_try_from_float_ref(f)
            }
        }

        impl ConvertibleFrom<&Float> for $t {
            /// Determines whether a [`Float`] can be converted to a signed primitive integer,
            /// taking the [`Float`] by reference.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::primitive_int_from_float#convertible_from).
            #[inline]
            fn convertible_from(f: &Float) -> bool {
                signed_convertible_from_float::<$t>(f)
            }
        }
    };
}
apply_to_signeds!(impl_signed_from);
