use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoShl, ModPowerOfTwoShlAssign, ModPowerOfTwoShr, ModPowerOfTwoShrAssign, UnsignedAbs,
};
use natural::Natural;

macro_rules! impl_mod_power_of_two_shr_signed {
    ($t:ident) => {
        /// Shifts a `Natural` right (divides it by a power of 2) mod 2<sup>`pow`</sup>, taking the
        /// `Natural` by value. Assumes the input is already reduced mod 2<sup>`pow`</sup>.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(m)
        ///
        /// where n = `self.significant_bits()` + `bits`, m = `bits`
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShr;
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!(Natural::from(123u32).mod_power_of_two_shr(-5i16, 8), 96);
        /// assert_eq!(Natural::from(123u32).mod_power_of_two_shr(-100i64, 80), 0);
        /// assert_eq!(Natural::from(123u32).mod_power_of_two_shr(2i8, 8), 30);
        /// ```
        impl ModPowerOfTwoShr<$t> for Natural {
            type Output = Natural;

            #[inline]
            fn mod_power_of_two_shr(mut self, bits: $t, pow: u64) -> Natural {
                self.mod_power_of_two_shr_assign(bits, pow);
                self
            }
        }

        /// Shifts a `Natural` right (divides it by a power of 2) mod 2<sup>`pow`</sup>, taking the
        /// `Natural` by reference. Assumes the input is already reduced mod 2<sup>`pow`</sup>.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(n)
        ///
        /// where n = `self.significant_bits()` + `bits`
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShr;
        /// use malachite_nz::natural::Natural;
        ///
        /// assert_eq!((&Natural::from(123u32)).mod_power_of_two_shr(-5i16, 8), 96);
        /// assert_eq!((&Natural::from(123u32)).mod_power_of_two_shr(-100i64, 80), 0);
        /// assert_eq!((&Natural::from(123u32)).mod_power_of_two_shr(2i8, 8), 30);
        /// ```
        impl<'a> ModPowerOfTwoShr<$t> for &'a Natural {
            type Output = Natural;

            fn mod_power_of_two_shr(self, bits: $t, pow: u64) -> Natural {
                if bits >= 0 {
                    self >> bits.unsigned_abs()
                } else {
                    self.mod_power_of_two_shl(bits.unsigned_abs(), pow)
                }
            }
        }

        /// Shifts a `Natural` right (divides it by a power of 2) mod 2<sup>`pow`</sup>, in place.
        /// Assumes the input is already reduced mod 2<sup>`pow`</sup>.
        ///
        /// Time: worst case O(n)
        ///
        /// Additional memory: worst case O(m)
        ///
        /// where n = `self.significant_bits()` + `bits`, m = `bits`
        ///
        /// # Examples
        /// ```
        /// extern crate malachite_base;
        /// extern crate malachite_nz;
        ///
        /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShrAssign;
        /// use malachite_nz::natural::Natural;
        ///
        /// let mut n = Natural::from(123u32);
        /// n.mod_power_of_two_shr_assign(-5i16, 8);
        /// assert_eq!(n, 96);
        ///
        /// let mut n = Natural::from(123u32);
        /// n.mod_power_of_two_shr_assign(-100i64, 80);
        /// assert_eq!(n, 0);
        ///
        /// let mut n = Natural::from(123u32);
        /// n.mod_power_of_two_shr_assign(2i8, 8);
        /// assert_eq!(n, 30);
        /// ```
        impl ModPowerOfTwoShrAssign<$t> for Natural {
            fn mod_power_of_two_shr_assign(&mut self, bits: $t, pow: u64) {
                if bits >= 0 {
                    *self >>= bits.unsigned_abs();
                } else {
                    self.mod_power_of_two_shl_assign(bits.unsigned_abs(), pow);
                }
            }
        }
    };
}
apply_to_signeds!(impl_mod_power_of_two_shr_signed);
