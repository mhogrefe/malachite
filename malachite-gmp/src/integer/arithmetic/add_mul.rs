use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use malachite_base::traits::{AddMul, AddMulAssign};

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`, b,
/// and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), Integer::from(4u32)), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(Integer::from(65536u32),
///                         Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-65537000000000000");
/// }
/// ```
impl AddMul<Integer, Integer> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: Integer, c: Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` and
/// b by value and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(Integer::from(65536u32),
///                         &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-65537000000000000");
/// }
/// ```
impl<'a> AddMul<Integer, &'a Integer> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: Integer, c: &'a Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` and
/// c by value and b by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), Integer::from(4u32)), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(&Integer::from(65536u32),
///                         Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-65537000000000000");
/// }
/// ```
impl<'a> AddMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: &'a Integer, c: Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self` by
/// value and b and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!(Integer::from_str("-1000000000000").unwrap()
///                         .add_mul(&Integer::from(65536u32),
///                         &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                "-65537000000000000");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    fn add_mul(mut self, b: &'a Integer, c: &'b Integer) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), taking `self`, b,
/// and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMul;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), &Integer::from(4u32)), 22);
///     assert_eq!((&Integer::from_str("-1000000000000").unwrap())
///                         .add_mul(&Integer::from(65536u32),
///                         &Integer::from_str("-1000000000000").unwrap()).to_string(),
///                 "-65537000000000000");
/// }
/// ```
impl<'a, 'b, 'c> AddMul<&'a Integer, &'b Integer> for &'c Integer {
    type Output = Integer;

    fn add_mul(self, b: &'a Integer, c: &'b Integer) -> Integer {
        if let Small(small_b) = *b {
            self.add_mul(c, small_b)
        } else if let Small(small_c) = *c {
            self.add_mul(b, small_c)
        } else {
            let mut result = unsafe {
                let mut result: mpz_t = mem::uninitialized();
                match *self {
                    Small(small) => gmp::mpz_init_set_si(&mut result, small.into()),
                    Large(ref large) => gmp::mpz_init_set(&mut result, large),
                }
                if let Large(ref large_b) = *b {
                    if let &Large(ref large_c) = c {
                        gmp::mpz_addmul(&mut result, large_b, large_c)
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

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(Integer::from(65536u32), Integer::from_str("-1000000000000").unwrap());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl AddMulAssign<Integer, Integer> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: Integer) {
        self.add_mul_assign(&b, &c);
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b by value and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), &Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(Integer::from(65536u32), &Integer::from_str("-1000000000000").unwrap());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMulAssign<Integer, &'a Integer> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: &'a Integer) {
        self.add_mul_assign(&b, c);
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b by reference and c by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(&Integer::from(65536u32), Integer::from_str("-1000000000000").unwrap());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a> AddMulAssign<&'a Integer, Integer> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: Integer) {
        self.add_mul_assign(b, &c);
    }
}

/// Adds the product of a `Integer` (b) and a `Integer` (c) to a `Integer` (self), in place, taking
/// b and c by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::AddMulAssign;
/// use malachite_gmp::integer::Integer;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), &Integer::from(4u32));
///     assert_eq!(x, 22);
///
///     let mut x = Integer::from_str("-1000000000000").unwrap();
///     x.add_mul_assign(&Integer::from(65536u32), &Integer::from_str("-1000000000000").unwrap());
///     assert_eq!(x.to_string(), "-65537000000000000");
/// }
/// ```
impl<'a, 'b> AddMulAssign<&'a Integer, &'b Integer> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: &'b Integer) {
        if let Small(small_b) = *b {
            self.add_mul_assign(c, small_b);
        } else if let Small(small_c) = *c {
            self.add_mul_assign(b, small_c);
        } else {
            {
                let large_self = self.promote_in_place();
                unsafe {
                    if let Large(ref large_b) = *b {
                        if let &Large(ref large_c) = c {
                            gmp::mpz_addmul(large_self, large_b, large_c)
                        }
                    }
                }
            }
            self.demote_if_small();
        }
    }
}
