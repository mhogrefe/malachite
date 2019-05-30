use std::cmp::Ordering;

use num::comparison::traits::{OrdAbs, PartialOrdAbs};

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_comparison_traits {
    ($t:ident, $width:expr) => {
        impl PartialOrdAbs<$t> for $t {
            #[inline]
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                Some(self.cmp_abs(other))
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
impl_comparison_traits!(i8, 8);
impl_comparison_traits!(i16, 16);
impl_comparison_traits!(i32, 32);
impl_comparison_traits!(i64, 64);
impl_comparison_traits!(i128, 128);
impl_comparison_traits!(isize, 0usize.trailing_zeros());
