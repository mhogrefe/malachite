use malachite_base::num::arithmetic::traits::{CoprimeWith, DivisibleBy, Gcd, Parity};
use natural::Natural;

#[doc(hidden)]
pub fn coprime_with_check_2(x: Natural, y: Natural) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == 1
}

#[doc(hidden)]
pub fn coprime_with_check_2_3(x: Natural, y: Natural) -> bool {
    (x.odd() || y.odd())
        && (!(&x).divisible_by(Natural::from(3u32)) || !(&y).divisible_by(Natural::from(3u32)))
        && x.gcd(y) == 1
}

#[doc(hidden)]
pub fn coprime_with_check_2_3_5(x: Natural, y: Natural) -> bool {
    if x.even() && y.even() {
        false
    } else {
        let x15 = &x % Natural::from(15u32);
        let y15 = &y % Natural::from(15u32);
        if (x15 == 0 || x15 == 3 || x15 == 6 || x15 == 9 || x15 == 12)
            && (y15 == 0 || y15 == 3 || y15 == 6 || y15 == 9 || y15 == 12)
        {
            return false;
        }
        if (x15 == 0 || x15 == 5 || x15 == 10) && (y15 == 0 || y15 == 5 || y15 == 10) {
            return false;
        }
        x.gcd(y) == 1
    }
}

#[doc(hidden)]
pub fn coprime_with_check_2_val_ref(x: Natural, y: &Natural) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == 1
}

#[doc(hidden)]
pub fn coprime_with_check_2_ref_val(x: &Natural, y: Natural) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == 1
}

#[doc(hidden)]
pub fn coprime_with_check_2_ref_ref(x: &Natural, y: &Natural) -> bool {
    (x.odd() || y.odd()) && x.gcd(y) == 1
}

impl CoprimeWith<Natural> for Natural {
    /// Returns whether two `Natural`s are coprime; that is, whether they have no common factor
    /// other than 1. Both `Natural`s are taken by value.
    ///
    /// Every number is coprime with 1. No number is coprime with 0, except 1.
    ///
    /// $f(x, y) = (\gcd(x, y) = 1)$.
    ///
    /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CoprimeWith;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).coprime_with(Natural::from(5u32)), true);
    /// assert_eq!(Natural::from(12u32).coprime_with(Natural::from(90u32)), false);
    /// ```
    #[inline]
    fn coprime_with(self, other: Natural) -> bool {
        coprime_with_check_2(self, other)
    }
}

impl<'a> CoprimeWith<&'a Natural> for Natural {
    /// Returns whether two `Natural`s are coprime; that is, whether they have no common factor
    /// other than 1. The first `Natural` is taken by value and the second by reference.
    ///
    /// Every number is coprime with 1. No number is coprime with 0, except 1.
    ///
    /// $f(x, y) = (\gcd(x, y) = 1)$.
    ///
    /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CoprimeWith;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).coprime_with(&Natural::from(5u32)), true);
    /// assert_eq!(Natural::from(12u32).coprime_with(&Natural::from(90u32)), false);
    /// ```
    #[inline]
    fn coprime_with(self, other: &'a Natural) -> bool {
        coprime_with_check_2_val_ref(self, other)
    }
}

impl<'a> CoprimeWith<Natural> for &'a Natural {
    /// Returns whether two `Natural`s are coprime; that is, whether they have no common factor
    /// other than 1. The first `Natural` is taken by reference and the second by value.
    ///
    /// Every number is coprime with 1. No number is coprime with 0, except 1.
    ///
    /// $f(x, y) = (\gcd(x, y) = 1)$.
    ///
    /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CoprimeWith;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).coprime_with(Natural::from(5u32)), true);
    /// assert_eq!((&Natural::from(12u32)).coprime_with(Natural::from(90u32)), false);
    /// ```
    #[inline]
    fn coprime_with(self, other: Natural) -> bool {
        coprime_with_check_2_ref_val(self, other)
    }
}

impl<'a, 'b> CoprimeWith<&'b Natural> for &'a Natural {
    /// Returns whether two `Natural`s are coprime; that is, whether they have no common factor
    /// other than 1. Both `Natural`s are taken by reference.
    ///
    /// Every number is coprime with 1. No number is coprime with 0, except 1.
    ///
    /// $f(x, y) = (\gcd(x, y) = 1)$.
    ///
    /// $f(x, y) = ((k,m,n \in \N \land x=km \land y=kn) \implies k=1)$.
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::CoprimeWith;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).coprime_with(Natural::from(5u32)), true);
    /// assert_eq!((&Natural::from(12u32)).coprime_with(Natural::from(90u32)), false);
    /// ```
    fn coprime_with(self, other: &'b Natural) -> bool {
        coprime_with_check_2_ref_ref(self, other)
    }
}
