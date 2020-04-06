use malachite_base::num::arithmetic::traits::{ModSub, ModSubAssign};

use natural::Natural;

impl ModSub<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod a `Natural`, taking all three `Natural`s by
    /// value. Assumes the inputs are already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32).mod_sub(Natural::from(3u32), Natural::from(5u32)).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_sub(Natural::from(9u32), Natural::from(10u32)).to_string(),
    ///     "8"
    /// );
    /// ```
    #[inline]
    fn mod_sub(mut self, other: Natural, m: Natural) -> Natural {
        self.mod_sub_assign(other, m);
        self
    }
}

impl<'a> ModSub<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod a `Natural`, taking the first two `Natural`s by
    /// value and the modulus by reference. Assumes the inputs are already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32).mod_sub(Natural::from(3u32), &Natural::from(5u32)).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_sub(Natural::from(9u32), &Natural::from(10u32)).to_string(),
    ///     "8"
    /// );
    /// ```
    #[inline]
    fn mod_sub(mut self, other: Natural, m: &'a Natural) -> Natural {
        self.mod_sub_assign(other, m);
        self
    }
}

impl<'a> ModSub<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod a `Natural`, taking the left `Natural` and the
    /// modulus by value and the right `Natural` by reference. Assumes the inputs are already
    /// reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32).mod_sub(&Natural::from(3u32), Natural::from(5u32)).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_sub(&Natural::from(9u32), Natural::from(10u32)).to_string(),
    ///     "8"
    /// );
    /// ```
    #[inline]
    fn mod_sub(mut self, other: &'a Natural, m: Natural) -> Natural {
        self.mod_sub_assign(other, m);
        self
    }
}

impl<'a, 'b> ModSub<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod a `Natural`, taking the left `Natural` by value
    /// and the right `Natural` and modulus by reference. Assumes the inputs are already reduced mod
    /// `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(4u32).mod_sub(&Natural::from(3u32), &Natural::from(5u32)).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_sub(&Natural::from(9u32), &Natural::from(10u32)).to_string(),
    ///     "8"
    /// );
    /// ```
    #[inline]
    fn mod_sub(mut self, other: &'a Natural, m: &'b Natural) -> Natural {
        self.mod_sub_assign(other, m);
        self
    }
}

impl<'a> ModSub<Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod a `Natural`, taking the left `Natural` by
    /// reference and the right `Natural` and modulus by value. Assumes the inputs are already
    /// reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_sub(Natural::from(3u32), Natural::from(5u32)).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_sub(Natural::from(9u32), Natural::from(10u32)).to_string(),
    ///     "8"
    /// );
    /// ```
    fn mod_sub(self, other: Natural, m: Natural) -> Natural {
        if *self >= other {
            self - other
        } else {
            m - other + self
        }
    }
}

impl<'a, 'b> ModSub<Natural, &'b Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod a `Natural`, taking the left `Natural` and
    /// modulus by reference and the right `Natural` by value. Assumes the inputs are already
    /// reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_sub(Natural::from(3u32), &Natural::from(5u32)).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_sub(Natural::from(9u32), &Natural::from(10u32)).to_string(),
    ///     "8"
    /// );
    /// ```
    fn mod_sub(self, other: Natural, m: &'b Natural) -> Natural {
        if *self >= other {
            self - other
        } else {
            m - other + self
        }
    }
}

impl<'a, 'b> ModSub<&'b Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod a `Natural`, taking the first two `Natural`s by
    /// reference and the modulus by value. Assumes the inputs are already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_sub(&Natural::from(3u32), Natural::from(5u32)).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_sub(&Natural::from(9u32), Natural::from(10u32)).to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_subN from fmpz_mod/sub.c, FLINT Dev 1, where b and c are taken by
    /// reference and m is taken by value.
    fn mod_sub(self, other: &'b Natural, m: Natural) -> Natural {
        if self >= other {
            self - other
        } else {
            m - other + self
        }
    }
}

