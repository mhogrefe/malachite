use num::arithmetic::traits::DivisibleBy;

//TODO

macro_rules! impl_divisible_by_unsigned {
    ($t:ident) => {
        impl DivisibleBy for $t {
            fn divisible_by(self, other: $t) -> bool {
                self == 0 || other != 0 && self % other == 0
            }
        }
    };
}
impl_divisible_by_unsigned!(u8);
impl_divisible_by_unsigned!(u16);
impl_divisible_by_unsigned!(u32);
impl_divisible_by_unsigned!(u64);
impl_divisible_by_unsigned!(u128);
impl_divisible_by_unsigned!(usize);

macro_rules! impl_divisible_by_signed {
    ($t:ident) => {
        impl DivisibleBy for $t {
            fn divisible_by(self, other: $t) -> bool {
                self == 0 || self == $t::MIN && other == -1 || other != 0 && self % other == 0
            }
        }
    };
}
impl_divisible_by_signed!(i8);
impl_divisible_by_signed!(i16);
impl_divisible_by_signed!(i32);
impl_divisible_by_signed!(i64);
impl_divisible_by_signed!(i128);
impl_divisible_by_signed!(isize);
