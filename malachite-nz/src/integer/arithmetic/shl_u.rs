use std::ops::{Shl, ShlAssign};

use integer::Integer;

macro_rules! impl_integer_shl_unsigned {
    ($t:ident) => {
        /// Shifts a `Integer` left (multiplies it by a power of 2), taking the `Integer` by value.
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
        /// fn main() {
        ///     assert_eq!((Integer::ZERO << 10u8).to_string(), "0");
        ///     assert_eq!((Integer::from(123) << 2u16).to_string(), "492");
        ///     assert_eq!((Integer::from(123) << 100u32).to_string(),
        ///         "155921023828072216384094494261248");
        ///     assert_eq!((Integer::from(-123) << 2u64).to_string(), "-492");
        ///     assert_eq!((Integer::from(-123) << 100u8).to_string(),
        ///         "-155921023828072216384094494261248");
        /// }
        /// ```
        impl Shl<$t> for Integer {
            type Output = Integer;

            fn shl(self, other: $t) -> Integer {
                Integer {
                    sign: self.sign,
                    abs: self.abs << other,
                }
            }
        }

        /// Shifts a `Integer` left (multiplies it by a power of 2), taking the `Integer` by
        /// reference.
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
        /// fn main() {
        ///     assert_eq!((&Integer::ZERO << 10u8).to_string(), "0");
        ///     assert_eq!((&Integer::from(123) << 2u16).to_string(), "492");
        ///     assert_eq!((&Integer::from(123) << 100u32).to_string(),
        ///         "155921023828072216384094494261248");
        ///     assert_eq!((&Integer::from(-123) << 2u64).to_string(), "-492");
        ///     assert_eq!((&Integer::from(-123) << 100u8).to_string(),
        ///         "-155921023828072216384094494261248");
        /// }
        /// ```
        impl<'a> Shl<$t> for &'a Integer {
            type Output = Integer;

            fn shl(self, other: $t) -> Integer {
                Integer {
                    sign: self.sign,
                    abs: &self.abs << other,
                }
            }
        }

        /// Shifts a `Integer` left (multiplies it by a power of 2) in place.
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
        /// use malachite_base::num::basic::traits::{NegativeOne, One};
        /// use malachite_nz::integer::Integer;
        ///
        /// fn main() {
        ///     let mut x = Integer::ONE;
        ///     x <<= 1u8;
        ///     x <<= 2u16;
        ///     x <<= 3u32;
        ///     x <<= 4u64;
        ///     assert_eq!(x.to_string(), "1024");
        ///     let mut x = Integer::NEGATIVE_ONE;
        ///     x <<= 1u8;
        ///     x <<= 2u16;
        ///     x <<= 3u32;
        ///     x <<= 4u64;
        ///     assert_eq!(x.to_string(), "-1024");
        /// }
        /// ```
        impl ShlAssign<$t> for Integer {
            fn shl_assign(&mut self, other: $t) {
                self.abs <<= other;
            }
        }
    };
}
impl_integer_shl_unsigned!(u8);
impl_integer_shl_unsigned!(u16);
impl_integer_shl_unsigned!(u32);
impl_integer_shl_unsigned!(u64);
impl_integer_shl_unsigned!(u128);
impl_integer_shl_unsigned!(usize);
