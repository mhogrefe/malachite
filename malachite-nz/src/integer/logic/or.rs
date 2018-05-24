use integer::Integer;
use malachite_base::limbs::limbs_leading_zero_limbs;
use natural::Natural::{self, Large, Small};
use std::cmp::min;
use std::iter::repeat;
use std::ops::{BitOr, BitOrAssign};
use std::u32;

pub fn limbs_or_pos_neg(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        let mut result_limbs = vec![0; x_i];
        result_limbs.push(xs[x_i].wrapping_neg());
        result_limbs.extend(xs[x_i + 1..].iter().map(|x| !x));
        result_limbs.extend(repeat(u32::MAX).take(y_i - xs_len));
        result_limbs.push(ys[y_i] - 1);
        result_limbs.extend_from_slice(&ys[y_i + 1..]);
        return result_limbs;
    } else if x_i >= ys_len {
        return ys.to_vec();
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    let mut result_limbs = vec![0; min_i];
    if x_i == y_i {
        result_limbs.push((!xs[x_i] & (ys[y_i] - 1)) + 1);
    } else if x_i > y_i {
        result_limbs.extend_from_slice(&ys[y_i..x_i]);
        result_limbs.push(!xs[x_i] & ys[x_i]);
    } else {
        result_limbs.push(xs[x_i].wrapping_neg());
        result_limbs.extend(xs[x_i + 1..y_i].iter().map(|x| !x));
        result_limbs.push(!xs[y_i] & (ys[y_i] - 1));
    };
    let min_len = min(xs_len, ys_len);
    result_limbs.extend(
        xs[max_i + 1..min_len]
            .iter()
            .zip(ys[max_i + 1..min_len].iter())
            .map(|(x, y)| !x & y),
    );
    if xs_len < ys_len {
        result_limbs.extend_from_slice(&ys[xs_len..]);
    }
    result_limbs
}

pub fn limbs_or_neg_neg(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        return xs.to_vec();
    } else if x_i >= ys_len {
        return ys.to_vec();
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    let mut result_limbs = vec![0; min_i];
    if x_i > y_i {
        result_limbs.extend_from_slice(&ys[y_i..x_i]);
    } else if y_i > x_i {
        result_limbs.extend_from_slice(&xs[x_i..y_i]);
    }
    result_limbs.push(if x_i == y_i {
        ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1
    } else if x_i > y_i {
        (xs[x_i] - 1) & ys[x_i]
    } else {
        xs[y_i] & (ys[y_i] - 1)
    });
    result_limbs.extend(
        xs[max_i + 1..]
            .iter()
            .zip(ys[max_i + 1..].iter())
            .map(|(x, y)| x & y),
    );
    result_limbs
}

/// Takes the bitwise or of two `Integer`s, taking both by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) | Integer::from(-456)).to_string(), "-67");
/// assert_eq!((-Integer::trillion() | -(Integer::trillion() + 1u32)).to_string(), "-999999995905");
/// ```
impl BitOr<Integer> for Integer {
    type Output = Integer;

    fn bitor(mut self, other: Integer) -> Integer {
        self |= other;
        self
    }
}

/// Takes the bitwise or of two `Integer`s, taking the left `Integer` by value and the right
/// `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) | &Integer::from(-456)).to_string(), "-67");
/// assert_eq!((-Integer::trillion() | &-(Integer::trillion() + 1u32)).to_string(),
///     "-999999995905");
/// ```
impl<'a> BitOr<&'a Integer> for Integer {
    type Output = Integer;

    fn bitor(mut self, other: &'a Integer) -> Integer {
        self |= other;
        self
    }
}

/// Takes the bitwise or of two `Integer`s, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(-123) | Integer::from(-456)).to_string(), "-67");
/// assert_eq!((&-Integer::trillion() | -(Integer::trillion() + 1u32)).to_string(),
///     "-999999995905");
/// ```
impl<'a> BitOr<Integer> for &'a Integer {
    type Output = Integer;

    fn bitor(self, mut other: Integer) -> Integer {
        other |= self;
        other
    }
}

/// Takes the bitwise or of two `Integer`s, taking both `Integer`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(-123) | &Integer::from(-456)).to_string(), "-67");
/// assert_eq!((&-Integer::trillion() | &-(Integer::trillion() + 1u32)).to_string(),
///     "-999999995905");
/// ```
impl<'a, 'b> BitOr<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn bitor(self, other: &'a Integer) -> Integer {
        match (self.sign, other.sign) {
            (true, true) => Integer {
                sign: true,
                abs: &self.abs | &other.abs,
            },
            (true, false) => Integer {
                sign: false,
                abs: self.abs.or_pos_neg(&other.abs),
            },
            (false, true) => Integer {
                sign: false,
                abs: other.abs.or_pos_neg(&self.abs),
            },
            (false, false) => Integer {
                sign: false,
                abs: self.abs.or_neg_neg(&other.abs),
            },
        }
    }
}

/// Bitwise-ors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
/// value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x |= Integer::from(0x0000_000f);
///     x |= Integer::from(0x0000_0f00);
///     x |= Integer::from(0x000f_0000);
///     x |= Integer::from(0x0f00_0000);
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl BitOrAssign<Integer> for Integer {
    fn bitor_assign(&mut self, other: Integer) {
        //TODO
        *self = &*self | &other;
    }
}

/// Bitwise-ors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
/// reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x |= &Integer::from(0x0000_000f);
///     x |= &Integer::from(0x0000_0f00);
///     x |= &Integer::from(0x000f_0000);
///     x |= &Integer::from(0x0f00_0000);
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl<'a> BitOrAssign<&'a Integer> for Integer {
    fn bitor_assign(&mut self, other: &'a Integer) {
        //TODO
        *self = &*self | other;
    }
}

impl Natural {
    fn or_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Small(y)) => self.or_pos_u32_neg(y.wrapping_neg()),
            (&Small(x), _) => other.or_neg_u32_pos(x),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_or_pos_neg(xs, ys));
                result.trim();
                result
            }
        }
    }

    fn or_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Small(y)) => self.or_neg_u32_neg(y.wrapping_neg()),
            (&Small(x), _) => other.or_neg_u32_neg(x.wrapping_neg()),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_or_neg_neg(xs, ys));
                result.trim();
                result
            }
        }
    }
}
