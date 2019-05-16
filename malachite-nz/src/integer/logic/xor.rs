use std::ops::{BitXor, BitXorAssign};

use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_set_zero};
use malachite_base::num::traits::WrappingNegAssign;

use integer::Integer;
use natural::arithmetic::sub::{
    limbs_sub, limbs_sub_in_place_left, limbs_sub_to_out, limbs_vec_sub_in_place_right,
};
use natural::logic::not::limbs_not_in_place;
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, returns the limbs of the bitwise xor of the `Integer`s. `xs` and `ys` may not be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len() + ys.len()`, m = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_xor_neg_neg;
///
/// assert_eq!(limbs_xor_neg_neg(&[1, 2], &[100, 200]), &[99, 202]);
/// assert_eq!(limbs_xor_neg_neg(&[1, 2, 5], &[100, 200]), &[99, 202, 5]);
/// ```
pub fn limbs_xor_neg_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        let (result, borrow) = limbs_sub(ys, xs);
        assert!(!borrow);
        return result;
    } else if x_i >= ys_len {
        let (result, borrow) = limbs_sub(xs, ys);
        assert!(!borrow);
        return result;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    let mut result_limbs = vec![0; min_i];
    if x_i == y_i {
        result_limbs.push(xs[x_i].wrapping_neg() ^ ys[x_i].wrapping_neg());
    } else {
        let (min_zs, max_zs) = if x_i <= y_i { (xs, ys) } else { (ys, xs) };
        result_limbs.push(min_zs[min_i].wrapping_neg());
        result_limbs.extend(min_zs[min_i + 1..max_i].iter().map(|limb| !limb));
        result_limbs.push((max_zs[max_i] - 1) ^ min_zs[max_i]);
    }
    result_limbs.extend(
        xs[max_i + 1..]
            .iter()
            .zip(ys[max_i + 1..].iter())
            .map(|(x, y)| x ^ y),
    );
    if xs_len > ys_len {
        result_limbs.extend_from_slice(&xs[ys_len..]);
    } else if xs_len < ys_len {
        result_limbs.extend_from_slice(&ys[xs_len..]);
    }
    result_limbs
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, writes the max(`xs.len()`, `ys.len()`) limbs of the bitwise xor of the `Integer`s to
/// an output slice. `xs` and `ys` may not be empty or only contain zeros. The output slice must be
/// at least as long as the longer input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than the
/// longer of `xs` and `ys`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_xor_neg_neg_to_out;
///
/// let mut result = vec![0, 0];
/// limbs_xor_neg_neg_to_out(&mut result, &[1, 2], &[100, 200]);
/// assert_eq!(result, &[99, 202]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_xor_neg_neg_to_out(&mut result, &[1, 2, 5], &[100, 200]);
/// assert_eq!(result, &[99, 202, 5, 10]);
/// ```
pub fn limbs_xor_neg_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    assert!(out.len() >= ys_len);
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        assert!(!limbs_sub_to_out(out, ys, xs));
        return;
    } else if x_i >= ys_len {
        assert!(!limbs_sub_to_out(out, xs, ys));
        return;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    limbs_set_zero(&mut out[..min_i]);
    if x_i == y_i {
        out[x_i] = xs[x_i].wrapping_neg() ^ ys[x_i].wrapping_neg();
    } else {
        let (min_zs, max_zs) = if x_i <= y_i { (xs, ys) } else { (ys, xs) };
        out[min_i] = min_zs[min_i].wrapping_neg();
        for (out, &z) in out[min_i + 1..max_i]
            .iter_mut()
            .zip(min_zs[min_i + 1..max_i].iter())
        {
            *out = !z;
        }
        out[max_i] = (max_zs[max_i] - 1) ^ min_zs[max_i];
    }
    for (out, (&x, &y)) in out[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
    {
        *out = x ^ y;
    }
    if xs_len > ys_len {
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
    } else if xs_len < ys_len {
        out[xs_len..ys_len].copy_from_slice(&ys[xs_len..]);
    }
}

fn limbs_xor_neg_neg_in_place_helper(xs: &mut [Limb], ys: &[Limb], x_i: usize, y_i: usize) {
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    if x_i == y_i {
        xs[x_i] = xs[x_i].wrapping_neg() ^ ys[x_i].wrapping_neg();
    } else if x_i <= y_i {
        xs[min_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[min_i + 1..max_i]);
        xs[max_i] ^= ys[max_i] - 1;
    } else {
        xs[min_i] = ys[min_i].wrapping_neg();
        for (x, &y) in xs[min_i + 1..max_i].iter_mut().zip(ys[min_i + 1..].iter()) {
            *x = !y;
        }
        xs[max_i] -= 1;
        xs[max_i] ^= ys[max_i];
    }
    for (x, &y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
        *x ^= y;
    }
}

/// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of the
/// negatives of two `Integer`s, writes the limbs of the bitwise xor of the `Integer`s to the `Vec`.
/// `xs` and `ys` may not be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len() + ys.len()`, m = `max(1, ys.len() - xs.len())`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_xor_neg_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// limbs_xor_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[99, 202]);
///
/// let mut xs = vec![100, 200];
/// limbs_xor_neg_neg_in_place_left(&mut xs, &[1, 2, 5]);
/// assert_eq!(xs, &[99, 202, 5]);
///
/// let mut xs = vec![1, 2, 5];
/// limbs_xor_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[99, 202, 5]);
/// ```
pub fn limbs_xor_neg_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        assert!(!limbs_vec_sub_in_place_right(ys, xs));
        return;
    } else if x_i >= ys_len {
        assert!(!limbs_sub_in_place_left(xs, ys));
        return;
    }
    limbs_xor_neg_neg_in_place_helper(xs, ys, x_i, y_i);
    if xs_len < ys_len {
        xs.extend_from_slice(&ys[xs_len..]);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, writes the limbs of the bitwise xor of the `Integer`s to the longer slice (or the
/// first one, if they are equally long). `xs` and `ys` may not be empty or only contain zeros.
/// Returns `false` when the output is to the first slice and `true` when it's to the second slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_xor_neg_neg_in_place_either;
///
/// let mut xs = vec![1, 2];
/// let mut ys = vec![100, 200];
/// assert_eq!(limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[99, 202]);
/// assert_eq!(ys, &[100, 200]);
///
/// let mut xs = vec![1, 2, 5];
/// let mut ys = vec![100, 200];
/// assert_eq!(limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[99, 202, 5]);
/// assert_eq!(ys, &[100, 200]);
///
/// let mut xs = vec![100, 200];
/// let mut ys = vec![1, 2, 5];
/// assert_eq!(limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, &[100, 200]);
/// assert_eq!(ys, &[99, 202, 5]);
/// ```
pub fn limbs_xor_neg_neg_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        assert!(!limbs_sub_in_place_left(ys, xs));
        return true;
    } else if x_i >= ys_len {
        assert!(!limbs_sub_in_place_left(xs, ys));
        return false;
    }
    if xs_len >= ys_len {
        limbs_xor_neg_neg_in_place_helper(xs, ys, x_i, y_i);
        false
    } else {
        limbs_xor_neg_neg_in_place_helper(ys, xs, y_i, x_i);
        true
    }
}

