use std::cmp::Ordering;

use num::comparison::traits::OrdAbs;

macro_rules! impl_comparison_traits {
    ($t:ident) => {
        impl OrdAbs for $t {
            #[inline]
            fn cmp_abs(&self, other: &Self) -> Ordering {
                self.cmp(other)
            }
        }
    };
}

impl_comparison_traits!(u8);
impl_comparison_traits!(u16);
impl_comparison_traits!(u32);
impl_comparison_traits!(u64);
impl_comparison_traits!(u128);
impl_comparison_traits!(usize);
