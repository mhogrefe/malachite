use std::cmp::Ordering;

use num::comparison::traits::{OrdAbs, PartialOrdAbs};

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_comparison_traits {
    ($t:ident) => {
        impl PartialOrdAbs<$t> for $t {
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                Some(self.cmp_abs(other))
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
impl_comparison_traits!(i8);
impl_comparison_traits!(i16);
impl_comparison_traits!(i32);
impl_comparison_traits!(i64);
impl_comparison_traits!(i128);
impl_comparison_traits!(isize);