/// Takes the bitwise xor of two `Integer`s, taking both by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits() + other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) ^ Integer::from(-456)).to_string(), "445");
/// assert_eq!((-Integer::trillion() ^ -(Integer::trillion() + 1u32)).to_string(), "8191");
/// ```
impl BitXor<Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn bitxor(mut self, other: Integer) -> Integer {
        self ^= other;
        self
    }
}

/// Takes the bitwise xor of two `Integer`s, taking the left `Integer` by value and the right
/// `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `self.significant_bits() + other.significant_bits()`, m = `other.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) ^ &Integer::from(-456)).to_string(), "445");
/// assert_eq!((-Integer::trillion() ^ &-(Integer::trillion() + 1u32)).to_string(), "8191");
/// ```
impl<'a> BitXor<&'a Integer> for Integer {
    type Output = Integer;

    #[inline]
    fn bitxor(mut self, other: &'a Integer) -> Integer {
        self ^= other;
        self
    }
}

/// Takes the bitwise xor of two `Integer`s, taking the left `Integer` by reference and the right
/// `Integer` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.significant_bits() + ys.significant_bits()`, m = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(-123) ^ Integer::from(-456)).to_string(), "445");
/// assert_eq!((&-Integer::trillion() ^ -(Integer::trillion() + 1u32)).to_string(), "8191");
/// ```
impl<'a> BitXor<Integer> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn bitxor(self, mut other: Integer) -> Integer {
        other ^= self;
        other
    }
}