impl<'a, 'b, 'c> ModSub<&'b Natural, &'c Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural` mod a `Natural`, taking all three `Natural`s by
    /// reference. Assumes the inputs are already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(4u32)).mod_sub(&Natural::from(3u32), &Natural::from(5u32)).to_string(),
    ///     "1"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_sub(&Natural::from(9u32), &Natural::from(10u32)).to_string(),
    ///     "8"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_subN from fmpz_mod/sub.c, FLINT Dev 1, where b, c, and m are taken by
    /// reference.
    fn mod_sub(self, other: &'b Natural, m: &'c Natural) -> Natural {
        if self >= other {
            self - other
        } else {
            m - other + self
        }
    }
}

impl ModSubAssign<Natural, Natural> for Natural {
    /// Subtracts a `Natural` from a `Natural` mod a `Natural` in place, taking the right `Natural`
    /// and modulus by value. Assumes the inputs are already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_sub_assign(Natural::from(3u32), Natural::from(5u32));
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_sub_assign(Natural::from(9u32), Natural::from(10u32));
    /// assert_eq!(x.to_string(), "8");
    /// ```
    ///
    /// This is _fmpz_mod_subN from fmpz_mod/sub.c, FLINT Dev 1, where b, c, and m are taken by
    /// value and a == b.
    fn mod_sub_assign(&mut self, other: Natural, m: Natural) {
        if *self >= other {
            *self -= other;
        } else {
            *self += m - other;
        }
    }
}

impl<'a> ModSubAssign<Natural, &'a Natural> for Natural {
    /// Subtracts a `Natural` from a `Natural` mod a `Natural` in place, taking the right `Natural`
    /// by value and the modulus by reference. Assumes the inputs are already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_sub_assign(Natural::from(3u32), &Natural::from(5u32));
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_sub_assign(Natural::from(9u32), &Natural::from(10u32));
    /// assert_eq!(x.to_string(), "8");
    /// ```
    ///
    /// This is _fmpz_mod_subN from fmpz_mod/sub.c, FLINT Dev 1, where b and c are taken by value,
    /// m is taken by reference, and a == b.
    fn mod_sub_assign(&mut self, other: Natural, m: &'a Natural) {
        if *self >= other {
            *self -= other;
        } else {
            *self += m - other;
        }
    }
}

impl<'a> ModSubAssign<&'a Natural, Natural> for Natural {
    /// Subtracts a `Natural` from a `Natural` mod a `Natural` in place, taking the right `Natural`
    /// by reference and the modulus by value. Assumes the inputs are already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_sub_assign(&Natural::from(3u32), Natural::from(5u32));
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_sub_assign(&Natural::from(9u32), Natural::from(10u32));
    /// assert_eq!(x.to_string(), "8");
    /// ```
    ///
    /// This is _fmpz_mod_subN from fmpz_mod/sub.c, FLINT Dev 1, where b and m are taken by value,
    /// c is taken by reference, and a == b.
    fn mod_sub_assign(&mut self, other: &'a Natural, m: Natural) {
        if *self >= *other {
            *self -= other;
        } else {
            *self += m - other;
        }
    }
}

impl<'a, 'b> ModSubAssign<&'a Natural, &'b Natural> for Natural {
    /// Subtracts a `Natural` from a `Natural` mod a `Natural` in place, taking the right `Natural`
    /// and modulus by reference. Assumes the inputs are already reduced mod `m`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `m.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(4u32);
    /// x.mod_sub_assign(&Natural::from(3u32), &Natural::from(5u32));
    /// assert_eq!(x.to_string(), "1");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_sub_assign(&Natural::from(9u32), &Natural::from(10u32));
    /// assert_eq!(x.to_string(), "8");
    /// ```
    ///
    /// This is _fmpz_mod_subN from fmpz_mod/sub.c, FLINT Dev 1, where b is taken by value, c and m
    /// are taken by reference, and a == b.
    fn mod_sub_assign(&mut self, other: &'a Natural, m: &'b Natural) {
        if *self >= *other {
            *self -= other;
        } else {
            *self += m - other;
        }
    }
}
