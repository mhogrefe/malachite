use natural::Natural;
use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign, Parity};

pub fn _gcd_euclidean(x: Natural, y: Natural) -> Natural {
    if y == 0 {
        x
    } else {
        let r = x % &y;
        _gcd_euclidean(y, r)
    }
}

pub fn _gcd_binary(x: Natural, y: Natural) -> Natural {
    if x == y {
        x
    } else if x == 0 {
        y
    } else if y == 0 {
        x
    } else if x.even() {
        if y.odd() {
            _gcd_binary(x >> 1, y)
        } else {
            _gcd_binary(x >> 1, y >> 1) << 1
        }
    } else if y.even() {
        _gcd_binary(x, y >> 1)
    } else if x > y {
        let u = (x - &y) >> 1;
        _gcd_binary(u, y)
    } else {
        let u = (y - &x) >> 1;
        _gcd_binary(u, x)
    }
}

impl Gcd<Natural> for Natural {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two `Natural`s, taking both `Natural`s by
    /// value.
    /// 
    /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which makes sense
    /// if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).gcd(Natural::from(5u32)), 1);
    /// assert_eq!(Natural::from(12u32).gcd(Natural::from(90u32)), 6);
    /// ```
    fn gcd(mut self, other: Natural) -> Natural {
        self.gcd_assign(other);
        self
    }
}

impl<'a> Gcd<&'a Natural> for Natural {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two `Natural`s, taking the first `Natural` by
    /// value and the second by reference.
    /// 
    /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which makes sense
    /// if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).gcd(&Natural::from(5u32)), 1);
    /// assert_eq!(Natural::from(12u32).gcd(&Natural::from(90u32)), 6);
    /// ```
    #[inline]
    fn gcd(mut self, other: &'a Natural) -> Natural {
        self.gcd_assign(other);
        self
    }
}

impl<'a> Gcd<Natural> for &'a Natural {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two `Natural`s, taking the first `Natural` by
    /// reference and the second by value.
    /// 
    /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which makes sense
    /// if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).gcd(Natural::from(5u32)), 1);
    /// assert_eq!((&Natural::from(12u32)).gcd(Natural::from(90u32)), 6);
    /// ```
    #[inline]
    fn gcd(self, mut other: Natural) -> Natural {
        other.gcd_assign(self);
        other
    }
}

impl<'a, 'b> Gcd<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two `Natural`s, taking both `Natural`s by
    /// reference.
    /// 
    /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which makes sense
    /// if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).gcd(&Natural::from(5u32)), 1);
    /// assert_eq!((&Natural::from(12u32)).gcd(&Natural::from(90u32)), 6);
    /// ```
    #[inline]
    fn gcd(self, other: &'a Natural) -> Natural {
        _gcd_binary(self.clone(), other.clone())
    }
}

impl GcdAssign<Natural> for Natural {
    /// Replaces a `Natural` by its GCD (greatest common divisor) with another `Natural`, taking
    /// the `Natural` on the right-hand side by value.
    ///
    /// $$
    /// x \gets \gcd(x, y).
    /// $$
    /// 
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.gcd_assign(Natural::from(5u32));
    /// assert_eq!(x, 1);
    /// 
    /// let mut x = Natural::from(12u32);
    /// x.gcd_assign(Natural::from(90u32));
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    fn gcd_assign(&mut self, other: Natural) {
        *self = _gcd_binary(self.clone(), other);
    }
}

impl<'a> GcdAssign<&'a Natural> for Natural {
    /// Replaces a `Natural` by its GCD (greatest common divisor) with another `Natural`, taking
    /// the `Natural` on the right-hand side by reference.
    ///
    /// $$
    /// x \gets \gcd(x, y).
    /// $$
    /// 
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.gcd_assign(&Natural::from(5u32));
    /// assert_eq!(x, 1);
    /// 
    /// let mut x = Natural::from(12u32);
    /// x.gcd_assign(&Natural::from(90u32));
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    fn gcd_assign(&mut self, other: &'a Natural) {
        *self = _gcd_binary(self.clone(), other.clone());
    }
}
