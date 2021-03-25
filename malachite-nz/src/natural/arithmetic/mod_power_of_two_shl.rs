use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwo, ModPowerOfTwoAssign, ModPowerOfTwoShl, ModPowerOfTwoShlAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use natural::Natural;
use std::ops::{Shr, ShrAssign};

fn _mod_power_of_two_shl_unsigned<T>(x: &Natural, bits: T, pow: u64) -> Natural
where
    u64: ExactFrom<T>,
{
    let bits = u64::exact_from(bits);
    if bits >= pow {
        Natural::ZERO
    } else {
        x.mod_power_of_two(pow - bits) << bits
    }
}

fn _mod_power_of_two_shl_assign_unsigned<T>(x: &mut Natural, bits: T, pow: u64)
where
    u64: ExactFrom<T>,
{
    let bits = u64::exact_from(bits);
    if bits >= pow {
        *x = Natural::ZERO;
    } else {
        x.mod_power_of_two_assign(pow - bits);
        *x <<= bits;
    }
}

macro_rules! impl_mod_power_of_two_shl_unsigned {
    ($t:ident) => {
        impl ModPowerOfTwoShl<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod 2<sup>`pow`</sup>,
            /// taking the `Natural` by value. Assumes the input is already reduced mod
            /// 2<sup>`pow`</sup>.
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(123u32).mod_power_of_two_shl(5u16, 8), 96);
            /// assert_eq!(Natural::from(123u32).mod_power_of_two_shl(100u64, 80), 0);
            /// ```
            #[inline]
            fn mod_power_of_two_shl(mut self, bits: $t, pow: u64) -> Natural {
                self.mod_power_of_two_shl_assign(bits, pow);
                self
            }
        }

        impl<'a> ModPowerOfTwoShl<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod 2<sup>`pow`</sup>,
            /// taking the `Natural` by reference. Assumes the input is already reduced mod
            /// 2<sup>`pow`</sup>.
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::from(123u32)).mod_power_of_two_shl(5u16, 8), 96);
            /// assert_eq!((&Natural::from(123u32)).mod_power_of_two_shl(100u64, 80), 0);
            /// ```
            #[inline]
            fn mod_power_of_two_shl(self, bits: $t, pow: u64) -> Natural {
                _mod_power_of_two_shl_unsigned(self, bits, pow)
            }
        }

        impl ModPowerOfTwoShlAssign<$t> for Natural {
            /// Shifts a `Natural` left (multiplies it by a power of 2) mod 2<sup>`pow`</sup>, in
            /// place. Assumes the input is already reduced mod 2<sup>`pow`</sup>.
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShlAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut n = Natural::from(123u32);
            /// n.mod_power_of_two_shl_assign(5u16, 8);
            /// assert_eq!(n, 96);
            ///
            /// let mut n = Natural::from(123u32);
            /// n.mod_power_of_two_shl_assign(100u64, 80);
            /// assert_eq!(n, 0);
            /// ```
            #[inline]
            fn mod_power_of_two_shl_assign(&mut self, bits: $t, pow: u64) {
                _mod_power_of_two_shl_assign_unsigned(self, bits, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_two_shl_unsigned);

fn _mod_power_of_two_shl_signed<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Natural,
    bits: S,
    pow: u64,
) -> Natural
where
    &'a Natural: ModPowerOfTwoShl<U, Output = Natural> + Shr<U, Output = Natural>,
{
    if bits >= S::ZERO {
        x.mod_power_of_two_shl(bits.unsigned_abs(), pow)
    } else {
        x >> bits.unsigned_abs()
    }
}

fn _mod_power_of_two_shl_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Natural,
    bits: S,
    pow: u64,
) where
    Natural: ModPowerOfTwoShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        x.mod_power_of_two_shl_assign(bits.unsigned_abs(), pow);
    } else {
        *x >>= bits.unsigned_abs();
    }
}

macro_rules! impl_mod_power_of_two_shl_signed {
    ($t:ident) => {
        impl ModPowerOfTwoShl<$t> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod 2<sup>`pow`</sup>,
            /// taking the `Natural` by value. Assumes the input is already reduced mod
            /// 2<sup>`pow`</sup>.
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(123u32).mod_power_of_two_shl(5i16, 8), 96);
            /// assert_eq!(Natural::from(123u32).mod_power_of_two_shl(100i64, 80), 0);
            /// assert_eq!(Natural::from(123u32).mod_power_of_two_shl(-2i8, 8), 30);
            /// ```
            #[inline]
            fn mod_power_of_two_shl(mut self, bits: $t, pow: u64) -> Natural {
                self.mod_power_of_two_shl_assign(bits, pow);
                self
            }
        }

        impl<'a> ModPowerOfTwoShl<$t> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod 2<sup>`pow`</sup>,
            /// taking the `Natural` by reference. Assumes the input is already reduced mod
            /// 2<sup>`pow`</sup>.
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::from(123u32)).mod_power_of_two_shl(5i16, 8), 96);
            /// assert_eq!((&Natural::from(123u32)).mod_power_of_two_shl(100i64, 80), 0);
            /// assert_eq!((&Natural::from(123u32)).mod_power_of_two_shl(-2i8, 8), 30);
            /// ```
            #[inline]
            fn mod_power_of_two_shl(self, bits: $t, pow: u64) -> Natural {
                _mod_power_of_two_shl_signed(self, bits, pow)
            }
        }

        impl ModPowerOfTwoShlAssign<$t> for Natural {
            /// Shifts a `Natural` left (multiplies it by a power of 2) mod 2<sup>`pow`</sup>, in
            /// place. Assumes the input is already reduced mod 2<sup>`pow`</sup>.
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
            /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoShlAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut n = Natural::from(123u32);
            /// n.mod_power_of_two_shl_assign(5i16, 8);
            /// assert_eq!(n, 96);
            ///
            /// let mut n = Natural::from(123u32);
            /// n.mod_power_of_two_shl_assign(100i64, 80);
            /// assert_eq!(n, 0);
            ///
            /// let mut n = Natural::from(123u32);
            /// n.mod_power_of_two_shl_assign(-2i8, 8);
            /// assert_eq!(n, 30);
            /// ```
            #[inline]
            fn mod_power_of_two_shl_assign(&mut self, bits: $t, pow: u64) {
                _mod_power_of_two_shl_assign_signed(self, bits, pow)
            }
        }
    };
}
apply_to_signeds!(impl_mod_power_of_two_shl_signed);
