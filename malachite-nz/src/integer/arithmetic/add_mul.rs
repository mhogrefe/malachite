// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::arithmetic::sub_mul::{
    limbs_overflowing_sub_mul, limbs_overflowing_sub_mul_in_place_left,
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left,
};
use crate::integer::Integer;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::Zero;

impl Natural {
    // self - b * c, returns sign (true means non-negative)
    fn add_mul_limb_neg(&self, b: &Natural, c: Limb) -> (Natural, bool) {
        match (self, b, c) {
            (x, &Natural::ZERO, _) | (x, _, 0) => (x.clone(), true),
            (x, y, 1) if x >= y => (x - y, true),
            (x, y, 1) => (y - x, false),
            (Natural(Large(ref xs)), Natural(Large(ref ys)), z) => {
                let (out_limbs, sign) = limbs_overflowing_sub_mul_limb(xs, ys, z);
                (Natural::from_owned_limbs_asc(out_limbs), sign)
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
            (_, &mut Natural::ZERO, _) | (_, _, 0) => true,
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
            (_, &Natural::ZERO, _) | (_, _, 0) => true,
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
                (Natural::from_owned_limbs_asc(out_limbs), sign)
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
            (&mut Natural::ZERO, y, z) => {
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
            (&mut Natural::ZERO, y, z) => {
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
            (&mut Natural::ZERO, y, z) => {
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
            (&mut Natural::ZERO, y, z) => {
                *self = y * z;
                false
            }
            (_, &Natural(Large(ref ys)), &Natural(Large(ref zs))) => {
                self.add_mul_assign_neg_large(ys, zs)
            }
        }
    }
}

impl AddMul<Integer, Integer> for Integer {
    type Output = Integer;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, taking all three by value.
    ///
    /// $f(x, y, z) = x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10u32).add_mul(Integer::from(3u32), Integer::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12))
    ///         .add_mul(Integer::from(0x10000), -Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    #[inline]
    fn add_mul(mut self, y: Integer, z: Integer) -> Integer {
        self.add_mul_assign(y, z);
        self
    }
}

impl<'a> AddMul<Integer, &'a Integer> for Integer {
    type Output = Integer;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, taking the first two by value
    /// and the third by reference.
    ///
    /// $f(x, y, z) = x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10u32).add_mul(Integer::from(3u32), &Integer::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12))
    ///         .add_mul(Integer::from(0x10000), &-Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    #[inline]
    fn add_mul(mut self, y: Integer, z: &'a Integer) -> Integer {
        self.add_mul_assign(y, z);
        self
    }
}

impl<'a> AddMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, taking the first and third by
    /// value and the second by reference.
    ///
    /// $f(x, y, z) = x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10u32).add_mul(&Integer::from(3u32), Integer::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12))
    ///         .add_mul(&Integer::from(0x10000), -Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    #[inline]
    fn add_mul(mut self, y: &'a Integer, z: Integer) -> Integer {
        self.add_mul_assign(y, z);
        self
    }
}

impl<'a, 'b> AddMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, taking the first by value and
    /// the second and third by reference.
    ///
    /// $f(x, y, z) = x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10u32).add_mul(&Integer::from(3u32), &Integer::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12))
    ///         .add_mul(&Integer::from(0x10000), &-Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    #[inline]
    fn add_mul(mut self, y: &'a Integer, z: &'b Integer) -> Integer {
        self.add_mul_assign(y, z);
        self
    }
}

impl AddMul<&Integer, &Integer> for &Integer {
    type Output = Integer;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, taking all three by
    /// reference.
    ///
    /// $f(x, y, z) = x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(10u32)).add_mul(&Integer::from(3u32), &Integer::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12))
    ///         .add_mul(&Integer::from(0x10000), &-Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    fn add_mul(self, y: &Integer, z: &Integer) -> Integer {
        if self.sign == (y.sign == z.sign) {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(&y.abs, &z.abs),
            }
        } else {
            let (abs, abs_result_sign) = self.abs.add_mul_neg(&y.abs, &z.abs);
            Integer {
                sign: (self.sign == abs_result_sign) || abs == 0,
                abs,
            }
        }
    }
}

impl AddMulAssign<Integer, Integer> for Integer {
    /// Adds the product of two other [`Integer`]s to an [`Integer`] in place, taking both
    /// [`Integer`]s on the right-hand side by value.
    ///
    /// $x \gets x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.add_mul_assign(Integer::from(3u32), Integer::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.add_mul_assign(Integer::from(0x10000), -Integer::from(10u32).pow(12));
    /// assert_eq!(x, -65537000000000000i64);
    /// ```
    fn add_mul_assign(&mut self, y: Integer, z: Integer) {
        if self.sign == (y.sign == z.sign) {
            self.abs.add_mul_assign(y.abs, z.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg(y.abs, z.abs);
            self.sign = (self.sign == sign) || self.abs == 0;
        }
    }
}

impl<'a> AddMulAssign<Integer, &'a Integer> for Integer {
    /// Adds the product of two other [`Integer`]s to an [`Integer`] in place, taking the first
    /// [`Integer`] on the right-hand side by value and the second by reference.
    ///
    /// $x \gets x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.add_mul_assign(Integer::from(3u32), &Integer::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.add_mul_assign(Integer::from(0x10000), &-Integer::from(10u32).pow(12));
    /// assert_eq!(x, -65537000000000000i64);
    /// ```
    fn add_mul_assign(&mut self, y: Integer, z: &'a Integer) {
        if self.sign == (y.sign == z.sign) {
            self.abs.add_mul_assign(y.abs, &z.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_val_ref(y.abs, &z.abs);
            self.sign = (self.sign == sign) || self.abs == 0;
        }
    }
}

impl<'a> AddMulAssign<&'a Integer, Integer> for Integer {
    /// Adds the product of two other [`Integer`]s to an [`Integer`] in place, taking the first
    /// [`Integer`] on the right-hand side by reference and the second by value.
    ///
    /// $x \gets x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.add_mul_assign(&Integer::from(3u32), Integer::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.add_mul_assign(&Integer::from(0x10000), -Integer::from(10u32).pow(12));
    /// assert_eq!(x, -65537000000000000i64);
    /// ```
    fn add_mul_assign(&mut self, y: &'a Integer, z: Integer) {
        if self.sign == (y.sign == z.sign) {
            self.abs.add_mul_assign(&y.abs, z.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_ref_val(&y.abs, z.abs);
            self.sign = (self.sign == sign) || self.abs == 0;
        }
    }
}

impl<'a, 'b> AddMulAssign<&'a Integer, &'b Integer> for Integer {
    /// Adds the product of two other [`Integer`]s to an [`Integer`] in place, taking both
    /// [`Integer`]s on the right-hand side by reference.
    ///
    /// $x \gets x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.add_mul_assign(&Integer::from(3u32), &Integer::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.add_mul_assign(&Integer::from(0x10000), &-Integer::from(10u32).pow(12));
    /// assert_eq!(x, -65537000000000000i64);
    /// ```
    fn add_mul_assign(&mut self, y: &'a Integer, z: &'b Integer) {
        if self.sign == (y.sign == z.sign) {
            self.abs.add_mul_assign(&y.abs, &z.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_ref_ref(&y.abs, &z.abs);
            self.sign = (self.sign == sign) || self.abs == 0;
        }
    }
}
