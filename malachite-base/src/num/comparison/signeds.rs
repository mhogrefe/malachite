use std::cmp::Ordering;

use num::comparison::traits::OrdAbs;

macro_rules! impl_comparison_traits {
    ($t:ident, $width:expr) => {
        impl OrdAbs for $t {
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.cmp(other)
            }
        }
    };
}

impl_comparison_traits!(u8, 8);
impl_comparison_traits!(u16, 16);
impl_comparison_traits!(u32, 32);
impl_comparison_traits!(u64, 64);
impl_comparison_traits!(u128, 128);
impl_comparison_traits!(usize, 0usize.trailing_zeros());
