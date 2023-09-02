/// The [`PrimitiveFloat`](floats::PrimitiveFloat) trait.
pub mod floats;
/// The [`PrimitiveInt`](integers::PrimitiveInt) trait.
///
/// ```
/// use malachite_base::comparison::traits::{Max, Min};
/// use malachite_base::num::basic::integers::PrimitiveInt;
/// use malachite_base::num::basic::traits::{One, OneHalf, Two, Zero};
///
/// assert_eq!(u32::WIDTH, 32);
/// assert_eq!(u32::LOG_WIDTH, 5);
/// assert_eq!(u32::WIDTH_MASK, 0x1f);
///
/// assert_eq!(u32::ZERO, 0);
/// assert_eq!(u32::ONE, 1);
/// assert_eq!(i16::TWO, 2);
///
/// assert_eq!(u32::MAX, 0xffffffff);
/// assert_eq!(u32::MIN, 0);
/// assert_eq!(i32::MAX, 0x7fffffff);
/// assert_eq!(i32::MIN, -0x80000000);
/// ```
pub mod integers;
/// The [`PrimitiveSigned`](signeds::PrimitiveSigned) trait.
///
/// ```
/// use malachite_base::num::basic::traits::NegativeOne;
///
/// assert_eq!(i16::NEGATIVE_ONE, -1);
/// ```
pub mod signeds;
/// Traits for constants.
pub mod traits;
/// The [`PrimitiveUnsigned`](unsigneds::PrimitiveUnsigned) trait.
pub mod unsigneds;
