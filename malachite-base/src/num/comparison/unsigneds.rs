use std::cmp::Ordering;

use num::arithmetic::traits::UnsignedAbs;
use num::comparison::traits::OrdAbs;

macro_rules! impl_comparison_traits {
    ($t:ident) => {
        impl OrdAbs for $t {
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.unsigned_abs().cmp(&other.unsigned_abs())
            }
        }
    };
}

impl_comparison_traits!(i8);
impl_comparison_traits!(i16);
impl_comparison_traits!(i32);
impl_comparison_traits!(i64);
impl_comparison_traits!(i128);
impl_comparison_traits!(isize);
