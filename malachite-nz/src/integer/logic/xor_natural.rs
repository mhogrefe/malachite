use std::cmp::max;
use std::iter::repeat;
use std::ops::{BitXor, BitXorAssign};

use malachite_base::comparison::Max;
use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_set_zero};
use malachite_base::num::arithmetic::traits::WrappingNegAssign;

use integer::Integer;
use natural::logic::not::limbs_not_in_place;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

fn limbs_xor_pos_neg_helper(input: Limb, boundary_limb_seen: &mut bool) -> Limb {
    if *boundary_limb_seen {
        !input
    } else if input == 0 {
        0
    } else {
        *boundary_limb_seen = true;
        input.wrapping_neg()
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, returns the limbs of the bitwise xor of the `Integer`s. `xs` and `ys` may
/// not be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::xor_natural::limbs_xor_pos_neg;
///
/// assert_eq!(limbs_xor_pos_neg(&[1, 2], &[100, 200]), &[99, 202]);
/// assert_eq!(limbs_xor_pos_neg(&[1, 2, 5], &[100, 200]), &[99, 202, 5]);
/// ```
///
/// This is mpz_xor from mpz/xor.c where res is returned, the first input is positive, and the
/// second is negative.
pub fn limbs_xor_pos_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
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
        let mut result_limbs = ys.to_vec();
        result_limbs.extend_from_slice(&xs[ys_len..]);
        return result_limbs;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    let mut result_limbs = vec![0; min_i];
    let mut boundary_limb_seen = false;
    if x_i == y_i {
        result_limbs.push(limbs_xor_pos_neg_helper(
            xs[x_i] ^ ys[y_i].wrapping_neg(),
            &mut boundary_limb_seen,
        ));
    } else if x_i > y_i {
        boundary_limb_seen = true;
        result_limbs.extend_from_slice(&ys[y_i..x_i]);
        result_limbs.push(xs[x_i] ^ ys[x_i]);
    } else {
        boundary_limb_seen = true;
        result_limbs.push(xs[x_i].wrapping_neg());
        result_limbs.extend(xs[x_i + 1..y_i].iter().map(|x| !x));
        result_limbs.push(xs[y_i] ^ (ys[y_i] - 1));
    }
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter());
    if boundary_limb_seen {
        result_limbs.extend(xys.map(|(x, y)| x ^ y));
    } else {
        for (&x, &y) in xys {
            result_limbs.push(limbs_xor_pos_neg_helper(x ^ !y, &mut boundary_limb_seen));
        }
    }
    if xs_len != ys_len {
        let zs = if xs_len > ys_len {
            &xs[ys_len..]
        } else {
            &ys[xs_len..]
        };
        if boundary_limb_seen {
            result_limbs.extend_from_slice(zs);
        } else {
            for &z in zs.iter() {
                result_limbs.push(limbs_xor_pos_neg_helper(!z, &mut boundary_limb_seen));
            }
        }
    }
    result_limbs
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the limbs of the bitwise xor of the `Integer`s to an output slice.
/// `xs` and `ys` may not be empty or only contain zeros. The output slice must be at least as long
/// as the longer of the two input slices. max(`xs.len()`, `ys.len()`) limbs will be written; if the
/// number of significant limbs of the result is lower, some of the written limbs will be zero.
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
/// use malachite_nz::integer::logic::xor_natural::limbs_xor_pos_neg_to_out;
///
/// let mut result = vec![0, 0];
/// limbs_xor_pos_neg_to_out(&mut result, &[1, 2], &[100, 200]);
/// assert_eq!(result, &[99, 202]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_xor_pos_neg_to_out(&mut result, &[1, 2, 5], &[100, 200]);
/// assert_eq!(result, &[99, 202, 5, 10]);
/// ```
///
/// This is mpz_xor from mpz/xor.c where the first input is positive and the second is negative.
pub fn limbs_xor_pos_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    assert!(out.len() >= ys_len);
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        limbs_set_zero(&mut out[..x_i]);
        out[x_i] = xs[x_i].wrapping_neg();
        for (out, &x) in out[x_i + 1..xs_len].iter_mut().zip(xs[x_i + 1..].iter()) {
            *out = !x;
        }
        for out in out[xs_len..y_i].iter_mut() {
            *out = Limb::MAX;
        }
        out[y_i] = ys[y_i] - 1;
        out[y_i + 1..ys_len].copy_from_slice(&ys[y_i + 1..]);
        return;
    } else if x_i >= ys_len {
        out[..ys_len].copy_from_slice(ys);
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
        return;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    limbs_set_zero(&mut out[..min_i]);
    let mut boundary_limb_seen = false;
    if x_i == y_i {
        out[x_i] =
            limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_limb_seen);
    } else if x_i > y_i {
        boundary_limb_seen = true;
        out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        out[x_i] = xs[x_i] ^ ys[x_i];
    } else {
        boundary_limb_seen = true;
        out[x_i] = xs[x_i].wrapping_neg();
        for (out, &x) in out[x_i + 1..y_i].iter_mut().zip(xs[x_i + 1..y_i].iter()) {
            *out = !x;
        }
        out[y_i] = xs[y_i] ^ (ys[y_i] - 1);
    }
    {
        let xys = out[max_i + 1..]
            .iter_mut()
            .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()));
        if boundary_limb_seen {
            for (out, (&x, &y)) in xys {
                *out = x ^ y;
            }
        } else {
            for (out, (&x, &y)) in xys {
                *out = limbs_xor_pos_neg_helper(x ^ !y, &mut boundary_limb_seen);
            }
        }
    }
    if xs_len != ys_len {
        let (min_len, max_len, zs) = if xs_len > ys_len {
            (ys_len, xs_len, &xs[ys_len..])
        } else {
            (xs_len, ys_len, &ys[xs_len..])
        };
        if boundary_limb_seen {
            out[min_len..max_len].copy_from_slice(zs);
        } else {
            for (out, &z) in out[min_len..].iter_mut().zip(zs.iter()) {
                *out = limbs_xor_pos_neg_helper(!z, &mut boundary_limb_seen);
            }
        }
    }
}

