use num::arithmetic::traits::CheckedAdd;

macro_rules! impl_checked_add {
    ($t:ident) => {
        impl CheckedAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_add(self, rhs: $t) -> Option<$t> {
                $t::checked_add(self, rhs)
            }
        }
    };
}

impl_checked_add!(u8);
impl_checked_add!(u16);
impl_checked_add!(u32);
impl_checked_add!(u64);
impl_checked_add!(u128);
impl_checked_add!(usize);
impl_checked_add!(i8);
impl_checked_add!(i16);
impl_checked_add!(i32);
impl_checked_add!(i64);
impl_checked_add!(i128);
impl_checked_add!(isize);
