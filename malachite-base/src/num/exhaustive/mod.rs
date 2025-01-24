// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::iterators::{nonzero_values, NonzeroValues};
use crate::num::arithmetic::traits::{PowerOf2, RoundToMultipleOfPowerOf2};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::float::NiceFloat;
use crate::num::iterators::{ruler_sequence, RulerSequence};
use crate::num::logic::traits::{BitAccess, NotAssign, SignificantBits};
use crate::rounding_modes::RoundingMode::*;
use crate::tuples::exhaustive::{
    exhaustive_dependent_pairs, lex_dependent_pairs, ExhaustiveDependentPairs,
    ExhaustiveDependentPairsYsGenerator, LexDependentPairs,
};
use alloc::vec::IntoIter;
use alloc::vec::Vec;
use core::iter::{once, Chain, Once, Rev};
use core::marker::PhantomData;
use itertools::{Interleave, Itertools};

/// Generates all primitive integers in an interval.
///
/// This `struct` is created by [`primitive_int_increasing_range`] and
/// [`primitive_int_increasing_inclusive_range`]; see their documentation for more.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PrimitiveIntIncreasingRange<T: PrimitiveInt> {
    a: Option<T>,
    b: Option<T>,
}

impl<T: PrimitiveInt> Iterator for PrimitiveIntIncreasingRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.a == self.b {
            None
        } else {
            let result = self.a;
            self.a = result.and_then(|x| x.checked_add(T::ONE));
            result
        }
    }
}

impl<T: PrimitiveInt> DoubleEndedIterator for PrimitiveIntIncreasingRange<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.a == self.b {
            None
        } else {
            self.b = Some(self.b.map_or(T::MAX, |b| b - T::ONE));
            self.b
        }
    }
}

/// Generates all values of a signed integer type in an interval, in order of increasing absolute
/// value.
///
/// This `enum` is created by [`exhaustive_signed_range`] and [`exhaustive_signed_inclusive_range`];
/// see their documentation for more.
#[derive(Clone, Debug)]
pub enum ExhaustiveSignedRange<T: PrimitiveSigned> {
    NonNegative(PrimitiveIntIncreasingRange<T>),
    NonPositive(Rev<PrimitiveIntIncreasingRange<T>>),
    BothSigns(ExhaustiveSigneds<T>),
}

impl<T: PrimitiveSigned> Iterator for ExhaustiveSignedRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            ExhaustiveSignedRange::NonNegative(ref mut xs) => xs.next(),
            ExhaustiveSignedRange::NonPositive(ref mut xs) => xs.next(),
            ExhaustiveSignedRange::BothSigns(ref mut xs) => xs.next(),
        }
    }
}

#[doc(hidden)]
pub type PrimitiveIntUpDown<T> =
    Interleave<PrimitiveIntIncreasingRange<T>, Rev<PrimitiveIntIncreasingRange<T>>>;

/// Generates all unsigned integers in ascending order.
///
/// The output is $(k)_{k=0}^{2^W-1}$, where $W$ is the width of the type.
///
/// The output length is $2^W$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_unsigneds::<u8>(), 10),
///     "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_unsigneds<T: PrimitiveUnsigned>() -> PrimitiveIntIncreasingRange<T> {
    primitive_int_increasing_inclusive_range(T::ZERO, T::MAX)
}

/// Generates all positive primitive integers in ascending order.
///
/// Let $L=2^W-1$ if `T` is unsigned and $L=2^{W-1}-1$ if `T` is signed, where $W$ is the width of
/// the type.
///
/// The output is $(k)_{k=1}^{L}$.
///
/// The output length is $L$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_positive_primitive_ints;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_positive_primitive_ints::<u8>(), 10),
///     "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_positive_primitive_ints<T: PrimitiveInt>() -> PrimitiveIntIncreasingRange<T> {
    primitive_int_increasing_inclusive_range(T::ONE, T::MAX)
}

pub type ExhaustiveSigneds<T> = Chain<Once<T>, PrimitiveIntUpDown<T>>;

/// Generates all signed integers in order of increasing absolute value.
///
/// When two numbers have the same absolute value, the positive one comes first.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i, j \\in [-2^{W-1}, 2^{W-1})$, where $W$ is the width of
/// the type, and $i < j$.
///
/// The output length is $2^W$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_signeds;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_signeds::<i8>(), 10),
///     "[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_signeds<T: PrimitiveSigned>() -> ExhaustiveSigneds<T> {
    once(T::ZERO).chain(exhaustive_nonzero_signeds())
}

/// Generates all natural (non-negative) signed integers in ascending order.
///
/// The output is $(k)_{k=0}^{2^{W-1}-1}$, where $W$ is the width of the type.
///
/// The output length is $2^{W-1}$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_natural_signeds;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_natural_signeds::<i8>(), 10),
///     "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_natural_signeds<T: PrimitiveSigned>() -> PrimitiveIntIncreasingRange<T> {
    primitive_int_increasing_inclusive_range(T::ZERO, T::MAX)
}

/// Generates all negative signed integers in descending order.
///
/// The output is $(-k)_{k=1}^{2^{W-1}}$, where $W$ is the width of the type.
///
/// The output length is $2^{W-1}$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_negative_signeds;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_negative_signeds::<i8>(), 10),
///     "[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_negative_signeds<T: PrimitiveSigned>() -> Rev<PrimitiveIntIncreasingRange<T>> {
    primitive_int_increasing_range(T::MIN, T::ZERO).rev()
}

/// Generates all nonzero signed integers in order of increasing absolute value.
///
/// When two numbers have the same absolute value, the positive one comes first.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i, j \\in [-2^{W-1}, 2^{W-1}) \\setminus \\{0\\}$, where
/// $W$ is the width of the type, and $i < j$.
///
/// The output length is $2^W-1$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_nonzero_signeds;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_nonzero_signeds::<i8>(), 10),
///     "[1, -1, 2, -2, 3, -3, 4, -4, 5, -5, ...]"
/// )
/// ```
#[inline]
pub fn exhaustive_nonzero_signeds<T: PrimitiveSigned>() -> PrimitiveIntUpDown<T> {
    exhaustive_positive_primitive_ints().interleave(exhaustive_negative_signeds())
}

/// Generates all primitive integers in the half-open interval $[a, b)$, in ascending order.
///
/// $a$ must be less than or equal to $b$. If $a$ and $b$ are equal, the range is empty. This
/// function cannot create a range that includes `T::MAX`; for that, use
/// [`primitive_int_increasing_inclusive_range`].
///
/// The output is $(k)_{k=a}^{b-1}$.
///
/// The output length is $b - a$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::primitive_int_increasing_range;
///
/// assert_eq!(
///     primitive_int_increasing_range::<i8>(-5, 5).collect_vec(),
///     &[-5, -4, -3, -2, -1, 0, 1, 2, 3, 4]
/// )
/// ```
#[inline]
pub fn primitive_int_increasing_range<T: PrimitiveInt>(
    a: T,
    b: T,
) -> PrimitiveIntIncreasingRange<T> {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    PrimitiveIntIncreasingRange {
        a: Some(a),
        b: Some(b),
    }
}

/// Generates all primitive integers in the closed interval $[a, b]$, in ascending order.
///
/// $a$ must be less than or equal to $b$. If $a$ and $b$ are equal, the range contains a single
/// element.
///
/// The output is $(k)_{k=a}^{b}$.
///
/// The output length is $b - a + 1$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::primitive_int_increasing_inclusive_range;
///
/// assert_eq!(
///     primitive_int_increasing_inclusive_range::<i8>(-5, 5).collect_vec(),
///     &[-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]
/// )
/// ```
#[inline]
pub fn primitive_int_increasing_inclusive_range<T: PrimitiveInt>(
    a: T,
    b: T,
) -> PrimitiveIntIncreasingRange<T> {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    PrimitiveIntIncreasingRange {
        a: Some(a),
        b: b.checked_add(T::ONE),
    }
}

/// Generates all signed integers in the half-open interval $[a, b)$, in order of increasing
/// absolute value.
///
/// When two numbers have the same absolute value, the positive one comes first. $a$ must be less
/// than or equal to $b$. If $a$ and $b$ are equal, the range is empty. This function cannot create
/// a range that includes `T::MAX`; for that, use [`exhaustive_signed_inclusive_range`].
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i, j \\in [0, b - a)$ and $i < j$.
///
/// The output length is $b - a$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::exhaustive_signed_range;
///
/// assert_eq!(
///     exhaustive_signed_range::<i8>(-5, 5).collect_vec(),
///     &[0, 1, -1, 2, -2, 3, -3, 4, -4, -5]
/// )
/// ```
pub fn exhaustive_signed_range<T: PrimitiveSigned>(a: T, b: T) -> ExhaustiveSignedRange<T> {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    if a >= T::ZERO {
        ExhaustiveSignedRange::NonNegative(primitive_int_increasing_range(a, b))
    } else if b <= T::ZERO {
        ExhaustiveSignedRange::NonPositive(primitive_int_increasing_range(a, b).rev())
    } else {
        ExhaustiveSignedRange::BothSigns(
            once(T::ZERO).chain(
                primitive_int_increasing_range(T::ONE, b)
                    .interleave(primitive_int_increasing_range(a, T::ZERO).rev()),
            ),
        )
    }
}

