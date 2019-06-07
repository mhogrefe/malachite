use std::cmp::Ordering;

use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};

use integer::Integer;
use natural::arithmetic::mul::limbs_mul_greater_to_out;
use natural::arithmetic::sub::{limbs_slice_sub_in_place_right, limbs_sub_in_place_left};
use natural::comparison::ord::limbs_cmp;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Given the limbs of three `Natural`s a, b, and c, calculates a - b * c, returning the limbs of
/// the absolute value and the sign (true means non-negative). All of the input slices should be
/// non-empty and have no trailing zeros.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(m + n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///       m = `xs.len()`
///
/// # Panics
/// Panics if `ys` or `zs` are empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::add_mul::limbs_overflowing_sub_mul;
///
/// assert_eq!(limbs_overflowing_sub_mul(&[123, 456], &[123, 789], &[321, 654]),
///         (vec![39360, 333255, 516006], false));
/// assert_eq!(limbs_overflowing_sub_mul(&[123, 456, 789, 987, 654], &[123, 789], &[321, 654]),
///         (vec![4294927936, 4294634040, 4294452078, 986, 654], true));
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, where w, x, and y are positive, sub is negative, and w
/// is returned instead of overwriting the first input. w_sign is also returned.
pub fn limbs_overflowing_sub_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> (Vec<Limb>, bool) {
    let mut xs = xs.to_vec();
    let sign = limbs_overflowing_sub_mul_in_place_left(&mut xs, ys, zs);
    (xs, sign)
}

/// Given the limbs of three `Natural`s a, b, and c, calculates a - b * c, writing the limbs of the
/// absolute value to the first (left) slice and returning the sign (true means non-negative). All
/// of the input slices should be non-empty and have no trailing zeros.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///       m = `xs.len()`
///
/// # Panics
/// Panics if `ys` or `zs` are empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::add_mul::limbs_overflowing_sub_mul_in_place_left;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_overflowing_sub_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]), false);
/// assert_eq!(xs, &[39360, 333255, 516006]);
///
/// let mut xs = vec![123, 456, 789, 987, 654];
/// assert_eq!(limbs_overflowing_sub_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]), true);
/// assert_eq!(xs, &[4294927936, 4294634040, 4294452078, 986, 654]);
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, where w, x, and y are positive, sub is negative, and
/// w_sign is returned.
pub fn limbs_overflowing_sub_mul_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    zs: &[Limb],
) -> bool {
    if ys.len() >= zs.len() {
        limbs_overflowing_sub_mul_greater_in_place_left(xs, ys, zs)
    } else {
        limbs_overflowing_sub_mul_greater_in_place_left(xs, zs, ys)
    }
}

// zs.len() >= ys.len()
fn limbs_overflowing_sub_mul_greater_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    zs: &[Limb],
) -> bool {
    let xs_len = xs.len();
    let product_len = ys.len() + zs.len();
    let mut product = vec![0; product_len];
    if limbs_mul_greater_to_out(&mut product, ys, zs) == 0 {
        product.pop();
    }
    assert_ne!(*product.last().unwrap(), 0);
    if limbs_cmp(xs, &product) == Ordering::Less {
        if xs_len < product_len {
            xs.resize(product.len(), 0);
        }
        assert!(!limbs_slice_sub_in_place_right(
            &product,
            &mut xs[..product.len()],
            xs_len,
        ));
        false
    } else {
        assert!(!limbs_sub_in_place_left(xs, &product));
        !limbs_test_zero(xs)
    }
}

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
    fn add_mul_assign_neg_large(&mut self, b: &Natural, c: &Natural) -> bool {
        let mut sign = false;
        if let Large(ref b_limbs) = *b {
            if let Large(ref c_limbs) = c {
                let self_limbs = self.promote_in_place();
                sign = limbs_overflowing_sub_mul_in_place_left(self_limbs, b_limbs, c_limbs);
            }
        }
        self.trim();
        sign
    }

    // self - &b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_neg(&self, b: &Natural, c: &Natural) -> (Natural, bool) {
        if let Small(small_b) = *b {
            self.add_mul_limb_neg(c, small_b)
        } else if let Small(small_c) = *c {
            self.add_mul_limb_neg(b, small_c)
        } else if let Small(small_a) = *self {
            (b * c - small_a, false)
        } else {
            if let Large(ref a_limbs) = *self {
                if let Large(ref b_limbs) = *b {
                    if let Large(ref c_limbs) = *c {
                        let (limbs, sign) = limbs_overflowing_sub_mul(a_limbs, b_limbs, c_limbs);
                        let mut result = Large(limbs);
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
        if let Small(small_b) = b {
            self.add_mul_assign_limb_neg(c, small_b)
        } else if let Small(small_c) = c {
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
        if let Small(small_b) = b {
            self.add_mul_assign_limb_neg_ref(c, small_b)
        } else if let Small(small_c) = *c {
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
        if let Small(small_b) = *b {
            self.add_mul_assign_limb_neg(c, small_b)
        } else if let Small(small_c) = c {
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
        if let Small(small_b) = *b {
            self.add_mul_assign_limb_neg_ref(c, small_b)
        } else if let Small(small_c) = *c {
            self.add_mul_assign_limb_neg_ref(b, small_c)
        } else if *self == 0 as Limb {
            *self = b * c;
            false
        } else {
            self.add_mul_assign_neg_large(b, c)
        }
    }
}
