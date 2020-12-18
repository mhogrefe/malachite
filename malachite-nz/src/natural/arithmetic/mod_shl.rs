use std::cmp::Ordering;
use std::ops::{Shr, ShrAssign};

use malachite_base::num::arithmetic::traits::{
    ModMul, ModMulAssign, ModPow, ModShl, ModShlAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::{Two, Zero};

use natural::InnerNatural::Small;
use natural::Natural;

fn _mod_shl_ref_val_unsigned<T: Copy + Eq + Zero>(x: &Natural, bits: T, m: Natural) -> Natural
where
    Natural: From<T>,
{
    if bits == T::ZERO {
        x.clone()
    } else {
        match m {
            natural_one!() | natural_two!() => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits), &m), m),
        }
    }
}

fn _mod_shl_ref_ref_unsigned<T: Copy + Eq + Zero>(x: &Natural, bits: T, m: &Natural) -> Natural
where
    Natural: From<T>,
{
    if bits == T::ZERO {
        x.clone()
    } else {
        match m {
            natural_one!() | natural_two!() => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits), m), m),
        }
    }
}

fn _mod_shl_assign_unsigned<T: Copy + Eq + Zero>(x: &mut Natural, bits: T, m: Natural)
where
    Natural: From<T>,
{
    if bits != T::ZERO {
        match m {
            natural_one!() | natural_two!() => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits), &m), m),
        }
    }
}

fn _mod_shl_assign_ref_unsigned<T: Copy + Eq + Zero>(x: &mut Natural, bits: T, m: &Natural)
where
    Natural: From<T>,
{
    if bits != T::ZERO {
        match m {
            natural_one!() | natural_two!() => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits), m), m),
        }
    }
}

macro_rules! impl_mod_shl_unsigned {
    ($t:ident) => {
        impl ModShl<$t, Natural> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m`, taking `self` and
            /// `m` by value. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(8u32).mod_shl(2u16, Natural::from(10u32)), 2);
            /// assert_eq!(
            ///     Natural::from(123456u32).mod_shl(
            ///         100u64,
            ///         Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shl(mut self, bits: $t, m: Natural) -> Natural {
                self.mod_shl_assign(bits, m);
                self
            }
        }

        impl<'a> ModShl<$t, &'a Natural> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m`, taking `self` by
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
            /// use malachite_base::num::arithmetic::traits::ModShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(8u32).mod_shl(2u16, &Natural::from(10u32)), 2);
            /// assert_eq!(
            ///     Natural::from(123456u32).mod_shl(
            ///         100u64,
            ///         &Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shl(mut self, bits: $t, m: &'a Natural) -> Natural {
                self.mod_shl_assign(bits, m);
                self
            }
        }

        impl<'a> ModShl<$t, Natural> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m`, taking `self` by
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
            /// use malachite_base::num::arithmetic::traits::ModShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::from(8u32)).mod_shl(2u16, Natural::from(10u32)), 2);
            /// assert_eq!(
            ///     (&Natural::from(123456u32)).mod_shl(
            ///         100u64,
            ///         Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shl(self, bits: $t, m: Natural) -> Natural {
                _mod_shl_ref_val_unsigned(self, bits, m)
            }
        }

        impl<'a, 'b> ModShl<$t, &'b Natural> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m`, taking `self` and
            /// `m` by reference. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::from(8u32)).mod_shl(2u16, &Natural::from(10u32)), 2);
            /// assert_eq!(
            ///     (&Natural::from(123456u32)).mod_shl(
            ///         100u64,
            ///         &Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shl(self, bits: $t, m: &'b Natural) -> Natural {
                _mod_shl_ref_ref_unsigned(self, bits, m)
            }
        }

        impl ModShlAssign<$t, Natural> for Natural {
            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m` in place, taking `m`
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
            /// use malachite_base::num::arithmetic::traits::ModShlAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::from(8u32);
            /// x.mod_shl_assign(2u16, Natural::from(10u32));
            /// assert_eq!(x, 2);
            ///
            /// let mut x = Natural::from(123456u32);
            /// x.mod_shl_assign(100u64, Natural::from_str("12345678987654321").unwrap());
            /// assert_eq!(x, Natural::from_str("7436663564915145").unwrap());
            /// ```
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: Natural) {
                _mod_shl_assign_unsigned(self, bits, m);
            }
        }

        impl<'a> ModShlAssign<$t, &'a Natural> for Natural {
            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m` in place, taking `m`
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
            /// use malachite_base::num::arithmetic::traits::ModShlAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::from(8u32);
            /// x.mod_shl_assign(2u16, &Natural::from(10u32));
            /// assert_eq!(x, 2);
            ///
            /// let mut x = Natural::from(123456u32);
            /// x.mod_shl_assign(100u64, &Natural::from_str("12345678987654321").unwrap());
            /// assert_eq!(x, Natural::from_str("7436663564915145").unwrap());
            /// ```
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: &'a Natural) {
                _mod_shl_assign_ref_unsigned(self, bits, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_shl_unsigned);

fn _mod_shl_ref_val_signed<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
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
        Ordering::Less => x >> bits_abs,
        Ordering::Greater => match m {
            natural_one!() | natural_two!() => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits_abs), &m), m),
        },
    }
}

fn _mod_shl_ref_ref_signed<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
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
        Ordering::Less => x >> bits_abs,
        Ordering::Greater => match m {
            natural_one!() | natural_two!() => Natural::ZERO,
            _ => x.mod_mul(Natural::TWO.mod_pow(Natural::from(bits_abs), m), m),
        },
    }
}

fn _mod_shl_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Natural,
    bits: S,
    m: Natural,
) where
    Natural: From<U> + ShrAssign<U>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => {}
        Ordering::Less => *x >>= bits_abs,
        Ordering::Greater => match m {
            natural_one!() | natural_two!() => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits_abs), &m), m),
        },
    }
}

