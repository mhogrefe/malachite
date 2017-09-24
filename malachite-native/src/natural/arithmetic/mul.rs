use natural::arithmetic::add::mpn_add_in_place;
use natural::arithmetic::add_mul_u32::mpn_addmul_1;
use natural::arithmetic::mul_u32::mpn_mul_1;
use natural::Natural::{self, Large, Small};
use std::ops::{Mul, MulAssign};

//TODO use better algorithms

const MUL_BASECASE_MAX_UN: usize = 500;
const MUL_TOOM22_THRESHOLD: usize = 300;

// Multiply u by v and write the result to prod. Must have u.len() >= v.len(). prod must be 0
// initially.
//
// Note that prod gets u.len() + v.len() limbs stored, even if the actual result only needs u.len +
// v.len() - 1.
//
// There's no good reason to call here with vsize >= MUL_TOOM22_THRESHOLD. Currently this is
// allowed, but it might not be in the future.
//
// This is the most critical code for multiplication. All multiplies rely on this, both small and
// huge. Small ones arrive here immediately, huge ones arrive here as this is the base case for
// Karatsuba's recursive algorithm.
fn mpn_mul_basecase(prod: &mut [u32], u: &[u32], v: &[u32]) {
    let u_len = u.len();
    let carry = mpn_mul_1(prod, u, v[0]);
    if carry != 0 {
        prod[u_len] = carry;
    }
    for (i, y) in v.iter().enumerate().skip(1) {
        if *y != 0 {
            let carry = mpn_addmul_1(&mut prod[i..], u, *y);
            if carry != 0 {
                prod[u_len + i] = carry;
            }
        }
    }
}

// 1 < v.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < u.len()
//
// This is currently not measurably better than just basecase.
fn mpn_mul_basecase_mem_opt_helper(prod: &mut [u32], u: &[u32], v: &[u32]) {
    let u_len = u.len();
    let v_len = v.len();
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in u.chunks(MUL_BASECASE_MAX_UN) {
        if chunk.len() >= v_len {
            mpn_mul(&mut prod[offset..], chunk, v);
        } else {
            mpn_mul(&mut prod[offset..], v, chunk);
        }
        if offset != 0 {
            mpn_add_in_place(&mut prod[offset..], &triangle_buffer[0..v_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < u_len {
            &triangle_buffer[0..v_len].copy_from_slice(&prod[offset..offset + v_len]);
        }
    }
}

fn mpn_mul_basecase_mem_opt(prod: &mut [u32], u: &[u32], v: &[u32]) {
    let v_len = v.len();
    if v_len > 1 && v_len < MUL_TOOM22_THRESHOLD && u.len() > MUL_BASECASE_MAX_UN {
        mpn_mul_basecase_mem_opt_helper(prod, u, v)
    } else {
        mpn_mul_basecase(prod, u, v)
    }
}

// Multiply s1 and s2, and write the (s1.len()+s2.len())-limb result to r. Return the most
// significant limb of the result. The destination has to have space for s1.len() + s2.len() limbs,
// even if the product’s most significant limb is zero. s1.len() >= s2.len()
pub fn mpn_mul(r: &mut [u32], s1: &[u32], s2: &[u32]) -> u32 {
    mpn_mul_basecase(r, s1, s2);
    r[s1.len() + s2.len() - 1]
}

fn mul_helper(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        mpn_mul(&mut product_limbs, xs, ys);
    } else {
        mpn_mul(&mut product_limbs, ys, xs);
    }
    product_limbs
}

fn mul_basecase_mem_opt_helper(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        mpn_mul_basecase_mem_opt(&mut product_limbs, xs, ys);
    } else {
        mpn_mul_basecase_mem_opt(&mut product_limbs, ys, xs);
    }
    product_limbs
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(1u32) * Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) * Natural::from(0u32)).to_string(), "0");
/// assert_eq!((Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
/// assert_eq!((Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///            .unwrap()).to_string(), "121932631112635269000000");
/// ```
impl Mul<Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(1u32) * &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) * &Natural::from(0u32)).to_string(), "0");
/// assert_eq!((Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
/// assert_eq!((Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///            .unwrap()).to_string(), "121932631112635269000000");
/// ```
impl<'a> Mul<&'a Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: &'a Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by reference and the right
/// `Natural` by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(1u32) * Natural::from(123u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) * Natural::from(0u32)).to_string(), "0");
/// assert_eq!((&Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
/// assert_eq!((&Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///            .unwrap()).to_string(), "121932631112635269000000");
/// ```
impl<'a> Mul<Natural> for &'a Natural {
    type Output = Natural;

    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(1u32) * &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) * &Natural::from(0u32)).to_string(), "0");
/// assert_eq!((&Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
/// assert_eq!((&Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///            .unwrap()).to_string(), "121932631112635269000000");
/// ```
impl<'a, 'b> Mul<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn mul(self, other: &'a Natural) -> Natural {
        if let Small(y) = *other {
            self * y
        } else if let Small(x) = *self {
            other * x
        } else {
            match (self, other) {
                (&Large(ref xs), &Large(ref ys)) => {
                    let mut product = Large(mul_helper(xs, ys));
                    product.trim();
                    product
                }
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(1u32);
/// x *= Natural::from_str("1000").unwrap();
/// x *= Natural::from_str("2000").unwrap();
/// x *= Natural::from_str("3000").unwrap();
/// x *= Natural::from_str("4000").unwrap();
/// assert_eq!(x.to_string(), "24000000000000");
/// ```
impl MulAssign<Natural> for Natural {
    fn mul_assign(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref mut ys)) => {
                    *xs = mul_helper(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(1u32);
/// x *= &Natural::from_str("1000").unwrap();
/// x *= &Natural::from_str("2000").unwrap();
/// x *= &Natural::from_str("3000").unwrap();
/// x *= &Natural::from_str("4000").unwrap();
/// assert_eq!(x.to_string(), "24000000000000");
/// ```
impl<'a> MulAssign<&'a Natural> for Natural {
    fn mul_assign(&mut self, other: &'a Natural) {
        if let Small(y) = *other {
            *self *= y;
        } else if let Small(x) = *self {
            *self = other * x;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    *xs = mul_helper(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

impl Natural {
    pub fn _mul_assign_basecase_mem_opt(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref ys)) => {
                    *xs = mul_basecase_mem_opt_helper(xs, ys)
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}
