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

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`, b,
/// and c by value.
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
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), Integer::from(4u32)), 22);
///     assert_eq!((-Integer::trillion()).add_mul(Integer::from(0x1_0000),
///         -Integer::trillion()).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMul<Integer, Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: Integer, c: Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` and
/// b by value and c by reference.
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
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!((-Integer::trillion()).add_mul(Integer::from(0x1_0000),
///         &(-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMul<Integer, &'a Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: Integer, c: &'a Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` and
/// c by value and b by reference.
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
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), Integer::from(4u32)), 22);
///     assert_eq!((-Integer::trillion()).add_mul(&Integer::from(0x1_0000),
///         -Integer::trillion()).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: &'a Integer, c: Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` by
/// value and b and c by reference.
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
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!((-Integer::trillion()).add_mul(&Integer::from(0x1_0000),
///         &(-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: &'a Integer, c: &'b Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`, b,
/// and c by reference.
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
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!((&(-Integer::trillion())).add_mul(&Integer::from(0x1_0000),
///         &(-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b, 'c> AddMul<&'a Integer, &'b Integer> for &'c Integer {
    type Output = Integer;

    fn add_mul(self, b: &'a Integer, c: &'b Integer) -> Integer {
        if self.sign == (b.sign == c.sign) {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(&b.abs, &c.abs),
            }
        } else {
            let (abs, abs_result_sign) = self.abs.add_mul_neg(&b.abs, &c.abs);
            Integer {
                sign: (self.sign == abs_result_sign) || abs == 0 as Limb,
                abs,
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b and c by value.
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
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(Integer::from(0x1_0000), -Integer::trillion());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl AddMulAssign<Integer, Integer> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: Integer) {
        if self.sign == (b.sign == c.sign) {
            self.abs.add_mul_assign(b.abs, c.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg(b.abs, c.abs);
            self.sign = (self.sign == sign) || self.abs == 0 as Limb;
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b by value and c by reference.
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
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), &Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(Integer::from(0x1_0000), &(-Integer::trillion()));
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMulAssign<Integer, &'a Integer> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: &'a Integer) {
        if self.sign == (b.sign == c.sign) {
            self.abs.add_mul_assign(b.abs, &c.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_val_ref(b.abs, &c.abs);
            self.sign = (self.sign == sign) || self.abs == 0 as Limb;
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b by reference and c by value.
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
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(&Integer::from(0x1_0000), -Integer::trillion());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMulAssign<&'a Integer, Integer> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: Integer) {
        if self.sign == (b.sign == c.sign) {
            self.abs.add_mul_assign(&b.abs, c.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_ref_val(&b.abs, c.abs);
            self.sign = (self.sign == sign) || self.abs == 0 as Limb;
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b and c by reference.
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
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), &Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(&Integer::from(0x1_0000), &(-Integer::trillion()));
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b> AddMulAssign<&'a Integer, &'b Integer> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: &'b Integer) {
        if self.sign == (b.sign == c.sign) {
            self.abs.add_mul_assign(&b.abs, &c.abs);
        } else {
            let sign = self.abs.add_mul_assign_neg_ref_ref(&b.abs, &c.abs);
            self.sign = (self.sign == sign) || self.abs == 0 as Limb;
        }
    }
}

impl Natural {
    // self - b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_limb_neg(&self, b: &Natural, c: Limb) -> (Natural, bool) {
        if c == 0 || *b == 0 as Limb {
            return (self.clone(), true);
        }
        if c == 1 {
            return if self >= b {
                (self - b, true)
            } else {
                (b - self, false)
            };
        }
        match (self, b) {
            (Natural(Large(ref a_limbs)), Natural(Large(ref b_limbs))) => {
                let (limbs, sign) = limbs_overflowing_sub_mul_limb(a_limbs, b_limbs, c);
                let mut result = Natural(Large(limbs));
                result.trim();
                (result, sign)
            }
            _ => {
                let bc = b * Natural::from(c);
                if *self >= bc {
                    (self - bc, true)
                } else {
                    (bc - self, false)
                }
            }
        }
    }

    // self -= b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_assign_limb_neg(&mut self, mut b: Natural, c: Limb) -> bool {
        if c == 0 || b == 0 as Limb {
            return true;
        }
        if c == 1 {
            let sign = *self >= b;
            if sign {
                self.sub_assign_no_panic(b);
            } else {
                self.sub_right_assign_no_panic(&b);
            }
            return sign;
        }
        let (fallback, (right, mut sign)) = match (&mut *self, &mut b) {
            (&mut Natural(Large(ref mut a_limbs)), &mut Natural(Large(ref mut b_limbs))) => (
                false,
                limbs_overflowing_sub_mul_limb_in_place_either(a_limbs, b_limbs, c),
            ),
            _ => (true, (false, false)),
        };
        if fallback {
            let bc = b * Natural::from(c);
            sign = *self >= bc;
            if sign {
                self.sub_assign_no_panic(bc);
            } else {
                self.sub_right_assign_no_panic(&bc);
            }
        } else if right {
            b.trim();
            *self = b;
        } else {
            self.trim();
        }
        sign
    }

    // self -= &b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_assign_limb_neg_ref(&mut self, b: &Natural, c: Limb) -> bool {
        if c == 0 || *b == 0 as Limb {
            return true;
        }
        if c == 1 {
            let sign = *self >= *b;
            if sign {
                self.sub_assign_ref_no_panic(b);
            } else {
                self.sub_right_assign_no_panic(b);
            }
            return sign;
        }
        let (mut sign, fallback) = match (&mut *self, b) {
            (&mut Natural(Large(ref mut a_limbs)), &Natural(Large(ref b_limbs))) => (
                limbs_overflowing_sub_mul_limb_in_place_left(a_limbs, b_limbs, c),
                false,
            ),
            _ => (false, true),
        };
        if fallback {
            let bc = b * Natural::from(c);
            sign = *self >= bc;
            if sign {
                self.sub_assign_no_panic(bc);
            } else {
                self.sub_right_assign_no_panic(&bc);
            }
        } else {
            self.trim();
        }
        sign
    }

    fn add_mul_assign_neg_large(&mut self, b: &Natural, c: &Natural) -> bool {
        let mut sign = false;
        if let Natural(Large(ref b_limbs)) = *b {
            if let Natural(Large(ref c_limbs)) = c {
                let self_limbs = self.promote_in_place();
                sign = limbs_overflowing_sub_mul_in_place_left(self_limbs, b_limbs, c_limbs);
            }
        }
        self.trim();
        sign
    }

    // self - &b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_neg(&self, b: &Natural, c: &Natural) -> (Natural, bool) {
        if let Natural(Small(small_b)) = *b {
            self.add_mul_limb_neg(c, small_b)
        } else if let Natural(Small(small_c)) = *c {
            self.add_mul_limb_neg(b, small_c)
        } else if let Natural(Small(small_a)) = *self {
            ((b * c).sub_limb(small_a), false)
        } else {
            if let Natural(Large(ref a_limbs)) = *self {
                if let Natural(Large(ref b_limbs)) = *b {
                    if let Natural(Large(ref c_limbs)) = *c {
                        let (limbs, sign) = limbs_overflowing_sub_mul(a_limbs, b_limbs, c_limbs);
                        let mut result = Natural(Large(limbs));
                        result.trim();
                        return (result, sign);
                    }
                }
            }
            unreachable!();
        }
    }

    // self -= b * c, returns sign (true means non-negative)
    fn add_mul_assign_neg(&mut self, b: Natural, c: Natural) -> bool {
        if let Natural(Small(small_b)) = b {
            self.add_mul_assign_limb_neg(c, small_b)
        } else if let Natural(Small(small_c)) = c {
            self.add_mul_assign_limb_neg(b, small_c)
        } else if *self == 0 as Limb {
            *self = b * c;
            false
        } else {
            self.add_mul_assign_neg_large(&b, &c)
        }
    }

    // self -= b * &c, returns sign (true means non-negative)
    fn add_mul_assign_neg_val_ref(&mut self, b: Natural, c: &Natural) -> bool {
        if let Natural(Small(small_b)) = b {
            self.add_mul_assign_limb_neg_ref(c, small_b)
        } else if let Natural(Small(small_c)) = *c {
            self.add_mul_assign_limb_neg(b, small_c)
        } else if *self == 0 as Limb {
            *self = b * c;
            false
        } else {
            self.add_mul_assign_neg_large(&b, c)
        }
    }

    // self -= &b * c, returns sign (true means non-negative)
    fn add_mul_assign_neg_ref_val(&mut self, b: &Natural, c: Natural) -> bool {
        if let Natural(Small(small_b)) = *b {
            self.add_mul_assign_limb_neg(c, small_b)
        } else if let Natural(Small(small_c)) = c {
            self.add_mul_assign_limb_neg_ref(b, small_c)
        } else if *self == 0 as Limb {
            *self = b * c;
            false
        } else {
            self.add_mul_assign_neg_large(b, &c)
        }
    }

    // self -= &b * &c, returns sign (true means non-negative)
    fn add_mul_assign_neg_ref_ref(&mut self, b: &Natural, c: &Natural) -> bool {
        if let Natural(Small(small_b)) = *b {
            self.add_mul_assign_limb_neg_ref(c, small_b)
        } else if let Natural(Small(small_c)) = *c {
            self.add_mul_assign_limb_neg_ref(b, small_c)
        } else if *self == 0 as Limb {
            *self = b * c;
            false
        } else {
            self.add_mul_assign_neg_large(b, c)
        }
    }
}
