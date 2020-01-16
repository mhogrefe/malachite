use num::logic::traits::{
    CountOnes, CountZeros, HammingDistance, LeadingZeros, NotAssign, RotateLeft, RotateRight,
    TrailingZeros,
};

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_logic_traits {
    ($t:ident) => {
        impl CountZeros for $t {
            #[inline]
            fn count_zeros(self) -> u32 {
                $t::count_zeros(self)
            }
        }

        impl CountOnes for $t {
            #[inline]
            fn count_ones(self) -> u32 {
                $t::count_ones(self)
            }
        }

        impl LeadingZeros for $t {
            #[inline]
            fn leading_zeros(self) -> u32 {
                $t::leading_zeros(self)
            }
        }

        impl TrailingZeros for $t {
            #[inline]
            fn trailing_zeros(self) -> u32 {
                $t::trailing_zeros(self)
            }
        }

        impl RotateLeft for $t {
            #[inline]
            fn rotate_left(self, n: u32) -> $t {
                $t::rotate_left(self, n)
            }
        }

        impl RotateRight for $t {
            #[inline]
            fn rotate_right(self, n: u32) -> $t {
                $t::rotate_right(self, n)
            }
        }

        //TODO docs, test
        impl NotAssign for $t {
            #[inline]
            fn not_assign(&mut self) {
                *self = !*self;
            }
        }

        impl HammingDistance<$t> for $t {
            #[inline]
            fn hamming_distance(self, other: $t) -> u64 {
                u64::from((self ^ other).count_ones())
            }
        }
    };
}

impl_logic_traits!(u8);
impl_logic_traits!(u16);
impl_logic_traits!(u32);
impl_logic_traits!(u64);
impl_logic_traits!(u128);
impl_logic_traits!(usize);
impl_logic_traits!(i8);
impl_logic_traits!(i16);
impl_logic_traits!(i32);
impl_logic_traits!(i64);
impl_logic_traits!(i128);
impl_logic_traits!(isize);
