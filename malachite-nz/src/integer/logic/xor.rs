use std::ops::{BitXor, BitXorAssign};

use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_set_zero};
use malachite_base::num::arithmetic::traits::WrappingNegAssign;

use integer::Integer;
use natural::arithmetic::add_limb::{
    limbs_add_limb, limbs_add_limb_to_out, limbs_slice_add_limb_in_place,
};
use natural::arithmetic::sub::{
    limbs_sub, limbs_sub_in_place_left, limbs_sub_to_out, limbs_vec_sub_in_place_right,
};
use natural::arithmetic::sub_limb::{
    limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
};
use natural::logic::not::limbs_not_in_place;
use natural::Natural::{self, Large, Small};
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
pub fn limbs_neg_xor_limb(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
    if limb == 0 {
        return limbs.to_vec();
    }
    let head = limbs[0];
    let tail = &limbs[1..];
    let mut result_limbs = Vec::with_capacity(limbs.len());
    if head != 0 {
        let head = head.wrapping_neg() ^ limb;
        if head == 0 {
            result_limbs.push(0);
            result_limbs.extend_from_slice(&limbs_add_limb(tail, 1));
        } else {
            result_limbs.push(head.wrapping_neg());
            result_limbs.extend_from_slice(tail);
        }
    } else {
        result_limbs.push(limb.wrapping_neg());
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
pub fn limbs_neg_xor_limb_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) -> bool {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    if limb == 0 {
        out[..len].copy_from_slice(in_limbs);
        return false;
    }
    let head = in_limbs[0];
    let tail = &in_limbs[1..];
    if head != 0 {
        let head = head.wrapping_neg() ^ limb;
        if head == 0 {
            out[0] = 0;
            limbs_add_limb_to_out(&mut out[1..len], tail, 1)
        } else {
            out[0] = head.wrapping_neg();
            out[1..len].copy_from_slice(tail);
            false
        }
    } else {
        out[0] = limb.wrapping_neg();
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
pub fn limbs_slice_neg_xor_limb_in_place(limbs: &mut [Limb], limb: Limb) -> bool {
    if limb == 0 {
        return false;
    }
    let (head, tail) = limbs.split_at_mut(1);
    let head = &mut head[0];
    if *head != 0 {
        *head = head.wrapping_neg() ^ limb;
        if *head == 0 {
            limbs_slice_add_limb_in_place(tail, 1)
        } else {
            head.wrapping_neg_assign();
            false
        }
    } else {
        *head = limb.wrapping_neg();
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
pub fn limbs_vec_neg_xor_limb_in_place(limbs: &mut Vec<Limb>, limb: Limb) {
    if limbs_slice_neg_xor_limb_in_place(limbs, limb) {
        limbs.push(1);
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
pub fn limbs_pos_xor_limb_neg(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
    let lowest_limb = limbs[0] ^ limb;
    let mut result_limbs;
    if lowest_limb == 0 {
        result_limbs = limbs_add_limb(&limbs[1..], 1);
        result_limbs.insert(0, 0);
    } else {
        result_limbs = limbs.to_vec();
        result_limbs[0] = lowest_limb.wrapping_neg();
    }
    result_limbs
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
pub fn limbs_pos_xor_limb_neg_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) -> bool {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    let lowest_limb = in_limbs[0] ^ limb;
    if lowest_limb == 0 {
        out[0] = 0;
        limbs_add_limb_to_out(&mut out[1..len], &in_limbs[1..], 1)
    } else {
        out[0] = lowest_limb.wrapping_neg();
        out[1..len].copy_from_slice(&in_limbs[1..]);
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
pub fn limbs_slice_pos_xor_limb_neg_in_place(limbs: &mut [Limb], limb: Limb) -> bool {
    let (head, tail) = limbs.split_at_mut(1);
    let head = &mut head[0];
    *head ^= limb;
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
pub fn limbs_vec_pos_xor_limb_neg_in_place(limbs: &mut Vec<Limb>, limb: Limb) {
    if limbs_slice_pos_xor_limb_neg_in_place(limbs, limb) {
        limbs.push(1);
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
pub fn limbs_neg_xor_limb_neg(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut result_limbs;
    if limbs[0] == 0 {
        let (result, carry) = limbs_sub_limb(limbs, 1);
        result_limbs = result;
        assert!(!carry);
        result_limbs[0] = limb;
    } else {
        result_limbs = limbs.to_vec();
        result_limbs[0] = limbs[0].wrapping_neg() ^ limb;
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
pub fn limbs_neg_xor_limb_neg_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    if in_limbs[0] == 0 {
        out[0] = limb;
        assert!(!limbs_sub_limb_to_out(&mut out[1..len], &in_limbs[1..], 1));
    } else {
        out[0] = in_limbs[0].wrapping_neg() ^ limb;
        out[1..len].copy_from_slice(&in_limbs[1..]);
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
pub fn limbs_neg_xor_limb_neg_in_place(limbs: &mut [Limb], limb: Limb) {
    if limbs[0] == 0 {
        assert!(!limbs_sub_limb_in_place(&mut limbs[1..], 1));
        limbs[0] = limb;
    } else {
        limbs[0] = limbs[0].wrapping_neg() ^ limb;
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
/// This is mpz_xor from mpz/xor.c where res is returned and both inputs are negative.
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
///
/// This is mpz_xor from mpz/xor.c where both inputs are negative.
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
///
/// This is mpz_xor from mpz/xor.c where res == op1 and both inputs are negative.
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
///
/// This is mpz_ior from mpz/ior.c where both inputs are negative and the result is written to the
/// longer input slice.
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
/// use malachite_base::num::basic::traits::NegativeOne;
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
/// use malachite_base::num::basic::traits::NegativeOne;
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
    pub(crate) fn xor_assign_neg_limb_pos(&mut self, other: Limb) {
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

    pub(crate) fn xor_neg_limb_pos(&self, other: Limb) -> Natural {
        match *self {
            Small(ref small) => {
                let result = small.wrapping_neg() ^ other;
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                }
            }
            Large(ref limbs) => {
                let mut result = Large(limbs_neg_xor_limb(limbs, other));
                result.trim();
                result
            }
        }
    }

    pub(crate) fn xor_assign_pos_limb_neg(&mut self, other: Limb) {
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

    pub(crate) fn xor_pos_limb_neg(&self, other: Limb) -> Natural {
        match *self {
            Small(small) => {
                let result = small ^ other;
                if result == 0 {
                    Large(vec![0, 1])
                } else {
                    Small(result.wrapping_neg())
                }
            }
            Large(ref limbs) => Large(limbs_pos_xor_limb_neg(limbs, other)),
        }
    }

    fn xor_assign_neg_limb_neg(&mut self, other: Limb) {
        match *self {
            Small(ref mut small) => *small = small.wrapping_neg() ^ other,
            Large(ref mut limbs) => limbs_neg_xor_limb_neg_in_place(limbs, other),
        }
        self.trim();
    }

    fn xor_neg_limb_neg(&self, other: Limb) -> Natural {
        match *self {
            Small(small) => Small(small.wrapping_neg() ^ other),
            Large(ref limbs) => {
                let mut result = Large(limbs_neg_xor_limb_neg(limbs, other));
                result.trim();
                result
            }
        }
    }

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
