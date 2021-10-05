use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign, Gcd, Lcm, LcmAssign};
use malachite_base::num::basic::traits::Zero;
use natural::Natural;

impl Lcm<Natural> for Natural {
    type Output = Natural;

    /// Computes the LCM (least common multiple) of two `Natural`s, taking both `Natural`s by
    /// value.
    ///
    /// $$
    /// f(x, y) = \operatorname{lcm}(x, y).
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).lcm(Natural::from(5u32)), 15);
    /// assert_eq!(Natural::from(12u32).lcm(Natural::from(90u32)), 180);
    /// ```
    fn lcm(mut self, other: Natural) -> Natural {
        self.lcm_assign(other);
        self
    }
}

impl<'a> Lcm<&'a Natural> for Natural {
    type Output = Natural;

    /// Computes the LCM (least common multiple) of two `Natural`s, taking the first `Natural` by
    /// value and the second by reference.
    ///
    /// $$
    /// f(x, y) = \operatorname{lcm}(x, y).
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).lcm(&Natural::from(5u32)), 15);
    /// assert_eq!(Natural::from(12u32).lcm(&Natural::from(90u32)), 180);
    /// ```
    #[inline]
    fn lcm(mut self, other: &'a Natural) -> Natural {
        self.lcm_assign(other);
        self
    }
}

impl<'a> Lcm<Natural> for &'a Natural {
    type Output = Natural;

    /// Computes the LCM (least common multiple) of two `Natural`s, taking the first `Natural` by
    /// reference and the second by value.
    ///
    /// $$
    /// f(x, y) = \operatorname{lcm}(x, y).
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).lcm(Natural::from(5u32)), 15);
    /// assert_eq!((&Natural::from(12u32)).lcm(Natural::from(90u32)), 180);
    /// ```
    #[inline]
    fn lcm(self, mut other: Natural) -> Natural {
        other.lcm_assign(self);
        other
    }
}

impl<'a, 'b> Lcm<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Computes the LCM (least common multiple) of two `Natural`s, taking both `Natural`s by
    /// reference.
    ///
    /// $$
    /// f(x, y) = \operatorname{lcm}(x, y).
    /// $$
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::Lcm;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).lcm(&Natural::from(5u32)), 15);
    /// assert_eq!((&Natural::from(12u32)).lcm(&Natural::from(90u32)), 180);
    /// ```
    #[inline]
    fn lcm(self, other: &'a Natural) -> Natural {
        if *self == 0 || *other == 0 {
            return Natural::ZERO;
        }
        let gcd = self.gcd(other);
        if self >= other {
            self.div_exact(gcd) * other
        } else {
            other.div_exact(gcd) * self
        }
    }
}

impl LcmAssign<Natural> for Natural {
    /// Replaces a `Natural` by its LCM (least common multiple) with another `Natural`, taking the
    /// `Natural` on the right-hand side by value.
    ///
    /// $$
    /// x \gets \operatorname{lcm}(x, y).
    /// $$
    /// 
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::LcmAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.lcm_assign(Natural::from(5u32));
    /// assert_eq!(x, 15);
    ///
    /// let mut x = Natural::from(12u32);
    /// x.lcm_assign(Natural::from(90u32));
    /// assert_eq!(x, 180);
    /// ```
    #[inline]
    fn lcm_assign(&mut self, other: Natural) {
        if *self == 0 {
            return;
        } else if other == 0 {
            *self = Natural::ZERO;
            return;
        }
        self.div_exact_assign((&*self).gcd(&other));
        *self *= other;
    }
}

impl<'a> LcmAssign<&'a Natural> for Natural {
    /// Replaces a `Natural` by its LCM (least common multiple) with another `Natural`, taking the
    /// `Natural` on the right-hand side by reference.
    ///
    /// $$
    /// x \gets \operatorname{lcm}(x, y).
    /// $$
    /// 
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::LcmAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.lcm_assign(&Natural::from(5u32));
    /// assert_eq!(x, 15);
    ///
    /// let mut x = Natural::from(12u32);
    /// x.lcm_assign(&Natural::from(90u32));
    /// assert_eq!(x, 180);
    /// ```
    #[inline]
    fn lcm_assign(&mut self, other: &'a Natural) {
        if *self == 0 {
            return;
        } else if *other == 0 {
            *self = Natural::ZERO;
            return;
        }
        self.div_exact_assign((&*self).gcd(other));
        *self *= other;
    }
}