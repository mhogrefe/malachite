use std::cmp::Ordering;

use num::arithmetic::traits::UnsignedAbs;
use num::comparison::traits::OrdAbs;

macro_rules! impl_comparison_traits {
    ($t:ident, $width:expr) => {
        impl OrdAbs for $t {
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.unsigned_abs().cmp(&other.unsigned_abs())
            }
        }
    };
}

impl_comparison_traits!(i8, 8);
impl_comparison_traits!(i16, 16);
impl_comparison_traits!(i32, 32);
impl_comparison_traits!(i64, 64);
impl_comparison_traits!(i128, 128);
impl_comparison_traits!(isize, 0usize.trailing_zeros());
