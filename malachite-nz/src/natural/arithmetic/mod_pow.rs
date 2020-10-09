use malachite_base::num::arithmetic::traits::ModMulAssign;
use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::BitIterable;

use natural::Natural;

//TODO use test-utils version
fn _simple_binary_mod_pow(x: &Natural, exp: &Natural, m: &Natural) -> Natural {
    if *m == 1 {
        return Natural::ZERO;
    }
    let mut out = Natural::ONE;
    for bit in exp.bits().rev() {
        out.mod_mul_assign(out.clone(), m); // TODO use mod_square_assign
        if bit {
            out.mod_mul_assign(x, m);
        }
    }
    out
}

impl ModPow<Natural, Natural> for Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod a `Natural`, taking all three `Natural`s by
    /// value. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(4u32).mod_pow(Natural::from(13u32), Natural::from(497u32)), 445);
    /// assert_eq!(Natural::from(10u32).mod_pow(Natural::from(1000u32), Natural::from(30u32)), 10);
    /// ```
    fn mod_pow(self, exp: Natural, m: Natural) -> Natural {
        _simple_binary_mod_pow(&self, &exp, &m)
    }
}

impl<'a> ModPow<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod a `Natural`, taking the first two `Natural`s by
    /// value and the third by reference. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(4u32).mod_pow(Natural::from(13u32), &Natural::from(497u32)), 445);
    /// assert_eq!(Natural::from(10u32).mod_pow(Natural::from(1000u32), &Natural::from(30u32)), 10);
    /// ```
    fn mod_pow(self, exp: Natural, m: &'a Natural) -> Natural {
        _simple_binary_mod_pow(&self, &exp, m)
    }
}

impl<'a> ModPow<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod a `Natural`, taking the first and third
    /// `Natural`s by value and the second by reference. Assumes the base is already reduced mod
    /// `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(4u32).mod_pow(&Natural::from(13u32), Natural::from(497u32)), 445);
    /// assert_eq!(Natural::from(10u32).mod_pow(&Natural::from(1000u32), Natural::from(30u32)), 10);
    /// ```
    fn mod_pow(self, exp: &'a Natural, m: Natural) -> Natural {
        _simple_binary_mod_pow(&self, exp, &m)
    }
}

impl<'a, 'b> ModPow<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod a `Natural`, taking the first `Natural` by
    /// value and the second and third by reference. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(4u32).mod_pow(&Natural::from(13u32), &Natural::from(497u32)), 445);
    /// assert_eq!(
    ///     Natural::from(10u32).mod_pow(&Natural::from(1000u32), &Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    fn mod_pow(self, exp: &'a Natural, m: &'b Natural) -> Natural {
        _simple_binary_mod_pow(&self, exp, m)
    }
}

impl<'a> ModPow<Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod a `Natural`, taking the first `Natural` by
    /// reference and the second and third by value. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_pow(Natural::from(13u32), Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_pow(Natural::from(1000u32), Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    fn mod_pow(self, exp: Natural, m: Natural) -> Natural {
        _simple_binary_mod_pow(self, &exp, &m)
    }
}

impl<'a, 'b> ModPow<Natural, &'b Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod a `Natural`, taking the first and third
    /// `Natural`s by reference and the second by value. Assumes the base is already reduced mod
    /// `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_pow(Natural::from(13u32), &Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_pow(Natural::from(1000u32), &Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    fn mod_pow(self, exp: Natural, m: &'b Natural) -> Natural {
        _simple_binary_mod_pow(self, &exp, m)
    }
}

impl<'a, 'b> ModPow<&'b Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod a `Natural`, taking the first two `Natural` by
    /// reference and the third by value. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_pow(&Natural::from(13u32), Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_pow(&Natural::from(1000u32), Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    fn mod_pow(self, exp: &'b Natural, m: Natural) -> Natural {
        _simple_binary_mod_pow(self, exp, &m)
    }
}

impl<'a, 'b, 'c> ModPow<&'b Natural, &'c Natural> for &'a Natural {
    type Output = Natural;

    /// Raises a `Natural` to a `Natural` power mod a `Natural`, taking all three `Natural`s by
    /// reference. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPow;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_pow(&Natural::from(13u32), &Natural::from(497u32)),
    ///     445
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_pow(&Natural::from(1000u32), &Natural::from(30u32)),
    ///     10
    /// );
    /// ```
    fn mod_pow(self, exp: &'b Natural, m: &'c Natural) -> Natural {
        _simple_binary_mod_pow(self, exp, m)
    }
}

impl ModPowAssign<Natural, Natural> for Natural {
    /// Raises a `Natural` to a `Natural` power mod a `Natural` in place, taking the second and
    /// third `Natural`s by value. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_pow_assign(Natural::from(13u32), Natural::from(497u32));
    /// assert_eq!(x, 445);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_pow_assign(Natural::from(1000u32), Natural::from(30u32));
    /// assert_eq!(x, 10);
    /// ```
    fn mod_pow_assign(&mut self, exp: Natural, m: Natural) {
        *self = _simple_binary_mod_pow(&*self, &exp, &m);
    }
}

impl<'a> ModPowAssign<Natural, &'a Natural> for Natural {
    /// Raises a `Natural` to a `Natural` power mod a `Natural` in place, taking the second
    /// `Natural` by value and the third by reference. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_pow_assign(Natural::from(13u32), &Natural::from(497u32));
    /// assert_eq!(x, 445);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_pow_assign(Natural::from(1000u32), &Natural::from(30u32));
    /// assert_eq!(x, 10);
    /// ```
    fn mod_pow_assign(&mut self, exp: Natural, m: &'a Natural) {
        *self = _simple_binary_mod_pow(&*self, &exp, m);
    }
}

impl<'a> ModPowAssign<&'a Natural, Natural> for Natural {
    /// Raises a `Natural` to a `Natural` power mod a `Natural` in place, taking the second
    /// `Natural` by reference and the third by value. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_pow_assign(&Natural::from(13u32), Natural::from(497u32));
    /// assert_eq!(x, 445);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_pow_assign(&Natural::from(1000u32), Natural::from(30u32));
    /// assert_eq!(x, 10);
    /// ```
    fn mod_pow_assign(&mut self, exp: &'a Natural, m: Natural) {
        *self = _simple_binary_mod_pow(&*self, exp, &m);
    }
}

impl<'a, 'b> ModPowAssign<&'a Natural, &'b Natural> for Natural {
    /// Raises a `Natural` to a `Natural` power mod a `Natural` in place, taking the second and
    /// third `Natural`s by reference. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_pow_assign(&Natural::from(13u32), &Natural::from(497u32));
    /// assert_eq!(x, 445);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_pow_assign(&Natural::from(1000u32), &Natural::from(30u32));
    /// assert_eq!(x, 10);
    /// ```
    fn mod_pow_assign(&mut self, exp: &'a Natural, m: &'b Natural) {
        *self = _simple_binary_mod_pow(&*self, exp, m);
    }
}
