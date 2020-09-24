use num::basic::integers::PrimitiveInt;
use num::logic::traits::BitAccess;

fn _get_bit_unsigned<T: PrimitiveInt>(x: &T, index: u64) -> bool {
    index < T::WIDTH && *x & T::power_of_two(index) != T::ZERO
}

fn _set_bit_unsigned<T: PrimitiveInt>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x |= T::power_of_two(index);
    } else {
        panic!(
            "Cannot set bit {} in non-negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

fn _clear_bit_unsigned<T: PrimitiveInt>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x &= !T::power_of_two(index);
    }
}

macro_rules! impl_bit_access_unsigned {
    ($t:ident) => {
        /// Provides functions for accessing and modifying the `index`th bit of a primitive unsigned
        /// integer, or the coefficient of 2<sup>`index`</sup> in its binary expansion.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::logic::traits::BitAccess;
        ///
        /// let mut x = 0;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = 0u64;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive unsigned integer, or the
            /// coefficient of 2<sup>`index`</sup> in its binary expansion, is 0 or 1. `false`
            /// means 0, `true` means 1.
            ///
            /// Getting bits beyond the type's width is allowed; those bits are false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// assert_eq!(123u8.get_bit(2), false);
            /// assert_eq!(123u16.get_bit(3), true);
            /// assert_eq!(123u32.get_bit(100), false);
            /// assert_eq!(1000000000000u64.get_bit(12), true);
            /// assert_eq!(1000000000000u64.get_bit(100), false);
            /// ```
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                _get_bit_unsigned(self, index)
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of
            /// 2<sup>`index`</sup> in its binary expansion, to 1.
            ///
            /// Setting bits beyond the type's width is disallowed.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// let mut x = 0u8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            /// ```
            #[inline]
            fn set_bit(&mut self, index: u64) {
                _set_bit_unsigned(self, index)
            }

            /// Sets the `index`th bit of a primitive unsigned integer, or the coefficient of
            /// 2<sup>`index`</sup> in its binary expansion, to 0.
            ///
            /// Clearing bits beyond the type's width is allowed; since those bits are already
            /// false, clearing them does nothing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// let mut x = 0x7fu8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            /// ```
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                _clear_bit_unsigned(self, index)
            }
        }
    };
}
apply_to_unsigneds!(impl_bit_access_unsigned);

fn _get_bit_signed<T: PrimitiveInt>(x: &T, index: u64) -> bool {
    if index < T::WIDTH {
        *x & (T::ONE << index) != T::ZERO
    } else {
        *x < T::ZERO
    }
}

fn _set_bit_signed<T: PrimitiveInt>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x |= T::ONE << index;
    } else if *x >= T::ZERO {
        panic!(
            "Cannot set bit {} in non-negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

fn _clear_bit_signed<T: PrimitiveInt>(x: &mut T, index: u64) {
    if index < T::WIDTH {
        *x &= !(T::ONE << index);
    } else if *x < T::ZERO {
        panic!(
            "Cannot clear bit {} in negative value of width {}",
            index,
            T::WIDTH
        );
    }
}

macro_rules! impl_bit_access_signed {
    ($t:ident) => {
        /// Provides functions for accessing and modifying the `index`th bit of a primitive signed
        /// integer, or the coefficient of 2<sup>`index`</sup> in its binary expansion.
        ///
        /// Negative integers are represented in two's complement.
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::logic::traits::BitAccess;
        ///
        /// let mut x = 0i8;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, 100);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -0x100i16;
        /// x.assign_bit(2, true);
        /// x.assign_bit(5, true);
        /// x.assign_bit(6, true);
        /// assert_eq!(x, -156);
        /// x.assign_bit(2, false);
        /// x.assign_bit(5, false);
        /// x.assign_bit(6, false);
        /// assert_eq!(x, -256);
        ///
        /// let mut x = 0i32;
        /// x.flip_bit(10);
        /// assert_eq!(x, 1024);
        /// x.flip_bit(10);
        /// assert_eq!(x, 0);
        ///
        /// let mut x = -1i64;
        /// x.flip_bit(10);
        /// assert_eq!(x, -1025);
        /// x.flip_bit(10);
        /// assert_eq!(x, -1);
        /// ```
        impl BitAccess for $t {
            /// Determines whether the `index`th bit of a primitive signed integer, or the
            /// coefficient of 2<sup>`index`</sup> in its binary expansion, is 0 or 1. `false` means
            /// 0, `true` means 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Accessing bits beyond the type's width is allowed; those bits are false if the
            /// integer is non-negative and true if it is negative.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// assert_eq!(123i8.get_bit(2), false);
            /// assert_eq!(123i16.get_bit(3), true);
            /// assert_eq!(123i32.get_bit(100), false);
            /// assert_eq!((-123i8).get_bit(0), true);
            /// assert_eq!((-123i16).get_bit(1), false);
            /// assert_eq!((-123i32).get_bit(100), true);
            /// assert_eq!(1000000000000i64.get_bit(12), true);
            /// assert_eq!(1000000000000i64.get_bit(100), false);
            /// assert_eq!((-1000000000000i64).get_bit(12), true);
            /// assert_eq!((-1000000000000i64).get_bit(100), true);
            /// ```
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                _get_bit_signed(self, index)
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<sup>`index`</sup> in its binary expansion, to 1.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Setting bits beyond the type's width is disallowed if the integer is non-negative;
            /// if it is negative, it's allowed but does nothing since those bits are already true.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self >= 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// let mut x = 0i8;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -0x100i16;
            /// x.set_bit(2);
            /// x.set_bit(5);
            /// x.set_bit(6);
            /// assert_eq!(x, -156);
            /// ```
            #[inline]
            fn set_bit(&mut self, index: u64) {
                _set_bit_signed(self, index)
            }

            /// Sets the `index`th bit of a primitive signed integer, or the coefficient of
            /// 2<sup>`index`</sup> in its binary expansion, to 0.
            ///
            /// Negative integers are represented in two's complement.
            ///
            /// Clearing bits beyond the type's width is disallowed if the integer is negative; if
            /// it is non-negative, it's allowed but does nothing since those bits are already
            /// false.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `index >= Self::WIDTH && self < 0`.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::BitAccess;
            ///
            /// let mut x = 0x7fi8;
            /// x.clear_bit(0);
            /// x.clear_bit(1);
            /// x.clear_bit(3);
            /// x.clear_bit(4);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -156i16;
            /// x.clear_bit(2);
            /// x.clear_bit(5);
            /// x.clear_bit(6);
            /// assert_eq!(x, -256);
            /// ```
            #[inline]
            fn clear_bit(&mut self, index: u64) {
                _clear_bit_signed(self, index)
            }
        }
    };
}
apply_to_signeds!(impl_bit_access_signed);
