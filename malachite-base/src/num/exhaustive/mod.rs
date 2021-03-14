use iterators::{nonzero_values, NonzeroValues};
use itertools::{Interleave, Itertools};
use num::basic::integers::PrimitiveInt;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::float::nice_float::NiceFloat;
use num::float::PrimitiveFloat;
use std::iter::{once, Chain, Once, Rev};

/// Generates all primitive integers in an interval.
///
/// This `struct` is created by the `primitive_int_increasing_range` and
/// `primitive_int_increasing_inclusive_range` functions. See their documentation for more.
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
/// This `struct` is created by the `exhaustive_signed_range` and
/// `exhaustive_signed_inclusive_range` functions. See their documentation for more.
#[derive(Clone, Debug)]
pub enum ExhaustiveSignedRange<T: PrimitiveSigned> {
    NonNegative(PrimitiveIntIncreasingRange<T>),
    NonPositive(Rev<PrimitiveIntIncreasingRange<T>>),
    BothSigns(Chain<Once<T>, PrimitiveIntUpDown<T>>),
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
/// The output is $(k)_{k=0}^{2^W-1}$, where $W$ is `T::WIDTH`.
///
/// The output length is $2^W$, where $W$ is `T::WIDTH`.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
///
/// assert_eq!(
///     exhaustive_unsigneds::<u8>().take(10).collect_vec(),
///     &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
/// )
/// ```
#[inline]
pub fn exhaustive_unsigneds<T: PrimitiveUnsigned>() -> PrimitiveIntIncreasingRange<T> {
    primitive_int_increasing_inclusive_range(T::ZERO, T::MAX)
}

/// Generates all positive primitive integers in ascending order.
///
/// Let $L=2^W-1$ if `T` is unsigned and $L=2^{W-1}-1$ if `T` is signed, where $W$ is `T::WIDTH`.
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
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::exhaustive_positive_primitive_ints;
///
/// assert_eq!(
///     exhaustive_positive_primitive_ints::<u8>().take(10).collect_vec(),
///     &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
/// )
/// ```
#[inline]
pub fn exhaustive_positive_primitive_ints<T: PrimitiveInt>() -> PrimitiveIntIncreasingRange<T> {
    primitive_int_increasing_inclusive_range(T::ONE, T::MAX)
}

/// Generates all signed integers in order of increasing absolute value.
///
/// When two numbers have the same absolute value, the positive one comes first.
///
/// The output satisfies
/// $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|, \operatorname{sgn}(-x_j))$ whenever
/// $i, j \\in [-2^{W-1}, 2^{W-1})$, where $W$ is `T::WIDTH`, and $i < j$.
///
/// The output length is $2^W$, where $W$ is `T::WIDTH`.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::exhaustive_signeds;
///
/// assert_eq!(
///     exhaustive_signeds::<i8>().take(10).collect_vec(),
///     &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5]
/// )
/// ```
#[inline]
pub fn exhaustive_signeds<T: PrimitiveSigned>() -> Chain<Once<T>, PrimitiveIntUpDown<T>> {
    once(T::ZERO).chain(exhaustive_nonzero_signeds())
}

/// Generates all natural (non-negative) signed integers in ascending order.
///
/// The output is $(k)_{k=0}^{2^{W-1}-1}$, where $W$ is `T::WIDTH`.
///
/// The output length is $2^{W-1}$, where $W$ is `T::WIDTH`.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::exhaustive_natural_signeds;
///
/// assert_eq!(
///     exhaustive_natural_signeds::<i8>().take(10).collect_vec(),
///     &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
/// )
/// ```
#[inline]
pub fn exhaustive_natural_signeds<T: PrimitiveSigned>() -> PrimitiveIntIncreasingRange<T> {
    primitive_int_increasing_inclusive_range(T::ZERO, T::MAX)
}

/// Generates all negative signed integers in descending order.
///
/// The output is $(-k)_{k=1}^{2^{W-1}}$, where $W$ is `T::WIDTH`.
///
/// The output length is $2^{W-1}$, where $W$ is `T::WIDTH`.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::exhaustive_negative_signeds;
///
/// assert_eq!(
///     exhaustive_negative_signeds::<i8>().take(10).collect_vec(),
///     &[-1, -2, -3, -4, -5, -6, -7, -8, -9, -10]
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
/// The output satisfies
/// $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|, \operatorname{sgn}(-x_j))$ whenever
/// $i, j \\in [-2^{W-1}, 2^{W-1}) \\setminus \\{0\\}$, where $W$ is `T::WIDTH`, and $i < j$.
///
/// The output length is $2^W-1$, where $W$ is `T::WIDTH`.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::exhaustive_nonzero_signeds;
///
/// assert_eq!(
///     exhaustive_nonzero_signeds::<i8>().take(10).collect_vec(),
///     &[1, -1, 2, -2, 3, -3, 4, -4, 5, -5]
/// )
/// ```
#[inline]
pub fn exhaustive_nonzero_signeds<T: PrimitiveSigned>() -> PrimitiveIntUpDown<T> {
    exhaustive_positive_primitive_ints().interleave(exhaustive_negative_signeds())
}

/// Generates all primitive integers in the half-open interval $[a, b)$, in ascending order.
///
/// `a` must be less than or equal to `b`. If `a` and `b` are equal, the range is empty. This
/// function cannot create a range that includes `T::MAX`; for that, use
/// `primitive_int_increasing_inclusive_range`.
///
/// The output is $(k)_{k=a}^{b-1}$.
///
/// The output length is $b - a$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
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
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    PrimitiveIntIncreasingRange {
        a: Some(a),
        b: Some(b),
    }
}

