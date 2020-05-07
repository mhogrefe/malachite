use std::ops::{Shr, ShrAssign};

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::round::RoundingMode;

use integer::Integer;

macro_rules! impl_integer_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for Integer {
            type Output = Integer;

            /// Shifts a `Integer` right (divides it by a power of 2 and takes the floor), taking
            /// the `Integer` by value.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!((Integer::ZERO >> 10u8).to_string(), "0");
            /// assert_eq!((Integer::from(492) >> 2u16).to_string(), "123");
            /// assert_eq!((-Integer::trillion() >> 10u32).to_string(), "-976562500");
            /// ```
            #[inline]
            fn shr(mut self, other: $t) -> Integer {
                self >>= other;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts a `Integer` right (divides it by a power of 2 and takes the floor), taking
            /// the `Integer` by reference.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(`other`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!((&Integer::ZERO >> 10u8).to_string(), "0");
            /// assert_eq!((&Integer::from(492) >> 2u16).to_string(), "123");
            /// assert_eq!((&(-Integer::trillion()) >> 10u64).to_string(), "-976562500");
            /// ```
            fn shr(self, other: $t) -> Integer {
                match *self {
                    Integer {
                        sign: true,
                        ref abs,
                    } => Integer {
                        sign: true,
                        abs: abs >> other,
                    },
                    Integer {
                        sign: false,
                        ref abs,
                    } => {
                        let abs_shifted = abs.shr_round(other, RoundingMode::Ceiling);
                        if abs_shifted == 0 {
                            Integer::ZERO
                        } else {
                            Integer {
                                sign: false,
                                abs: abs_shifted,
                            }
                        }
                    }
                }
            }
        }

        impl ShrAssign<$t> for Integer {
            /// Shifts a `Integer` right (divides it by a power of 2 and takes the floor) in place.
            ///
            /// Time: worst case O(`other`)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_nz::integer::Integer;
            ///
            /// let mut x = Integer::from(1024);
            /// x >>= 1u8;
            /// x >>= 2u16;
            /// x >>= 3u32;
            /// x >>= 4u64;
            /// assert_eq!(x.to_string(), "1");
            /// ```
            fn shr_assign(&mut self, other: $t) {
                match *self {
                    Integer {
                        sign: true,
                        ref mut abs,
                    } => {
                        *abs >>= other;
                    }
                    Integer {
                        sign: false,
                        ref mut abs,
                    } => {
                        abs.shr_round_assign(other, RoundingMode::Ceiling);
                        if *abs == 0 {
                            self.sign = true;
                        }
                    }
                }
            }
        }
    };
}
impl_integer_shr_unsigned!(u8);
impl_integer_shr_unsigned!(u16);
impl_integer_shr_unsigned!(u32);
impl_integer_shr_unsigned!(u64);
impl_integer_shr_unsigned!(u128);
impl_integer_shr_unsigned!(usize);
