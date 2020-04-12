use num::arithmetic::traits::CheckedNeg;

macro_rules! impl_checked_neg {
    ($t:ident) => {
        impl CheckedNeg for $t {
            type Output = $t;

            #[inline]
            fn checked_neg(self) -> Option<$t> {
                $t::checked_neg(self)
            }
        }
    };
}

impl_checked_neg!(u8);
impl_checked_neg!(u16);
impl_checked_neg!(u32);
impl_checked_neg!(u64);
impl_checked_neg!(u128);
impl_checked_neg!(usize);
impl_checked_neg!(i8);
impl_checked_neg!(i16);
impl_checked_neg!(i32);
impl_checked_neg!(i64);
impl_checked_neg!(i128);
impl_checked_neg!(isize);
