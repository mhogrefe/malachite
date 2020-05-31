use num::arithmetic::traits::CheckedMul;

macro_rules! impl_checked_mul {
    ($t:ident) => {
        impl CheckedMul<$t> for $t {
            type Output = $t;

            #[inline]
            fn checked_mul(self, other: $t) -> Option<$t> {
                $t::checked_mul(self, other)
            }
        }
    };
}

impl_checked_mul!(u8);
impl_checked_mul!(u16);
impl_checked_mul!(u32);
impl_checked_mul!(u64);
impl_checked_mul!(u128);
impl_checked_mul!(usize);
impl_checked_mul!(i8);
impl_checked_mul!(i16);
impl_checked_mul!(i32);
impl_checked_mul!(i64);
impl_checked_mul!(i128);
impl_checked_mul!(isize);