/// Generates all signed integers in the closed interval $[a, b]$, in order of increasing absolute
/// value.
///
/// When two numbers have the same absolute value, the positive one comes first. $a$ must be less
/// than or equal to $b$. If $a$ and $b$ are equal, the range contains a single element.
///
/// The output satisfies $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|,
/// \operatorname{sgn}(-x_j))$ whenever $i, j \\in [0, b - a]$ and $i < j$.
///
/// The output length is $b - a + 1$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if $a > b$.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::exhaustive_signed_inclusive_range;
///
/// assert_eq!(
///     exhaustive_signed_inclusive_range::<i8>(-5, 5).collect_vec(),
///     &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5]
/// )
/// ```
pub fn exhaustive_signed_inclusive_range<T: PrimitiveSigned>(
    a: T,
    b: T,
) -> ExhaustiveSignedRange<T> {
    assert!(a <= b, "a must be less than or equal to b. a: {a}, b: {b}");
    if a >= T::ZERO {
        ExhaustiveSignedRange::NonNegative(primitive_int_increasing_inclusive_range(a, b))
    } else if b <= T::ZERO {
        ExhaustiveSignedRange::NonPositive(primitive_int_increasing_inclusive_range(a, b).rev())
    } else {
        ExhaustiveSignedRange::BothSigns(
            once(T::ZERO).chain(
                primitive_int_increasing_inclusive_range(T::ONE, b)
                    .interleave(primitive_int_increasing_inclusive_range(a, T::NEGATIVE_ONE).rev()),
            ),
        )
    }
}

/// Generates all primitive floats in an interval, in increasing order.
///
/// This `struct` implements [`DoubleEndedIterator`], so you can reverse it to generate floats in
/// decreasing order.
///
/// Positive zero and negative zero are both generated. Negative zero is considered to be less than
/// positive zero.
///
/// This `struct` is created by [`primitive_float_increasing_range`] and
/// [`primitive_float_increasing_inclusive_range`]; see their documentation for more.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PrimitiveFloatIncreasingRange<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    xs: PrimitiveIntIncreasingRange<u64>,
}

impl<T: PrimitiveFloat> Iterator for PrimitiveFloatIncreasingRange<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.xs.next().map(T::from_ordered_representation)
    }
}

impl<T: PrimitiveFloat> DoubleEndedIterator for PrimitiveFloatIncreasingRange<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.xs.next_back().map(T::from_ordered_representation)
    }
}

/// Generates all primitive floats in the half-open interval $[a, b)$, in ascending order.
///
/// Positive and negative zero are treated as two distinct values, with negative zero being smaller
/// than zero.
///
/// `NiceFloat(a)` must be less than or equal to `NiceFloat(b)`. If `NiceFloat(a)` and
/// `NiceFloat(b)` are equal, the range is empty. This function cannot create a range that includes
/// `INFINITY`; for that, use [`primitive_float_increasing_inclusive_range`].
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is $(\varphi^{-1}(k))_{k=\varphi(a)}^{\varphi(b)-1}$.
///
/// The output length is $\varphi(b) - \varphi(a)$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `NiceFloat(a) > NiceFloat(b)`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::primitive_float_increasing_range;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         primitive_float_increasing_range::<f32>(1.0, 2.0).map(NiceFloat),
///         20
///     ),
///     "[1.0, 1.0000001, 1.0000002, 1.0000004, 1.0000005, 1.0000006, 1.0000007, 1.0000008, \
///     1.000001, 1.0000011, 1.0000012, 1.0000013, 1.0000014, 1.0000015, 1.0000017, 1.0000018, \
///     1.0000019, 1.000002, 1.0000021, 1.0000023, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         primitive_float_increasing_range::<f32>(1.0, 2.0)
///             .rev()
///             .map(NiceFloat),
///         20,
///     ),
///     "[1.9999999, 1.9999998, 1.9999996, 1.9999995, 1.9999994, 1.9999993, 1.9999992, 1.999999, \
///     1.9999989, 1.9999988, 1.9999987, 1.9999986, 1.9999985, 1.9999983, 1.9999982, 1.9999981, \
///     1.999998, 1.9999979, 1.9999977, 1.9999976, ...]",
/// );
/// ```
pub fn primitive_float_increasing_range<T: PrimitiveFloat>(
    a: T,
    b: T,
) -> PrimitiveFloatIncreasingRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(
        NiceFloat(a) <= NiceFloat(b),
        "a must be less than or equal to b. a: {}, b: {}",
        NiceFloat(a),
        NiceFloat(b)
    );
    PrimitiveFloatIncreasingRange {
        phantom: PhantomData,
        xs: primitive_int_increasing_range(
            a.to_ordered_representation(),
            b.to_ordered_representation(),
        ),
    }
}

/// Generates all primitive floats in the closed interval $[a, b]$, in ascending order.
///
/// Positive and negative zero are treated as two distinct values, with negative zero being smaller
/// than zero.
///
/// `NiceFloat(a)` must be less than or equal to `NiceFloat(b)`. If `NiceFloat(a)` and
/// `NiceFloat(b)` are equal, the range contains a single element.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is $(\varphi^{-1}(k))_{k=\varphi(a)}^\varphi(b)$.
///
/// The output length is $\varphi(b) - \varphi(a) + 1$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `NiceFloat(a) > NiceFloat(b)`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::primitive_float_increasing_inclusive_range;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         primitive_float_increasing_inclusive_range::<f32>(1.0, 2.0).map(NiceFloat),
///         20
///     ),
///     "[1.0, 1.0000001, 1.0000002, 1.0000004, 1.0000005, 1.0000006, 1.0000007, 1.0000008, \
///     1.000001, 1.0000011, 1.0000012, 1.0000013, 1.0000014, 1.0000015, 1.0000017, 1.0000018, \
///     1.0000019, 1.000002, 1.0000021, 1.0000023, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         primitive_float_increasing_inclusive_range::<f32>(1.0, 2.0)
///             .rev()
///             .map(NiceFloat),
///         20
///     ),
///     "[2.0, 1.9999999, 1.9999998, 1.9999996, 1.9999995, 1.9999994, 1.9999993, 1.9999992, \
///     1.999999, 1.9999989, 1.9999988, 1.9999987, 1.9999986, 1.9999985, 1.9999983, 1.9999982, \
///     1.9999981, 1.999998, 1.9999979, 1.9999977, ...]"
/// );
/// ```
pub fn primitive_float_increasing_inclusive_range<T: PrimitiveFloat>(
    a: T,
    b: T,
) -> PrimitiveFloatIncreasingRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(
        NiceFloat(a) <= NiceFloat(b),
        "a must be less than or equal to b. a: {}, b: {}",
        NiceFloat(a),
        NiceFloat(b)
    );
    PrimitiveFloatIncreasingRange {
        phantom: PhantomData,
        xs: primitive_int_increasing_inclusive_range(
            a.to_ordered_representation(),
            b.to_ordered_representation(),
        ),
    }
}

/// Generates all finite positive primitive floats, in ascending order.
///
/// Positive and negative zero are both excluded.
///
/// [`MIN_POSITIVE_SUBNORMAL`](super::basic::floats::PrimitiveFloat::MIN_POSITIVE_SUBNORMAL) is
/// generated first and [`MAX_FINITE`](super::basic::floats::PrimitiveFloat::MAX_FINITE) is
/// generated last. The returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is $(\varphi^{-1}(k))_{k=2^M(2^E-1)+2}^{2^{M+1}(2^E-1)}$.
///
/// The output length is $2^M(2^E-1)-1$.
/// - For [`f32`], this is $2^{31}-2^{23}-1$, or 2139095039.
/// - For [`f64`], this is $2^{63}-2^{52}-1$, or 9218868437227405311.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::positive_finite_primitive_floats_increasing;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         positive_finite_primitive_floats_increasing::<f32>().map(NiceFloat),
///         20
///     ),
///     "[1.0e-45, 3.0e-45, 4.0e-45, 6.0e-45, 7.0e-45, 8.0e-45, 1.0e-44, 1.1e-44, 1.3e-44, \
///     1.4e-44, 1.5e-44, 1.7e-44, 1.8e-44, 2.0e-44, 2.1e-44, 2.2e-44, 2.4e-44, 2.5e-44, 2.7e-44, \
///     2.8e-44, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         positive_finite_primitive_floats_increasing::<f32>()
///             .rev()
///             .map(NiceFloat),
///         20
///     ),
///     "[3.4028235e38, 3.4028233e38, 3.402823e38, 3.4028229e38, 3.4028227e38, 3.4028225e38, \
///     3.4028222e38, 3.402822e38, 3.4028218e38, 3.4028216e38, 3.4028214e38, 3.4028212e38, \
///     3.402821e38, 3.4028208e38, 3.4028206e38, 3.4028204e38, 3.4028202e38, 3.40282e38, \
///     3.4028198e38, 3.4028196e38, ...]"
/// );
/// ```
#[inline]
pub fn positive_finite_primitive_floats_increasing<T: PrimitiveFloat>(
) -> PrimitiveFloatIncreasingRange<T> {
    primitive_float_increasing_inclusive_range(T::MIN_POSITIVE_SUBNORMAL, T::MAX_FINITE)
}

