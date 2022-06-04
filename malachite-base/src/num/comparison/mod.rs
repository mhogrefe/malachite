/// [`PartialOrdAbs`](`traits::PartialOrdAbs`) and [`OrdAbs`](`traits::OrdAbs`), traits for
/// comparing the absolute values of numbers by order.
///
/// # partial_cmp_abs
/// ```
/// use malachite_base::num::comparison::traits::PartialOrdAbs;
/// use std::cmp::Ordering;
///
/// assert_eq!(123i32.partial_cmp_abs(&-456), Some(Ordering::Less));
/// assert_eq!(123i32.partial_cmp_abs(&-123), Some(Ordering::Equal));
/// ```
///
/// # cmp_abs
/// ```
/// use malachite_base::num::comparison::traits::OrdAbs;
/// use std::cmp::Ordering;
///
/// assert_eq!(123u32.cmp_abs(&456), Ordering::Less);
/// assert_eq!(123u32.cmp_abs(&123), Ordering::Equal);
/// assert_eq!(123i32.cmp_abs(&-456), Ordering::Less);
/// assert_eq!(123i32.cmp_abs(&-123), Ordering::Equal);
/// ```
pub mod cmp_abs;
/// [`EqAbs`](`traits::EqAbs`), a trait for comparing the absolute values of numbers by equality.
///
/// # eq_abs
/// ```
/// use malachite_base::num::comparison::traits::EqAbs;
///
/// assert_eq!(123u32.eq_abs(&456), false);
/// assert_eq!(123u32.eq_abs(&123), true);
/// assert_eq!(123i32.eq_abs(&-456), false);
/// assert_eq!(123i32.eq_abs(&-123), true);
/// ```
pub mod eq_abs;
/// Various traits for comparing numbers.
pub mod traits;