fn limbs_xor_pos_neg_in_place_left_helper(
    xs: &mut [Limb],
    ys: &[Limb],
    x_i: usize,
    y_i: usize,
) -> bool {
    let max_i = max(x_i, y_i);
    let mut boundary_limb_seen = false;
    if x_i == y_i {
        xs[x_i] =
            limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_limb_seen);
    } else if x_i > y_i {
        boundary_limb_seen = true;
        xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        xs[x_i] ^= ys[x_i];
    } else {
        boundary_limb_seen = true;
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..y_i]);
        xs[y_i] ^= ys[y_i] - 1;
    }
    let xys = xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter());
    if boundary_limb_seen {
        for (x, &y) in xys {
            *x ^= y;
        }
    } else {
        for (x, &y) in xys {
            *x = limbs_xor_pos_neg_helper(*x ^ !y, &mut boundary_limb_seen);
        }
    }
    boundary_limb_seen
}

/// Interpreting a `Vec` of `Limb`s and a slice of `Limb`s as the limbs (in ascending order) of one
/// `Integer` and the negative of another, writes the limbs of the bitwise xor of the `Integer`s to
/// the `Vec`. `xs` and `ys` may not be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::xor_natural::limbs_xor_pos_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// limbs_xor_pos_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[99, 202]);
///
/// let mut xs = vec![1, 2, 5];
/// limbs_xor_pos_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[99, 202, 5]);
/// ```
///
/// This is mpz_xor from mpz/xor.c where res == op1 and the first input is positive and the second
/// is negative.
pub fn limbs_xor_pos_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
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
        xs[..ys_len].copy_from_slice(ys);
        return;
    }
    let mut boundary_limb_seen = limbs_xor_pos_neg_in_place_left_helper(xs, ys, x_i, y_i);
    if xs_len > ys_len {
        if !boundary_limb_seen {
            for x in xs[ys_len..].iter_mut() {
                *x = limbs_xor_pos_neg_helper(!*x, &mut boundary_limb_seen);
            }
        }
    } else if xs_len < ys_len {
        if boundary_limb_seen {
            xs.extend_from_slice(&ys[xs_len..]);
        } else {
            for &y in ys[xs_len..].iter() {
                xs.push(limbs_xor_pos_neg_helper(!y, &mut boundary_limb_seen));
            }
        }
    }
}