/// Generates all finite negative primitive floats, in ascending order.
///
/// Positive and negative zero are both excluded.
///
/// [`-MAX_FINITE`](super::basic::floats::PrimitiveFloat::MAX_FINITE) is generated first and
/// [`-MIN_POSITIVE_SUBNORMAL`](super::basic::floats::PrimitiveFloat::MIN_POSITIVE_SUBNORMAL) is
/// generated last. The returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is $(\varphi^{-1}(k))_{k=1}^{2^M(2^E-1)-1}$.
///
/// The output length is $2^M(2^E-1)-1$.
/// - For [`f32`], this is $2^{31}-2^{23}-1$, or 2139095039.
/// - For [`f64`], this is $2^{63}-2^{52}-1$, or 9218868437227405311.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::negative_finite_primitive_floats_increasing;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         negative_finite_primitive_floats_increasing::<f32>().map(NiceFloat),
///         20
///     ),
///     "[-3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, \
///     -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, \
///     -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, \
///     -3.40282e38, -3.4028198e38, -3.4028196e38, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         negative_finite_primitive_floats_increasing::<f32>()
///             .rev()
///             .map(NiceFloat),
///         20
///     ),
///     "[-1.0e-45, -3.0e-45, -4.0e-45, -6.0e-45, -7.0e-45, -8.0e-45, -1.0e-44, -1.1e-44, \
///     -1.3e-44, -1.4e-44, -1.5e-44, -1.7e-44, -1.8e-44, -2.0e-44, -2.1e-44, -2.2e-44, -2.4e-44, \
///     -2.5e-44, -2.7e-44, -2.8e-44, ...]"
/// );
/// ```
#[inline]
pub fn negative_finite_primitive_floats_increasing<T: PrimitiveFloat>(
) -> PrimitiveFloatIncreasingRange<T> {
    primitive_float_increasing_inclusive_range(-T::MAX_FINITE, -T::MIN_POSITIVE_SUBNORMAL)
}

/// Generates all finite nonzero primitive floats, in ascending order.
///
/// Positive and negative zero are both excluded.
///
/// [-`MAX_FINITE`](super::basic::floats::PrimitiveFloat::MAX_FINITE) is generated first and
/// [`MAX_FINITE`](super::basic::floats::PrimitiveFloat::MAX_FINITE) is generated last. The returned
/// iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is
/// $$
/// (\varphi^{-1}(k))_ {k=1}^{2^M(2^E-1)-1} ⧺ (\varphi^{-1}(k))_ {k=2^M(2^E-1)+2}^{2^{M+1}(2^E-1)}
/// $$.
///
/// The output length is $2^{M+1}(2^E-1)-2$.
/// - For [`f32`], this is $2^{32}-2^{24}-2$, or 4278190078.
/// - For [`f64`], this is $2^{64}-2^{53}-2$, or 18437736874454810622.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::nonzero_finite_primitive_floats_increasing;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         nonzero_finite_primitive_floats_increasing::<f32>().map(NiceFloat),
///         20
///     ),
///     "[-3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, \
///     -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, \
///     -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, \
///     -3.40282e38, -3.4028198e38, -3.4028196e38, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         nonzero_finite_primitive_floats_increasing::<f32>()
///             .rev()
///             .map(NiceFloat),
///         20
///     ),
///     "[3.4028235e38, 3.4028233e38, 3.402823e38, 3.4028229e38, 3.4028227e38, 3.4028225e38, \
///     3.4028222e38, 3.402822e38, 3.4028218e38, 3.4028216e38, 3.4028214e38, 3.4028212e38, \
///     3.402821e38, 3.4028208e38, 3.4028206e38, 3.4028204e38, 3.4028202e38, 3.40282e38, \
///     3.4028198e38, 3.4028196e38, ...]"
/// );
/// ```
#[inline]
pub fn nonzero_finite_primitive_floats_increasing<T: PrimitiveFloat>(
) -> NonzeroValues<PrimitiveFloatIncreasingRange<T>> {
    nonzero_values(finite_primitive_floats_increasing())
}

/// Generates all finite primitive floats, in ascending order.
///
/// Positive and negative zero are both included. Negative zero comes first.
///
/// [`-MAX_FINITE`](super::basic::floats::PrimitiveFloat::MAX_FINITE) is generated first and
/// [`MAX_FINITE`](super::basic::floats::PrimitiveFloat::MAX_FINITE) is generated last. The
/// returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is $(\varphi^{-1}(k))_{k=1}^{2^{M+1}(2^E-1)}$.
///
/// The output length is $2^{M+1}(2^E-1)$.
/// - For [`f32`], this is $2^{32}-2^{24}$, or 4278190080.
/// - For [`f64`], this is $2^{64}-2^{53}$, or 18437736874454810624.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::finite_primitive_floats_increasing;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         finite_primitive_floats_increasing::<f32>().map(NiceFloat),
///         20
///     ),
///     "[-3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, \
///     -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, \
///     -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, \
///     -3.40282e38, -3.4028198e38, -3.4028196e38, ...]",
/// );
/// assert_eq!(
///     prefix_to_string(
///         finite_primitive_floats_increasing::<f32>()
///             .rev()
///             .map(NiceFloat),
///         20
///     ),
///     "[3.4028235e38, 3.4028233e38, 3.402823e38, 3.4028229e38, 3.4028227e38, 3.4028225e38, \
///     3.4028222e38, 3.402822e38, 3.4028218e38, 3.4028216e38, 3.4028214e38, 3.4028212e38, \
///     3.402821e38, 3.4028208e38, 3.4028206e38, 3.4028204e38, 3.4028202e38, 3.40282e38, \
///     3.4028198e38, 3.4028196e38, ...]"
/// );
/// ```
#[inline]
pub fn finite_primitive_floats_increasing<T: PrimitiveFloat>() -> PrimitiveFloatIncreasingRange<T> {
    primitive_float_increasing_inclusive_range(-T::MAX_FINITE, T::MAX_FINITE)
}

/// Generates all positive primitive floats, in ascending order.
///
/// Positive and negative zero are both excluded.
///
/// [`MIN_POSITIVE_SUBNORMAL`](super::basic::floats::PrimitiveFloat::MIN_POSITIVE_SUBNORMAL) is
/// generated first and `INFINITY` is generated last. The returned iterator is
/// double-ended, so it may be reversed.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is $(\varphi^{-1}(k))_{k=2^M(2^E-1)+2}^{2^{M+1}(2^E-1)+1}$.
///
/// The output length is $2^M(2^E-1)$.
/// - For [`f32`], this is $2^{31}-2^{23}$, or 2139095040.
/// - For [`f64`], this is $2^{63}-2^{52}$, or 9218868437227405312.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::positive_primitive_floats_increasing;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         positive_primitive_floats_increasing::<f32>().map(NiceFloat),
///         20
///     ),
///     "[1.0e-45, 3.0e-45, 4.0e-45, 6.0e-45, 7.0e-45, 8.0e-45, 1.0e-44, 1.1e-44, 1.3e-44, \
///     1.4e-44, 1.5e-44, 1.7e-44, 1.8e-44, 2.0e-44, 2.1e-44, 2.2e-44, 2.4e-44, 2.5e-44, 2.7e-44, \
///     2.8e-44, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         positive_primitive_floats_increasing::<f32>()
///             .rev()
///             .map(NiceFloat),
///         20
///     ),
///     "[Infinity, 3.4028235e38, 3.4028233e38, 3.402823e38, 3.4028229e38, 3.4028227e38, \
///     3.4028225e38, 3.4028222e38, 3.402822e38, 3.4028218e38, 3.4028216e38, 3.4028214e38, \
///     3.4028212e38, 3.402821e38, 3.4028208e38, 3.4028206e38, 3.4028204e38, 3.4028202e38, \
///     3.40282e38, 3.4028198e38, ...]"
/// );
/// ```
#[inline]
pub fn positive_primitive_floats_increasing<T: PrimitiveFloat>() -> PrimitiveFloatIncreasingRange<T>
{
    primitive_float_increasing_inclusive_range(T::MIN_POSITIVE_SUBNORMAL, T::INFINITY)
}

/// Generates all negative primitive floats, in ascending order.
///
/// Positive and negative zero are both excluded.
///
/// `NEGATIVE_INFINITY` is generated first and
/// [`-MIN_POSITIVE_SUBNORMAL`](super::basic::floats::PrimitiveFloat::MIN_POSITIVE_SUBNORMAL) is
/// generated last. The returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is $(\varphi^{-1}(k))_{k=0}^{2^M(2^E-1)-1}$.
///
/// The output length is $2^M(2^E-1)$.
/// - For [`f32`], this is $2^{31}-2^{23}$, or 2139095040.
/// - For [`f64`], this is $2^{63}-2^{52}$, or 9218868437227405312.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::negative_primitive_floats_increasing;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         negative_primitive_floats_increasing::<f32>().map(NiceFloat),
///         20
///     ),
///     "[-Infinity, -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, \
///     -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, \
///     -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, \
///     -3.40282e38, -3.4028198e38, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         negative_primitive_floats_increasing::<f32>()
///             .rev()
///             .map(NiceFloat),
///         20
///     ),
///     "[-1.0e-45, -3.0e-45, -4.0e-45, -6.0e-45, -7.0e-45, -8.0e-45, -1.0e-44, -1.1e-44, \
///     -1.3e-44, -1.4e-44, -1.5e-44, -1.7e-44, -1.8e-44, -2.0e-44, -2.1e-44, -2.2e-44, -2.4e-44, \
///     -2.5e-44, -2.7e-44, -2.8e-44, ...]"
/// );
/// ```
#[inline]
pub fn negative_primitive_floats_increasing<T: PrimitiveFloat>() -> PrimitiveFloatIncreasingRange<T>
{
    primitive_float_increasing_inclusive_range(T::NEGATIVE_INFINITY, -T::MIN_POSITIVE_SUBNORMAL)
}

