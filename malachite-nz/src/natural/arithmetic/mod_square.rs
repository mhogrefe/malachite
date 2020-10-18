use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign, ModSquare, ModSquareAssign};
use malachite_base::num::basic::traits::Two;
use natural::Natural;

impl ModSquare<Natural> for Natural {
    type Output = Natural;

    /// Squares a `Natural` mod a `Natural`, taking both `Natural`s by value. Assumes the base is
    /// already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(2u32).mod_square(Natural::from(10u32)), 4);
    /// assert_eq!(Natural::from(100u32).mod_square(Natural::from(497u32)), 60);
    /// ```
    fn mod_square(self, m: Natural) -> Natural {
        (&self).mod_pow(&Natural::TWO, &m)
    }
}

impl<'a> ModSquare<&'a Natural> for Natural {
    type Output = Natural;

    /// Squares a `Natural` mod a `Natural`, taking the first `Natural`s by value and the second by
    /// reference. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(2u32).mod_square(&Natural::from(10u32)), 4);
    /// assert_eq!(Natural::from(100u32).mod_square(&Natural::from(497u32)), 60);
    /// ```
    fn mod_square(self, m: &'a Natural) -> Natural {
        (&self).mod_pow(&Natural::TWO, m)
    }
}

impl<'a> ModSquare<Natural> for &'a Natural {
    type Output = Natural;

    /// Squares a `Natural` mod a `Natural`, taking the first `Natural` by reference and the second
    /// by value. Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(2u32)).mod_square(Natural::from(10u32)), 4);
    /// assert_eq!((&Natural::from(100u32)).mod_square(Natural::from(497u32)), 60);
    /// ```
    fn mod_square(self, m: Natural) -> Natural {
        self.mod_pow(&Natural::TWO, &m)
    }
}

impl<'a, 'b> ModSquare<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Squares a `Natural` mod a `Natural`, taking both `Natural`s by reference. Assumes the base
    /// is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(2u32)).mod_square(&Natural::from(10u32)), 4);
    /// assert_eq!((&Natural::from(100u32)).mod_square(&Natural::from(497u32)), 60);
    /// ```
    fn mod_square(self, m: &'b Natural) -> Natural {
        self.mod_pow(&Natural::TWO, m)
    }
}

impl ModSquareAssign<Natural> for Natural {
    /// Squares a `Natural` mod a `Natural` in place, taking the second `Natural` by value. Assumes
    /// the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(2u32);
    /// x.mod_square_assign(Natural::from(10u32));
    /// assert_eq!(x, 4);
    ///
    /// let mut x = Natural::from(100u32);
    /// x.mod_square_assign(Natural::from(497u32));
    /// assert_eq!(x, 60);
    /// ```
    #[inline]
    fn mod_square_assign(&mut self, m: Natural) {
        self.mod_pow_assign(&Natural::TWO, &m);
    }
}

impl<'a> ModSquareAssign<&'a Natural> for Natural {
    /// Squares a `Natural` mod a `Natural` in place, taking the second `Natural` by reference.
    /// Assumes the base is already reduced mod `m`.
    ///
    /// //TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(2u32);
    /// x.mod_square_assign(&Natural::from(10u32));
    /// assert_eq!(x, 4);
    ///
    /// let mut x = Natural::from(100u32);
    /// x.mod_square_assign(&Natural::from(497u32));
    /// assert_eq!(x, 60);
    /// ```
    #[inline]
    fn mod_square_assign(&mut self, m: &'a Natural) {
        self.mod_pow_assign(&Natural::TWO, m);
    }
}
