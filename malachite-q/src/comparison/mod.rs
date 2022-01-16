pub mod cmp;
pub mod cmp_abs;
pub mod partial_cmp_abs_integer;
pub mod partial_cmp_abs_natural;
/// This module provides trait implementations for comparing the absolute value of a `Rational` to
/// the absolute value of a primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Rational.partial_cmp_abs(&PrimitiveInt)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_q::Rational;
/// use std::cmp::Ordering;
/// use std::str::FromStr;
///
/// assert!(Rational::from_str("22/7").unwrap().gt_abs(&3u32));
/// assert!(Rational::from_str("22/7").unwrap().lt_abs(&4u32));
/// assert!(Rational::from_str("-22/7").unwrap().gt_abs(&3u32));
/// assert!(Rational::from_str("-22/7").unwrap().lt_abs(&4u32));
///
/// assert!(Rational::from_str("22/7").unwrap().gt_abs(&3i32));
/// assert!(Rational::from_str("22/7").unwrap().lt_abs(&4i32));
/// assert!(Rational::from_str("-22/7").unwrap().gt_abs(&-3i32));
/// assert!(Rational::from_str("-22/7").unwrap().lt_abs(&-4i32));
/// ```
///
/// # PrimitiveInt.partial_cmp_abs(&Rational)
/// ```
/// extern crate malachite_base;
/// extern crate malachite_q;
///
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_q::Rational;
/// use std::cmp::Ordering;
/// use std::str::FromStr;
///
/// assert!(3u32.lt_abs(&Rational::from_str("22/7").unwrap()));
/// assert!(4u32.gt_abs(&Rational::from_str("22/7").unwrap()));
/// assert!(3u32.lt_abs(&Rational::from_str("-22/7").unwrap()));
/// assert!(4u32.gt_abs(&Rational::from_str("-22/7").unwrap()));
///
/// assert!(3i32.lt_abs(&Rational::from_str("22/7").unwrap()));
/// assert!(4i32.gt_abs(&Rational::from_str("22/7").unwrap()));
/// assert!((-3i32).lt_abs(&Rational::from_str("-22/7").unwrap()));
/// assert!((-4i32).gt_abs(&Rational::from_str("-22/7").unwrap()));
/// ```
pub mod partial_cmp_abs_primitive_int;
pub mod partial_cmp_integer;
pub mod partial_cmp_natural;
/// This module provides trait implementations for comparing a `Rational` to a primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Rational.partial_cmp(&PrimitiveInt)
/// ```
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert!(Rational::from_str("22/7").unwrap() > 3u32);
/// assert!(Rational::from_str("22/7").unwrap() < 4u32);
/// assert!(Rational::from_str("-22/7").unwrap() < 3u32);
/// assert!(Rational::from_str("-22/7").unwrap() < 4u32);
///
/// assert!(Rational::from_str("22/7").unwrap() > 3i32);
/// assert!(Rational::from_str("22/7").unwrap() < 4i32);
/// assert!(Rational::from_str("-22/7").unwrap() < -3i32);
/// assert!(Rational::from_str("-22/7").unwrap() > -4i32);
/// ```
///
/// # PrimitiveInt.partial_cmp(&Rational)
/// ```
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert!(3u32 < Rational::from_str("22/7").unwrap());
/// assert!(4u32 > Rational::from_str("22/7").unwrap());
/// assert!(3u32 > Rational::from_str("-22/7").unwrap());
/// assert!(4u32 > Rational::from_str("-22/7").unwrap());
///
/// assert!(3i32 < Rational::from_str("22/7").unwrap());
/// assert!(4i32 > Rational::from_str("22/7").unwrap());
/// assert!(-3i32 > Rational::from_str("-22/7").unwrap());
/// assert!(-4i32 < Rational::from_str("-22/7").unwrap());
/// ```
pub mod partial_cmp_primitive_int;
pub mod partial_eq_integer;
pub mod partial_eq_natural;
/// This module provides trait implementations for comparing the equality of a `Rational` and a
/// primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Rational == PrimitiveInt
/// ```
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert!(Rational::from(123) == 123u64);
/// assert!(Rational::from(-123) != 123u64);
/// assert!(Rational::from_str("22/7").unwrap() != 123u64);
/// assert!(Rational::from_str("-22/7").unwrap() != 123u64);
///
/// assert!(Rational::from(123) == 123i64);
/// assert!(Rational::from(-123) == -123i64);
/// assert!(Rational::from_str("22/7").unwrap() != -123i64);
/// assert!(Rational::from_str("-22/7").unwrap() != -123i64);
///
/// ```
///
/// # PrimitiveInt == Rational
/// ```
/// use malachite_q::Rational;
/// use std::str::FromStr;
///
/// assert!(123u64 == Rational::from(123));
/// assert!(123u64 != Rational::from(-123));
/// assert!(123u64 != Rational::from_str("22/7").unwrap());
/// assert!(123u64 != Rational::from_str("-22/7").unwrap());
///
/// assert!(123i64 == Rational::from(123));
/// assert!(-123i64 == Rational::from(-123));
/// assert!(-123i64 != Rational::from_str("22/7").unwrap());
/// assert!(-123i64 != Rational::from_str("-22/7").unwrap());
/// ```
pub mod partial_eq_primitive_int;