/// Generates all nonzero primitive floats, in ascending order.
///
/// Positive and negative zero are both excluded.
///
/// `NEGATIVE_INFINITY` is generated first and `INFINITY` is generated last. The returned
/// iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is
/// $$
/// (\varphi^{-1}(k))_ {k=0}^{2^M(2^E-1)-1} ⧺ (\varphi^{-1}(k))_
/// {k=2^M(2^E-1)+2}^{2^{M+1}(2^E-1)+1} $$.
///
/// The output length is $2^{M+1}(2^E-1)$.
/// - For [`f32`], this is $2^{32}-2^{24}$, or 4278190080.
/// - For [`f64`], this is $2^{64}-2^{53}$, or 18437736874454810624.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::nonzero_primitive_floats_increasing;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         nonzero_primitive_floats_increasing::<f32>().map(NiceFloat),
///         20
///     ),
///     "[-Infinity, -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, \
///     -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, \
///     -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, \
///     -3.40282e38, -3.4028198e38, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         nonzero_primitive_floats_increasing::<f32>()
///             .rev()
///             .map(NiceFloat),
///         20
///     ),
///     "[Infinity, 3.4028235e38, 3.4028233e38, 3.402823e38, 3.4028229e38, 3.4028227e38, \
///     3.4028225e38, 3.4028222e38, 3.402822e38, 3.4028218e38, 3.4028216e38, 3.4028214e38, \
///     3.4028212e38, 3.402821e38, 3.4028208e38, 3.4028206e38, 3.4028204e38, 3.4028202e38, \
///     3.40282e38, 3.4028198e38, ...]"
/// );
/// ```
#[inline]
pub fn nonzero_primitive_floats_increasing<T: PrimitiveFloat>(
) -> NonzeroValues<PrimitiveFloatIncreasingRange<T>> {
    nonzero_values(primitive_floats_increasing())
}

/// Generates all primitive floats, except `NaN`, in ascending order.
///
/// Positive and negative zero are both included. Negative zero comes first.
///
/// `NEGATIVE_INFINITY` is generated first and `INFINITY` is generated last. The returned iterator
/// is double-ended, so it may be reversed.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output is $(\varphi^{-1}(k))_{k=0}^{2^{M+1}(2^E-1)+1}$.
///
/// The output length is $2^{M+1}(2^E-1)+2$.
/// - For [`f32`], this is $2^{32}-2^{24}+2$, or 4278190082.
/// - For [`f64`], this is $2^{64}-2^{53}+2$, or 18437736874454810626.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::primitive_floats_increasing;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(primitive_floats_increasing::<f32>().map(NiceFloat), 20),
///     "[-Infinity, -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, \
///     -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, \
///     -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, \
///     -3.40282e38, -3.4028198e38, ...]"
/// );
/// assert_eq!(
///     prefix_to_string(
///         primitive_floats_increasing::<f32>().rev().map(NiceFloat),
///         20
///     ),
///     "[Infinity, 3.4028235e38, 3.4028233e38, 3.402823e38, 3.4028229e38, 3.4028227e38, \
///     3.4028225e38, 3.4028222e38, 3.402822e38, 3.4028218e38, 3.4028216e38, 3.4028214e38, \
///     3.4028212e38, 3.402821e38, 3.4028208e38, 3.4028206e38, 3.4028204e38, 3.4028202e38, \
///     3.40282e38, 3.4028198e38, ...]"
/// );
/// ```
pub fn primitive_floats_increasing<T: PrimitiveFloat>() -> PrimitiveFloatIncreasingRange<T> {
    primitive_float_increasing_inclusive_range(T::NEGATIVE_INFINITY, T::INFINITY)
}

/// Generates all finite positive primitive floats with a specified `sci_exponent` and precision.
///
/// This `struct` is created by [`exhaustive_primitive_floats_with_sci_exponent_and_precision`]; see
/// its documentation for more.
#[derive(Clone, Debug, Default)]
pub struct ConstantPrecisionPrimitiveFloats<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    n: u64,
    increment: u64,
    i: u64,
    count: u64,
}

impl<T: PrimitiveFloat> Iterator for ConstantPrecisionPrimitiveFloats<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.i == self.count {
            None
        } else {
            let out = T::from_bits(self.n);
            self.i += 1;
            if self.i < self.count {
                self.n += self.increment;
            }
            Some(out)
        }
    }
}

/// Generates all finite positive primitive floats with a specified `sci_exponent` and precision.
///
/// Positive and negative zero are both excluded.
///
/// A finite positive primitive float may be uniquely expressed as $x = m_s2^e_s$, where $1 \leq m_s
/// < 2$ and $e_s$ is an integer; then $e_s$ is the sci-exponent. An integer $e_s$ occurs as the
/// sci-exponent of a float iff $2-2^{E-1}-M \leq e_s < 2^{E-1}$.
///
/// In the above equation, $m$ is a dyadic rational. Let $p$ be the smallest integer such that
/// $m2^{p-1}$ is an integer. Then $p$ is the float's precision. It is also the number of
/// significant bits.
///
/// For example, consider the float $100.0$. It may be written as $\frac{25}{16}2^6$, so
/// $m=\frac{25}{16}$ and $e=6$. We can write $m$ in binary as $1.1001_2$. Thus, the sci-exponent is
/// 6 and the precision is 5.
///
/// If $p$ is 1, the output length is 1; otherwise, it is $2^{p-2}$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if the sci-exponent is less than
/// [`MIN_EXPONENT`](super::basic::floats::PrimitiveFloat::MIN_EXPONENT) or greater than
/// [`MAX_EXPONENT`](super::basic::floats::PrimitiveFloat::MAX_EXPONENT), or if the precision is
/// zero or too large for the given sci-exponent (this can be checked using
/// [`max_precision_for_sci_exponent`](super::basic::floats::PrimitiveFloat::max_precision_for_sci_exponent)).
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::num::exhaustive::*;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     exhaustive_primitive_floats_with_sci_exponent_and_precision::<f32>(0, 3)
///         .map(NiceFloat)
///         .collect_vec(),
///     [1.25, 1.75].iter().copied().map(NiceFloat).collect_vec()
/// );
/// assert_eq!(
///     exhaustive_primitive_floats_with_sci_exponent_and_precision::<f32>(0, 5)
///         .map(NiceFloat)
///         .collect_vec(),
///     [1.0625, 1.1875, 1.3125, 1.4375, 1.5625, 1.6875, 1.8125, 1.9375]
///         .iter()
///         .copied()
///         .map(NiceFloat)
///         .collect_vec()
/// );
/// assert_eq!(
///     exhaustive_primitive_floats_with_sci_exponent_and_precision::<f32>(6, 5)
///         .map(NiceFloat)
///         .collect_vec(),
///     [68.0, 76.0, 84.0, 92.0, 100.0, 108.0, 116.0, 124.0]
///         .iter()
///         .copied()
///         .map(NiceFloat)
///         .collect_vec()
/// );
/// ```
pub fn exhaustive_primitive_floats_with_sci_exponent_and_precision<T: PrimitiveFloat>(
    sci_exponent: i64,
    precision: u64,
) -> ConstantPrecisionPrimitiveFloats<T> {
    assert!(sci_exponent >= T::MIN_EXPONENT);
    assert!(sci_exponent <= T::MAX_EXPONENT);
    assert_ne!(precision, 0);
    let max_precision = T::max_precision_for_sci_exponent(sci_exponent);
    assert!(precision <= max_precision);
    let increment = u64::power_of_2(max_precision - precision + 1);
    let first_mantissa = if precision == 1 {
        1
    } else {
        u64::power_of_2(precision - 1) | 1
    };
    let first = T::from_integer_mantissa_and_exponent(
        first_mantissa,
        sci_exponent - i64::exact_from(precision) + 1,
    )
    .unwrap()
    .to_bits();
    let count = if precision == 1 {
        1
    } else {
        u64::power_of_2(precision - 2)
    };
    ConstantPrecisionPrimitiveFloats {
        phantom: PhantomData,
        n: first,
        increment,
        i: 0,
        count,
    }
}

#[derive(Clone, Debug)]
struct PrimitiveFloatsWithExponentGenerator<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    sci_exponent: i64,
}

impl<T: PrimitiveFloat>
    ExhaustiveDependentPairsYsGenerator<u64, T, ConstantPrecisionPrimitiveFloats<T>>
    for PrimitiveFloatsWithExponentGenerator<T>
{
    #[inline]
    fn get_ys(&self, &precision: &u64) -> ConstantPrecisionPrimitiveFloats<T> {
        exhaustive_primitive_floats_with_sci_exponent_and_precision(self.sci_exponent, precision)
    }
}