/// Takes the bitwise xor of two `Integer`s, taking both `Integer`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `self.significant_bits() + other.significant_bits()`,
///     m = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(-123) ^ &Integer::from(-456)).to_string(), "445");
/// assert_eq!((&-Integer::trillion() ^ &-(Integer::trillion() + 1u32)).to_string(), "8191");
/// ```
impl<'a, 'b> BitXor<&'a Integer> for &'b Integer {
    type Output = Integer;

    fn bitxor(self, other: &'a Integer) -> Integer {
        match (self.sign, other.sign) {
            (true, true) => Integer {
                sign: true,
                abs: &self.abs ^ &other.abs,
            },
            (true, false) => Integer {
                sign: false,
                abs: self.abs.xor_pos_neg(&other.abs),
            },
            (false, true) => Integer {
                sign: false,
                abs: other.abs.xor_pos_neg(&self.abs),
            },
            (false, false) => Integer {
                sign: true,
                abs: self.abs.xor_neg_neg(&other.abs),
            },
        }
    }
}

/// Bitwise-xors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
/// value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits() + other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(0xffff_ffffu32);
///     x ^= Integer::from(0x0000_000f);
///     x ^= Integer::from(0x0000_0f00);
///     x ^= Integer::from(0x000f_0000);
///     x ^= Integer::from(0x0f00_0000);
///     assert_eq!(x, 0xf0f0_f0f0u32);
/// }
/// ```
impl BitXorAssign<Integer> for Integer {
    fn bitxor_assign(&mut self, other: Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitxor_assign(other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.xor_assign_pos_neg(other.abs)
            }
            (false, true) => self.abs.xor_assign_neg_pos(other.abs),
            (false, false) => {
                self.sign = true;
                self.abs.xor_assign_neg_neg(other.abs)
            }
        }
    }
}

/// Bitwise-xors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
/// reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.significant_bits() + ys.significant_bits()`, m = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(0xffff_ffffu32);
///     x ^= &Integer::from(0x0000_000f);
///     x ^= &Integer::from(0x0000_0f00);
///     x ^= &Integer::from(0x000f_0000);
///     x ^= &Integer::from(0x0f00_0000);
///     assert_eq!(x, 0xf0f0_f0f0u32);
/// }
/// ```
impl<'a> BitXorAssign<&'a Integer> for Integer {
    fn bitxor_assign(&mut self, other: &'a Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitxor_assign(&other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.xor_assign_pos_neg_ref(&other.abs)
            }
            (false, true) => self.abs.xor_assign_neg_pos_ref(&other.abs),
            (false, false) => {
                self.sign = true;
                self.abs.xor_assign_neg_neg_ref(&other.abs)
            }
        }
    }
}

impl Natural {
    fn xor_assign_neg_neg(&mut self, other: Natural) {
        let new_self_value = if let Small(y) = other {
            self.xor_assign_neg_limb_neg(y.wrapping_neg());
            None
        } else if let Small(ref mut x) = *self {
            let mut new_self_value = other.clone();
            new_self_value.xor_assign_neg_limb_neg(x.wrapping_neg());
            Some(new_self_value)
        } else if let Large(mut ys) = other {
            if let Large(ref mut xs) = *self {
                if limbs_xor_neg_neg_in_place_either(xs, &mut ys) {
                    *xs = ys;
                }
            }
            self.trim();
            None
        } else {
            None
        };
        if let Some(new_self_value) = new_self_value {
            *self = new_self_value;
        }
    }

    fn xor_assign_neg_neg_ref(&mut self, other: &Natural) {
        let new_self_value = if let Small(y) = *other {
            self.xor_assign_neg_limb_neg(y.wrapping_neg());
            None
        } else if let Small(ref mut x) = *self {
            let mut new_self_value = other.clone();
            new_self_value.xor_assign_neg_limb_neg(x.wrapping_neg());
            Some(new_self_value)
        } else if let Large(ref ys) = *other {
            if let Large(ref mut xs) = *self {
                limbs_xor_neg_neg_in_place_left(xs, ys);
            }
            self.trim();
            None
        } else {
            None
        };
        if let Some(new_self_value) = new_self_value {
            *self = new_self_value;
        }
    }

    fn xor_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Small(y)) => self.xor_neg_limb_neg(y.wrapping_neg()),
            (&Small(x), _) => other.xor_neg_limb_neg(x.wrapping_neg()),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_xor_neg_neg(xs, ys));
                result.trim();
                result
            }
        }
    }
}
