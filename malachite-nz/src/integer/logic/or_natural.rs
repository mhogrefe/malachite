use std::cmp::max;
use std::iter::repeat;
use std::ops::{BitOr, BitOrAssign};

use malachite_base::comparison::Max;
use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_set_zero};
use malachite_base::num::traits::WrappingNegAssign;

use integer::Integer;
use natural::logic::not::{limbs_not_in_place, limbs_not_to_out};
use natural::Natural::{self, Large, Small};
use platform::Limb;

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, returns the limbs of the bitwise or of the `Integer`s. `xs` and `ys` may
/// not be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len() + ys.len()`, m = `ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or_natural::limbs_or_pos_neg;
///
/// assert_eq!(limbs_or_pos_neg(&[1, 2], &[100, 200]), &[99, 200]);
/// assert_eq!(limbs_or_pos_neg(&[1, 2], &[100, 200, 300]), &[99, 200, 300]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where res is returned, the first input is positive, and the
/// second is negative.
pub fn limbs_or_pos_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
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
        result_limbs.extend(repeat(Limb::MAX).take(y_i - xs_len));
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
    result_limbs.extend(
        xs[max_i + 1..]
            .iter()
            .zip(ys[max_i + 1..].iter())
            .map(|(x, y)| !x & y),
    );
    if xs_len < ys_len {
        result_limbs.extend_from_slice(&ys[xs_len..]);
    }
    result_limbs
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the limbs of the bitwise or of the `Integer`s to an output slice.
/// `xs` and `ys` may not be empty or only contain zeros. The output slice must be at least as long
/// as the second input slice. `ys.len()` limbs will be written; if the number of significant limbs
/// of the result is lower, some of the written limbs will be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than `ys`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or_natural::limbs_or_pos_neg_to_out;
///
/// let mut result = vec![0, 0];
/// limbs_or_pos_neg_to_out(&mut result, &[1, 2], &[100, 200]);
/// assert_eq!(result, &[99, 200]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_or_pos_neg_to_out(&mut result, &[1, 2], &[100, 200, 300]);
/// assert_eq!(result, &[99, 200, 300, 10]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where the first input is positive and the second is negative.
pub fn limbs_or_pos_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= ys_len);
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        limbs_set_zero(&mut out[..x_i]);
        out[x_i] = xs[x_i].wrapping_neg();
        limbs_not_to_out(&mut out[x_i + 1..xs_len], &xs[x_i + 1..]);
        for x in out[xs_len..y_i].iter_mut() {
            *x = Limb::MAX;
        }
        out[y_i] = ys[y_i] - 1;
        out[y_i + 1..ys_len].copy_from_slice(&ys[y_i + 1..]);
        return;
    } else if x_i >= ys_len {
        out[..ys_len].copy_from_slice(ys);
        return;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    limbs_set_zero(&mut out[..min_i]);
    if x_i == y_i {
        out[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
    } else if x_i > y_i {
        out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        out[x_i] = !xs[x_i] & ys[x_i];
    } else {
        out[x_i] = xs[x_i].wrapping_neg();
        limbs_not_to_out(&mut out[x_i + 1..y_i], &xs[x_i + 1..y_i]);
        out[y_i] = !xs[y_i] & (ys[y_i] - 1);
    };
    for (out, (x, y)) in out[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
    {
        *out = !x & y;
    }
    if xs_len < ys_len {
        out[xs_len..ys_len].copy_from_slice(&ys[xs_len..]);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the limbs of the bitwise or of the `Integer`s to the first (left)
/// slice. `xs` and `ys` may not be empty or only contain zeros. Returns whether the result is too
/// large to be contained in the first slice; if it is, only the lowest `xs.len()` limbs are
/// written.
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
/// use malachite_nz::integer::logic::or_natural::limbs_slice_or_pos_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// assert_eq!(limbs_slice_or_pos_neg_in_place_left(&mut xs, &[100, 200]), false);
/// assert_eq!(xs, &[99, 200]);
///
/// let mut xs = vec![1, 2];
/// assert_eq!(limbs_slice_or_pos_neg_in_place_left(&mut xs, &[100, 200, 300]), true);
/// assert_eq!(xs, &[99, 200]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where res == op1, the first input is positive and the second is
/// negative, and the length of op1 is not changed; instead, a carry is returned.
pub fn limbs_slice_or_pos_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..]);
        return true;
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        limbs_set_zero(&mut xs[ys_len..]);
        return false;
    }
    let max_i = max(x_i, y_i);
    if x_i == y_i {
        xs[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
    } else if x_i > y_i {
        xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        xs[x_i] = !xs[x_i] & ys[x_i];
    } else {
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..y_i]);
        xs[y_i] = !xs[y_i] & (ys[y_i] - 1);
    };
    if xs_len < ys_len {
        for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..xs_len].iter()) {
            *x = !*x & y
        }
        true
    } else {
        for (x, y) in xs[max_i + 1..ys_len].iter_mut().zip(ys[max_i + 1..].iter()) {
            *x = !*x & y
        }
        limbs_set_zero(&mut xs[ys_len..]);
        false
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the limbs of the bitwise or of the `Integer`s to the first (left)
/// slice. `xs` and `ys` may not be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::or_natural::limbs_vec_or_pos_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// limbs_vec_or_pos_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[99, 200]);
///
/// let mut xs = vec![1, 2];
/// limbs_vec_or_pos_neg_in_place_left(&mut xs, &[100, 200, 300]);
/// assert_eq!(xs, &[99, 200, 300]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where res == op1 and the first input is positive and the second
/// is negative.
pub fn limbs_vec_or_pos_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..]);
        xs.extend(repeat(Limb::MAX).take(y_i - xs_len));
        xs.push(ys[y_i] - 1);
        xs.extend_from_slice(&ys[y_i + 1..]);
        return;
    } else if x_i >= ys_len {
        *xs = ys.to_vec();
        return;
    }
    let max_i = max(x_i, y_i);
    if x_i == y_i {
        xs[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
    } else if x_i > y_i {
        xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        xs[x_i] = !xs[x_i] & ys[x_i];
    } else {
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..y_i]);
        xs[y_i] = !xs[y_i] & (ys[y_i] - 1);
    };
    if xs_len < ys_len {
        for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..xs_len].iter()) {
            *x = !*x & y
        }
        xs.extend_from_slice(&ys[xs_len..]);
    } else {
        for (x, y) in xs[max_i + 1..ys_len].iter_mut().zip(ys[max_i + 1..].iter()) {
            *x = !*x & y
        }
        xs.truncate(ys_len);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the limbs of the bitwise or of the `Integer`s to the second (right)
/// slice. `xs` and `ys` may not be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::or_natural::limbs_or_pos_neg_in_place_right;
///
/// let mut ys = vec![100, 200];
/// limbs_or_pos_neg_in_place_right(&[1, 2], &mut ys);
/// assert_eq!(ys, &[99, 200]);
///
/// let mut ys = vec![100, 200];
/// limbs_or_pos_neg_in_place_right(&[1, 2, 5], &mut ys);
/// assert_eq!(ys, &[99, 200]);
///
/// let mut ys = vec![1, 2, 5];
/// limbs_or_pos_neg_in_place_right(&[100, 200], &mut ys);
/// assert_eq!(ys, &[1, 2, 5]);
/// ```
///
/// This is mpz_ior from mpz/ior.c where res == op2 and the first input is positive and the second
/// is negative.
pub fn limbs_or_pos_neg_in_place_right(xs: &[Limb], ys: &mut [Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        ys[x_i] = xs[x_i].wrapping_neg();
        limbs_not_to_out(&mut ys[x_i + 1..xs_len], &xs[x_i + 1..]);
        for y in ys[xs_len..y_i].iter_mut() {
            *y = Limb::MAX;
        }
        ys[y_i] -= 1;
        return;
    } else if x_i >= ys_len {
        return;
    }
    let max_i = max(x_i, y_i);
    if x_i == y_i {
        ys[y_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
    } else if x_i > y_i {
        ys[x_i] &= !xs[x_i];
    } else {
        ys[x_i] = xs[x_i].wrapping_neg();
        limbs_not_to_out(&mut ys[x_i + 1..y_i], &xs[x_i + 1..y_i]);
        ys[y_i] = !xs[y_i] & (ys[y_i] - 1);
    };
    if xs_len < ys_len {
        for (x, y) in xs[max_i + 1..].iter().zip(ys[max_i + 1..xs_len].iter_mut()) {
            *y &= !x;
        }
    } else {
        for (x, y) in xs[max_i + 1..ys_len].iter().zip(ys[max_i + 1..].iter_mut()) {
            *y &= !x;
        }
    }
}

/// Takes the bitwise or of an `Integer` and a `Natural`, taking both by value.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`,
///     n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Integer::from(-123) | Natural::from(456u32)).to_string(), "-51");
/// assert_eq!((-Integer::trillion() | (Natural::trillion() + 1u32)).to_string(), "-4095");
/// ```
impl BitOr<Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn bitor(mut self, other: Natural) -> Integer {
        self |= other;
        self
    }
}

/// Takes the bitwise or of an `Integer` and a `Natural`, taking the `Integer` by value and the
/// `Natural` by reference.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`, n = `other.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Integer::from(-123) | &Natural::from(456u32)).to_string(), "-51");
/// assert_eq!((-Integer::trillion() | &(Natural::trillion() + 1u32)).to_string(), "-4095");
/// ```
impl<'a> BitOr<&'a Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn bitor(mut self, other: &'a Natural) -> Integer {
        self |= other;
        self
    }
}