#[inline]
fn exhaustive_primitive_floats_with_sci_exponent_helper<T: PrimitiveFloat>(
    sci_exponent: i64,
) -> LexDependentPairs<
    u64,
    T,
    PrimitiveFloatsWithExponentGenerator<T>,
    PrimitiveIntIncreasingRange<u64>,
    ConstantPrecisionPrimitiveFloats<T>,
> {
    lex_dependent_pairs(
        primitive_int_increasing_inclusive_range(
            1,
            T::max_precision_for_sci_exponent(sci_exponent),
        ),
        PrimitiveFloatsWithExponentGenerator {
            phantom: PhantomData,
            sci_exponent,
        },
    )
}

/// Generates all positive finite primitive floats with a specified `sci_exponent`.
///
/// This `struct` is created by [`exhaustive_primitive_floats_with_sci_exponent`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustivePrimitiveFloatsWithExponent<T: PrimitiveFloat>(
    LexDependentPairs<
        u64,
        T,
        PrimitiveFloatsWithExponentGenerator<T>,
        PrimitiveIntIncreasingRange<u64>,
        ConstantPrecisionPrimitiveFloats<T>,
    >,
);

impl<T: PrimitiveFloat> Iterator for ExhaustivePrimitiveFloatsWithExponent<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all positive finite primitive floats with a specified sci-exponent.
///
/// Positive and negative zero are both excluded.
///
/// A finite positive primitive float may be uniquely expressed as $x = m_s2^e_s$, where $1 \leq m_s
/// < 2$ and $e_s$ is an integer; then $e$ is the sci-exponent. An integer $e_s$ occurs as the
/// sci-exponent of a float iff $2-2^{E-1}-M \leq e_s < 2^{E-1}$.
///
/// If $e_s \geq 2-2^{E-1}$ (the float is normal), the output length is $2^M$.
/// - For [`f32`], this is $2^{23}$, or 8388608.
/// - For [`f64`], this is $2^{52}$, or 4503599627370496.
///
/// If $e_s < 2-2^{E-1}$ (the float is subnormal), the output length is $2^{e_s+2^{E-1}+M-2}$.
/// - For [`f32`], this is $2^{e_s+149}$.
/// - For [`f64`], this is $2^{e_s+1074}$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if the sci-exponent is less than
/// [`MIN_EXPONENT`](super::basic::floats::PrimitiveFloat::MIN_EXPONENT) or greater than
/// [`MAX_EXPONENT`](super::basic::floats::PrimitiveFloat::MAX_EXPONENT).
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_primitive_floats_with_sci_exponent;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_primitive_floats_with_sci_exponent::<f32>(0).map(NiceFloat),
///         20
///     ),
///     "[1.0, 1.5, 1.25, 1.75, 1.125, 1.375, 1.625, 1.875, 1.0625, 1.1875, 1.3125, 1.4375, \
///     1.5625, 1.6875, 1.8125, 1.9375, 1.03125, 1.09375, 1.15625, 1.21875, ...]",
/// );
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_primitive_floats_with_sci_exponent::<f32>(4).map(NiceFloat),
///         20
///     ),
///     "[16.0, 24.0, 20.0, 28.0, 18.0, 22.0, 26.0, 30.0, 17.0, 19.0, 21.0, 23.0, 25.0, 27.0, \
///     29.0, 31.0, 16.5, 17.5, 18.5, 19.5, ...]"
/// );
/// assert_eq!(
///     exhaustive_primitive_floats_with_sci_exponent::<f32>(-147)
///         .map(NiceFloat)
///         .collect_vec(),
///     [6.0e-45, 8.0e-45, 7.0e-45, 1.0e-44]
///         .iter()
///         .copied()
///         .map(NiceFloat)
///         .collect_vec()
/// );
/// ```
#[inline]
pub fn exhaustive_primitive_floats_with_sci_exponent<T: PrimitiveFloat>(
    sci_exponent: i64,
) -> ExhaustivePrimitiveFloatsWithExponent<T> {
    ExhaustivePrimitiveFloatsWithExponent(exhaustive_primitive_floats_with_sci_exponent_helper(
        sci_exponent,
    ))
}

#[derive(Clone, Debug)]
struct ExhaustivePositiveFinitePrimitiveFloatsGenerator<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveFloat>
    ExhaustiveDependentPairsYsGenerator<i64, T, ExhaustivePrimitiveFloatsWithExponent<T>>
    for ExhaustivePositiveFinitePrimitiveFloatsGenerator<T>
{
    #[inline]
    fn get_ys(&self, &sci_exponent: &i64) -> ExhaustivePrimitiveFloatsWithExponent<T> {
        exhaustive_primitive_floats_with_sci_exponent(sci_exponent)
    }
}

#[inline]
fn exhaustive_positive_finite_primitive_floats_helper<T: PrimitiveFloat>(
) -> ExhaustiveDependentPairs<
    i64,
    T,
    RulerSequence<usize>,
    ExhaustivePositiveFinitePrimitiveFloatsGenerator<T>,
    ExhaustiveSignedRange<i64>,
    ExhaustivePrimitiveFloatsWithExponent<T>,
> {
    exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_signed_inclusive_range(T::MIN_EXPONENT, T::MAX_EXPONENT),
        ExhaustivePositiveFinitePrimitiveFloatsGenerator {
            phantom: PhantomData,
        },
    )
}

/// Generates all positive finite primitive floats.
///
/// This `struct` is created by [`exhaustive_positive_finite_primitive_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveFinitePrimitiveFloats<T: PrimitiveFloat>(
    ExhaustiveDependentPairs<
        i64,
        T,
        RulerSequence<usize>,
        ExhaustivePositiveFinitePrimitiveFloatsGenerator<T>,
        ExhaustiveSignedRange<i64>,
        ExhaustivePrimitiveFloatsWithExponent<T>,
    >,
);

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveFinitePrimitiveFloats<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.0.next().map(|p| p.1)
    }
}

/// Generates all positive finite primitive floats.
///
/// Positive and negative zero are both excluded.
///
/// Roughly speaking, the simplest floats are generated first. If you want to generate the floats in
/// ascending order instead, use [`positive_finite_primitive_floats_increasing`].
///
/// The output length is $2^M(2^E-1)-1$.
/// - For [`f32`], this is $2^{31}-2^{23}-1$, or 2139095039.
/// - For [`f64`], this is $2^{63}-2^{52}-1$, or 9218868437227405311.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_positive_finite_primitive_floats;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_positive_finite_primitive_floats::<f32>().map(NiceFloat),
///         50
///     ),
///     "[1.0, 2.0, 1.5, 0.5, 1.25, 3.0, 1.75, 4.0, 1.125, 2.5, 1.375, 0.75, 1.625, 3.5, 1.875, \
///     0.25, 1.0625, 2.25, 1.1875, 0.625, 1.3125, 2.75, 1.4375, 6.0, 1.5625, 3.25, 1.6875, 0.875, \
///     1.8125, 3.75, 1.9375, 8.0, 1.03125, 2.125, 1.09375, 0.5625, 1.15625, 2.375, 1.21875, 5.0, \
///     1.28125, 2.625, 1.34375, 0.6875, 1.40625, 2.875, 1.46875, 0.375, 1.53125, 3.125, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_positive_finite_primitive_floats<T: PrimitiveFloat>(
) -> ExhaustivePositiveFinitePrimitiveFloats<T> {
    ExhaustivePositiveFinitePrimitiveFloats(exhaustive_positive_finite_primitive_floats_helper())
}

/// Generates all negative finite primitive floats.
///
/// This `struct` is created by [`exhaustive_negative_finite_primitive_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNegativeFinitePrimitiveFloats<T: PrimitiveFloat>(
    ExhaustivePositiveFinitePrimitiveFloats<T>,
);

impl<T: PrimitiveFloat> Iterator for ExhaustiveNegativeFinitePrimitiveFloats<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.0.next().map(|f| -f)
    }
}

/// Generates all negative finite primitive floats.
///
/// Positive and negative zero are both excluded.
///
/// Roughly speaking, the simplest floats are generated first. If you want to generate the floats in
/// ascending order instead, use [`negative_finite_primitive_floats_increasing`].
///
/// The output length is $2^M(2^E-1)-1$.
/// - For [`f32`], this is $2^{31}-2^{23}-1$, or 2139095039.
/// - For [`f64`], this is $2^{63}-2^{52}-1$, or 9218868437227405311.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_negative_finite_primitive_floats;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_negative_finite_primitive_floats::<f32>().map(NiceFloat),
///         50
///     ),
///     "[-1.0, -2.0, -1.5, -0.5, -1.25, -3.0, -1.75, -4.0, -1.125, -2.5, -1.375, -0.75, -1.625, \
///     -3.5, -1.875, -0.25, -1.0625, -2.25, -1.1875, -0.625, -1.3125, -2.75, -1.4375, -6.0, \
///     -1.5625, -3.25, -1.6875, -0.875, -1.8125, -3.75, -1.9375, -8.0, -1.03125, -2.125, \
///     -1.09375, -0.5625, -1.15625, -2.375, -1.21875, -5.0, -1.28125, -2.625, -1.34375, -0.6875, \
///     -1.40625, -2.875, -1.46875, -0.375, -1.53125, -3.125, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_negative_finite_primitive_floats<T: PrimitiveFloat>(
) -> ExhaustiveNegativeFinitePrimitiveFloats<T> {
    ExhaustiveNegativeFinitePrimitiveFloats(exhaustive_positive_finite_primitive_floats())
}

