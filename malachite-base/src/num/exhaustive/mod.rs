use std::iter::Rev;
use std::iter::{once, Chain, Once};

use itertools::{Interleave, Itertools};

use num::basic::integers::PrimitiveInt;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;

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
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_unsigneds;
///
/// assert_eq!(
///     exhaustive_unsigneds::<u8>().take(10).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_positive_primitive_ints;
///
/// assert_eq!(
///     exhaustive_positive_primitive_ints::<u8>().take(10).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_signeds;
///
/// assert_eq!(
///     exhaustive_signeds::<i8>().take(10).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_natural_signeds;
///
/// assert_eq!(
///     exhaustive_natural_signeds::<i8>().take(10).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_negative_signeds;
///
/// assert_eq!(
///     exhaustive_negative_signeds::<i8>().take(10).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_nonzero_signeds;
///
/// assert_eq!(
///     exhaustive_nonzero_signeds::<i8>().take(10).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::primitive_int_increasing_range;
///
/// assert_eq!(
///     primitive_int_increasing_range::<i8>(-5, 5).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::primitive_int_increasing_inclusive_range;
///
/// assert_eq!(
///     primitive_int_increasing_inclusive_range::<i8>(-5, 5).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_signed_range;
///
/// assert_eq!(
///     exhaustive_signed_range::<i8>(-5, 5).collect::<Vec<_>>(),
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
///
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `a` > `b`.
///
/// # Examples
/// ```
/// use malachite_base::num::exhaustive::exhaustive_signed_inclusive_range;
///
/// assert_eq!(
///     exhaustive_signed_inclusive_range::<i8>(-5, 5).collect::<Vec<_>>(),
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
