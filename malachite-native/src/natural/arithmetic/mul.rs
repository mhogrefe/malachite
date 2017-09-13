use natural::arithmetic::add::mpn_add_in_place;
use natural::arithmetic::add_mul_u32::large_add_mul_u32_mut_a;
use natural::arithmetic::mul_u32::{large_mul_u32, large_mul_u32_to_buffer};
use natural::Natural::{self, Large, Small};
use std::ops::{Mul, MulAssign};

//TODO use better algorithms

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
                    let mut product = Large(if xs.len() >= ys.len() {
                        basecase_mul(xs, ys)
                    } else {
                        basecase_mul(ys, xs)
                    });
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

fn mul_helper(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    if xs.len() >= ys.len() {
        basecase_mul(xs, ys)
    } else {
        basecase_mul(ys, xs)
    }
}

// xs.len() >= ys.len()
fn basecase_mul_with_mem_opt(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let ys_len = ys.len();
    if ys_len > 1 && ys_len < MUL_TOOM22_THRESHOLD && xs.len() > MUL_BASECASE_MAX_UN {
        basecase_mem_opt_mul(xs, ys)
    } else {
        basecase_mul(xs, ys)
    }
}

// xs.len() >= ys.len(), ys cannot be empty
fn basecase_mul_to_buffer(buffer: &mut [u32], xs: &[u32], ys: &[u32]) {
    large_mul_u32_to_buffer(buffer, xs, ys[0]);
    for (i, y) in ys.iter().enumerate().skip(1) {
        if *y != 0 {
            large_add_mul_u32_mut_a(&mut buffer[i..], xs, *y);
        }
    }
}

// ys cannot be empty
fn basecase_mul(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut product_limbs = large_mul_u32(xs, ys[0]);
    product_limbs.resize(xs.len() + ys.len(), 0);
    for (i, y) in ys.iter().enumerate().skip(1) {
        if *y != 0 {
            large_add_mul_u32_mut_a(&mut product_limbs[i..], xs, *y);
        }
    }
    product_limbs
}


const MUL_BASECASE_MAX_UN: usize = 500;
const MUL_TOOM22_THRESHOLD: usize = 300;

// 1 < ys.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < xs.len()
//
// This is currently not measurably better than just basecase.
fn basecase_mem_opt_mul(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let x_len = xs.len();
    let y_len = ys.len();
    let output_length = x_len + y_len;
    let mut buffer = Vec::with_capacity(output_length);
    buffer.resize(output_length, 0);
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in xs.chunks(MUL_BASECASE_MAX_UN) {
        if chunk.len() >= y_len {
            basecase_mul_to_buffer(&mut buffer[offset..], chunk, ys);
        } else {
            basecase_mul_to_buffer(&mut buffer[offset..], ys, chunk);
        }
        if offset != 0 {
            mpn_add_in_place(&mut buffer[offset..], &triangle_buffer[0..y_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < x_len {
            &triangle_buffer[0..y_len].copy_from_slice(&buffer[offset..offset + y_len]);
        }
    }
    buffer
}

impl Natural {
    pub fn _basecase_mul_assign_with_mem_opt(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref ys)) => {
                    *xs = if xs.len() >= ys.len() {
                        basecase_mul_with_mem_opt(xs, ys)
                    } else {
                        basecase_mul_with_mem_opt(ys, xs)
                    };
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}
