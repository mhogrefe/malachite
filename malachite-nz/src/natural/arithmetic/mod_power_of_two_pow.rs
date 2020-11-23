use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoMulAssign, ModPowerOfTwoPow, ModPowerOfTwoPowAssign, ModPowerOfTwoSquareAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::BitIterable;

use natural::Natural;

//TODO use test-utils version
fn _simple_binary_mod_power_of_two(x: &Natural, exp: &Natural, pow: u64) -> Natural {
    if pow == 0 {
        return Natural::ZERO;
    }
    let mut out = Natural::ONE;
    for bit in exp.bits().rev() {
        out.mod_power_of_two_square_assign(pow);
        if bit {
            out.mod_power_of_two_mul_assign(x, pow);
        }
    }
    out
}

impl ModPowerOfTwoPow<Natural> for Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup>, taking both `Natural`s by
    /// value. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).mod_power_of_two_pow(Natural::from(10u32), 8), 169);
    /// assert_eq!(
    ///     Natural::from(11u32).mod_power_of_two_pow(Natural::from(1000u32), 30),
    ///     289109473
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_pow(mut self, exp: Natural, pow: u64) -> Natural {
        self.mod_power_of_two_pow_assign(exp, pow);
        self
    }
}

impl<'a> ModPowerOfTwoPow<&'a Natural> for Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup>, taking the base by value and
    /// the exponent by reference. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).mod_power_of_two_pow(&Natural::from(10u32), 8), 169);
    /// assert_eq!(
    ///     Natural::from(11u32).mod_power_of_two_pow(&Natural::from(1000u32), 30),
    ///     289109473
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_pow(mut self, exp: &Natural, pow: u64) -> Natural {
        self.mod_power_of_two_pow_assign(exp, pow);
        self
    }
}

impl<'a> ModPowerOfTwoPow<Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup>, taking the base by reference
    /// and the exponent by value. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).mod_power_of_two_pow(Natural::from(10u32), 8), 169);
    /// assert_eq!(
    ///     (&Natural::from(11u32)).mod_power_of_two_pow(Natural::from(1000u32), 30),
    ///     289109473
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_pow(self, exp: Natural, pow: u64) -> Natural {
        _simple_binary_mod_power_of_two(self, &exp, pow)
    }
}

impl<'a, 'b> ModPowerOfTwoPow<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup>, taking both `Natural`s by
    /// reference. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).mod_power_of_two_pow(&Natural::from(10u32), 8), 169);
    /// assert_eq!(
    ///     (&Natural::from(11u32)).mod_power_of_two_pow(&Natural::from(1000u32), 30),
    ///     289109473
    /// );
    /// ```
    #[inline]
    fn mod_power_of_two_pow(self, exp: &Natural, pow: u64) -> Natural {
        _simple_binary_mod_power_of_two(self, exp, pow)
    }
}

impl ModPowerOfTwoPowAssign<Natural> for Natural {
    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup> in place, taking the exponent
    /// by value. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_two_pow_assign(Natural::from(10u32), 8);
    /// assert_eq!(x, 169);
    ///
    /// let mut x = Natural::from(11u32);
    /// x.mod_power_of_two_pow_assign(Natural::from(1000u32), 30);
    /// assert_eq!(x, 289109473);
    /// ```
    #[inline]
    fn mod_power_of_two_pow_assign(&mut self, exp: Natural, pow: u64) {
        *self = _simple_binary_mod_power_of_two(&*self, &exp, pow);
    }
}

impl<'a> ModPowerOfTwoPowAssign<&'a Natural> for Natural {
    /// Raises a `Natural` to a `Natural` power mod 2<sup>`pow`</sup> in place, taking the exponent
    /// by reference. Assumes the base is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOfTwoPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_two_pow_assign(&Natural::from(10u32), 8);
    /// assert_eq!(x, 169);
    ///
    /// let mut x = Natural::from(11u32);
    /// x.mod_power_of_two_pow_assign(&Natural::from(1000u32), 30);
    /// assert_eq!(x, 289109473);
    /// ```
    #[inline]
    fn mod_power_of_two_pow_assign(&mut self, exp: &Natural, pow: u64) {
        *self = _simple_binary_mod_power_of_two(&*self, exp, pow);
    }
}