fn _mod_shl_assign_ref_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Natural,
    bits: S,
    m: &Natural,
) where
    Natural: From<U> + ShrAssign<U>,
{
    let bits_abs = bits.unsigned_abs();
    match bits.cmp(&S::ZERO) {
        Ordering::Equal => {}
        Ordering::Less => *x >>= bits_abs,
        Ordering::Greater => match m {
            natural_one!() | natural_two!() => *x = Natural::ZERO,
            _ => x.mod_mul_assign(Natural::TWO.mod_pow(Natural::from(bits_abs), m), m),
        },
    }
}

macro_rules! impl_mod_shl_signed {
    ($t:ident) => {
        impl ModShl<$t, Natural> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m`, taking `self` and
            /// `m` by value. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(8u32).mod_shl(2i8, Natural::from(10u32)), 2);
            /// assert_eq!(Natural::from(10u32).mod_shl(-100i32, Natural::from(10u32)), 0);
            /// assert_eq!(
            ///     Natural::from(123456u32).mod_shl(
            ///         100i64,
            ///         Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shl(mut self, bits: $t, m: Natural) -> Natural {
                self.mod_shl_assign(bits, m);
                self
            }
        }

        impl<'a> ModShl<$t, &'a Natural> for Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m`, taking `self` by
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
            /// use malachite_base::num::arithmetic::traits::ModShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!(Natural::from(8u32).mod_shl(2i8, &Natural::from(10u32)), 2);
            /// assert_eq!(Natural::from(10u32).mod_shl(-100i32, &Natural::from(10u32)), 0);
            /// assert_eq!(
            ///     Natural::from(123456u32).mod_shl(
            ///         100i64,
            ///         &Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shl(mut self, bits: $t, m: &'a Natural) -> Natural {
                self.mod_shl_assign(bits, m);
                self
            }
        }

        impl<'a> ModShl<$t, Natural> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m`, taking `self` by
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
            /// use malachite_base::num::arithmetic::traits::ModShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::from(8u32)).mod_shl(2i8, Natural::from(10u32)), 2);
            /// assert_eq!((&Natural::from(10u32)).mod_shl(-100i32, Natural::from(10u32)), 0);
            /// assert_eq!(
            ///     (&Natural::from(123456u32)).mod_shl(
            ///         100i64,
            ///         Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shl(self, bits: $t, m: Natural) -> Natural {
                _mod_shl_ref_val_signed(self, bits, m)
            }
        }

        impl<'a, 'b> ModShl<$t, &'b Natural> for &'a Natural {
            type Output = Natural;

            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m`, taking `self` and
            /// `m` by reference. Assumes that `self` is already reduced mod `m`.
            ///
            /// TODO complexity
            ///
            /// # Examples
            /// ```
            /// extern crate malachite_base;
            /// extern crate malachite_nz;
            ///
            /// use std::str::FromStr;
            /// use malachite_base::num::arithmetic::traits::ModShl;
            /// use malachite_nz::natural::Natural;
            ///
            /// assert_eq!((&Natural::from(8u32)).mod_shl(2i8, &Natural::from(10u32)), 2);
            /// assert_eq!((&Natural::from(10u32)).mod_shl(-100i32, &Natural::from(10u32)), 0);
            /// assert_eq!(
            ///     (&Natural::from(123456u32)).mod_shl(
            ///         100i64,
            ///         &Natural::from_str("12345678987654321").unwrap()
            ///     ),
            ///     Natural::from_str("7436663564915145").unwrap()
            /// );
            /// ```
            #[inline]
            fn mod_shl(self, bits: $t, m: &'b Natural) -> Natural {
                _mod_shl_ref_ref_signed(self, bits, m)
            }
        }

        impl ModShlAssign<$t, Natural> for Natural {
            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m` in place, taking `m`
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
            /// use malachite_base::num::arithmetic::traits::ModShlAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::from(8u32);
            /// x.mod_shl_assign(2i8, Natural::from(10u32));
            /// assert_eq!(x, 2);
            ///
            /// let mut x = Natural::from(10u32);
            /// x.mod_shl_assign(-100i32, Natural::from(10u32));
            /// assert_eq!(x, 0);
            ///
            /// let mut x = Natural::from(123456u32);
            /// x.mod_shl_assign(100i64, Natural::from_str("12345678987654321").unwrap());
            /// assert_eq!(x, Natural::from_str("7436663564915145").unwrap());
            /// ```
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: Natural) {
                _mod_shl_assign_signed(self, bits, m);
            }
        }

        impl<'a> ModShlAssign<$t, &'a Natural> for Natural {
            /// Shifts a `Natural` left (multiplies it by a power of 2) mod `m` in place, taking `m`
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
            /// use malachite_base::num::arithmetic::traits::ModShlAssign;
            /// use malachite_nz::natural::Natural;
            ///
            /// let mut x = Natural::from(8u32);
            /// x.mod_shl_assign(2i8, &Natural::from(10u32));
            /// assert_eq!(x, 2);
            ///
            /// let mut x = Natural::from(10u32);
            /// x.mod_shl_assign(-100i32, &Natural::from(10u32));
            /// assert_eq!(x, 0);
            ///
            /// let mut x = Natural::from(123456u32);
            /// x.mod_shl_assign(100i64, &Natural::from_str("12345678987654321").unwrap());
            /// assert_eq!(x, Natural::from_str("7436663564915145").unwrap());
            /// ```
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: &'a Natural) {
                _mod_shl_assign_ref_signed(self, bits, m);
            }
        }
    };
}
apply_to_signeds!(impl_mod_shl_signed);
