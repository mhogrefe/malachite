use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;

use integer::Integer;
use natural::Natural;

fn _partial_cmp_unsigned<T>(x: &Integer, other: &T) -> Option<Ordering>
where
    Natural: PartialOrd<T>,
{
    if x.sign {
        x.abs.partial_cmp(other)
    } else {
        Some(Ordering::Less)
    }
}

macro_rules! impl_unsigned {
    ($t: ident) => {
        impl PartialOrd<$t> for Integer {
            /// Compares an `Integer` to a value of unsigned primitive integer type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
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
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                _partial_cmp_unsigned(self, other)
            }
        }

        impl PartialOrd<Integer> for $t {
            /// Compares a value of unsigned primitive integer type to an `Integer`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
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
            /// ```
            #[inline]
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_unsigned);

fn _partial_cmp_signed<U: PartialOrd<Natural>, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &Integer,
    other: &S,
) -> Option<Ordering>
where
    Natural: PartialOrd<U>,
{
    if x.sign {
        if *other >= S::ZERO {
            x.abs.partial_cmp(&other.unsigned_abs())
        } else {
            Some(Ordering::Greater)
        }
    } else if *other >= S::ZERO {
        Some(Ordering::Less)
    } else {
        other.unsigned_abs().partial_cmp(&x.abs)
    }
}

macro_rules! impl_signed {
    ($t: ident) => {
        impl PartialOrd<$t> for Integer {
            /// Compares an `Integer` to a value of signed primitive integer type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
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
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                _partial_cmp_signed(self, other)
            }
        }

        impl PartialOrd<Integer> for $t {
            /// Compares a value of signed primitive integer type to an `Integer`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_nz::integer::Integer;
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
            #[inline]
            fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_signed);
