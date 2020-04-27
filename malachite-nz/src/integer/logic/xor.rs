use std::cmp::{max, Ordering};
use std::iter::repeat;
use std::ops::{BitXor, BitXorAssign};

use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero};

use integer::Integer;
use natural::arithmetic::add::{
    limbs_add_limb, limbs_add_limb_to_out, limbs_slice_add_limb_in_place,
};
use natural::arithmetic::sub::{
    limbs_sub, limbs_sub_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place,
    limbs_sub_limb_to_out, limbs_sub_to_out, limbs_vec_sub_in_place_right,
};
use natural::logic::not::limbs_not_in_place;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise xor of the `Integer` and a `Limb`. `limbs` cannot be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb;
///
/// assert_eq!(limbs_neg_xor_limb(&[123, 456], 789), &[880, 456]);
/// assert_eq!(limbs_neg_xor_limb(&[0xffff_fffe, 0xffff_ffff, 0xffff_ffff], 2), &[0, 0, 0, 1]);
/// ```
pub fn limbs_neg_xor_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    if y == 0 {
        return xs.to_vec();
    }
    let head = xs[0];
    let tail = &xs[1..];
    let mut result_limbs = Vec::with_capacity(xs.len());
    if head != 0 {
        let head = head.wrapping_neg() ^ y;
        if head == 0 {
            result_limbs.push(0);
            result_limbs.extend_from_slice(&limbs_add_limb(tail, 1));
        } else {
            result_limbs.push(head.wrapping_neg());
            result_limbs.extend_from_slice(tail);
        }
    } else {
        result_limbs.push(y.wrapping_neg());
        result_limbs.extend_from_slice(&limbs_sub_limb(tail, 1).0);
    }
    result_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise and of the `Integer`, writes the limbs of the bitwise
