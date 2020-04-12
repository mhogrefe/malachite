use num::arithmetic::traits::CheckedAbs;

macro_rules! impl_checked_abs {
    ($t:ident) => {
        impl CheckedAbs for $t {
            type Output = $t;

            #[inline]
            fn checked_abs(self) -> Option<$t> {
                $t::checked_abs(self)
            }
        }
    };
}

impl_checked_abs!(i8);
impl_checked_abs!(i16);
impl_checked_abs!(i32);
impl_checked_abs!(i64);
impl_checked_abs!(i128);
impl_checked_abs!(isize);
