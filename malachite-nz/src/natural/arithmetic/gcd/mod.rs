use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign, Parity};
use natural::arithmetic::eq_mod::_limbs_mod_exact_odd_limb;
use natural::arithmetic::mod_op::_limbs_mod_limb_alt_2;
use natural::Natural;
use platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};
use std::cmp::min;

/// This is MPN_MOD_OR_MODEXACT_1_ODD from gmp-impl.h, GMP 6.2.1, where size > 1.
fn limbs_mod_or_modexact(ns: &[Limb], d: Limb) -> Limb {
    if ns.len() < BMOD_1_TO_MOD_1_THRESHOLD {
        _limbs_mod_exact_odd_limb(ns, d, 0)
    } else {
        _limbs_mod_limb_alt_2(ns, d)
    }
}

/// This is mpn_gcd_1 from mpn/generic/gcd_1.c, GMP 6.2.1.
#[doc(hidden)]
pub fn limbs_gcd_limb(xs: &[Limb], mut y: Limb) -> Limb {
    assert!(xs.len() > 1);
    assert_ne!(y, 0);
    let mut x = xs[0];
    let mut zeros = y.trailing_zeros();
    y >>= zeros;
    if x != 0 {
        zeros = min(zeros, x.trailing_zeros());
    }
    x = limbs_mod_or_modexact(xs, y);
    if x != 0 {
        y.gcd_assign(x >> x.trailing_zeros());
    }
    y << zeros
}

pub fn gcd_euclidean_nz(x: Natural, y: Natural) -> Natural {
    if y == 0 {
        x
    } else {
        let r = x % &y;
        gcd_euclidean_nz(y, r)
    }
}

// recursive implementation overflows stack, so using a loop instead
pub fn gcd_binary_nz(mut x: Natural, mut y: Natural) -> Natural {
    let mut twos = 0;
    loop {
        if x == y {
            return x << twos;
        } else if x == 0 {
            return y << twos;
        } else if y == 0 {
            return x << twos;
        } else if x.even() {
            x >>= 1;
            if y.even() {
                y >>= 1;
                twos += 1;
            }
        } else if y.even() {
            y >>= 1;
        } else if x > y {
            x -= &y;
            x >>= 1;
        } else {
            y -= &x;
            y >>= 1;
        }
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
        gcd_binary_nz(self.clone(), other.clone())
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
        *self = gcd_binary_nz(self.clone(), other);
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
        *self = gcd_binary_nz(self.clone(), other.clone());
    }
}

pub mod half_gcd;
pub mod matrix_2_2;
