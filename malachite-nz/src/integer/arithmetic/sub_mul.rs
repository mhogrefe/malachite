use integer::Integer;
use malachite_base::num::{AddMul, AddMulAssign, SubMul, SubMulAssign};
use natural::arithmetic::add_mul::mpz_aorsmul;
use natural::Natural::{Large, Small};

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`, b,
/// and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
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

    fn sub_mul(mut self, b: Integer, c: Integer) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` and
/// b by value and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
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

    fn sub_mul(mut self, b: Integer, c: &'a Integer) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` and
/// c by value and b by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
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

    fn sub_mul(mut self, b: &'a Integer, c: Integer) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` by
/// value and b and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
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

    fn sub_mul(mut self, b: &'a Integer, c: &'b Integer) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`, b,
/// and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
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
        match (self, b, c) {
            (
                &Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => -(b * c),
            (
                a,
                &Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul(c, b),
            (
                a,
                &Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul(c, b),
            (
                a,
                b,
                &Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.sub_mul(b, c),
            (
                a,
                b,
                &Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.add_mul(b, c),
            (
                &Integer {
                    sign: a_sign,
                    abs: ref a_abs,
                },
                &Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                &Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !a_sign;
                let mut result_limbs = a_abs.to_limbs_le();
                mpz_aorsmul(
                    &mut result_sign,
                    &mut result_limbs,
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    false,
                );
                result_sign = !result_sign;
                let mut abs_result = Large(result_limbs);
                abs_result.trim();
                Integer {
                    sign: result_sign,
                    abs: abs_result,
                }
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMulAssign;
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
        match (self, b, c) {
            (
                a @ &mut Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => *a = -(b * c),
            (
                a,
                Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul_assign(c, b),
            (
                a,
                Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul_assign(c, b),
            (
                a,
                b,
                Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.sub_mul_assign(b, c),
            (
                a,
                b,
                Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.add_mul_assign(b, c),
            (
                &mut Integer {
                    sign: ref mut a_sign,
                    abs: ref mut a_abs,
                },
                Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !*a_sign;
                mpz_aorsmul(
                    &mut result_sign,
                    a_abs.promote_in_place(),
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    false,
                );
                *a_sign = !result_sign;
                a_abs.trim();
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b by value and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMulAssign;
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
        match (self, b, c) {
            (
                a @ &mut Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => *a = -(b * c),
            (
                a,
                Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul_assign(c, b),
            (
                a,
                Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul_assign(c, b),
            (
                a,
                b,
                &Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.sub_mul_assign(b, c),
            (
                a,
                b,
                &Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.add_mul_assign(b, c),
            (
                &mut Integer {
                    sign: ref mut a_sign,
                    abs: ref mut a_abs,
                },
                Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                &Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !*a_sign;
                mpz_aorsmul(
                    &mut result_sign,
                    a_abs.promote_in_place(),
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    false,
                );
                *a_sign = !result_sign;
                a_abs.trim();
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b by reference and c by value.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMulAssign;
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
        match (self, b, c) {
            (
                a @ &mut Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => *a = -(b * c),
            (
                a,
                &Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul_assign(c, b),
            (
                a,
                &Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul_assign(c, b),
            (
                a,
                b,
                Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.sub_mul_assign(b, c),
            (
                a,
                b,
                Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.add_mul_assign(b, c),
            (
                &mut Integer {
                    sign: ref mut a_sign,
                    abs: ref mut a_abs,
                },
                &Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !*a_sign;
                mpz_aorsmul(
                    &mut result_sign,
                    a_abs.promote_in_place(),
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    false,
                );
                *a_sign = !result_sign;
                a_abs.trim();
            }
        }
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMulAssign;
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
        match (self, b, c) {
            (
                a @ &mut Integer {
                    sign: true,
                    abs: Small(0),
                },
                b,
                c,
            ) => *a = -(b * c),
            (
                a,
                &Integer {
                    sign: true,
                    abs: Small(b),
                },
                c,
            ) => a.sub_mul_assign(c, b),
            (
                a,
                &Integer {
                    sign: false,
                    abs: Small(b),
                },
                c,
            ) => a.add_mul_assign(c, b),
            (
                a,
                b,
                &Integer {
                    sign: true,
                    abs: Small(c),
                },
            ) => a.sub_mul_assign(b, c),
            (
                a,
                b,
                &Integer {
                    sign: false,
                    abs: Small(c),
                },
            ) => a.add_mul_assign(b, c),
            (
                &mut Integer {
                    sign: ref mut a_sign,
                    abs: ref mut a_abs,
                },
                &Integer {
                    sign: b_sign,
                    abs: Large(ref b_limbs),
                },
                &Integer {
                    sign: c_sign,
                    abs: Large(ref c_limbs),
                },
            ) => {
                let mut result_sign = !*a_sign;
                mpz_aorsmul(
                    &mut result_sign,
                    a_abs.promote_in_place(),
                    !b_sign,
                    b_limbs,
                    !c_sign,
                    c_limbs,
                    false,
                );
                *a_sign = !result_sign;
                a_abs.trim();
            }
        }
    }
}
