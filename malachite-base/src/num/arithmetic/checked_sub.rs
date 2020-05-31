use num::arithmetic::traits::CheckedSub;

macro_rules! impl_checked_sub {
    ($t:ident) => {
        impl CheckedSub<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_sub(self, other: $t) -> Option<$t> {
                $t::checked_sub(self, other)
            }
        }
    };
}

impl_checked_sub!(u8);
impl_checked_sub!(u16);
impl_checked_sub!(u32);
impl_checked_sub!(u64);
impl_checked_sub!(u128);
impl_checked_sub!(usize);
impl_checked_sub!(i8);
impl_checked_sub!(i16);
impl_checked_sub!(i32);
impl_checked_sub!(i64);
impl_checked_sub!(i128);
impl_checked_sub!(isize);