/// Takes the bitwise or of an `Integer` and a `Natural`, taking the `Integer` by reference and the
/// `Natural` by value.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`, n = `other.significant_bits`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Integer::from(-123) | Natural::from(456u32)).to_string(), "-51");
/// assert_eq!((&-Integer::trillion() | (Natural::trillion() + 1u32)).to_string(), "-4095");
/// ```
impl<'a> BitOr<Natural> for &'a Integer {
    type Output = Integer;

    fn bitor(self, mut other: Natural) -> Integer {
        if self.sign {
            Integer {
                sign: true,
                abs: &self.abs | other,
            }
        } else {
            other.or_assign_pos_neg_ref(&self.abs);
            Integer {
                sign: false,
                abs: other,
            }
        }
    }
}

/// Bitwise-ors an `Integer` with a `Natural` in place, taking the `Natural` by reference.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`,
///     n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Integer::from(-123) | &Natural::from(456u32)).to_string(), "-51");
/// assert_eq!((&-Integer::trillion() | &(Natural::trillion() + 1u32)).to_string(), "-4095");
/// ```
impl<'a, 'b> BitOr<&'a Natural> for &'b Integer {
    type Output = Integer;

    fn bitor(self, other: &'a Natural) -> Integer {
        if self.sign {
            Integer {
                sign: true,
                abs: &self.abs | other,
            }
        } else {
            Integer {
                sign: false,
                abs: other.or_pos_neg(&self.abs),
            }
        }
    }
}

