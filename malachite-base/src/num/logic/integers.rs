use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;
use num::logic::traits::{
    BitAccess, BitConvertible, CountOnes, CountZeros, LeadingZeros, NotAssign, Rotate,
    TrailingZeros,
};

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_logic_traits {
    ($t:ident) => {
        impl CountZeros for $t {
            #[inline]
            fn count_zeros(self) -> u64 {
                u64::from($t::count_zeros(self))
            }
        }

        impl CountOnes for $t {
            #[inline]
            fn count_ones(self) -> u64 {
                u64::from($t::count_ones(self))
            }
        }

        impl LeadingZeros for $t {
            #[inline]
            fn leading_zeros(self) -> u64 {
                u64::from($t::leading_zeros(self))
            }
        }

        impl TrailingZeros for $t {
            #[inline]
            fn trailing_zeros(self) -> u64 {
                u64::from($t::trailing_zeros(self))
            }
        }

        impl Rotate for $t {
            /// Rotate a value `n` bits to the left. Bits that leave the value from the left come
            /// back from the right.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::Rotate;
            ///
            /// assert_eq!(Rotate::rotate_left(123u8, 0), 123);
            /// assert_eq!(Rotate::rotate_left(123u8, 5), 111);
            /// assert_eq!(Rotate::rotate_left(123u8, 1_005), 111);
            /// ```
            #[inline]
            fn rotate_left(self, n: u64) -> $t {
                $t::rotate_left(self, u32::wrapping_from(n))
            }

            /// Rotate a value `n` bits to the right. Bits that leave the value from the right come
            /// back from the left.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::Rotate;
            ///
            /// assert_eq!(Rotate::rotate_right(123u8, 0), 123);
            /// assert_eq!(Rotate::rotate_right(123u8, 3), 111);
            /// assert_eq!(Rotate::rotate_right(123u8, 1_003), 111);
            /// ```
            #[inline]
            fn rotate_right(self, n: u64) -> $t {
                $t::rotate_right(self, u32::wrapping_from(n))
            }
        }

        impl NotAssign for $t {
            /// Replace a number with its bitwise negation.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::NotAssign;
            ///
            /// let mut x = 123u16;
            /// x.not_assign();
            /// assert_eq!(x, 65_412);
            /// ```
            #[inline]
            fn not_assign(&mut self) {
                *self = !*self;
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

pub fn _get_bits_naive<T: BitAccess, U: BitAccess + Zero>(n: &T, start: u64, end: u64) -> U {
    let mut result = U::ZERO;
    for i in start..end {
        if n.get_bit(i) {
            result.set_bit(i - start);
        }
    }
    result
}

pub fn _assign_bits_naive<T: BitAccess, U: BitAccess>(n: &mut T, start: u64, end: u64, bits: &U) {
    for i in start..end {
        n.assign_bit(i, bits.get_bit(i - start));
    }
}

pub fn _to_bits_asc_alt<T: BitConvertible>(n: &T) -> Vec<bool> {
    let mut bits = n.to_bits_desc();
    bits.reverse();
    bits
}

pub fn _to_bits_desc_alt<T: BitConvertible>(n: &T) -> Vec<bool> {
    let mut bits = n.to_bits_asc();
    bits.reverse();
    bits
}

pub fn _from_bits_asc_alt<T: BitConvertible>(bits: &[bool]) -> T {
    let mut bits = bits.to_vec();
    bits.reverse();
    T::from_bits_desc(&bits)
}

pub fn _from_bits_desc_alt<T: BitConvertible>(bits: &[bool]) -> T {
    let mut bits = bits.to_vec();
    bits.reverse();
    T::from_bits_asc(&bits)
}
