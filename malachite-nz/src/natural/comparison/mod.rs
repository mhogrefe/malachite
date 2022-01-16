pub mod cmp;
/// This module provides trait implementations for comparing the absolute values of a `Natural`
/// and a primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Natural.cmp_abs(&PrimitiveInt)
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(122u32).lt_abs(&123u64));
/// assert!(Natural::from(122u32).le_abs(&123u64));
/// assert!(Natural::from(123u32).lt_abs(&124u64));
/// assert!(Natural::from(123u32).le_abs(&124u64));
/// assert!(Natural::trillion().gt_abs(&123u64));
/// assert!(Natural::trillion().ge_abs(&123u64));
///
/// assert!(Natural::from(122u32).lt_abs(&-123i64));
/// assert!(Natural::from(122u32).le_abs(&-123i64));
/// assert!(Natural::from(124u32).gt_abs(&-123i64));
/// assert!(Natural::from(124u32).ge_abs(&-123i64));
/// assert!(Natural::trillion().gt_abs(&123i64));
/// assert!(Natural::trillion().ge_abs(&123i64));
/// ```
///
/// # PrimitiveInt.cmp_abs(&Natural)
/// ```
/// extern crate malachite_base;
///
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use malachite_nz::natural::Natural;
///
/// assert!(123u64.gt_abs(&Natural::from(122u32)));
/// assert!(123u64.ge_abs(&Natural::from(122u32)));
/// assert!(124u64.gt_abs(&Natural::from(123u32)));
/// assert!(124u64.ge_abs(&Natural::from(123u32)));
/// assert!(123u64.lt_abs(&Natural::trillion()));
/// assert!(123u64.le_abs(&Natural::trillion()));
///
/// assert!((-123i64).gt_abs(&Natural::from(122u32)));
/// assert!((-123i64).ge_abs(&Natural::from(122u32)));
/// assert!((-123i64).lt_abs(&Natural::from(124u32)));
/// assert!((-123i64).le_abs(&Natural::from(124u32)));
/// assert!(123i64.lt_abs(&Natural::trillion()));
/// assert!(123i64.le_abs(&Natural::trillion()));
/// ```
pub mod partial_cmp_abs_primitive_int;
/// This module provides trait implementations for comparing a `Natural` to a primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Natural.cmp(&PrimitiveInt)
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) > 122u32);
/// assert!(Natural::from(123u32) >= 122u32);
/// assert!(Natural::from(123u32) < 124u32);
/// assert!(Natural::from(123u32) <= 124u32);
/// assert!(Natural::trillion() > 123u32);
/// assert!(Natural::trillion() >= 123u32);
///
/// assert!(Natural::from(123u32) > 122u8);
/// assert!(Natural::from(123u32) >= 122u8);
/// assert!(Natural::from(123u32) < 124u8);
/// assert!(Natural::from(123u32) <= 124u8);
/// assert!(Natural::trillion() > 123u8);
/// assert!(Natural::trillion() >= 123u8);
///
/// assert!(Natural::from(123u32) > 122u64);
/// assert!(Natural::from(123u32) >= 122u64);
/// assert!(Natural::from(123u32) < 124u64);
/// assert!(Natural::from(123u32) <= 124u64);
/// assert!(Natural::trillion() > 123u64);
/// assert!(Natural::trillion() >= 123u64);
///
/// assert!(Natural::from(123u32) > 122i64);
/// assert!(Natural::from(123u32) >= 122i64);
/// assert!(Natural::from(123u32) < 124i64);
/// assert!(Natural::from(123u32) <= 124i64);
/// assert!(Natural::from(123u32) > -124i64);
/// assert!(Natural::from(123u32) >= -124i64);
/// assert!(Natural::trillion() > 123i64);
/// assert!(Natural::trillion() >= 123i64);
/// ```
///
/// # PrimitiveInt.cmp(&Natural)
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(122u32 < Natural::from(123u32));
/// assert!(122u32 <= Natural::from(123u32));
/// assert!(124u32 > Natural::from(123u32));
/// assert!(124u32 >= Natural::from(123u32));
/// assert!(123u32 < Natural::trillion());
/// assert!(123u32 <= Natural::trillion());
///
/// assert!(122u8 < Natural::from(123u32));
/// assert!(122u8 <= Natural::from(123u32));
/// assert!(124u8 > Natural::from(123u32));
/// assert!(124u8 >= Natural::from(123u32));
/// assert!(123u8 < Natural::trillion());
/// assert!(123u8 <= Natural::trillion());
///
/// assert!(122u64 < Natural::from(123u32));
/// assert!(122u64 <= Natural::from(123u32));
/// assert!(124u64 > Natural::from(123u32));
/// assert!(124u64 >= Natural::from(123u32));
/// assert!(123u64 < Natural::trillion());
/// assert!(123u64 <= Natural::trillion());
///
/// assert!(122i64 < Natural::from(123u32));
/// assert!(122i64 <= Natural::from(123u32));
/// assert!(124i64 > Natural::from(123u32));
/// assert!(124i64 >= Natural::from(123u32));
/// assert!(-124i64 < Natural::from(123u32));
/// assert!(-124i64 <= Natural::from(123u32));
/// assert!(123i64 < Natural::trillion());
/// assert!(123i64 <= Natural::trillion());
/// ```
pub mod partial_cmp_primitive_int;
/// This module provides trait implementations for comparing the equality of a `Natural` and a
/// primitive integer.
///
/// Here are usage examples of the macro-generated functions:
///
/// # Natural == PrimitiveInt
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) == 123u32);
/// assert!(Natural::from(123u32) != 5u32);
///
/// assert!(123u32 == Natural::from(123u32));
/// assert!(5u32 != Natural::from(123u32));
///
/// assert!(Natural::from(123u32) == 123u64);
/// assert!(Natural::from(123u32) != 5u64);
///
/// assert!(Natural::from(123u32) == 123i64);
/// assert!(Natural::from(123u32) != -123i64);
/// ```
///
/// # PrimitiveInt == Natural
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) == 123u8);
/// assert!(Natural::from(123u32) != 5u8);
///
/// assert!(123u8 == Natural::from(123u32));
/// assert!(5u8 != Natural::from(123u32));
///
/// assert!(123u64 == Natural::from(123u32));
/// assert!(5u64 != Natural::from(123u32));
///
/// assert!(123i64 == Natural::from(123u32));
/// assert!(-123i64 != Natural::from(123u32));
/// ```
pub mod partial_eq_primitive_int;