/// Generates all nonzero finite primitive floats.
///
/// This `struct` is created by [`exhaustive_nonzero_finite_primitive_floats`]; see its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct ExhaustiveNonzeroFinitePrimitiveFloats<T: PrimitiveFloat> {
    toggle: bool,
    xs: ExhaustivePositiveFinitePrimitiveFloats<T>,
    x: T,
}

impl<T: PrimitiveFloat> Iterator for ExhaustiveNonzeroFinitePrimitiveFloats<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.toggle.not_assign();
        Some(if self.toggle {
            self.x = self.xs.next().unwrap();
            self.x
        } else {
            -self.x
        })
    }
}

/// Generates all nonzero finite primitive floats.
///
/// Positive and negative zero are both excluded.
///
/// Roughly speaking, the simplest floats are generated first. If you want to generate the floats in
/// ascending order instead, use [`nonzero_finite_primitive_floats_increasing`].
///
/// The output length is $2^{M+1}(2^E-1)-2$.
/// - For [`f32`], this is $2^{32}-2^{24}-2$, or 4278190078.
/// - For [`f64`], this is $2^{64}-2^{53}-2$, or 18437736874454810622.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_nonzero_finite_primitive_floats;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_nonzero_finite_primitive_floats::<f32>().map(NiceFloat),
///         50
///     ),
///     "[1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, -3.0, 1.75, -1.75, 4.0, \
///     -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, 1.625, -1.625, 3.5, -3.5, \
///     1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, 1.1875, -1.1875, 0.625, -0.625, \
///     1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0, 1.5625, -1.5625, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_nonzero_finite_primitive_floats<T: PrimitiveFloat>(
) -> ExhaustiveNonzeroFinitePrimitiveFloats<T> {
    ExhaustiveNonzeroFinitePrimitiveFloats {
        toggle: false,
        xs: exhaustive_positive_finite_primitive_floats(),
        x: T::ZERO,
    }
}

pub type ExhaustiveFinitePrimitiveFloats<T> =
    Chain<IntoIter<T>, ExhaustiveNonzeroFinitePrimitiveFloats<T>>;

/// Generates all finite primitive floats.
///
/// Positive and negative zero are both included.
///
/// Roughly speaking, the simplest floats are generated first. If you want to generate the floats in
/// ascending order instead, use [`finite_primitive_floats_increasing`].
///
/// The output length is $2^{M+1}(2^E-1)$.
/// - For [`f32`], this is $2^{32}-2^{24}$, or 4278190080.
/// - For [`f64`], this is $2^{64}-2^{53}$, or 18437736874454810624.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_finite_primitive_floats;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_finite_primitive_floats::<f32>().map(NiceFloat),
///         50
///     ),
///     "[0.0, -0.0, 1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, -3.0, 1.75, \
///     -1.75, 4.0, -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, 1.625, -1.625, \
///     3.5, -3.5, 1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, 1.1875, -1.1875, \
///     0.625, -0.625, 1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_finite_primitive_floats<T: PrimitiveFloat>(
) -> Chain<IntoIter<T>, ExhaustiveNonzeroFinitePrimitiveFloats<T>> {
    ::alloc::vec![T::ZERO, T::NEGATIVE_ZERO]
        .into_iter()
        .chain(exhaustive_nonzero_finite_primitive_floats())
}

/// Generates all positive primitive floats.
///
/// Positive and negative zero are both excluded.
///
/// Roughly speaking, the simplest floats are generated first. If you want to generate the floats in
/// ascending order instead, use [`positive_primitive_floats_increasing`].
///
/// The output length is $2^M(2^E-1)$.
/// - For [`f32`], this is $2^{31}-2^{23}$, or 2139095040.
/// - For [`f64`], this is $2^{63}-2^{52}$, or 9218868437227405312.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_positive_primitive_floats;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_positive_primitive_floats::<f32>().map(NiceFloat),
///         50
///     ),
///     "[Infinity, 1.0, 2.0, 1.5, 0.5, 1.25, 3.0, 1.75, 4.0, 1.125, 2.5, 1.375, 0.75, 1.625, \
///     3.5, 1.875, 0.25, 1.0625, 2.25, 1.1875, 0.625, 1.3125, 2.75, 1.4375, 6.0, 1.5625, 3.25, \
///     1.6875, 0.875, 1.8125, 3.75, 1.9375, 8.0, 1.03125, 2.125, 1.09375, 0.5625, 1.15625, \
///     2.375, 1.21875, 5.0, 1.28125, 2.625, 1.34375, 0.6875, 1.40625, 2.875, 1.46875, 0.375, \
///     1.53125, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_positive_primitive_floats<T: PrimitiveFloat>(
) -> Chain<Once<T>, ExhaustivePositiveFinitePrimitiveFloats<T>> {
    once(T::INFINITY).chain(exhaustive_positive_finite_primitive_floats())
}

/// Generates all negative primitive floats.
///
/// Positive and negative zero are both excluded.
///
/// Roughly speaking, the simplest floats are generated first. If you want to generate the floats in
/// ascending order instead, use [`negative_primitive_floats_increasing`].
///
/// The output length is $2^M(2^E-1)$.
/// - For [`f32`], this is $2^{31}-2^{23}$, or 2139095040.
/// - For [`f64`], this is $2^{63}-2^{52}$, or 9218868437227405312.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_negative_primitive_floats;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_negative_primitive_floats::<f32>().map(NiceFloat),
///         50
///     ),
///     "[-Infinity, -1.0, -2.0, -1.5, -0.5, -1.25, -3.0, -1.75, -4.0, -1.125, -2.5, -1.375, \
///     -0.75, -1.625, -3.5, -1.875, -0.25, -1.0625, -2.25, -1.1875, -0.625, -1.3125, -2.75, \
///     -1.4375, -6.0, -1.5625, -3.25, -1.6875, -0.875, -1.8125, -3.75, -1.9375, -8.0, -1.03125, \
///     -2.125, -1.09375, -0.5625, -1.15625, -2.375, -1.21875, -5.0, -1.28125, -2.625, -1.34375, \
///     -0.6875, -1.40625, -2.875, -1.46875, -0.375, -1.53125, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_negative_primitive_floats<T: PrimitiveFloat>(
) -> Chain<Once<T>, ExhaustiveNegativeFinitePrimitiveFloats<T>> {
    once(T::NEGATIVE_INFINITY).chain(exhaustive_negative_finite_primitive_floats())
}

/// Generates all nonzero primitive floats.
///
/// Positive and negative zero are both excluded. NaN is excluded as well.
///
/// Roughly speaking, the simplest floats are generated first. If you want to generate the floats in
/// ascending order instead, use [`nonzero_primitive_floats_increasing`].
///
/// The output length is $2^{M+1}(2^E-1)$.
/// - For [`f32`], this is $2^{32}-2^{24}$, or 4278190080.
/// - For [`f64`], this is $2^{64}-2^{53}$, or 18437736874454810624.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_nonzero_primitive_floats;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_nonzero_primitive_floats::<f32>().map(NiceFloat),
///         50
///     ),
///     "[Infinity, -Infinity, 1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, \
///     -3.0, 1.75, -1.75, 4.0, -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, \
///     1.625, -1.625, 3.5, -3.5, 1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, \
///     1.1875, -1.1875, 0.625, -0.625, 1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0, \
///     ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_nonzero_primitive_floats<T: PrimitiveFloat>(
) -> Chain<IntoIter<T>, ExhaustiveNonzeroFinitePrimitiveFloats<T>> {
    ::alloc::vec![T::INFINITY, T::NEGATIVE_INFINITY]
        .into_iter()
        .chain(exhaustive_nonzero_finite_primitive_floats())
}

/// Generates all primitive floats.
///
/// Positive and negative zero are both included.
///
/// Roughly speaking, the simplest floats are generated first. If you want to generate the floats
/// (except `NaN`) in ascending order instead, use [`primitive_floats_increasing`].
///
/// The output length is $2^{M+1}(2^E-1)+2$.
/// - For [`f32`], this is $2^{32}-2^{24}+2$, or 4278190082.
/// - For [`f64`], this is $2^{64}-2^{53}+2$, or 18437736874454810626.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_primitive_floats;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(exhaustive_primitive_floats::<f32>().map(NiceFloat), 50),
///     "[NaN, Infinity, -Infinity, 0.0, -0.0, 1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, \
///     -1.25, 3.0, -3.0, 1.75, -1.75, 4.0, -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, \
///     -0.75, 1.625, -1.625, 3.5, -3.5, 1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, \
///     -2.25, 1.1875, -1.1875, 0.625, -0.625, 1.3125, -1.3125, 2.75, -2.75, 1.4375, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_primitive_floats<T: PrimitiveFloat>(
) -> Chain<IntoIter<T>, ExhaustiveNonzeroFinitePrimitiveFloats<T>> {
    ::alloc::vec![T::NAN, T::INFINITY, T::NEGATIVE_INFINITY, T::ZERO, T::NEGATIVE_ZERO]
        .into_iter()
        .chain(exhaustive_nonzero_finite_primitive_floats())
}

