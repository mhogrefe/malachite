use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use malachite_base::traits::{SubMul, SubMulAssign};

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), taking
/// `self`, b, and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(Integer::from(3u32), Integer::from(-4)), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(Integer::from(-0x1_0000), -Integer::trillion())
///         .to_string(), "-65537000000000000");
/// }
/// ```
impl SubMul<Integer, Integer> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: Integer, c: Integer) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), taking
/// `self` and b by value and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
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

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), taking
/// `self` and c by value and b by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(&Integer::from(3u32), Integer::from(-4)), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(&Integer::from(-0x1_0000),
///         (-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> SubMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: &'a Integer, c: Integer) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), taking
/// `self` by value and b and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(&Integer::from(3u32), &Integer::from(-4)), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(&Integer::from(-0x1_0000),
///         &(-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b> SubMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: &'a Integer, c: &'b Integer) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), taking
/// `self`, b, and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).sub_mul(&Integer::from(3u32), &Integer::from(-4)), 22);
///     assert_eq!((&(-Integer::trillion())).sub_mul(&Integer::from(-0x1_0000),
///         &(-Integer::trillion())).to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b, 'c> SubMul<&'a Integer, &'b Integer> for &'c Integer {
    type Output = Integer;

    fn sub_mul(self, b: &'a Integer, c: &'b Integer) -> Integer {
        if let Small(small_b) = *b {
            self.sub_mul(c, small_b)
        } else if let Small(small_c) = *c {
            self.sub_mul(b, small_c)
        } else {
            let mut result = unsafe {
                let mut result: mpz_t = mem::uninitialized();
                match *self {
                    Small(small) => gmp::mpz_init_set_si(&mut result, small.into()),
                    Large(ref large) => gmp::mpz_init_set(&mut result, large),
                }
                if let Large(ref large_b) = *b {
                    if let &Large(ref large_c) = c {
                        gmp::mpz_submul(&mut result, large_b, large_c)
                    } else {
                        unreachable!()
                    }
                }
                Large(result)
            };
            result.demote_if_small();
            result
        }
    }
}

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), in place,
/// taking b and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_gmp::integer::Integer;
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
impl SubMulAssign<Integer, Integer> for Integer {
    fn sub_mul_assign(&mut self, b: Integer, c: Integer) {
        self.sub_mul_assign(&b, &c);
    }
}

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), in place,
/// taking b by value and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_gmp::integer::Integer;
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
        self.sub_mul_assign(&b, c);
    }
}

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), in place,
/// taking b by reference and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_gmp::integer::Integer;
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
        self.sub_mul_assign(b, &c);
    }
}

/// Subtracts the product of a `Integer` (b) and a `Integer` (c) from a `Integer` (self), in place,
/// taking b and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_gmp::integer::Integer;
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
        if let Small(small_b) = *b {
            self.sub_mul_assign(c, small_b);
        } else if let Small(small_c) = *c {
            self.sub_mul_assign(b, small_c);
        } else {
            {
                let large_self = self.promote_in_place();
                unsafe {
                    if let Large(ref large_b) = *b {
                        if let &Large(ref large_c) = c {
                            gmp::mpz_submul(large_self, large_b, large_c)
                        }
                    }
                }
            }
            self.demote_if_small();
        }
    }
}