/// xor of the `Integer` and a `Limb` to an output slice. The output slice must be at least as long
/// as the input slice. `limbs` cannot be empty or only contain zeros. Returns whether a carry
/// occurs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb_to_out;
///
/// let mut limbs = vec![0, 0, 0, 0];
/// assert_eq!(limbs_neg_xor_limb_to_out(&mut limbs, &[123, 456], 789), false);
/// assert_eq!(limbs, &[880, 456, 0, 0]);
///
/// let mut limbs = vec![10, 10, 10, 10];
/// assert_eq!(limbs_neg_xor_limb_to_out(&mut limbs, &[0xffff_fffe, 0xffff_ffff, 0xffff_ffff], 2),
///     true);
/// assert_eq!(limbs, &[0, 0, 0, 10]);
/// ```
pub fn limbs_neg_xor_limb_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) -> bool {
    let len = xs.len();
    assert!(out.len() >= len);
    if y == 0 {
        out[..len].copy_from_slice(xs);
        return false;
    }
    let head = xs[0];
    let tail = &xs[1..];
    if head != 0 {
        let head = head.wrapping_neg() ^ y;
        if head == 0 {
            out[0] = 0;
            limbs_add_limb_to_out(&mut out[1..len], tail, 1)
        } else {
            out[0] = head.wrapping_neg();
            out[1..len].copy_from_slice(tail);
            false
        }
    } else {
        out[0] = y.wrapping_neg();
        limbs_sub_limb_to_out(&mut out[1..len], tail, 1);
        false
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a `Limb` to the input slice.
/// `limbs` cannot be empty or only contain zeros. Returns whether a carry occurs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_slice_neg_xor_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_neg_xor_limb_in_place(&mut limbs, 789), false);
/// assert_eq!(limbs, &[880, 456]);
///
/// let mut limbs = vec![0xffff_fffe, 0xffff_ffff, 0xffff_ffff];
/// assert_eq!(limbs_slice_neg_xor_limb_in_place(&mut limbs, 2), true);
/// assert_eq!(limbs, &[0, 0, 0]);
/// ```
pub fn limbs_slice_neg_xor_limb_in_place(xs: &mut [Limb], y: Limb) -> bool {
    if y == 0 {
        return false;
    }
    let (head, tail) = xs.split_at_mut(1);
    let head = &mut head[0];
    if *head != 0 {
        *head = head.wrapping_neg() ^ y;
        if *head == 0 {
            limbs_slice_add_limb_in_place(tail, 1)
        } else {
            head.wrapping_neg_assign();
            false
        }
    } else {
        *head = y.wrapping_neg();
        limbs_sub_limb_in_place(tail, 1);
        false
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a `Limb` to the input slice.
/// `limbs` cannot be empty or only contain zeros. If a carry occurs, extends the `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_vec_neg_xor_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_neg_xor_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[880, 456]);
///
/// let mut limbs = vec![0xffff_fffe, 0xffff_ffff, 0xffff_ffff];
/// limbs_vec_neg_xor_limb_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[0, 0, 0, 1]);
/// ```
pub fn limbs_vec_neg_xor_limb_in_place(xs: &mut Vec<Limb>, y: Limb) {
    if limbs_slice_neg_xor_limb_in_place(xs, y) {
        xs.push(1);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, returns the
/// limbs of the bitwise xor of the `Integer` and a negative number whose lowest limb is given by
/// `limb` and whose other limbs are full of `true` bits. `limbs` may not be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_pos_xor_limb_neg;
///
/// assert_eq!(limbs_pos_xor_limb_neg(&[0, 2], 3), &[4294967293, 2]);
/// assert_eq!(limbs_pos_xor_limb_neg(&[1, 2, 3], 4), &[4294967291, 2, 3]);
/// assert_eq!(limbs_pos_xor_limb_neg(&[2, 0xffff_ffff], 2), &[0, 0, 1]);
/// ```
pub fn limbs_pos_xor_limb_neg(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let lowest_limb = xs[0] ^ y;
    let mut out;
    if lowest_limb == 0 {
        out = limbs_add_limb(&xs[1..], 1);
        out.insert(0, 0);
    } else {
        out = xs.to_vec();
        out[0] = lowest_limb.wrapping_neg();
    }
    out
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, writes the
/// limbs of the bitwise xor of the `Integer` and a negative number whose lowest limb is given by
/// `limb` and whose other limbs are full of `true` bits to an output slice. `in_limbs` may not be
/// empty or only contain zeros. The output slice must be at least as long as the input slice.
/// Returns whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Panics
/// Panics if `in_limbs` is empty or if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_pos_xor_limb_neg_to_out;
///
/// let mut result = vec![10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut result, &[0, 2], 3), false);
/// assert_eq!(result, &[4294967293, 2]);
///
/// let mut result = vec![10, 10, 10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut result, &[1, 2, 3], 4), false);
/// assert_eq!(result, &[4294967291, 2, 3, 10]);
///
/// let mut result = vec![10, 10, 10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut result, &[2, 0xffff_ffff], 2), true);
/// assert_eq!(result, &[0, 0, 10, 10]);
/// ```
pub fn limbs_pos_xor_limb_neg_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) -> bool {
    let len = xs.len();
    assert!(out.len() >= len);
    let lowest_limb = xs[0] ^ y;
    if lowest_limb == 0 {
        out[0] = 0;
        limbs_add_limb_to_out(&mut out[1..len], &xs[1..], 1)
    } else {
        out[0] = lowest_limb.wrapping_neg();
        out[1..len].copy_from_slice(&xs[1..]);
        false
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, takes the
/// bitwise xor of the `Integer` and a negative number whose lowest limb is given by `limb` and
/// whose other limbs are full of `true` bits, in place. `limbs` may not be empty. Returns whether
/// there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_slice_pos_xor_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut limbs, 3), false);
/// assert_eq!(limbs, &[4294967293, 2]);
///
/// let mut limbs = vec![1, 2, 3];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut limbs, 4), false);
/// assert_eq!(limbs, &[4294967291, 2, 3]);
///
/// let mut limbs = vec![2, 0xffff_ffff];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut limbs, 2), true);
/// assert_eq!(limbs, &[0, 0]);
/// ```
pub fn limbs_slice_pos_xor_limb_neg_in_place(xs: &mut [Limb], y: Limb) -> bool {
    let (head, tail) = xs.split_at_mut(1);
    let head = &mut head[0];
    *head ^= y;
    if *head == 0 {
        limbs_slice_add_limb_in_place(tail, 1)
    } else {
        *head = head.wrapping_neg();
        false
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of an `Integer`, takes the
/// bitwise xor of the `Integer` and a negative number whose lowest limb is given by `limb` and
/// whose other limbs are full of `true` bits, in place. `limbs` may not be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_vec_pos_xor_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[4294967293, 2]);
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut limbs, 4);
/// assert_eq!(limbs, &[4294967291, 2, 3]);
///
/// let mut limbs = vec![2, 0xffff_ffff];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[0, 0, 1]);
/// ```
pub fn limbs_vec_pos_xor_limb_neg_in_place(xs: &mut Vec<Limb>, y: Limb) {
    if limbs_slice_pos_xor_limb_neg_in_place(xs, y) {
        xs.push(1);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise xor of the `Integer` and a negative number whose
/// lowest limb is given by `limb` and whose other limbs are full of `true` bits. `limbs` may not be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb_neg;
///
/// assert_eq!(limbs_neg_xor_limb_neg(&[0, 2], 3), &[3, 1]);
/// assert_eq!(limbs_neg_xor_limb_neg(&[1, 2, 3], 4), &[4294967291, 2, 3]);
/// ```
pub fn limbs_neg_xor_limb_neg(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut result_limbs;
    if xs[0] == 0 {
        let (result, carry) = limbs_sub_limb(xs, 1);
        result_limbs = result;
        assert!(!carry);
        result_limbs[0] = y;
    } else {
        result_limbs = xs.to_vec();
        result_limbs[0] = xs[0].wrapping_neg() ^ y;
    }
    result_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a negative number whose
/// lowest limb is given by `limb` and whose other limbs are full of `true` bits to an output slice.
/// `in_limbs` may not be empty or only contain zeros. The output slice must be at least as long as
/// the input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `in_limbs.len()`
///
/// # Panics
/// Panics if `in_limbs` is empty or only contains zeros, or if `out` is shorter than
/// `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb_neg_to_out;
///
/// let mut result = vec![10, 10];
/// limbs_neg_xor_limb_neg_to_out(&mut result, &[0, 2], 3);
/// assert_eq!(result, &[3, 1]);
///
/// let mut result = vec![10, 10, 10, 10];
/// limbs_neg_xor_limb_neg_to_out(&mut result, &[1, 2, 3], 4);
/// assert_eq!(result, &[4294967291, 2, 3, 10]);
/// ```
pub fn limbs_neg_xor_limb_neg_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    let len = xs.len();
    assert!(out.len() >= len);
    if xs[0] == 0 {
        out[0] = y;
        assert!(!limbs_sub_limb_to_out(&mut out[1..len], &xs[1..], 1));
    } else {
        out[0] = xs[0].wrapping_neg() ^ y;
        out[1..len].copy_from_slice(&xs[1..]);
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, takes the bitwise xor of the `Integer` and a negative number whose lowest limb is
/// given by `limb` and whose other limbs are full of `true` bits, in place. `limbs` may not be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb_neg_in_place;
///
/// let mut limbs = vec![0, 2];
/// limbs_neg_xor_limb_neg_in_place(&mut limbs, 3);
/// assert_eq!(limbs, &[3, 1]);
///
/// let mut limbs = vec![1, 2, 3];
/// limbs_neg_xor_limb_neg_in_place(&mut limbs, 4);
/// assert_eq!(limbs, &[4294967291, 2, 3]);
/// ```
pub fn limbs_neg_xor_limb_neg_in_place(xs: &mut [Limb], y: Limb) {
    if xs[0] == 0 {
        assert!(!limbs_sub_limb_in_place(&mut xs[1..], 1));
        xs[0] = y;
    } else {
        xs[0] = xs[0].wrapping_neg() ^ y;
    }
}

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
/// use malachite_nz::integer::logic::xor::limbs_xor_pos_neg;
///
/// assert_eq!(limbs_xor_pos_neg(&[1, 2], &[100, 200]), &[99, 202]);
/// assert_eq!(limbs_xor_pos_neg(&[1, 2, 5], &[100, 200]), &[99, 202, 5]);
/// ```
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where res is returned, the first input is positive,
/// and the second is negative.
pub fn limbs_xor_pos_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
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
    let limb = match x_i.cmp(&y_i) {
        Ordering::Equal => {
            limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_limb_seen)
        }
        Ordering::Less => {
            boundary_limb_seen = true;
            result_limbs.push(xs[x_i].wrapping_neg());
            result_limbs.extend(xs[x_i + 1..y_i].iter().map(|x| !x));
            xs[y_i] ^ (ys[y_i] - 1)
        }
        Ordering::Greater => {
            boundary_limb_seen = true;
            result_limbs.extend_from_slice(&ys[y_i..x_i]);
            xs[x_i] ^ ys[x_i]
        }
    };
    result_limbs.push(limb);
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
/// use malachite_nz::integer::logic::xor::limbs_xor_pos_neg_to_out;
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
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where the first input is positive and the second is
/// negative.
pub fn limbs_xor_pos_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    assert!(out.len() >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        slice_set_zero(&mut out[..x_i]);
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
    slice_set_zero(&mut out[..min_i]);
    let mut boundary_limb_seen = false;
    match x_i.cmp(&y_i) {
        Ordering::Equal => {
            out[x_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_limb_seen);
        }
        Ordering::Less => {
            boundary_limb_seen = true;
            out[x_i] = xs[x_i].wrapping_neg();
            for (out, &x) in out[x_i + 1..y_i].iter_mut().zip(xs[x_i + 1..y_i].iter()) {
                *out = !x;
            }
            out[y_i] = xs[y_i] ^ (ys[y_i] - 1);
        }
        Ordering::Greater => {
            boundary_limb_seen = true;
            out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
            out[x_i] = xs[x_i] ^ ys[x_i];
        }
    }
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
    match x_i.cmp(&y_i) {
        Ordering::Equal => {
            xs[x_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_limb_seen);
        }
        Ordering::Less => {
            boundary_limb_seen = true;
            xs[x_i].wrapping_neg_assign();
            limbs_not_in_place(&mut xs[x_i + 1..y_i]);
            xs[y_i] ^= ys[y_i] - 1;
        }
        Ordering::Greater => {
            boundary_limb_seen = true;
            xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
            xs[x_i] ^= ys[x_i];
        }
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
/// use malachite_nz::integer::logic::xor::limbs_xor_pos_neg_in_place_left;
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
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where res == op1 and the first input is positive and
/// the second is negative.
pub fn limbs_xor_pos_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
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
    match xs_len.cmp(&ys_len) {
        Ordering::Less => {
            if boundary_limb_seen {
                xs.extend_from_slice(&ys[xs_len..]);
            } else {
                for &y in ys[xs_len..].iter() {
                    xs.push(limbs_xor_pos_neg_helper(!y, &mut boundary_limb_seen));
                }
            }
        }
        Ordering::Greater => {
            if !boundary_limb_seen {
                for x in xs[ys_len..].iter_mut() {
                    *x = limbs_xor_pos_neg_helper(!*x, &mut boundary_limb_seen);
                }
            }
        }
        _ => {}
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
    match x_i.cmp(&y_i) {
        Ordering::Equal => {
            ys[y_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_limb_seen);
        }
        Ordering::Less => {
            boundary_limb_seen = true;
            ys[x_i] = xs[x_i].wrapping_neg();
            for (y, &x) in ys[x_i + 1..].iter_mut().zip(xs[x_i + 1..y_i].iter()) {
                *y = !x;
            }
            ys[y_i] -= 1;
            ys[y_i] ^= xs[y_i];
        }
        Ordering::Greater => {
            boundary_limb_seen = true;
            ys[x_i] ^= xs[x_i];
        }
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
/// use malachite_nz::integer::logic::xor::limbs_xor_pos_neg_in_place_right;
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
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where res == op2 and the first input is positive and
/// the second is negative.
pub fn limbs_xor_pos_neg_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
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
/// use malachite_nz::integer::logic::xor::limbs_xor_pos_neg_in_place_either;
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
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where the first input is positive, the second is
/// negative, and the result is written to the longer input slice.
pub fn limbs_xor_pos_neg_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
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
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where res is returned and both inputs are negative.
pub fn limbs_xor_neg_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
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
    match xs_len.cmp(&ys_len) {
        Ordering::Less => result_limbs.extend_from_slice(&ys[xs_len..]),
        Ordering::Greater => result_limbs.extend_from_slice(&xs[ys_len..]),
        _ => {}
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
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where both inputs are negative.
pub fn limbs_xor_neg_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    assert!(out.len() >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
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
    slice_set_zero(&mut out[..min_i]);
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
    match xs_len.cmp(&ys_len) {
        Ordering::Less => out[xs_len..ys_len].copy_from_slice(&ys[xs_len..]),
        Ordering::Greater => out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]),
        _ => {}
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
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where res == op1 and both inputs are negative.
pub fn limbs_xor_neg_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
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
///
/// This is mpz_xor from mpz/xor.c, GMP 6.1.2, where both inputs are negative and the result is
/// written to the longer input slice.
pub fn limbs_xor_neg_neg_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) ^ Integer::from(-456)).to_string(), "445");
/// assert_eq!((-Integer::trillion() ^ -(Integer::trillion() + Integer::ONE)).to_string(), "8191");
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!((Integer::from(-123) ^ &Integer::from(-456)).to_string(), "445");
/// assert_eq!((-Integer::trillion() ^ &-(Integer::trillion() + Integer::ONE)).to_string(), "8191");
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(-123) ^ Integer::from(-456)).to_string(), "445");
/// assert_eq!((&-Integer::trillion() ^ -(Integer::trillion() + Integer::ONE)).to_string(), "8191");
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::basic::traits::One;
/// use malachite_nz::integer::Integer;
/// use std::str::FromStr;
///
/// assert_eq!((&Integer::from(-123) ^ &Integer::from(-456)).to_string(), "445");
/// assert_eq!(
///     (&-Integer::trillion() ^ &-(Integer::trillion() + Integer::ONE)).to_string(),
///     "8191"
/// );
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
/// use malachite_base::num::basic::traits::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// let mut x = Integer::from(0xffff_ffffu32);
/// x ^= Integer::from(0x0000_000f);
/// x ^= Integer::from(0x0000_0f00);
/// x ^= Integer::from(0x000f_0000);
/// x ^= Integer::from(0x0f00_0000);
/// assert_eq!(x, 0xf0f0_f0f0u32);
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
/// use malachite_base::num::basic::traits::NegativeOne;
/// use malachite_nz::integer::Integer;
///
/// let mut x = Integer::from(0xffff_ffffu32);
/// x ^= &Integer::from(0x0000_000f);
/// x ^= &Integer::from(0x0000_0f00);
/// x ^= &Integer::from(0x000f_0000);
/// x ^= &Integer::from(0x0f00_0000);
/// assert_eq!(x, 0xf0f0_f0f0u32);
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
    fn xor_assign_neg_limb_pos(&mut self, other: Limb) {
        if other == 0 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                let result = small.wrapping_neg() ^ other;
                if result == 0 {
                    None
                } else {
                    Some(result.wrapping_neg())
                }
            },
            { limbs_vec_neg_xor_limb_in_place(limbs, other) }
        );
        self.trim();
    }

    fn xor_neg_limb_pos(&self, other: Limb) -> Natural {
        match *self {
            Natural(Small(ref small)) => {
                let result = small.wrapping_neg() ^ other;
                Natural(if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                })
            }
            Natural(Large(ref limbs)) => {
                Natural::from_owned_limbs_asc(limbs_neg_xor_limb(limbs, other))
            }
        }
    }

    fn xor_assign_pos_limb_neg(&mut self, other: Limb) {
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                let result = *small ^ other;
                if result == 0 {
                    None
                } else {
                    Some(result.wrapping_neg())
                }
            },
            { limbs_vec_pos_xor_limb_neg_in_place(limbs, other) }
        );
    }

    fn xor_pos_limb_neg(&self, other: Limb) -> Natural {
        Natural(match *self {
            Natural(Small(small)) => {
                let result = small ^ other;
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                }
            }
            Natural(Large(ref limbs)) => Large(limbs_pos_xor_limb_neg(limbs, other)),
        })
    }

    fn xor_assign_neg_limb_neg(&mut self, other: Limb) {
        match *self {
            Natural(Small(ref mut small)) => *small = small.wrapping_neg() ^ other,
            Natural(Large(ref mut limbs)) => limbs_neg_xor_limb_neg_in_place(limbs, other),
        }
        self.trim();
    }

    fn xor_neg_limb_neg(&self, other: Limb) -> Natural {
        match *self {
            Natural(Small(small)) => Natural(Small(small.wrapping_neg() ^ other)),
            Natural(Large(ref limbs)) => {
                Natural::from_owned_limbs_asc(limbs_neg_xor_limb_neg(limbs, other))
            }
        }
    }

    fn xor_assign_pos_neg(&mut self, mut other: Natural) {
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

    fn xor_assign_pos_neg_ref(&mut self, other: &Natural) {
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

    fn xor_assign_neg_pos(&mut self, mut other: Natural) {
        other.xor_assign_pos_neg_ref(&*self);
        *self = other;
    }

    fn xor_assign_neg_pos_ref(&mut self, other: &Natural) {
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

    fn xor_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.xor_pos_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), _) => other.xor_neg_limb_pos(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_xor_pos_neg(xs, ys))
            }
        }
    }

    fn xor_assign_neg_neg(&mut self, other: Natural) {
        let new_self_value = if let Natural(Small(y)) = other {
            self.xor_assign_neg_limb_neg(y.wrapping_neg());
            None
        } else if let Natural(Small(ref mut x)) = *self {
            let mut new_self_value = other.clone();
            new_self_value.xor_assign_neg_limb_neg(x.wrapping_neg());
            Some(new_self_value)
        } else if let Natural(Large(mut ys)) = other {
            if let Natural(Large(ref mut xs)) = *self {
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
        let new_self_value = if let Natural(Small(y)) = *other {
            self.xor_assign_neg_limb_neg(y.wrapping_neg());
            None
        } else if let Natural(Small(ref mut x)) = *self {
            let mut new_self_value = other.clone();
            new_self_value.xor_assign_neg_limb_neg(x.wrapping_neg());
            Some(new_self_value)
        } else if let Natural(Large(ref ys)) = *other {
            if let Natural(Large(ref mut xs)) = *self {
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
            (_, &Natural(Small(y))) => self.xor_neg_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), _) => other.xor_neg_limb_neg(x.wrapping_neg()),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_xor_neg_neg(xs, ys))
            }
        }
    }
}