pub_test! {exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range<T: PrimitiveFloat>(
    a: T,
    b: T,
    sci_exponent: i64,
    precision: u64
) -> ConstantPrecisionPrimitiveFloats<T> {
    assert!(a.is_finite());
    assert!(b.is_finite());
    assert!(a > T::ZERO);
    assert!(b > T::ZERO);
    assert!(sci_exponent >= T::MIN_EXPONENT);
    assert!(sci_exponent <= T::MAX_EXPONENT);
    let (am, ae) = a.raw_mantissa_and_exponent();
    let (bm, be) = b.raw_mantissa_and_exponent();
    let ae_actual_sci_exponent = if ae == 0 {
        i64::wrapping_from(am.significant_bits()) + T::MIN_EXPONENT - 1
    } else {
        i64::wrapping_from(ae) - T::MAX_EXPONENT
    };
    let be_actual_sci_exponent = if be == 0 {
        i64::wrapping_from(bm.significant_bits()) + T::MIN_EXPONENT - 1
    } else {
        i64::wrapping_from(be) - T::MAX_EXPONENT
    };
    assert_eq!(ae_actual_sci_exponent, sci_exponent);
    assert_eq!(be_actual_sci_exponent, sci_exponent);
    assert!(am <= bm);
    assert_ne!(precision, 0);
    let max_precision = T::max_precision_for_sci_exponent(sci_exponent);
    assert!(precision <= max_precision);
    if precision == 1 && am == 0 {
        return ConstantPrecisionPrimitiveFloats {
            phantom: PhantomData,
            n: a.to_bits(),
            increment: 0,
            i: 0,
            count: 1,
        };
    }
    let trailing_zeros = max_precision - precision;
    let increment = u64::power_of_2(trailing_zeros + 1);
    let mut start_mantissa = am.round_to_multiple_of_power_of_2(trailing_zeros, Up).0;
    if !start_mantissa.get_bit(trailing_zeros) {
        start_mantissa.set_bit(trailing_zeros);
    }
    if start_mantissa > bm {
        return ConstantPrecisionPrimitiveFloats::default();
    }
    let mut end_mantissa = bm.round_to_multiple_of_power_of_2(trailing_zeros, Down).0;
    if !end_mantissa.get_bit(trailing_zeros) {
        let adjust = u64::power_of_2(trailing_zeros);
        if adjust > end_mantissa {
            return ConstantPrecisionPrimitiveFloats::default();
        }
        end_mantissa -= adjust;
    }
    assert!(start_mantissa <= end_mantissa);
    let count = ((end_mantissa - start_mantissa) >> (trailing_zeros + 1)) + 1;
    let first = T::from_raw_mantissa_and_exponent(start_mantissa, ae).to_bits();
    ConstantPrecisionPrimitiveFloats {
        phantom: PhantomData,
        n: first,
        increment,
        i: 0,
        count,
    }
}}

#[derive(Clone, Debug)]
struct PrimitiveFloatsWithExponentInRangeGenerator<T: PrimitiveFloat> {
    a: T,
    b: T,
    sci_exponent: i64,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveFloat>
    ExhaustiveDependentPairsYsGenerator<u64, T, ConstantPrecisionPrimitiveFloats<T>>
    for PrimitiveFloatsWithExponentInRangeGenerator<T>
{
    #[inline]
    fn get_ys(&self, &precision: &u64) -> ConstantPrecisionPrimitiveFloats<T> {
        exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range(
            self.a,
            self.b,
            self.sci_exponent,
            precision,
        )
    }
}

#[inline]
fn exhaustive_primitive_floats_with_sci_exponent_in_range_helper<T: PrimitiveFloat>(
    a: T,
    b: T,
    sci_exponent: i64,
) -> LexDependentPairs<
    u64,
    T,
    PrimitiveFloatsWithExponentInRangeGenerator<T>,
    PrimitiveIntIncreasingRange<u64>,
    ConstantPrecisionPrimitiveFloats<T>,
> {
    lex_dependent_pairs(
        primitive_int_increasing_inclusive_range(
            1,
            T::max_precision_for_sci_exponent(sci_exponent),
        ),
        PrimitiveFloatsWithExponentInRangeGenerator {
            a,
            b,
            sci_exponent,
            phantom: PhantomData,
        },
    )
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ExhaustivePrimitiveFloatsWithExponentInRange<T: PrimitiveFloat>(
    LexDependentPairs<
        u64,
        T,
        PrimitiveFloatsWithExponentInRangeGenerator<T>,
        PrimitiveIntIncreasingRange<u64>,
        ConstantPrecisionPrimitiveFloats<T>,
    >,
);

impl<T: PrimitiveFloat> Iterator for ExhaustivePrimitiveFloatsWithExponentInRange<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.0.next().map(|p| p.1)
    }
}

#[doc(hidden)]
#[inline]
pub fn exhaustive_primitive_floats_with_sci_exponent_in_range<T: PrimitiveFloat>(
    a: T,
    b: T,
    sci_exponent: i64,
) -> ExhaustivePrimitiveFloatsWithExponentInRange<T> {
    ExhaustivePrimitiveFloatsWithExponentInRange(
        exhaustive_primitive_floats_with_sci_exponent_in_range_helper(a, b, sci_exponent),
    )
}

#[derive(Clone, Debug)]
struct ExhaustivePositiveFinitePrimitiveFloatsInRangeGenerator<T: PrimitiveFloat> {
    a: T,
    b: T,
    a_sci_exponent: i64,
    b_sci_exponent: i64,
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveFloat>
    ExhaustiveDependentPairsYsGenerator<i64, T, ExhaustivePrimitiveFloatsWithExponentInRange<T>>
    for ExhaustivePositiveFinitePrimitiveFloatsInRangeGenerator<T>
{
    #[inline]
    fn get_ys(&self, &sci_exponent: &i64) -> ExhaustivePrimitiveFloatsWithExponentInRange<T> {
        let a = if sci_exponent == self.a_sci_exponent {
            self.a
        } else {
            T::from_integer_mantissa_and_exponent(1, sci_exponent).unwrap()
        };
        let b = if sci_exponent == self.b_sci_exponent {
            self.b
        } else {
            T::from_integer_mantissa_and_exponent(1, sci_exponent + 1)
                .unwrap()
                .next_lower()
        };
        exhaustive_primitive_floats_with_sci_exponent_in_range(a, b, sci_exponent)
    }
}

#[inline]
fn exhaustive_positive_finite_primitive_floats_in_range_helper<T: PrimitiveFloat>(
    a: T,
    b: T,
) -> ExhaustiveDependentPairs<
    i64,
    T,
    RulerSequence<usize>,
    ExhaustivePositiveFinitePrimitiveFloatsInRangeGenerator<T>,
    ExhaustiveSignedRange<i64>,
    ExhaustivePrimitiveFloatsWithExponentInRange<T>,
> {
    assert!(a.is_finite());
    assert!(b.is_finite());
    assert!(a > T::ZERO);
    assert!(a <= b);
    let (am, ae) = a.raw_mantissa_and_exponent();
    let (bm, be) = b.raw_mantissa_and_exponent();
    let a_sci_exponent = if ae == 0 {
        i64::wrapping_from(am.significant_bits()) + T::MIN_EXPONENT - 1
    } else {
        i64::wrapping_from(ae) - T::MAX_EXPONENT
    };
    let b_sci_exponent = if be == 0 {
        i64::wrapping_from(bm.significant_bits()) + T::MIN_EXPONENT - 1
    } else {
        i64::wrapping_from(be) - T::MAX_EXPONENT
    };
    exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_signed_inclusive_range(a_sci_exponent, b_sci_exponent),
        ExhaustivePositiveFinitePrimitiveFloatsInRangeGenerator {
            a,
            b,
            a_sci_exponent,
            b_sci_exponent,
            phantom: PhantomData,
        },
    )
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ExhaustivePositiveFinitePrimitiveFloatsInRange<T: PrimitiveFloat>(
    ExhaustiveDependentPairs<
        i64,
        T,
        RulerSequence<usize>,
        ExhaustivePositiveFinitePrimitiveFloatsInRangeGenerator<T>,
        ExhaustiveSignedRange<i64>,
        ExhaustivePrimitiveFloatsWithExponentInRange<T>,
    >,
);

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveFinitePrimitiveFloatsInRange<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.0.next().map(|p| p.1)
    }
}