/// Generates all primitive integers in the closed interval $[a, b]$, in ascending order.
///
/// `a` must be less than or equal to `b`. If `a` and `b` are equal, the range contains a single
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
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
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
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    PrimitiveIntIncreasingRange {
        a: Some(a),
        b: b.checked_add(T::ONE),
    }
}

/// Generates all signed integers in the half-open interval $[a, b)$, in order of increasing
/// absolute value.
///
/// When two numbers have the same absolute value, the positive one comes first. `a` must be less
/// than or equal to `b`. If `a` and `b` are equal, the range is empty. This function cannot create
/// a range that includes `T::MAX`; for that, use `exhaustive_signed_inclusive_range`.
///
/// The output satisfies
/// $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|, \operatorname{sgn}(-x_j))$ whenever
/// $i, j \\in [0, b - a)$ and $i < j$.
///
/// The output length is $b - a$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::exhaustive_signed_range;
///
/// assert_eq!(
///     exhaustive_signed_range::<i8>(-5, 5).collect_vec(),
///     &[0, 1, -1, 2, -2, 3, -3, 4, -4, -5]
/// )
/// ```
pub fn exhaustive_signed_range<T: PrimitiveSigned>(a: T, b: T) -> ExhaustiveSignedRange<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
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
/// When two numbers have the same absolute value, the positive one comes first. `a` must be less
/// than or equal to `b`. If `a` and `b` are equal, the range contains a single element.
///
/// The output satisfies
/// $(|x_i|, \operatorname{sgn}(-x_i)) <_\mathrm{lex} (|x_j|, \operatorname{sgn}(-x_j))$ whenever
/// $i, j \\in [0, b - a]$ and $i < j$.
///
/// The output length is $b - a + 1$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
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
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
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
/// This struct implements `DoubleEndedIterator`, so you can reverse it to generate floats in
/// decreasing order.
///
/// Positive zero and negative zero are both generated. Negative zero is considered to be less than
/// positive zero.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PrimitiveFloatIncreasingRange<T: PrimitiveFloat> {
    xs: PrimitiveIntIncreasingRange<T::UnsignedOfEqualWidth>,
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
/// If the interval contains zero, positive zero and negative zero are both generated. Negative zero
/// is considered to be less than positive zero.
///
/// `NiceFloat(a)` must be less than or equal to `NiceFloat(b)`. If `NiceFloat(a)` and
/// `NiceFloat(b)` are equal, the range is empty. This function cannot create a range that includes
/// `T::POSITIVE_INFINITY`; for that, use `primitive_float_increasing_inclusive_range`.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is $(\varphi^{-1}(k))_{k=\varphi(a)}^{\varphi(b)-1}$.
///
/// The output length is $\varphi(b) - \varphi(a)$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `NiceFloat(a)` > `NiceFloat(b)`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::primitive_float_increasing_range;
/// use malachite_base::num::float::nice_float::NiceFloat;
///
/// assert_eq!(
///     primitive_float_increasing_range::<f32>(1.0, 2.0).take(20).map(NiceFloat).collect_vec(),
///     &[
///         1.0, 1.0000001, 1.0000002, 1.0000004, 1.0000005, 1.0000006, 1.0000007, 1.0000008,
///         1.000001, 1.0000011, 1.0000012, 1.0000013, 1.0000014, 1.0000015, 1.0000017, 1.0000018,
///         1.0000019, 1.000002, 1.0000021, 1.0000023
///     ]
/// );
///
/// let mut end = primitive_float_increasing_range::<f32>(1.0, 2.0).rev().take(20).map(NiceFloat)
///         .collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         1.9999976, 1.9999977, 1.9999979, 1.999998, 1.9999981, 1.9999982, 1.9999983, 1.9999985,
///         1.9999986, 1.9999987, 1.9999988, 1.9999989, 1.999999, 1.9999992, 1.9999993, 1.9999994,
///         1.9999995, 1.9999996, 1.9999998, 1.9999999
///     ]
/// );
/// ```
pub fn primitive_float_increasing_range<T: PrimitiveFloat>(
    a: T,
    b: T,
) -> PrimitiveFloatIncreasingRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    if NiceFloat(a) > NiceFloat(b) {
        panic!(
            "a must be less than or equal to b. a: {}, b: {}",
            NiceFloat(a),
            NiceFloat(b)
        );
    }
    PrimitiveFloatIncreasingRange {
        xs: primitive_int_increasing_range(
            a.to_ordered_representation(),
            b.to_ordered_representation(),
        ),
    }
}

