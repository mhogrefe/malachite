use malachite_base::num::arithmetic::traits::{
    ModMul, ModMulAssign, ModPow, ModShr, ModShrAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::{Two, Zero};
use natural::InnerNatural::Small;
use natural::Natural;
use std::cmp::Ordering;
use std::ops::{Shr, ShrAssign};

fn mod_shr_ref_val<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Natural,
    bits: S,
    m: Natural,
) -> Natural
where
    Natural: From<U>,
    &'a Natural: Shr<U, Output = Natural>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => x.clone(),
        Ordering::Greater => x >> bits_abs,
        Ordering::Less => match m {
            natural_one!() | natural_two!() => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits_abs), &m), m),
        },
    }
}

fn mod_shr_ref_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Natural,
    bits: S,
    m: &Natural,
) -> Natural
where
    Natural: From<U>,
    &'a Natural: Shr<U, Output = Natural>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => x.clone(),
        Ordering::Greater => x >> bits_abs,
        Ordering::Less => match m {
            natural_one!() | natural_two!() => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits_abs), m), m),
        },
    }
}

fn mod_shr_assign<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Natural,
    bits: S,
    m: Natural,
) where
    Natural: From<U> + ShrAssign<U>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => {}
        Ordering::Greater => *x >>= bits_abs,
        Ordering::Less => match m {
            natural_one!() | natural_two!() => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits_abs), &m), m),
        },
    }
}

fn mod_shr_assign_ref<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Natural,
    bits: S,
    m: &Natural,
) where
    Natural: From<U> + ShrAssign<U>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => {}
        Ordering::Greater => *x >>= bits_abs,
        Ordering::Less => match m {
            natural_one!() | natural_two!() => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits_abs), m), m),
        },
    }
}

macro_rules! impl_mod_shr {
    ($t:ident) => {
        impl ModShr<$t, Natural> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2) mod `m`, taking `self` and `m`
            /// by value. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShr;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(8u32).mod_shr(-2i8, Natural::from(10u32)), 2);
            /// assert_eq!(Natural::from(10u32).mod_shr(100i32, Natural::from(10u32)), 0);
            /// assert_eq!(
            ///     Natural::from(123456u32).mod_shr(
            ///         -100i64,
            ///         Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shr(mut self, bits: $t, m: Natural) -> Natural {
                self.mod_shr_assign(bits, m);
                self
            }
        }

        impl<'a> ModShr<$t, &'a Natural> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2) mod `m`, taking `self` by
            /// value and `m` by reference. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShr;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(8u32).mod_shr(-2i8, &Natural::from(10u32)), 2);
            /// assert_eq!(Natural::from(10u32).mod_shr(100i32, &Natural::from(10u32)), 0);
            /// assert_eq!(
            ///     Natural::from(123456u32).mod_shr(
            ///         -100i64,
            ///         &Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shr(mut self, bits: $t, m: &'a Natural) -> Natural {
                self.mod_shr_assign(bits, m);
                self
            }
        }

        impl<'a> ModShr<$t, Natural> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2) mod `m`, taking `self` by
            /// reference and `m` by value. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShr;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::from(8u32)).mod_shr(-2i8, Natural::from(10u32)), 2);
            /// assert_eq!((&Natural::from(10u32)).mod_shr(100i32, Natural::from(10u32)), 0);
            /// assert_eq!(
            ///     (&Natural::from(123456u32)).mod_shr(
            ///         -100i64,
            ///         Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shr(self, bits: $t, m: Natural) -> Natural {
                mod_shr_ref_val(self, bits, m)
            }
        }

        impl<'a, 'b> ModShr<$t, &'b Natural> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` right (divides it by a power of 2) mod `m`, taking `self` and `m`
            /// by reference. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShr;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::from(8u32)).mod_shr(-2i8, &Natural::from(10u32)), 2);
            /// assert_eq!((&Natural::from(10u32)).mod_shr(100i32, &Natural::from(10u32)), 0);
            /// assert_eq!(
            ///     (&Natural::from(123456u32)).mod_shr(
            ///         -100i64,
            ///         &Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shr(self, bits: $t, m: &'b Natural) -> Natural {
                mod_shr_ref_ref(self, bits, m)
            }
        }

        impl ModShrAssign<$t, Natural> for Natural {
            /// Shifts a `Natural` right (divides it by a power of 2) mod `m` in place, taking `m`
            /// by value. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShrAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::from(8u32);
            /// x.mod_shr_assign(-2i8, Natural::from(10u32));
            /// assert_eq!(x, 2);
            ///
            /// let mut x = Natural::from(10u32);
            /// x.mod_shr_assign(100i32, Natural::from(10u32));
            /// assert_eq!(x, 0);
            ///
            /// let mut x = Natural::from(123456u32);
            /// x.mod_shr_assign(-100i64, Natural::from_str("12345678987654321").unwrap());
            /// assert_eq!(x, Natural::from_str("7436663564915145").unwrap());
            /// ```
            #[inline]
            fn mod_shr_assign(&mut self, bits: $t, m: Natural) {
                mod_shr_assign(self, bits, m);
            }
        }

        impl<'a> ModShrAssign<$t, &'a Natural> for Natural {
            /// Shifts a `Natural` right (divides it by a power of 2) mod `m` in place, taking `m`
            /// by reference. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShrAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::from(8u32);
            /// x.mod_shr_assign(-2i8, &Natural::from(10u32));
            /// assert_eq!(x, 2);
            ///
            /// let mut x = Natural::from(10u32);
            /// x.mod_shr_assign(100i32, &Natural::from(10u32));
            /// assert_eq!(x, 0);
            ///
            /// let mut x = Natural::from(123456u32);
            /// x.mod_shr_assign(-100i64, &Natural::from_str("12345678987654321").unwrap());
            /// assert_eq!(x, Natural::from_str("7436663564915145").unwrap());
            /// ```
            #[inline]
            fn mod_shr_assign(&mut self, bits: $t, m: &'a Natural) {
                mod_shr_assign_ref(self, bits, m);
            }
        }
    };
}
apply_to_signeds!(impl_mod_shr);
