use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign};
use natural::Natural;

impl ModAdd<Natural, Natural> for Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod a `Natural`, taking all three `Natural`s by value.
    /// Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO.mod_add(Natural::from(3u32), Natural::from(5u32)).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_add(Natural::from(5u32), Natural::from(10u32)).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b, c, and m are taken by
    /// value.
    #[inline]
    fn mod_add(mut self, other: Natural, m: Natural) -> Natural {
        self.mod_add_assign(other, m);
        self
    }
}

impl<'a> ModAdd<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod a `Natural`, taking the first two `Natural`s by value
    /// and the last by reference. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO.mod_add(Natural::from(3u32), &Natural::from(5u32)).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_add(Natural::from(5u32), &Natural::from(10u32)).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b and c are taken by value
    /// and m is taken by reference.
    #[inline]
    fn mod_add(mut self, other: Natural, m: &'a Natural) -> Natural {
        self.mod_add_assign(other, m);
        self
    }
}

impl<'a> ModAdd<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod a `Natural`, taking the first and third `Natural`s by
    /// value and the second by reference. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO.mod_add(&Natural::from(3u32), Natural::from(5u32)).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_add(&Natural::from(5u32), Natural::from(10u32)).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b and m are taken by value
    /// and c is taken by reference.
    #[inline]
    fn mod_add(mut self, other: &'a Natural, m: Natural) -> Natural {
        self.mod_add_assign(other, m);
        self
    }
}

impl<'a, 'b> ModAdd<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod a `Natural`, taking the first `Natural` by value and the
    /// other two by reference. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::ZERO.mod_add(&Natural::from(3u32), &Natural::from(5u32)).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     Natural::from(7u32).mod_add(&Natural::from(5u32), &Natural::from(10u32)).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b is taken by value and c and
    /// m are taken by reference.
    #[inline]
    fn mod_add(mut self, other: &'a Natural, m: &'b Natural) -> Natural {
        self.mod_add_assign(other, m);
        self
    }
}

impl<'a> ModAdd<Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod a `Natural`, taking the first `Natural` by reference and
    /// the other two by value. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_add(Natural::from(3u32), Natural::from(5u32)).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_add(Natural::from(5u32), Natural::from(10u32)).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b is taken by reference and c
    /// and m are taken by value.
    #[inline]
    fn mod_add(self, mut other: Natural, m: Natural) -> Natural {
        other.mod_add_assign(self, m);
        other
    }
}

impl<'a, 'b> ModAdd<Natural, &'b Natural> for &'a Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod a `Natural`, taking all the first and third `Natural`s
    /// by reference and the second by value. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_add(Natural::from(3u32), &Natural::from(5u32)).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_add(Natural::from(5u32), &Natural::from(10u32)).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b and m are taken by
    /// reference and c is taken by value.
    #[inline]
    fn mod_add(self, mut other: Natural, m: &'b Natural) -> Natural {
        other.mod_add_assign(self, m);
        other
    }
}

impl<'a, 'b> ModAdd<&'b Natural, Natural> for &'a Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod a `Natural`, taking the first two `Natural`s by
    /// reference and the third by value. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_add(&Natural::from(3u32), Natural::from(5u32)).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_add(&Natural::from(5u32), Natural::from(10u32)).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b and c are taken by
    /// reference and m is taken by value.
    fn mod_add(self, other: &'b Natural, m: Natural) -> Natural {
        let sum = self + other;
        if sum < m {
            sum
        } else {
            sum - m
        }
    }
}

impl<'a, 'b, 'c> ModAdd<&'b Natural, &'c Natural> for &'a Natural {
    type Output = Natural;

    /// Adds a `Natural` to a `Natural` mod a `Natural`, taking all three `Natural`s by reference.
    /// Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::ZERO).mod_add(&Natural::from(3u32), &Natural::from(5u32)).to_string(),
    ///     "3"
    /// );
    /// assert_eq!(
    ///     (&Natural::from(7u32)).mod_add(&Natural::from(5u32), &Natural::from(10u32)).to_string(),
    ///     "2"
    /// );
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b, c, and m are taken by
    /// reference.
    fn mod_add(self, other: &'b Natural, m: &'c Natural) -> Natural {
        let sum = self + other;
        if sum < *m {
            sum
        } else {
            sum - m
        }
    }
}

impl ModAddAssign<Natural, Natural> for Natural {
    /// Adds a `Natural` to a `Natural` mod a `Natural` in place, taking the second and third
    /// `Natural`s by value. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_add_assign(Natural::from(3u32), Natural::from(5u32));
    /// assert_eq!(x.to_string(), "3");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_add_assign(Natural::from(5u32), Natural::from(10u32));
    /// assert_eq!(x.to_string(), "2");
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b, c, and m are taken by
    /// value and a == b.
    fn mod_add_assign(&mut self, other: Natural, m: Natural) {
        *self += other;
        if *self >= m {
            *self -= m;
        }
    }
}

impl<'a> ModAddAssign<Natural, &'a Natural> for Natural {
    /// Adds a `Natural` to a `Natural` mod a `Natural` in place, taking the second `Natural` by
    /// value and the third by reference. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_add_assign(Natural::from(3u32), &Natural::from(5u32));
    /// assert_eq!(x.to_string(), "3");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_add_assign(Natural::from(5u32), &Natural::from(10u32));
    /// assert_eq!(x.to_string(), "2");
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b and c are taken by value,
    /// m is taken by reference, and a == b.
    fn mod_add_assign(&mut self, other: Natural, m: &'a Natural) {
        *self += other;
        if *self >= *m {
            *self -= m;
        }
    }
}

impl<'a> ModAddAssign<&'a Natural, Natural> for Natural {
    /// Adds a `Natural` to a `Natural` mod a `Natural` in place, taking the second `Natural` by
    /// reference and the third by value. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_add_assign(&Natural::from(3u32), Natural::from(5u32));
    /// assert_eq!(x.to_string(), "3");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_add_assign(&Natural::from(5u32), Natural::from(10u32));
    /// assert_eq!(x.to_string(), "2");
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b and m are taken by value,
    /// c is taken by reference, and a == b.
    fn mod_add_assign(&mut self, other: &'a Natural, m: Natural) {
        *self += other;
        if *self >= m {
            *self -= m;
        }
    }
}

impl<'a, 'b> ModAddAssign<&'a Natural, &'b Natural> for Natural {
    /// Adds a `Natural` to a `Natural` mod a `Natural` in place, taking the second and third
    /// `Natural`s by reference. Assumes the inputs are already reduced mod `m`.
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
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::ZERO;
    /// x.mod_add_assign(&Natural::from(3u32), &Natural::from(5u32));
    /// assert_eq!(x.to_string(), "3");
    ///
    /// let mut x = Natural::from(7u32);
    /// x.mod_add_assign(&Natural::from(5u32), &Natural::from(10u32));
    /// assert_eq!(x.to_string(), "2");
    /// ```
    ///
    /// This is _fmpz_mod_addN from fmpz_mod/add.c, FLINT Dev 1, where b is taken by value, c and m
    /// are taken by reference, and a == b.
    fn mod_add_assign(&mut self, other: &'a Natural, m: &'b Natural) {
        *self += other;
        if *self >= *m {
            *self -= m;
        }
    }
}