/// Bitwise-ors an `Integer` with a `Natural` in place, taking the `Natural` by value.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`,
///     n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x |= Natural::from(0x0000_000fu32);
///     x |= Natural::from(0x0000_0f00u32);
///     x |= Natural::from(0x000f_0000u32);
///     x |= Natural::from(0x0f00_0000u32);
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl BitOrAssign<Natural> for Integer {
    fn bitor_assign(&mut self, other: Natural) {
        if self.sign {
            self.abs.bitor_assign(other)
        } else {
            self.abs.or_assign_neg_pos(other)
        }
    }
}

/// Bitwise-ors an `Integer` with a `Natural` in place, taking the `Natural` by reference.
///
/// Time: worst case O(m)
///
/// Additional memory: worst case O(n)
///
/// where m = `self.significant_bits() + other.significant_bits`, n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x |= &Natural::from(0x0000_000fu32);
///     x |= &Natural::from(0x0000_0f00u32);
///     x |= &Natural::from(0x000f_0000u32);
///     x |= &Natural::from(0x0f00_0000u32);
///     assert_eq!(x, 0x0f0f_0f0f);
/// }
/// ```
impl<'a> BitOrAssign<&'a Natural> for Integer {
    fn bitor_assign(&mut self, other: &'a Natural) {
        if self.sign {
            self.abs.bitor_assign(other)
        } else {
            self.abs.or_assign_neg_pos_ref(other)
        }
    }
}

