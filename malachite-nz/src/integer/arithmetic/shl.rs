use integer::Integer;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use natural::Natural;
use std::ops::{Shl, ShlAssign, Shr};

fn shl_unsigned<T>(x: Integer, bits: T) -> Integer
where
    Natural: Shl<T, Output = Natural>,
{
    Integer {
        sign: x.sign,
        abs: x.abs << bits,
    }
}

fn shl_unsigned_ref<'a, T>(x: &'a Integer, bits: T) -> Integer
where
    &'a Natural: Shl<T, Output = Natural>,
{
    Integer {
        sign: x.sign,
        abs: &x.abs << bits,
    }
}

macro_rules! impl_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for Integer {
            type Output = Integer;

            /// Shifts an `Integer` left (multiplies it by a power of 2), taking the `Integer` by
            /// value.
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
            /// assert_eq!((Integer::ZERO << 10u8).to_string(), "0");
            /// assert_eq!((Integer::from(123) << 2u16).to_string(), "492");
            /// assert_eq!((Integer::from(123) << 100u32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((Integer::from(-123) << 2u64).to_string(), "-492");
            /// assert_eq!((Integer::from(-123) << 100u8).to_string(),
            ///     "-155921023828072216384094494261248");
            /// ```
            #[inline]
            fn shl(self, bits: $t) -> Integer {
                shl_unsigned(self, bits)
            }
        }

        impl<'a> Shl<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an `Integer` left (multiplies it by a power of 2), taking the `Integer` by
            /// reference.
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
            /// assert_eq!((&Integer::ZERO << 10u8).to_string(), "0");
            /// assert_eq!((&Integer::from(123) << 2u16).to_string(), "492");
            /// assert_eq!((&Integer::from(123) << 100u32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((&Integer::from(-123) << 2u64).to_string(), "-492");
            /// assert_eq!((&Integer::from(-123) << 100u8).to_string(),
            ///     "-155921023828072216384094494261248");
            /// ```
            #[inline]
            fn shl(self, bits: $t) -> Integer {
                shl_unsigned_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for Integer {
            /// Shifts an `Integer` left (multiplies it by a power of 2) in place.
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
            /// let mut x = Integer::ONE;
            /// x <<= 1u8;
            /// x <<= 2u16;
            /// x <<= 3u32;
            /// x <<= 4u64;
            /// assert_eq!(x.to_string(), "1024");
            /// let mut x = Integer::NEGATIVE_ONE;
            /// x <<= 1u8;
            /// x <<= 2u16;
            /// x <<= 3u32;
            /// x <<= 4u64;
            /// assert_eq!(x.to_string(), "-1024");
            /// ```
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                self.abs <<= bits;
            }
        }
    };
}
apply_to_unsigneds!(impl_shl_unsigned);

fn _shl_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Integer,
    bits: S,
) -> Integer
where
    &'a Integer: Shl<U, Output = Integer> + Shr<U, Output = Integer>,
{
    if bits >= S::ZERO {
        x << bits.unsigned_abs()
    } else {
        x >> bits.unsigned_abs()
    }
}

macro_rules! impl_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for Integer {
            type Output = Integer;

            /// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor), taking the `Integer` by value.
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
            /// assert_eq!((Integer::ZERO << 10i8).to_string(), "0");
            /// assert_eq!((Integer::from(123) << 2i16).to_string(), "492");
            /// assert_eq!((Integer::from(123) << 100i32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((Integer::from(-123) << 2i64).to_string(), "-492");
            /// assert_eq!((Integer::from(-123) << 100i8).to_string(),
            ///     "-155921023828072216384094494261248");
            /// assert_eq!((Integer::ZERO << -10i16).to_string(), "0");
            /// assert_eq!((Integer::from(492) << -2i32).to_string(), "123");
            /// assert_eq!((-Integer::trillion() << -10i64).to_string(), "-976562500");
            /// ```
            #[inline]
            fn shl(mut self, bits: $t) -> Integer {
                self <<= bits;
                self
            }
        }

        impl<'a> Shl<$t> for &'a Integer {
            type Output = Integer;

            /// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor), taking the `Integer` by reference.
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
            /// assert_eq!((&Integer::ZERO << 10i8).to_string(), "0");
            /// assert_eq!((&Integer::from(123) << 2i16).to_string(), "492");
            /// assert_eq!((&Integer::from(123) << 100i32).to_string(),
            ///     "155921023828072216384094494261248");
            /// assert_eq!((&Integer::from(-123) << 2i64).to_string(), "-492");
            /// assert_eq!((&Integer::from(-123) << 100i8).to_string(),
            ///     "-155921023828072216384094494261248");
            /// assert_eq!((&Integer::ZERO << -10i16).to_string(), "0");
            /// assert_eq!((&Integer::from(492) << -2i32).to_string(), "123");
            /// assert_eq!((&(-Integer::trillion()) << -10i64).to_string(), "-976562500");
            /// ```
            #[inline]
            fn shl(self, bits: $t) -> Integer {
                _shl_signed_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for Integer {
            /// Shifts an `Integer` left (multiplies it by a power of 2 or divides it by a power of
            /// 2 and takes the floor) in place.
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
            /// let mut x = Integer::ONE;
            /// x <<= 1i8;
            /// x <<= 2i16;
            /// x <<= 3i32;
            /// x <<= 4i64;
            /// assert_eq!(x.to_string(), "1024");
            /// let mut x = Integer::NEGATIVE_ONE;
            /// x <<= 1i8;
            /// x <<= 2i16;
            /// x <<= 3i32;
            /// x <<= 4i64;
            /// assert_eq!(x.to_string(), "-1024");
            ///
            /// let mut x = Integer::from(1024);
            /// x <<= -1i8;
            /// x <<= -2i16;
            /// x <<= -3i32;
            /// x <<= -4i64;
            /// assert_eq!(x.to_string(), "1");
            /// ```
            fn shl_assign(&mut self, bits: $t) {
                if bits >= 0 {
                    *self <<= bits.unsigned_abs();
                } else {
                    *self >>= bits.unsigned_abs();
                }
            }
        }
    };
}
apply_to_signeds!(impl_shl_signed);
