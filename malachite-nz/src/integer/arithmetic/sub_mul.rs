use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, NegAssign, SubMul, SubMulAssign,
};

use integer::Integer;
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
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(Integer::from(3u32), Integer::from(-4)), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(Integer::from(-0x1_0000),
///         -Integer::trillion()).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> SubMul<Integer, Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn sub_mul(mut self, b: Integer, c: Integer) -> Integer {
        self.sub_mul_assign(b, c);
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
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(Integer::from(3u32), &Integer::from(-4)), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(Integer::from(-0x1_0000),
///         &(-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> SubMul<Integer, &'a Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn sub_mul(mut self, b: Integer, c: &'a Integer) -> Integer {
        self.sub_mul_assign(b, c);
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
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(&Integer::from(3u32), Integer::from(-4)), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(&Integer::from(-0x1_0000),
///         -Integer::trillion()).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> SubMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn sub_mul(mut self, b: &'a Integer, c: Integer) -> Integer {
        self.sub_mul_assign(b, c);
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
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(&Integer::from(3u32), &Integer::from(-4)), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(&Integer::from(-0x1_0000),
///                         &(-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b> SubMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn sub_mul(mut self, b: &'a Integer, c: &'b Integer) -> Integer {
        self.sub_mul_assign(b, c);
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
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).sub_mul(&Integer::from(3u32), &Integer::from(-4)), 22);
///     assert_eq!((&(-Integer::trillion())).sub_mul(&Integer::from(-0x1_0000),
///                         &(-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b, 'c> SubMul<&'a Integer, &'b Integer> for &'c Integer {
    type Output = Integer;

    fn sub_mul(self, b: &'a Integer, c: &'b Integer) -> Integer {
        if self.sign == (b.sign != c.sign) {
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
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.sub_mul_assign(Integer::from(3u32), Integer::from(-4));
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.sub_mul_assign(Integer::from(-0x1_0000), -Integer::trillion());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> SubMulAssign<Integer, Integer> for Integer {
    fn sub_mul_assign(&mut self, b: Integer, c: Integer) {
        self.add_mul_assign(-b, c);
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
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.sub_mul_assign(Integer::from(3u32), &Integer::from(-4));
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.sub_mul_assign(Integer::from(-0x1_0000), &(-Integer::trillion()));
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> SubMulAssign<Integer, &'a Integer> for Integer {
    fn sub_mul_assign(&mut self, b: Integer, c: &'a Integer) {
        self.add_mul_assign(-b, c);
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
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.sub_mul_assign(&Integer::from(3u32), Integer::from(-4));
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.sub_mul_assign(&Integer::from(-0x1_0000), -Integer::trillion());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> SubMulAssign<&'a Integer, Integer> for Integer {
    fn sub_mul_assign(&mut self, b: &'a Integer, c: Integer) {
        self.add_mul_assign(b, -c);
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
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.sub_mul_assign(&Integer::from(3u32), &Integer::from(-4));
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.sub_mul_assign(&Integer::from(-0x1_0000), &(-Integer::trillion()));
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b> SubMulAssign<&'a Integer, &'b Integer> for Integer {
    fn sub_mul_assign(&mut self, b: &'a Integer, c: &'b Integer) {
        self.neg_assign();
        self.add_mul_assign(b, c);
        self.neg_assign();
    }
}
