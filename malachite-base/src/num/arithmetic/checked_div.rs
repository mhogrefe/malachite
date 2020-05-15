use num::arithmetic::traits::CheckedDiv;

macro_rules! impl_checked_div {
    ($t:ident) => {
        impl CheckedDiv<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_div(self, rhs: $t) -> Option<$t> {
                $t::checked_div(self, rhs)
            }
        }
    };
}

impl_checked_div!(u8);
impl_checked_div!(u16);
impl_checked_div!(u32);
impl_checked_div!(u64);
impl_checked_div!(u128);
impl_checked_div!(usize);
impl_checked_div!(i8);
impl_checked_div!(i16);
impl_checked_div!(i32);
impl_checked_div!(i64);
impl_checked_div!(i128);
impl_checked_div!(isize);
