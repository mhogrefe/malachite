use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};

use integer::arithmetic::sub_mul::{
    limbs_overflowing_sub_mul, limbs_overflowing_sub_mul_in_place_left,
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left,
};
use integer::Integer;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

impl<'a> AddMul<Integer, Integer> for Integer {
    type Output = Integer;

    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking
    /// `self`, b, and c by value.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), Integer::from(4u32)), 22);
    /// assert_eq!((-Integer::trillion()).add_mul(Integer::from(0x1_0000),
    ///     -Integer::trillion()).to_string(), "-65537000000000000");
    /// ```
    #[inline]
    fn add_mul(mut self, b: Integer, c: Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a> AddMul<Integer, &'a Integer> for Integer {
    type Output = Integer;

    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`
    /// and b by value and c by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), &Integer::from(4u32)), 22);
    /// assert_eq!((-Integer::trillion()).add_mul(Integer::from(0x1_0000),
    ///     &(-Integer::trillion())).to_string(), "-65537000000000000");
    /// ```
    #[inline]
    fn add_mul(mut self, b: Integer, c: &'a Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a> AddMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`
    /// and c by value and b by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), Integer::from(4u32)), 22);
    /// assert_eq!((-Integer::trillion()).add_mul(&Integer::from(0x1_0000),
    ///     -Integer::trillion()).to_string(), "-65537000000000000");
    /// ```
    #[inline]
    fn add_mul(mut self, b: &'a Integer, c: Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a, 'b> AddMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`
    /// by value and b and c by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), &Integer::from(4u32)), 22);
    /// assert_eq!((-Integer::trillion()).add_mul(&Integer::from(0x1_0000),
    ///     &(-Integer::trillion())).to_string(), "-65537000000000000");
    /// ```
    #[inline]
    fn add_mul(mut self, b: &'a Integer, c: &'b Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a, 'b, 'c> AddMul<&'a Integer, &'b Integer> for &'c Integer {
    type Output = Integer;

    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking
    /// `self`, b, and c by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(m + n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), &Integer::from(4u32)), 22);
    /// assert_eq!((&(-Integer::trillion())).add_mul(&Integer::from(0x1_0000),
    ///     &(-Integer::trillion())).to_string(), "-65537000000000000");
    /// ```
    fn add_mul(self, b: &'a Integer, c: &'b Integer) -> Integer {
        if self.sign == (b.sign == c.sign) {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(&b.abs, &c.abs),
            }
        } else {
            let (abs, abs_result_sign) = self.abs.add_mul_neg(&b.abs, &c.abs);
            Integer {
                sign: (self.sign == abs_result_sign) || abs == 0,
                abs,
            }
        }
    }
}

impl AddMulAssign<Integer, Integer> for Integer {
    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place,
    /// taking b and c by value.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.add_mul_assign(Integer::from(3u32), Integer::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::trillion();
    /// x.add_mul_assign(Integer::from(0x1_0000), -Integer::trillion());
    /// assert_eq!(x.to_string(), "-65537000000000000");
    /// ```
    fn add_mul_assign(&mut self, b: Integer, c: Integer) {
        if self.sign == (b.sign == c.sign) {
            self.abs.add_mul_assign(b.abs, c.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg(b.abs, c.abs);
            self.sign = (self.sign == sign) || self.abs == 0;
        }
    }
}

impl<'a> AddMulAssign<Integer, &'a Integer> for Integer {
    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place,
    /// taking b by value and c by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.add_mul_assign(Integer::from(3u32), &Integer::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::trillion();
    /// x.add_mul_assign(Integer::from(0x1_0000), &(-Integer::trillion()));
    /// assert_eq!(x.to_string(), "-65537000000000000");
    /// ```
    fn add_mul_assign(&mut self, b: Integer, c: &'a Integer) {
        if self.sign == (b.sign == c.sign) {
            self.abs.add_mul_assign(b.abs, &c.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_val_ref(b.abs, &c.abs);
            self.sign = (self.sign == sign) || self.abs == 0;
        }
    }
}

impl<'a> AddMulAssign<&'a Integer, Integer> for Integer {
    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place,
    /// taking b by reference and c by value.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.add_mul_assign(&Integer::from(3u32), Integer::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::trillion();
    /// x.add_mul_assign(&Integer::from(0x1_0000), -Integer::trillion());
    /// assert_eq!(x.to_string(), "-65537000000000000");
    /// ```
    fn add_mul_assign(&mut self, b: &'a Integer, c: Integer) {
        if self.sign == (b.sign == c.sign) {
            self.abs.add_mul_assign(&b.abs, c.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_ref_val(&b.abs, c.abs);
            self.sign = (self.sign == sign) || self.abs == 0;
        }
    }
}

impl<'a, 'b> AddMulAssign<&'a Integer, &'b Integer> for Integer {
    /// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place,
    /// taking b and c by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`b.significant_bits()`, `c.significant_bits()`)
    ///       m = `a.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.add_mul_assign(&Integer::from(3u32), &Integer::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::trillion();
    /// x.add_mul_assign(&Integer::from(0x1_0000), &(-Integer::trillion()));
    /// assert_eq!(x.to_string(), "-65537000000000000");
    /// ```
    fn add_mul_assign(&mut self, b: &'a Integer, c: &'b Integer) {
        if self.sign == (b.sign == c.sign) {
            self.abs.add_mul_assign(&b.abs, &c.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_ref_ref(&b.abs, &c.abs);
            self.sign = (self.sign == sign) || self.abs == 0;
        }
    }
}

impl Natural {
    // self - b * c, returns sign (true means non-negative)
    fn add_mul_limb_neg(&self, b: &Natural, c: Limb) -> (Natural, bool) {
        match (self, b, c) {
            (x, natural_zero!(), _) | (x, _, 0) => (x.clone(), true),
            (x, y, 1) if x >= y => (x - y, true),
            (x, y, 1) => (y - x, false),
            (Natural(Large(ref xs)), Natural(Large(ref ys)), z) => {
                let (out_limbs, sign) = limbs_overflowing_sub_mul_limb(xs, ys, z);
                let mut out = Natural(Large(out_limbs));
                out.trim();
                (out, sign)
            }
            (x, y, z) => {
                let yz = y * Natural::from(z);
                if *x >= yz {
                    (x - yz, true)
                } else {
                    (yz - x, false)
                }
            }
        }
    }

    // self -= b * c, returns sign (true means non-negative)
    fn add_mul_assign_limb_neg(&mut self, mut b: Natural, c: Limb) -> bool {
        match (&mut *self, &mut b, c) {
            (_, &mut natural_zero!(), _) | (_, _, 0) => true,
            (x, y, 1) if *x >= *y => {
                self.sub_assign_no_panic(b);
                true
            }
            (x, y, 1) => {
                x.sub_right_assign_no_panic(&*y);
                false
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys)), z) => {
                let (right, sign) = limbs_overflowing_sub_mul_limb_in_place_either(xs, ys, z);
                if right {
                    b.trim();
                    *self = b;
                } else {
                    self.trim();
                }
                sign
            }
            (x, _, z) => {
                let yz = b * Natural(Small(z));
                let sign = *x >= yz;
                if sign {
                    x.sub_assign_no_panic(yz);
                } else {
                    x.sub_right_assign_no_panic(&yz);
                }
                sign
            }
        }
    }

    // self -= &b * c, returns sign (true means non-negative)
    fn add_mul_assign_limb_neg_ref(&mut self, b: &Natural, c: Limb) -> bool {
        match (&mut *self, b, c) {
            (_, &natural_zero!(), _) | (_, _, 0) => true,
            (x, y, 1) if *x >= *y => {
                self.sub_assign_ref_no_panic(y);
                true
            }
            (x, y, 1) => {
                x.sub_right_assign_no_panic(y);
                false
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), z) => {
                let sign = limbs_overflowing_sub_mul_limb_in_place_left(xs, ys, z);
                self.trim();
                sign
            }
            (x, _, z) => {
                let yz = b * Natural(Small(z));
                let sign = *x >= yz;
                if sign {
                    x.sub_assign_no_panic(yz);
                } else {
                    x.sub_right_assign_no_panic(&yz);
                }
                sign
            }
        }
    }

    // self - &b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_neg(&self, b: &Natural, c: &Natural) -> (Natural, bool) {
        match (self, b, c) {
            (x, &Natural(Small(y)), z) => x.add_mul_limb_neg(z, y),
            (x, y, &Natural(Small(z))) => x.add_mul_limb_neg(y, z),
            (&Natural(Small(x)), y, z) => ((y * z).sub_limb(x), false),
            (Natural(Large(ref xs)), Natural(Large(ref ys)), Natural(Large(ref zs))) => {
                let (out_limbs, sign) = limbs_overflowing_sub_mul(xs, ys, zs);
                let mut out = Natural(Large(out_limbs));
                out.trim();
                (out, sign)
            }
        }
    }

    fn add_mul_assign_neg_large(&mut self, ys: &[Limb], zs: &[Limb]) -> bool {
        let xs = self.promote_in_place();
        let sign = limbs_overflowing_sub_mul_in_place_left(xs, ys, zs);
        self.trim();
        sign
    }

    // self -= b * c, returns sign (true means non-negative)
    fn add_mul_assign_neg(&mut self, b: Natural, c: Natural) -> bool {
        match (&mut *self, b, c) {
            (x, Natural(Small(y)), z) => x.add_mul_assign_limb_neg(z, y),
            (x, y, Natural(Small(z))) => x.add_mul_assign_limb_neg(y, z),
            (&mut natural_zero!(), y, z) => {
                *self = y * z;
                false
            }
            (_, Natural(Large(ref ys)), Natural(Large(ref zs))) => {
                self.add_mul_assign_neg_large(ys, zs)
            }
        }
    }

    // self -= b * &c, returns sign (true means non-negative)
    fn add_mul_assign_neg_val_ref(&mut self, b: Natural, c: &Natural) -> bool {
        match (&mut *self, b, c) {
            (x, Natural(Small(y)), z) => x.add_mul_assign_limb_neg_ref(z, y),
            (x, y, &Natural(Small(z))) => x.add_mul_assign_limb_neg(y, z),
            (&mut natural_zero!(), y, z) => {
                *self = y * z;
                false
            }
            (_, Natural(Large(ref ys)), &Natural(Large(ref zs))) => {
                self.add_mul_assign_neg_large(ys, zs)
            }
        }
    }

    // self -= &b * c, returns sign (true means non-negative)
    fn add_mul_assign_neg_ref_val(&mut self, b: &Natural, c: Natural) -> bool {
        match (&mut *self, b, c) {
            (x, &Natural(Small(y)), z) => x.add_mul_assign_limb_neg(z, y),
            (x, y, Natural(Small(z))) => x.add_mul_assign_limb_neg_ref(y, z),
            (&mut natural_zero!(), y, z) => {
                *self = y * z;
                false
            }
            (_, &Natural(Large(ref ys)), Natural(Large(ref zs))) => {
                self.add_mul_assign_neg_large(ys, zs)
            }
        }
    }

    // self -= &b * &c, returns sign (true means non-negative)
    fn add_mul_assign_neg_ref_ref(&mut self, b: &Natural, c: &Natural) -> bool {
        match (&mut *self, b, c) {
            (x, &Natural(Small(y)), z) => x.add_mul_assign_limb_neg_ref(z, y),
            (x, y, &Natural(Small(z))) => x.add_mul_assign_limb_neg_ref(y, z),
            (&mut natural_zero!(), y, z) => {
                *self = y * z;
                false
            }
            (_, &Natural(Large(ref ys)), &Natural(Large(ref zs))) => {
                self.add_mul_assign_neg_large(ys, zs)
            }
        }
    }
}