fn limbs_xor_pos_neg_in_place_right_helper(
    xs: &[Limb],
    ys: &mut [Limb],
    x_i: usize,
    y_i: usize,
) -> bool {
    let max_i = max(x_i, y_i);
    let mut boundary_limb_seen = false;
    if x_i == y_i {
        ys[y_i] =
            limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_limb_seen);
    } else if x_i > y_i {
        boundary_limb_seen = true;
        ys[x_i] ^= xs[x_i];
    } else {
        boundary_limb_seen = true;
        ys[x_i] = xs[x_i].wrapping_neg();
        for (y, &x) in ys[x_i + 1..].iter_mut().zip(xs[x_i + 1..y_i].iter()) {
            *y = !x;
        }
        ys[y_i] -= 1;
        ys[y_i] ^= xs[y_i];
    }
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter_mut());
    if boundary_limb_seen {
        for (&x, y) in xys {
            *y ^= x;
        }
    } else {
        for (&x, y) in xys {
            *y = limbs_xor_pos_neg_helper(x ^ !*y, &mut boundary_limb_seen);
        }
    }
    boundary_limb_seen
}

/// Interpreting a slice of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of one
/// `Integer` and the negative of another, writes the limbs of the bitwise xor of the `Integer`s to
/// the second (right) slice. `xs` and `ys` may not be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len() + ys.len()`, m = `max(1, xs.len() - ys.len())`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor_natural::limbs_xor_pos_neg_in_place_right;
///
/// let mut ys = vec![100, 200];
/// limbs_xor_pos_neg_in_place_right(&[1, 2], &mut ys);
/// assert_eq!(ys, &[99, 202]);
///
/// let mut ys = vec![100, 200];
/// limbs_xor_pos_neg_in_place_right(&[1, 2, 5], &mut ys);
/// assert_eq!(ys, &[99, 202, 5]);
/// ```
///
/// This is mpz_xor from mpz/xor.c where res == op2 and the first input is positive and the second
/// is negative.
pub fn limbs_xor_pos_neg_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        ys[x_i] = xs[x_i].wrapping_neg();
        for (y, &x) in ys[x_i + 1..].iter_mut().zip(xs[x_i + 1..].iter()) {
            *y = !x;
        }
        for y in ys.iter_mut().take(y_i).skip(xs_len) {
            *y = Limb::MAX;
        }
        ys[y_i] -= 1;
        return;
    } else if x_i >= ys_len {
        ys.extend_from_slice(&xs[ys_len..]);
        return;
    }
    let mut boundary_limb_seen = limbs_xor_pos_neg_in_place_right_helper(xs, ys, x_i, y_i);
    if xs_len > ys_len {
        if boundary_limb_seen {
            ys.extend_from_slice(&xs[ys_len..]);
        } else {
            for &x in xs[ys_len..].iter() {
                ys.push(limbs_xor_pos_neg_helper(!x, &mut boundary_limb_seen));
            }
        }
    } else if xs_len < ys_len && !boundary_limb_seen {
        for y in ys[xs_len..].iter_mut() {
            *y = limbs_xor_pos_neg_helper(!*y, &mut boundary_limb_seen);
        }
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of one `Integer` and the
/// negative of another, writes the limbs of the bitwise xor of the `Integer`s to the longer slice
/// (or the first one, if they are equally long). `xs` and `ys` may not be empty or only contain
/// zeros. Returns a `bool` which is `false` when the output is to the first slice and `true` when
/// it's to the second slice.
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
/// use malachite_nz::integer::logic::xor_natural::limbs_xor_pos_neg_in_place_either;
///
/// let mut xs = vec![1, 2];
/// let mut ys = vec![100, 200];
/// assert_eq!(limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[99, 202]);
/// assert_eq!(ys, &[100, 200]);
///
/// let mut xs = vec![1, 2, 5];
/// let mut ys = vec![100, 200];
/// assert_eq!(limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[99, 202, 5]);
/// assert_eq!(ys, &[100, 200]);
///
/// let mut xs = vec![100, 200];
/// let mut ys = vec![1, 2, 5];
/// assert_eq!(limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, &[100, 200]);
/// assert_eq!(ys, &[101, 202, 5]);
/// ```
///
/// This is mpz_xor from mpz/xor.c where the first input is positive, the second is negative, and
/// the result is written to the longer input slice.
pub fn limbs_xor_pos_neg_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        ys[x_i] = xs[x_i].wrapping_neg();
        for (y, &x) in ys[x_i + 1..].iter_mut().zip(xs[x_i + 1..].iter()) {
            *y = !x;
        }
        for y in ys[xs_len..y_i].iter_mut() {
            *y = Limb::MAX;
        }
        ys[y_i] -= 1;
        return true;
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        return false;
    }
    if xs_len >= ys_len {
        let mut boundary_limb_seen = limbs_xor_pos_neg_in_place_left_helper(xs, ys, x_i, y_i);
        if xs_len != ys_len && !boundary_limb_seen {
            for x in xs[ys_len..].iter_mut() {
                *x = limbs_xor_pos_neg_helper(!*x, &mut boundary_limb_seen);
            }
        }
        false
    } else {
        let mut boundary_limb_seen = limbs_xor_pos_neg_in_place_right_helper(xs, ys, x_i, y_i);
        if !boundary_limb_seen {
            for y in ys[xs_len..].iter_mut() {
                *y = limbs_xor_pos_neg_helper(!*y, &mut boundary_limb_seen);
            }
        }
        true
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
/// assert_eq!((Integer::from(-123) ^ Natural::from(456u32)).to_string(), "-435");
/// assert_eq!((-Integer::trillion() ^ (Natural::trillion() + 1u32)).to_string(), "-8191");
/// ```
impl BitXor<Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn bitxor(mut self, other: Natural) -> Integer {
        self ^= other;
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
/// assert_eq!((Integer::from(-123) ^ &Natural::from(456u32)).to_string(), "-435");
/// assert_eq!((-Integer::trillion() ^ &(Natural::trillion() + 1u32)).to_string(), "-8191");
/// ```
impl<'a> BitXor<&'a Natural> for Integer {
    type Output = Integer;

    #[inline]
    fn bitxor(mut self, other: &'a Natural) -> Integer {
        self ^= other;
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
/// assert_eq!((&Integer::from(-123) ^ Natural::from(456u32)).to_string(), "-435");
/// assert_eq!((&-Integer::trillion() ^ (Natural::trillion() + 1u32)).to_string(), "-8191");
/// ```
impl<'a> BitXor<Natural> for &'a Integer {
    type Output = Integer;

    fn bitxor(self, mut other: Natural) -> Integer {
        if self.sign {
            Integer {
                sign: true,
                abs: &self.abs ^ other,
            }
        } else {
            other.xor_assign_pos_neg_ref(&self.abs);
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
/// assert_eq!((&Integer::from(-123) ^ &Natural::from(456u32)).to_string(), "-435");
/// assert_eq!((&-Integer::trillion() ^ &(Natural::trillion() + 1u32)).to_string(), "-8191");
/// ```
impl<'a, 'b> BitXor<&'a Natural> for &'b Integer {
    type Output = Integer;

    fn bitxor(self, other: &'a Natural) -> Integer {
        if self.sign {
            Integer {
                sign: true,
                abs: &self.abs ^ other,
            }
        } else {
            Integer {
                sign: false,
                abs: other.xor_pos_neg(&self.abs),
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::from(0xffff_ffffu32);
///     x ^= Natural::from(0x0000_000fu32);
///     x ^= Natural::from(0x0000_0f00u32);
///     x ^= Natural::from(0x000f_0000u32);
///     x ^= Natural::from(0x0f00_0000u32);
///     assert_eq!(x, 0xf0f0_f0f0u32);
/// }
/// ```
impl BitXorAssign<Natural> for Integer {
    fn bitxor_assign(&mut self, other: Natural) {
        if self.sign {
            self.abs.bitxor_assign(other)
        } else {
            self.abs.xor_assign_neg_pos(other)
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
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::from(0xffff_ffffu32);
///     x ^= &Natural::from(0x0000_000fu32);
///     x ^= &Natural::from(0x0000_0f00u32);
///     x ^= &Natural::from(0x000f_0000u32);
///     x ^= &Natural::from(0x0f00_0000u32);
///     assert_eq!(x, 0xf0f0_f0f0u32);
/// }
/// ```
impl<'a> BitXorAssign<&'a Natural> for Integer {
    fn bitxor_assign(&mut self, other: &'a Natural) {
        if self.sign {
            self.abs.bitxor_assign(other)
        } else {
            self.abs.xor_assign_neg_pos_ref(other)
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
/// assert_eq!((Natural::from(456u32) ^ Integer::from(-123)).to_string(), "-435");
/// assert_eq!(((Natural::trillion() + 1u32) ^ -Integer::trillion()).to_string(), "-8191");
/// ```
impl BitXor<Integer> for Natural {
    type Output = Integer;

    #[inline]
    fn bitxor(self, other: Integer) -> Integer {
        other ^ self
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
/// assert_eq!((Natural::from(456u32) ^ &Integer::from(-123)).to_string(), "-435");
/// assert_eq!(((Natural::trillion() + 1u32) ^ &-Integer::trillion()).to_string(), "-8191");
/// ```
impl<'a> BitXor<&'a Integer> for Natural {
    type Output = Integer;

    #[inline]
    fn bitxor(self, other: &'a Integer) -> Integer {
        other ^ self
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
/// assert_eq!((&Natural::from(456u32) ^ Integer::from(-123)).to_string(), "-435");
/// assert_eq!((&(Natural::trillion() + 1u32) ^ -Integer::trillion()).to_string(), "-8191");
/// ```
impl<'a> BitXor<Integer> for &'a Natural {
    type Output = Integer;

    #[inline]
    fn bitxor(self, other: Integer) -> Integer {
        other ^ self
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
/// assert_eq!((&Natural::from(456u32) ^ &Integer::from(-123)).to_string(), "-435");
/// assert_eq!((&(Natural::trillion() + 1u32) ^ &-Integer::trillion()).to_string(), "-8191");
/// ```
impl<'a, 'b> BitXor<&'a Integer> for &'b Natural {
    type Output = Integer;

    #[inline]
    fn bitxor(self, other: &'a Integer) -> Integer {
        other ^ self
    }
}

impl Natural {
    pub(crate) fn xor_assign_pos_neg(&mut self, mut other: Natural) {
        if let Natural(Small(y)) = other {
            self.xor_assign_pos_limb_neg(y.wrapping_neg());
            return;
        } else if let Natural(Small(x)) = *self {
            if let Natural(Large(_)) = other {
                *self = other;
                self.xor_assign_neg_limb_pos(x);
            }
            return;
        } else if let Natural(Large(ref mut ys)) = other {
            let right = if let Natural(Large(ref mut xs)) = *self {
                limbs_xor_pos_neg_in_place_either(xs, ys)
            } else {
                unreachable!();
            };
            if !right {
                self.trim();
                return;
            }
        } else {
            return;
        };
        *self = other;
        self.trim();
    }

    pub(crate) fn xor_assign_pos_neg_ref(&mut self, other: &Natural) {
        if let Natural(Small(y)) = *other {
            self.xor_assign_pos_limb_neg(y.wrapping_neg());
        } else if let Natural(Small(x)) = *self {
            if let Natural(Large(_)) = *other {
                *self = other.clone();
                self.xor_assign_neg_limb_pos(x);
            }
        } else if let Natural(Large(ref ys)) = *other {
            if let Natural(Large(ref mut xs)) = *self {
                limbs_xor_pos_neg_in_place_left(xs, ys);
            }
            self.trim();
        }
    }

    pub(crate) fn xor_assign_neg_pos(&mut self, mut other: Natural) {
        other.xor_assign_pos_neg_ref(&*self);
        *self = other;
    }

    pub(crate) fn xor_assign_neg_pos_ref(&mut self, other: &Natural) {
        let new_self_value = if let Natural(Small(x)) = *self {
            let mut new_self_value = other.clone();
            new_self_value.xor_assign_pos_limb_neg(x.wrapping_neg());
            Some(new_self_value)
        } else if let Natural(Small(y)) = *other {
            if let Natural(Large(_)) = *self {
                self.xor_assign_neg_limb_pos(y);
            } else {
                unreachable!()
            };
            None
        } else if let Natural(Large(ref ys)) = *other {
            if let Natural(Large(ref mut xs)) = *self {
                limbs_xor_pos_neg_in_place_right(ys, xs);
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

    pub(crate) fn xor_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.xor_pos_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), _) => other.xor_neg_limb_pos(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                let mut result = Natural(Large(limbs_xor_pos_neg(xs, ys)));
                result.trim();
                result
            }
        }
    }
}
