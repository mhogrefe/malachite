use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use traits::{SubMul, SubMulAssign};

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` and b
/// by value.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::SubMul;
/// use std::str::FromStr;
///
/// assert_eq!(Integer::from(-10i32).sub_mul(Integer::from(3u32), 4u32), -22);
/// assert_eq!(Integer::from_str("1000000000000").unwrap()
///                     .sub_mul(Integer::from(65536u32), 65536u32).to_string(),
///            "995705032704");
/// ```
impl SubMul<Integer, u32> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: Integer, c: u32) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` by
/// value and b by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::SubMul;
/// use std::str::FromStr;
///
/// assert_eq!(Integer::from(-10i32).sub_mul(&Integer::from(3u32), 4u32), -22);
/// assert_eq!(Integer::from_str("1000000000000").unwrap()
///                     .sub_mul(&Integer::from(65536u32), 65536u32).to_string(),
///            "995705032704");
/// ```
impl<'a> SubMul<&'a Integer, u32> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: &'a Integer, c: u32) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` by
/// reference and b by value.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::SubMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(-10i32)).sub_mul(Integer::from(3u32), 4u32), -22);
/// assert_eq!((&Integer::from_str("1000000000000").unwrap())
///                     .sub_mul(Integer::from(65536u32), 65536u32).to_string(),
///            "995705032704");
/// ```
impl<'a> SubMul<Integer, u32> for &'a Integer {
    type Output = Integer;

    fn sub_mul(self, b: Integer, c: u32) -> Integer {
        self.sub_mul(&b, c)
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), taking `self` and b
/// by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::SubMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(-10i32)).sub_mul(&Integer::from(3u32), 4u32), -22);
/// assert_eq!((&Integer::from_str("1000000000000").unwrap())
///                     .sub_mul(&Integer::from(65536u32), 65536u32).to_string(),
///             "995705032704");
/// ```
impl<'a, 'b> SubMul<&'a Integer, u32> for &'b Integer {
    type Output = Integer;

    fn sub_mul(self, b: &'a Integer, c: u32) -> Integer {
        if c == 0 || *b == 0 {
            return self.clone();
        }
        if let Small(small_b) = *b {
            let product = small_b as i64 * c as i64;
            if product >= i32::min_value() as i64 && product <= u32::max_value() as i64 {
                return if product >= 0 {
                    self - (product as u32)
                } else {
                    self - (product as i32)
                };
            }
        }
        unsafe {
            let mut result: mpz_t = mem::uninitialized();
            match *self {
                Small(small) => gmp::mpz_init_set_si(&mut result, small.into()),
                Large(ref large) => gmp::mpz_init_set(&mut result, large),
            }
            match b {
                &Small(small) => {
                    let mut large_b: mpz_t = mem::uninitialized();
                    gmp::mpz_init_set_si(&mut large_b, small.into());
                    gmp::mpz_submul_ui(&mut result, &large_b, c.into());
                }
                &Large(ref large_b) => gmp::mpz_submul_ui(&mut result, large_b, c.into()),
            }
            let mut result = Large(result);
            result.demote_if_small();
            result
        }
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), in place, taking b
/// by value.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::SubMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Integer::from(-10i32);
/// x.sub_mul_assign(Integer::from(3u32), 4u32);
/// assert_eq!(x, -22);
///
/// let mut x = Integer::from_str("1000000000000").unwrap();
/// x.sub_mul_assign(Integer::from(65536u32), 65536u32);
/// assert_eq!(x.to_string(), "995705032704");
/// ```
impl SubMulAssign<Integer, u32> for Integer {
    fn sub_mul_assign(&mut self, b: Integer, c: u32) {
        self.sub_mul_assign(&b, c);
    }
}

/// Subs the product of an `Integer` (b) and a `u32` (c) to an `Integer` (self), in place, taking b
/// by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::SubMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Integer::from(-10i32);
/// x.sub_mul_assign(&Integer::from(3u32), 4u32);
/// assert_eq!(x, -22);
///
/// let mut x = Integer::from_str("1000000000000").unwrap();
/// x.sub_mul_assign(&Integer::from(65536u32), 65536u32);
/// assert_eq!(x.to_string(), "995705032704");
/// ```
impl<'a> SubMulAssign<&'a Integer, u32> for Integer {
    fn sub_mul_assign(&mut self, b: &'a Integer, c: u32) {
        if c == 0 || *b == 0 {
            return;
        }
        if let Small(small_b) = *b {
            let product = small_b as i64 * c as i64;
            if product >= i32::min_value() as i64 && product <= u32::max_value() as i64 {
                if product >= 0 {
                    *self -= product as u32;
                } else {
                    *self -= product as i32;
                }
                return;
            }
        }
        unsafe {
            let large_self = self.promote_in_place();
            match b {
                &Small(small) => {
                    let mut large_b: mpz_t = mem::uninitialized();
                    gmp::mpz_init_set_si(&mut large_b, small.into());
                    gmp::mpz_submul_ui(large_self, &large_b, c.into());
                }
                &Large(ref large_b) => gmp::mpz_submul_ui(large_self, large_b, c.into()),
            }
        }
        self.demote_if_small();
    }
}