#[doc(hidden)]
#[inline]
pub fn exhaustive_positive_finite_primitive_floats_in_range<T: PrimitiveFloat>(
    a: T,
    b: T,
) -> ExhaustivePositiveFinitePrimitiveFloatsInRange<T> {
    ExhaustivePositiveFinitePrimitiveFloatsInRange(
        exhaustive_positive_finite_primitive_floats_in_range_helper(a, b),
    )
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum ExhaustiveNonzeroFinitePrimitiveFloatsInRange<T: PrimitiveFloat> {
    AllPositive(ExhaustivePositiveFinitePrimitiveFloatsInRange<T>),
    AllNegative(ExhaustivePositiveFinitePrimitiveFloatsInRange<T>),
    PositiveAndNegative(
        bool,
        ExhaustivePositiveFinitePrimitiveFloatsInRange<T>,
        ExhaustivePositiveFinitePrimitiveFloatsInRange<T>,
    ),
}

impl<T: PrimitiveFloat> Iterator for ExhaustiveNonzeroFinitePrimitiveFloatsInRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            ExhaustiveNonzeroFinitePrimitiveFloatsInRange::AllPositive(ref mut xs) => xs.next(),
            ExhaustiveNonzeroFinitePrimitiveFloatsInRange::AllNegative(ref mut xs) => {
                xs.next().map(T::neg)
            }
            ExhaustiveNonzeroFinitePrimitiveFloatsInRange::PositiveAndNegative(
                ref mut toggle,
                ref mut pos_xs,
                ref mut neg_xs,
            ) => {
                toggle.not_assign();
                if *toggle {
                    pos_xs.next().or_else(|| neg_xs.next().map(T::neg))
                } else {
                    neg_xs.next().map(T::neg).or_else(|| pos_xs.next())
                }
            }
        }
    }
}

#[doc(hidden)]
#[inline]
pub fn exhaustive_nonzero_finite_primitive_floats_in_range<T: PrimitiveFloat>(
    a: T,
    b: T,
) -> ExhaustiveNonzeroFinitePrimitiveFloatsInRange<T> {
    assert!(a.is_finite());
    assert!(b.is_finite());
    assert!(a != T::ZERO);
    assert!(b != T::ZERO);
    assert!(a <= b);
    if a > T::ZERO {
        ExhaustiveNonzeroFinitePrimitiveFloatsInRange::AllPositive(
            exhaustive_positive_finite_primitive_floats_in_range(a, b),
        )
    } else if b < T::ZERO {
        ExhaustiveNonzeroFinitePrimitiveFloatsInRange::AllNegative(
            exhaustive_positive_finite_primitive_floats_in_range(-b, -a),
        )
    } else {
        ExhaustiveNonzeroFinitePrimitiveFloatsInRange::PositiveAndNegative(
            false,
            exhaustive_positive_finite_primitive_floats_in_range(T::MIN_POSITIVE_SUBNORMAL, b),
            exhaustive_positive_finite_primitive_floats_in_range(T::MIN_POSITIVE_SUBNORMAL, -a),
        )
    }
}

/// Generates all primitive floats in an interval.
///
/// This `enum` is created by [`exhaustive_primitive_float_range`] and
/// [`exhaustive_primitive_float_inclusive_range`]; see their documentation for more.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum ExhaustivePrimitiveFloatInclusiveRange<T: PrimitiveFloat> {
    JustSpecials(IntoIter<T>),
    NotJustSpecials(Chain<IntoIter<T>, ExhaustiveNonzeroFinitePrimitiveFloatsInRange<T>>),
}

impl<T: PrimitiveFloat> Iterator for ExhaustivePrimitiveFloatInclusiveRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            ExhaustivePrimitiveFloatInclusiveRange::JustSpecials(ref mut xs) => xs.next(),
            ExhaustivePrimitiveFloatInclusiveRange::NotJustSpecials(ref mut xs) => xs.next(),
        }
    }
}

/// Generates all primitive floats in the half-open interval $[a, b)$.
///
/// Positive and negative zero are treated as two distinct values, with negative zero being smaller
/// than zero.
///
/// The floats are generated in a way such that simpler floats (with lower precision) are generated
/// first. To generate floats in ascending order instead, use [`primitive_float_increasing_range`]
/// instead.
///
/// `NiceFloat(a)` must be less than or equal to `NiceFloat(b)`. If `NiceFloat(a)` and
/// `NiceFloat(b)` are equal, the range is empty.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output length is $\varphi(b) - \varphi(a)$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `NiceFloat(a) > NiceFloat(b)`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_primitive_float_range;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_primitive_float_range::<f32>(core::f32::consts::E, core::f32::consts::PI)
///             .map(NiceFloat),
///         50
///     ),
///     "[3.0, 2.75, 2.875, 3.125, 2.8125, 2.9375, 3.0625, 2.71875, 2.78125, 2.84375, 2.90625, \
///     2.96875, 3.03125, 3.09375, 2.734375, 2.765625, 2.796875, 2.828125, 2.859375, 2.890625, \
///     2.921875, 2.953125, 2.984375, 3.015625, 3.046875, 3.078125, 3.109375, 3.140625, \
///     2.7265625, 2.7421875, 2.7578125, 2.7734375, 2.7890625, 2.8046875, 2.8203125, 2.8359375, \
///     2.8515625, 2.8671875, 2.8828125, 2.8984375, 2.9140625, 2.9296875, 2.9453125, 2.9609375, \
///     2.9765625, 2.9921875, 3.0078125, 3.0234375, 3.0390625, 3.0546875, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_primitive_float_range<T: PrimitiveFloat>(
    a: T,
    b: T,
) -> ExhaustivePrimitiveFloatInclusiveRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(NiceFloat(a) <= NiceFloat(b));
    if NiceFloat(a) == NiceFloat(b) {
        ExhaustivePrimitiveFloatInclusiveRange::JustSpecials(Vec::new().into_iter())
    } else {
        exhaustive_primitive_float_inclusive_range(a, b.next_lower())
    }
}

/// Generates all primitive floats in the closed interval $[a, b]$.
///
/// Positive and negative zero are treated as two distinct values, with negative zero being smaller
/// than zero.
///
/// The floats are generated in a way such that simpler floats (with lower precision) are generated
/// first. To generate floats in ascending order instead, use
/// `primitive_float_increasing_inclusive_range` instead.
///
/// `NiceFloat(a)` must be less than or equal to `NiceFloat(b)`. If `NiceFloat(a)` and
/// `NiceFloat(b)` are equal, the range contains a single element.
///
/// Let $\varphi$ be
/// [`to_ordered_representation`](super::basic::floats::PrimitiveFloat::to_ordered_representation):
///
/// The output length is $\varphi(b) - \varphi(a) + 1$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `NiceFloat(a) > NiceFloat(b)`.
///
/// # Examples
/// ```
/// use malachite_base::iterators::prefix_to_string;
/// use malachite_base::num::exhaustive::exhaustive_primitive_float_inclusive_range;
/// use malachite_base::num::float::NiceFloat;
///
/// assert_eq!(
///     prefix_to_string(
///         exhaustive_primitive_float_inclusive_range::<f32>(
///             core::f32::consts::E,
///             core::f32::consts::PI
///         )
///         .map(NiceFloat),
///         50
///     ),
///     "[3.0, 2.75, 2.875, 3.125, 2.8125, 2.9375, 3.0625, 2.71875, 2.78125, 2.84375, 2.90625, \
///     2.96875, 3.03125, 3.09375, 2.734375, 2.765625, 2.796875, 2.828125, 2.859375, 2.890625, \
///     2.921875, 2.953125, 2.984375, 3.015625, 3.046875, 3.078125, 3.109375, 3.140625, \
///     2.7265625, 2.7421875, 2.7578125, 2.7734375, 2.7890625, 2.8046875, 2.8203125, 2.8359375, \
///     2.8515625, 2.8671875, 2.8828125, 2.8984375, 2.9140625, 2.9296875, 2.9453125, 2.9609375, \
///     2.9765625, 2.9921875, 3.0078125, 3.0234375, 3.0390625, 3.0546875, ...]"
/// );
/// ```
#[inline]
pub fn exhaustive_primitive_float_inclusive_range<T: PrimitiveFloat>(
    mut a: T,
    mut b: T,
) -> ExhaustivePrimitiveFloatInclusiveRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    assert!(NiceFloat(a) <= NiceFloat(b));
    let mut specials = Vec::new();
    if b == T::INFINITY {
        specials.push(T::INFINITY);
        if a == T::INFINITY {
            return ExhaustivePrimitiveFloatInclusiveRange::JustSpecials(specials.into_iter());
        }
        b = T::MAX_FINITE;
    }
    if a == T::NEGATIVE_INFINITY {
        specials.push(T::NEGATIVE_INFINITY);
        if b == T::NEGATIVE_INFINITY {
            return ExhaustivePrimitiveFloatInclusiveRange::JustSpecials(specials.into_iter());
        }
        a = -T::MAX_FINITE;
    }
    if NiceFloat(a) <= NiceFloat(T::ZERO) && NiceFloat(b) >= NiceFloat(T::ZERO) {
        specials.push(T::ZERO);
    }
    if NiceFloat(a) <= NiceFloat(T::NEGATIVE_ZERO) && NiceFloat(b) >= NiceFloat(T::NEGATIVE_ZERO) {
        specials.push(T::NEGATIVE_ZERO);
    }
    if a == T::ZERO {
        if b == T::ZERO {
            return ExhaustivePrimitiveFloatInclusiveRange::JustSpecials(specials.into_iter());
        }
        a = T::MIN_POSITIVE_SUBNORMAL;
    }
    if b == T::ZERO {
        b = -T::MIN_POSITIVE_SUBNORMAL;
    }
    ExhaustivePrimitiveFloatInclusiveRange::NotJustSpecials(
        specials
            .into_iter()
            .chain(exhaustive_nonzero_finite_primitive_floats_in_range(a, b)),
    )
}