/// Generates all primitive floats in the closed interval $[a, b]$, in ascending order.
///
/// If the interval contains zero, positive zero and negative zero are both generated. Negative zero
/// is considered to be less than positive zero.
///
/// `NiceFloat(a)` must be less than or equal to `NiceFloat(b)`. If `NiceFloat(a)` and
/// `NiceFloat(b)` are equal, the range is empty. If `NiceFloat(a)` and `NiceFloat(b)` are equal,
/// the range contains a single element.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is $(\varphi^{-1}(k))_{k=\varphi(a)}^\varphi(b)$.
///
/// The output length is $\varphi(b) - \varphi(a) + 1$.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `NiceFloat(a)` > `NiceFloat(b)`.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::primitive_float_increasing_inclusive_range;
/// use malachite_base::num::float::nice_float::NiceFloat;
///
/// assert_eq!(
///     primitive_float_increasing_inclusive_range::<f32>(1.0, 2.0).take(20).map(NiceFloat)
///         .collect_vec(),
///     &[
///         1.0, 1.0000001, 1.0000002, 1.0000004, 1.0000005, 1.0000006, 1.0000007, 1.0000008,
///         1.000001, 1.0000011, 1.0000012, 1.0000013, 1.0000014, 1.0000015, 1.0000017, 1.0000018,
///         1.0000019, 1.000002, 1.0000021, 1.0000023
///     ]
/// );
///
/// let mut end = primitive_float_increasing_inclusive_range::<f32>(1.0, 2.0).rev().take(20)
///         .map(NiceFloat).collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         1.9999977, 1.9999979, 1.999998, 1.9999981, 1.9999982, 1.9999983, 1.9999985, 1.9999986,
///         1.9999987, 1.9999988, 1.9999989, 1.999999, 1.9999992, 1.9999993, 1.9999994, 1.9999995,
///         1.9999996, 1.9999998, 1.9999999, 2.0
///     ]
/// );
/// ```
pub fn primitive_float_increasing_inclusive_range<T: PrimitiveFloat>(
    a: T,
    b: T,
) -> PrimitiveFloatIncreasingRange<T> {
    assert!(!a.is_nan());
    assert!(!b.is_nan());
    if NiceFloat(a) > NiceFloat(b) {
        panic!(
            "a must be less than or equal to b. a: {}, b: {}",
            NiceFloat(a),
            NiceFloat(b)
        );
    }
    PrimitiveFloatIncreasingRange {
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
/// `T::MIN_POSITIVE_SUBNORMAL` is generated first and `T::MAX_FINITE` is generated last. The
/// returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is $(\varphi^{-1}(k))_{k=2^M(2^E-1)+2}^{2^{M+1}(2^E-1)}$.
///
/// The output length is $2^M(2^E-1)-1$.
/// - For `f32`, this is $2^{31}-2^{23}-1$, or 2139095039.
/// - For `f64`, this is $2^{63}-2^{52}-1$, or 9218868437227405311.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::positive_finite_primitive_floats_increasing;
/// use malachite_base::num::float::nice_float::NiceFloat;
///
/// assert_eq!(
///     positive_finite_primitive_floats_increasing::<f32>().take(20).map(NiceFloat).collect_vec(),
///     &[
///         1.0e-45, 3.0e-45, 4.0e-45, 6.0e-45, 7.0e-45, 8.0e-45, 1.0e-44, 1.1e-44, 1.3e-44,
///         1.4e-44, 1.5e-44, 1.7e-44, 1.8e-44, 2.0e-44, 2.1e-44, 2.2e-44, 2.4e-44, 2.5e-44,
///         2.7e-44, 2.8e-44
///     ]
/// );
///
/// let mut end = positive_finite_primitive_floats_increasing::<f32>().rev().take(20).map(NiceFloat)
///         .collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         3.4028196e38, 3.4028198e38, 3.40282e38, 3.4028202e38, 3.4028204e38, 3.4028206e38,
///         3.4028208e38, 3.402821e38, 3.4028212e38, 3.4028214e38, 3.4028216e38, 3.4028218e38,
///         3.402822e38, 3.4028222e38, 3.4028225e38, 3.4028227e38, 3.4028229e38, 3.402823e38,
///         3.4028233e38, 3.4028235e38
///     ]
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
/// `-T::MAX_FINITE` is generated first and `-T::MIN_POSITIVE_SUBNORMAL` is generated last. The
/// returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is $(\varphi^{-1}(k))_{k=1}^{2^M(2^E-1)-1}$.
///
/// The output length is $2^M(2^E-1)-1$.
/// - For `f32`, this is $2^{31}-2^{23}-1$, or 2139095039.
/// - For `f64`, this is $2^{63}-2^{52}-1$, or 9218868437227405311.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::negative_finite_primitive_floats_increasing;
/// use malachite_base::num::float::nice_float::NiceFloat;
///
/// assert_eq!(
///     negative_finite_primitive_floats_increasing::<f32>().take(20).map(NiceFloat).collect_vec(),
///     &[
///         -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, -3.4028225e38,
///         -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, -3.4028212e38,
///         -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, -3.40282e38,
///         -3.4028198e38, -3.4028196e38
///     ]
/// );
///
/// let mut end = negative_finite_primitive_floats_increasing::<f32>().rev().take(20).map(NiceFloat)
///         .collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         -2.8e-44, -2.7e-44, -2.5e-44, -2.4e-44, -2.2e-44, -2.1e-44, -2.0e-44, -1.8e-44,
///         -1.7e-44, -1.5e-44, -1.4e-44, -1.3e-44, -1.1e-44, -1.0e-44, -8.0e-45, -7.0e-45,
///         -6.0e-45, -4.0e-45, -3.0e-45, -1.0e-45
///     ]
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
/// `-T::MAX_FINITE` is generated first and `T::MAX_FINITE` is generated last. The returned iterator
/// is double-ended, so it may be reversed.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is
/// $$
/// (\varphi^{-1}(k))_ {k=1}^{2^M(2^E-1)-1} ⧺ (\varphi^{-1}(k))_ {k=2^M(2^E-1)+2}^{2^{M+1}(2^E-1)}
/// $$.
///
/// The output length is $2^{M+1}(2^E-1)-2$.
/// - For `f32`, this is $2^{32}-2^{24}-2$, or 4278190078.
/// - For `f64`, this is $2^{64}-2^{53}-2$, or 18437736874454810622.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::nonzero_finite_primitive_floats_increasing;
/// use malachite_base::num::float::nice_float::NiceFloat;
///
/// assert_eq!(
///     nonzero_finite_primitive_floats_increasing::<f32>().take(20).map(NiceFloat).collect_vec(),
///     &[
///         -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, -3.4028225e38,
///         -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, -3.4028212e38,
///         -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, -3.40282e38,
///         -3.4028198e38, -3.4028196e38
///     ]
/// );
///
/// let mut end = nonzero_finite_primitive_floats_increasing::<f32>().rev().take(20).map(NiceFloat)
///         .collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         3.4028196e38, 3.4028198e38, 3.40282e38, 3.4028202e38, 3.4028204e38, 3.4028206e38,
///         3.4028208e38, 3.402821e38, 3.4028212e38, 3.4028214e38, 3.4028216e38, 3.4028218e38,
///         3.402822e38, 3.4028222e38, 3.4028225e38, 3.4028227e38, 3.4028229e38, 3.402823e38,
///         3.4028233e38, 3.4028235e38
///     ]
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
/// `-T::MAX_FINITE` is generated first and `T::MAX_FINITE` is generated last. The returned iterator
/// is double-ended, so it may be reversed.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is $(\varphi^{-1}(k))_{k=1}^{2^{M+1}(2^E-1)}$.
///
/// The output length is $2^{M+1}(2^E-1)$.
/// - For `f32`, this is $2^{32}-2^{24}$, or 4278190080.
/// - For `f64`, this is $2^{64}-2^{53}$, or 18437736874454810624.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::finite_primitive_floats_increasing;
/// use malachite_base::num::float::nice_float::NiceFloat;
///
/// assert_eq!(
///     finite_primitive_floats_increasing::<f32>().take(20).map(NiceFloat).collect_vec(),
///     &[
///         -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38, -3.4028227e38, -3.4028225e38,
///         -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38, -3.4028214e38, -3.4028212e38,
///         -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38, -3.4028202e38, -3.40282e38,
///         -3.4028198e38, -3.4028196e38
///     ]
/// );
///
/// let mut end = finite_primitive_floats_increasing::<f32>().rev().take(20).map(NiceFloat)
///         .collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         3.4028196e38, 3.4028198e38, 3.40282e38, 3.4028202e38, 3.4028204e38, 3.4028206e38,
///         3.4028208e38, 3.402821e38, 3.4028212e38, 3.4028214e38, 3.4028216e38, 3.4028218e38,
///         3.402822e38, 3.4028222e38, 3.4028225e38, 3.4028227e38, 3.4028229e38, 3.402823e38,
///         3.4028233e38, 3.4028235e38
///     ]
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
/// `T::MIN_POSITIVE_SUBNORMAL` is generated first and `T::POSITIVE_INFINITY` is generated last. The
/// returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is $(\varphi^{-1}(k))_{k=2^M(2^E-1)+2}^{2^{M+1}(2^E-1)+1}$.
///
/// The output length is $2^M(2^E-1)$.
/// - For `f32`, this is $2^{31}-2^{23}$, or 2139095040.
/// - For `f64`, this is $2^{63}-2^{52}$, or 9218868437227405312.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::positive_primitive_floats_increasing;
/// use malachite_base::num::float::nice_float::NiceFloat;
/// use malachite_base::num::float::PrimitiveFloat;
///
/// assert_eq!(
///     positive_primitive_floats_increasing::<f32>().take(20).map(NiceFloat).collect_vec(),
///     &[
///         1.0e-45, 3.0e-45, 4.0e-45, 6.0e-45, 7.0e-45, 8.0e-45, 1.0e-44, 1.1e-44, 1.3e-44,
///         1.4e-44, 1.5e-44, 1.7e-44, 1.8e-44, 2.0e-44, 2.1e-44, 2.2e-44, 2.4e-44, 2.5e-44,
///         2.7e-44, 2.8e-44
///     ]
/// );
///
/// let mut end = positive_primitive_floats_increasing::<f32>().rev().take(20).map(NiceFloat)
///     .collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         3.4028198e38, 3.40282e38, 3.4028202e38, 3.4028204e38, 3.4028206e38, 3.4028208e38,
///         3.402821e38, 3.4028212e38, 3.4028214e38, 3.4028216e38, 3.4028218e38, 3.402822e38,
///         3.4028222e38, 3.4028225e38, 3.4028227e38, 3.4028229e38, 3.402823e38, 3.4028233e38,
///         3.4028235e38, f32::POSITIVE_INFINITY
///     ]
/// );
/// ```
#[inline]
pub fn positive_primitive_floats_increasing<T: PrimitiveFloat>() -> PrimitiveFloatIncreasingRange<T>
{
    primitive_float_increasing_inclusive_range(T::MIN_POSITIVE_SUBNORMAL, T::POSITIVE_INFINITY)
}

/// Generates all negative primitive floats, in ascending order.
///
/// Positive and negative zero are both excluded.
///
/// `T::NEGATIVE_INFINITY` is generated first and `-T::MIN_POSITIVE_SUBNORMAL` is generated last.
/// The returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is $(\varphi^{-1}(k))_{k=0}^{2^M(2^E-1)-1}$.
///
/// The output length is $2^M(2^E-1)$.
/// - For `f32`, this is $2^{31}-2^{23}$, or 2139095040.
/// - For `f64`, this is $2^{63}-2^{52}$, or 9218868437227405312.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::negative_primitive_floats_increasing;
/// use malachite_base::num::float::nice_float::NiceFloat;
/// use malachite_base::num::float::PrimitiveFloat;
///
/// assert_eq!(
///     negative_primitive_floats_increasing::<f32>().take(20).map(NiceFloat).collect_vec(),
///     &[
///         f32::NEGATIVE_INFINITY, -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38,
///         -3.4028227e38, -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38,
///         -3.4028214e38, -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38,
///         -3.4028202e38, -3.40282e38, -3.4028198e38
///     ]
/// );
///
/// let mut end = negative_primitive_floats_increasing::<f32>().rev().take(20).map(NiceFloat)
///         .collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         -2.8e-44, -2.7e-44, -2.5e-44, -2.4e-44, -2.2e-44, -2.1e-44, -2.0e-44, -1.8e-44,
///         -1.7e-44, -1.5e-44, -1.4e-44, -1.3e-44, -1.1e-44, -1.0e-44, -8.0e-45, -7.0e-45,
///         -6.0e-45, -4.0e-45, -3.0e-45, -1.0e-45
///     ]
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
/// `T::NEGATIVE_INFINITY` is generated first and `T::POSITIVE_INFINITY` is generated last. The
/// returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is
/// $$
/// (\varphi^{-1}(k))_ {k=0}^{2^M(2^E-1)-1} ⧺ (\varphi^{-1}(k))_ {k=2^M(2^E-1)+2}^{2^{M+1}(2^E-1)+1}
/// $$.
///
/// The output length is $2^{M+1}(2^E-1)$.
/// - For `f32`, this is $2^{32}-2^{24}$, or 4278190080.
/// - For `f64`, this is $2^{64}-2^{53}$, or 18437736874454810624.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::nonzero_primitive_floats_increasing;
/// use malachite_base::num::float::nice_float::NiceFloat;
/// use malachite_base::num::float::PrimitiveFloat;
///
/// assert_eq!(
///     nonzero_primitive_floats_increasing::<f32>().take(20).map(NiceFloat).collect_vec(),
///     &[
///         f32::NEGATIVE_INFINITY, -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38,
///         -3.4028227e38, -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38,
///         -3.4028214e38, -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38,
///         -3.4028202e38, -3.40282e38, -3.4028198e38
///     ]
/// );
///
/// let mut end = nonzero_primitive_floats_increasing::<f32>().rev().take(20).map(NiceFloat)
///         .collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         3.4028198e38, 3.40282e38, 3.4028202e38, 3.4028204e38, 3.4028206e38, 3.4028208e38,
///         3.402821e38, 3.4028212e38, 3.4028214e38, 3.4028216e38, 3.4028218e38, 3.402822e38,
///         3.4028222e38, 3.4028225e38, 3.4028227e38, 3.4028229e38, 3.402823e38, 3.4028233e38,
///         3.4028235e38, f32::POSITIVE_INFINITY
///     ]
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
/// `T::NEGATIVE_INFINITY` is generated first and `T::POSITIVE_INFINITY` is generated last. The
/// returned iterator is double-ended, so it may be reversed.
///
/// Let $\varphi$ be `to_ordered_representation`:
///
/// The output is $(\varphi^{-1}(k))_{k=0}^{2^{M+1}(2^E-1)+1}$.
///
/// The output length is $2^{M+1}(2^E-1)+2$.
/// - For `f32`, this is $2^{32}-2^{24}+2$, or 4278190082.
/// - For `f64`, this is $2^{64}-2^{53}+2$, or 18437736874454810626.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::exhaustive::primitive_floats_increasing;
/// use malachite_base::num::float::nice_float::NiceFloat;
/// use malachite_base::num::float::PrimitiveFloat;
///
/// assert_eq!(
///     primitive_floats_increasing::<f32>().take(20).map(NiceFloat).collect_vec(),
///     &[
///         f32::NEGATIVE_INFINITY, -3.4028235e38, -3.4028233e38, -3.402823e38, -3.4028229e38,
///         -3.4028227e38, -3.4028225e38, -3.4028222e38, -3.402822e38, -3.4028218e38, -3.4028216e38,
///         -3.4028214e38, -3.4028212e38, -3.402821e38, -3.4028208e38, -3.4028206e38, -3.4028204e38,
///         -3.4028202e38, -3.40282e38, -3.4028198e38
///     ]
/// );
///
/// let mut end = primitive_floats_increasing::<f32>().rev().take(20).map(NiceFloat).collect_vec();
/// end.reverse();
/// assert_eq!(
///     end,
///     &[
///         3.4028198e38, 3.40282e38, 3.4028202e38, 3.4028204e38, 3.4028206e38, 3.4028208e38,
///         3.402821e38, 3.4028212e38, 3.4028214e38, 3.4028216e38, 3.4028218e38, 3.402822e38,
///         3.4028222e38, 3.4028225e38, 3.4028227e38, 3.4028229e38, 3.402823e38, 3.4028233e38,
///         3.4028235e38, f32::POSITIVE_INFINITY
///     ]
/// );
/// ```
pub fn primitive_floats_increasing<T: PrimitiveFloat>() -> PrimitiveFloatIncreasingRange<T> {
    primitive_float_increasing_inclusive_range(T::NEGATIVE_INFINITY, T::POSITIVE_INFINITY)
}