/// Takes the bitwise or of a `Natural` and an `Integer`, taking both by value.
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
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(456u32) | Integer::from(-123)).to_string(), "-51");
/// assert_eq!(((Natural::trillion() + 1u32) | -Integer::trillion()).to_string(), "-4095");
/// ```
impl BitOr<Integer> for Natural {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: Integer) -> Integer {
        other | self
    }
}

/// Takes the bitwise or of a `Natural` and an `Integer`, taking the `Natural` by value and the
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
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((Natural::from(456u32) | &Integer::from(-123)).to_string(), "-51");
/// assert_eq!(((Natural::trillion() + 1u32) | &-Integer::trillion()).to_string(), "-4095");
/// ```
impl<'a> BitOr<&'a Integer> for Natural {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: &'a Integer) -> Integer {
        other | self
    }
}

/// Takes the bitwise or of a `Natural` and an `Integer`, taking the `Natural` by reference and the
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
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(456u32) | Integer::from(-123)).to_string(), "-51");
/// assert_eq!((&(Natural::trillion() + 1u32) | -Integer::trillion()).to_string(), "-4095");
/// ```
impl<'a> BitOr<Integer> for &'a Natural {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: Integer) -> Integer {
        other | self
    }
}

/// Takes the bitwise or of a `Natural` and an `Integer`, taking both by reference.
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
/// use malachite_nz::natural::Natural;
///
/// assert_eq!((&Natural::from(456u32) | &Integer::from(-123)).to_string(), "-51");
/// assert_eq!((&(Natural::trillion() + 1u32) | &-Integer::trillion()).to_string(), "-4095");
/// ```
impl<'a, 'b> BitOr<&'a Integer> for &'b Natural {
    type Output = Integer;

    #[inline]
    fn bitor(self, other: &'a Integer) -> Integer {
        other | self
    }
}

impl Natural {
    pub(crate) fn or_assign_pos_neg_ref(&mut self, other: &Natural) {
        if let Small(y) = *other {
            self.or_assign_pos_limb_neg(y.wrapping_neg());
        } else if let Small(x) = *self {
            *self = other.or_neg_limb_pos(x);
        } else if let Large(ref ys) = *other {
            if let Large(ref mut xs) = *self {
                limbs_vec_or_pos_neg_in_place_left(xs, ys);
            }
            self.trim();
        }
    }

    pub(crate) fn or_assign_pos_neg(&mut self, other: Natural) {
        if let Small(y) = other {
            self.or_assign_pos_limb_neg(y.wrapping_neg());
        } else if let Small(x) = *self {
            *self = other;
            self.or_assign_neg_limb_pos(x);
        } else if let Large(mut ys) = other {
            if let Large(ref mut xs) = *self {
                limbs_or_pos_neg_in_place_right(xs, &mut ys);
                *xs = ys;
            }
            self.trim();
        }
    }

    pub(crate) fn or_assign_neg_pos_ref(&mut self, other: &Natural) {
        if let Small(y) = *other {
            self.or_assign_neg_limb_pos(y);
        } else if let Small(x) = *self {
            *self = other.or_pos_limb_neg(x.wrapping_neg());
        } else if let Large(ref ys) = *other {
            if let Large(ref mut xs) = *self {
                limbs_or_pos_neg_in_place_right(ys, xs);
            }
            self.trim();
        }
    }

    pub(crate) fn or_assign_neg_pos(&mut self, other: Natural) {
        if let Small(y) = other {
            self.or_assign_neg_limb_pos(y);
        } else if let Small(x) = *self {
            *self = other;
            self.or_assign_pos_limb_neg(x.wrapping_neg());
        } else if let Large(ref ys) = other {
            if let Large(ref mut xs) = *self {
                limbs_or_pos_neg_in_place_right(ys, xs);
            }
            self.trim();
        }
    }

    pub(crate) fn or_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Small(y)) => self.or_pos_limb_neg(y.wrapping_neg()),
            (&Small(x), _) => other.or_neg_limb_pos(x),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_or_pos_neg(xs, ys));
                result.trim();
                result
            }
        }
    }
}
