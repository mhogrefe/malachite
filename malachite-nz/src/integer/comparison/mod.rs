pub mod cmp;
pub mod cmp_abs;
pub mod partial_cmp_abs_natural;
/// This module provides trait implementations for comparing the absolute values of an `Integer`
/// and a primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Integer.cmp_abs(&PrimitiveInt)
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(-122).lt_abs(&123u64));
/// assert!(Integer::from(-122).le_abs(&123u64));
/// assert!(Integer::from(-123).lt_abs(&124u64));
/// assert!(Integer::from(-123).le_abs(&124u64));
/// assert!(Integer::trillion().gt_abs(&123u64));
/// assert!(Integer::trillion().ge_abs(&123u64));
/// assert!((-Integer::trillion()).gt_abs(&123u64));
/// assert!((-Integer::trillion()).ge_abs(&123u64));
///
/// assert!(Integer::from(-122).lt_abs(&-123i64));
/// assert!(Integer::from(-122).le_abs(&-123i64));
/// assert!(Integer::from(-124).gt_abs(&-123i64));
/// assert!(Integer::from(-124).ge_abs(&-123i64));
/// assert!(Integer::trillion().gt_abs(&123i64));
/// assert!(Integer::trillion().ge_abs(&123i64));
/// assert!((-Integer::trillion()).gt_abs(&123i64));
/// assert!((-Integer::trillion()).ge_abs(&123i64));
/// ```
///
/// # PrimitiveInt.cmp_abs(&Integer)
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::integer::Integer;
///
/// assert!(123u64.gt_abs(&Integer::from(-122)));
/// assert!(123u64.ge_abs(&Integer::from(-122)));
/// assert!(124u64.gt_abs(&Integer::from(-123)));
/// assert!(124u64.ge_abs(&Integer::from(-123)));
/// assert!(123u64.lt_abs(&Integer::trillion()));
/// assert!(123u64.le_abs(&Integer::trillion()));
/// assert!(123u64.lt_abs(&-Integer::trillion()));
/// assert!(123u64.le_abs(&-Integer::trillion()));
///
/// assert!((-123i64).gt_abs(&Integer::from(-122)));
/// assert!((-123i64).ge_abs(&Integer::from(-122)));
/// assert!((-123i64).lt_abs(&Integer::from(-124)));
/// assert!((-123i64).le_abs(&Integer::from(-124)));
/// assert!(123i64.lt_abs(&Integer::trillion()));
/// assert!(123i64.le_abs(&Integer::trillion()));
/// assert!(123i64.lt_abs(&-Integer::trillion()));
/// assert!(123i64.le_abs(&-Integer::trillion()));
/// ```
pub mod partial_cmp_abs_primitive_int;
pub mod partial_cmp_natural;
/// This module provides trait implementations for comparing an `Integer` to a primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Integer.cmp(&PrimitiveInt)
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(-123) < 122u64);
/// assert!(Integer::from(-123) <= 122u64);
/// assert!(Integer::from(-123) < 124u64);
/// assert!(Integer::from(-123) <= 124u64);
/// assert!(Integer::trillion() > 123u64);
/// assert!(Integer::trillion() >= 123u64);
/// assert!(-Integer::trillion() < 123u64);
/// assert!(-Integer::trillion() <= 123u64);
///
/// assert!(Integer::from(-123) < -122i64);
/// assert!(Integer::from(-123) <= -122i64);
/// assert!(Integer::from(-123) > -124i64);
/// assert!(Integer::from(-123) >= -124i64);
/// assert!(Integer::trillion() > 123i64);
/// assert!(Integer::trillion() >= 123i64);
/// assert!(-Integer::trillion() < 123i64);
/// assert!(-Integer::trillion() <= 123i64);
/// ```
///
/// # PrimitiveInt.cmp(&Integer)
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(122u64 > Integer::from(-123));
/// assert!(122u64 >= Integer::from(-123));
/// assert!(124u64 > Integer::from(-123));
/// assert!(124u64 >= Integer::from(-123));
/// assert!(123u64 < Integer::trillion());
/// assert!(123u64 <= Integer::trillion());
/// assert!(123u64 > -Integer::trillion());
/// assert!(123u64 >= -Integer::trillion());
///
/// assert!(-122i64 > Integer::from(-123));
/// assert!(-122i64 >= Integer::from(-123));
/// assert!(-124i64 < Integer::from(-123));
/// assert!(-124i64 <= Integer::from(-123));
/// assert!(123i64 < Integer::trillion());
/// assert!(123i64 <= Integer::trillion());
/// assert!(123i64 > -Integer::trillion());
/// assert!(123i64 >= -Integer::trillion());
/// ```
pub mod partial_cmp_primitive_int;
pub mod partial_eq_natural;
/// This module provides trait implementations for comparing the equality of an `Integer` and a
/// primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Integer == PrimitiveInt
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(123) == 123u64);
/// assert!(Integer::from(-123) != 123u64);
///
/// assert!(123u64 == Integer::from(123));
/// assert!(123u64 != Integer::from(-123));
/// ```
///
/// # PrimitiveInt == Integer
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(123u64 == Integer::from(123));
/// assert!(123u64 != Integer::from(-123));
///
/// assert!(23i64 != Integer::from(123));
/// assert!(-123i64 == Integer::from(-123));
/// ```
pub mod partial_eq_primitive_int;
