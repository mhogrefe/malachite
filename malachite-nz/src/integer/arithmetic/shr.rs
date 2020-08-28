use integer::Integer;
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode;
use natural::Natural;
use std::ops::{Shr, ShrAssign};

fn _shr_unsigned_ref<'a, T>(x: &'a Integer, bits: T) -> Integer
where
    &'a Natural: Shr<T, Output = Natural> + ShrRound<T, Output = Natural>,
{
    match *x {
        Integer {
            sign: true,
            ref abs,
        } => Integer {
            sign: true,
            abs: abs >> bits,
        },
        Integer {
            sign: false,
            ref abs,
        } => {
            let abs_shifted = abs.shr_round(bits, RoundingMode::Ceiling);
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

fn _shr_assign_unsigned<T>(x: &mut Integer, bits: T)
where
    Natural: ShrAssign<T> + ShrRoundAssign<T>,
{
    match *x {
        Integer {
            sign: true,
            ref mut abs,
        } => {
            *abs >>= bits;
        }
        Integer {
            sign: false,
            ref mut abs,
        } => {
            abs.shr_round_assign(bits, RoundingMode::Ceiling);
            if *abs == 0 {
                x.sign = true;
            }
        }
    }
}

macro_rules! impl_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for Integer {
            type Output = Integer;

            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor), taking
            /// the `Integer` by value.
            ///
            /// Time: worst case O(`bits`)
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
            fn shr(mut self, bits: $t) -> Integer {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor), taking
            /// the `Integer` by reference.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
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
            #[inline]
            fn shr(self, bits: $t) -> Integer {
                _shr_unsigned_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Integer {
            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor) in place.
            ///
            /// Time: worst case O(`bits`)
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
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                _shr_assign_unsigned(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_shr_unsigned);

//TODO clean

macro_rules! impl_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for Integer {
            type Output = Integer;

            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking the `Integer` by value.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!((Integer::ZERO >> 10i8).to_string(), "0");
            /// assert_eq!((Integer::from(492) >> 2i16).to_string(), "123");
            /// assert_eq!((-Integer::trillion() >> 10i64).to_string(), "-976562500");
            /// assert_eq!((Integer::ZERO >> -10i8).to_string(), "0");
            /// assert_eq!((Integer::from(123) >> -2i16).to_string(), "492");
            /// assert_eq!(
            ///     (Integer::from(123) >> -100i32).to_string(),
            ///     "155921023828072216384094494261248"
            /// );
            /// assert_eq!((Integer::from(-123) >> -2i64).to_string(), "-492");
            /// assert_eq!(
            ///     (Integer::from(-123) >> -100i8).to_string(),
            ///     "-155921023828072216384094494261248"
            /// );
            /// ```
            #[inline]
            fn shr(mut self, bits: $t) -> Integer {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking the `Integer` by reference.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::Zero;
            /// use malachite_nz::integer::Integer;
            ///
            /// assert_eq!((&Integer::ZERO >> 10i8).to_string(), "0");
            /// assert_eq!((&Integer::from(492) >> 2i16).to_string(), "123");
            /// assert_eq!((&(-Integer::trillion()) >> 10i32).to_string(), "-976562500");
            /// assert_eq!((&Integer::ZERO >> -10i64).to_string(), "0");
            /// assert_eq!((&Integer::from(123) >> -2i8).to_string(), "492");
            /// assert_eq!(
            ///     (&Integer::from(123) >> -100i16).to_string(),
            ///     "155921023828072216384094494261248"
            /// );
            /// assert_eq!((&Integer::from(-123) >> -2i32).to_string(), "-492");
            /// assert_eq!(
            ///     (&Integer::from(-123) >> -100i64).to_string(),
            ///     "-155921023828072216384094494261248"
            /// );
            /// ```
            fn shr(self, bits: $t) -> Integer {
                if bits >= 0 {
                    self >> bits.unsigned_abs()
                } else {
                    self << bits.unsigned_abs()
                }
            }
        }

        impl ShrAssign<$t> for Integer {
            /// Shifts an `Integer` right (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2) in place.
            ///
            /// Time: worst case O(`bits`)
            ///
            /// Additional memory: worst case O(`bits`)
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use malachite_base::num::basic::traits::{NegativeOne, One};
            /// use malachite_nz::integer::Integer;
            ///
            ///     let mut x = Integer::from(1024);
            ///     x >>= 1i8;
            ///     x >>= 2i16;
            ///     x >>= 3i32;
            ///     x >>= 4i64;
            ///     assert_eq!(x.to_string(), "1");
            ///
            ///     let mut x = Integer::ONE;
            ///     x >>= -1i8;
            ///     x >>= -2i16;
            ///     x >>= -3i32;
            ///     x >>= -4i64;
            ///     assert_eq!(x.to_string(), "1024");
            ///     let mut x = Integer::NEGATIVE_ONE;
            ///     x >>= -1i8;
            ///     x >>= -2i16;
            ///     x >>= -3i32;
            ///     x >>= -4i64;
            ///     assert_eq!(x.to_string(), "-1024");
            /// ```
            fn shr_assign(&mut self, bits: $t) {
                if bits >= 0 {
                    *self >>= bits.unsigned_abs();
                } else {
                    *self <<= bits.unsigned_abs();
                }
            }
        }
    };
}
apply_to_signeds!(impl_shr_signed);
