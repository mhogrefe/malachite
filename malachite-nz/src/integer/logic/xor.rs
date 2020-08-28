use integer::Integer;
use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero};
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
use std::cmp::{max, Ordering};
use std::iter::repeat;
use std::ops::{BitXor, BitXorAssign};

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise xor of the `Integer` and a `Limb`. `xs` cannot be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb;
///
/// assert_eq!(limbs_neg_xor_limb(&[123, 456], 789), &[880, 456]);
/// assert_eq!(limbs_neg_xor_limb(&[u32::MAX - 1, u32::MAX, u32::MAX], 2), &[0, 0, 0, 1]);
/// ```
pub fn limbs_neg_xor_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    if y == 0 {
        return xs.to_vec();
    }
    let head = xs[0];
    let tail = &xs[1..];
    let mut out = Vec::with_capacity(xs.len());
    if head != 0 {
        let head = head.wrapping_neg() ^ y;
        if head == 0 {
            out.push(0);
            out.extend_from_slice(&limbs_add_limb(tail, 1));
        } else {
            out.push(head.wrapping_neg());
            out.extend_from_slice(tail);
        }
    } else {
        out.push(y.wrapping_neg());
        out.extend_from_slice(&limbs_sub_limb(tail, 1).0);
    }
    out
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise and of the `Integer`, writes the limbs of the bitwise
/// xor of the `Integer` and a `Limb` to an output slice. The output slice must be at least as long
/// as the input slice. `xs` cannot be empty or only contain zeros. Returns whether a carry occurs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb_to_out;
///
/// let mut xs = vec![0, 0, 0, 0];
/// assert_eq!(limbs_neg_xor_limb_to_out(&mut xs, &[123, 456], 789), false);
/// assert_eq!(xs, &[880, 456, 0, 0]);
///
/// let mut xs = vec![10, 10, 10, 10];
/// assert_eq!(limbs_neg_xor_limb_to_out(&mut xs, &[u32::MAX - 1, u32::MAX, u32::MAX], 2),
///     true);
/// assert_eq!(xs, &[0, 0, 0, 10]);
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
/// `xs` cannot be empty or only contain zeros. Returns whether a carry occurs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_slice_neg_xor_limb_in_place;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_neg_xor_limb_in_place(&mut xs, 789), false);
/// assert_eq!(xs, &[880, 456]);
///
/// let mut xs = vec![u32::MAX - 1, u32::MAX, u32::MAX];
/// assert_eq!(limbs_slice_neg_xor_limb_in_place(&mut xs, 2), true);
/// assert_eq!(xs, &[0, 0, 0]);
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
/// `xs` cannot be empty or only contain zeros. If a carry occurs, extends the `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_vec_neg_xor_limb_in_place;
///
/// let mut xs = vec![123, 456];
/// limbs_vec_neg_xor_limb_in_place(&mut xs, 789);
/// assert_eq!(xs, &[880, 456]);
///
/// let mut xs = vec![u32::MAX - 1, u32::MAX, u32::MAX];
/// limbs_vec_neg_xor_limb_in_place(&mut xs, 2);
/// assert_eq!(xs, &[0, 0, 0, 1]);
/// ```
pub fn limbs_vec_neg_xor_limb_in_place(xs: &mut Vec<Limb>, y: Limb) {
    if limbs_slice_neg_xor_limb_in_place(xs, y) {
        xs.push(1);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, returns the
/// limbs of the bitwise xor of the `Integer` and a negative number whose lowest limb is given by
/// `y` and whose other limbs are full of `true` bits. `xs` may not be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_pos_xor_limb_neg;
///
/// assert_eq!(limbs_pos_xor_limb_neg(&[0, 2], 3), &[4294967293, 2]);
/// assert_eq!(limbs_pos_xor_limb_neg(&[1, 2, 3], 4), &[4294967291, 2, 3]);
/// assert_eq!(limbs_pos_xor_limb_neg(&[2, u32::MAX], 2), &[0, 0, 1]);
/// ```
pub fn limbs_pos_xor_limb_neg(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let (head, tail) = xs.split_first().unwrap();
    let lo = head ^ y;
    let mut out;
    if lo == 0 {
        out = limbs_add_limb(tail, 1);
        out.insert(0, 0);
    } else {
        out = xs.to_vec();
        out[0] = lo.wrapping_neg();
    }
    out
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, writes the
/// limbs of the bitwise xor of the `Integer` and a negative number whose lowest limb is given by
/// `y` and whose other limbs are full of `true` bits to an output slice. `xs` may not be empty or
/// only contain zeros. The output slice must be at least as long as the input slice. Returns
/// whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty or if `out` is shorter than `xs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_pos_xor_limb_neg_to_out;
///
/// let mut out = vec![10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut out, &[0, 2], 3), false);
/// assert_eq!(out, &[4294967293, 2]);
///
/// let mut out = vec![10, 10, 10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut out, &[1, 2, 3], 4), false);
/// assert_eq!(out, &[4294967291, 2, 3, 10]);
///
/// let mut out = vec![10, 10, 10, 10];
/// assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut out, &[2, u32::MAX], 2), true);
/// assert_eq!(out, &[0, 0, 10, 10]);
/// ```
pub fn limbs_pos_xor_limb_neg_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) -> bool {
    let (head, tail) = xs.split_first().unwrap();
    let (out_head, out_tail) = out[..xs.len()].split_first_mut().unwrap();
    let lo = head ^ y;
    if lo == 0 {
        *out_head = 0;
        limbs_add_limb_to_out(out_tail, tail, 1)
    } else {
        *out_head = lo.wrapping_neg();
        out_tail.copy_from_slice(tail);
        false
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, takes the
/// bitwise xor of the `Integer` and a negative number whose lowest limb is given by `y` and whose
/// other limbs are full of `true` bits, in place. `xs` may not be empty. Returns whether there is a
/// carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_slice_pos_xor_limb_neg_in_place;
///
/// let mut out = vec![0, 2];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut out, 3), false);
/// assert_eq!(out, &[4294967293, 2]);
///
/// let mut out = vec![1, 2, 3];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut out, 4), false);
/// assert_eq!(out, &[4294967291, 2, 3]);
///
/// let mut out = vec![2, u32::MAX];
/// assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut out, 2), true);
/// assert_eq!(out, &[0, 0]);
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
/// bitwise xor of the `Integer` and a negative number whose lowest limb is given by `y` and whose
/// other limbs are full of `true` bits, in place. `xs` may not be empty.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_vec_pos_xor_limb_neg_in_place;
///
/// let mut xs = vec![0, 2];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut xs, 3);
/// assert_eq!(xs, &[4294967293, 2]);
///
/// let mut xs = vec![1, 2, 3];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut xs, 4);
/// assert_eq!(xs, &[4294967291, 2, 3]);
///
/// let mut xs = vec![2, u32::MAX];
/// limbs_vec_pos_xor_limb_neg_in_place(&mut xs, 2);
/// assert_eq!(xs, &[0, 0, 1]);
/// ```
pub fn limbs_vec_pos_xor_limb_neg_in_place(xs: &mut Vec<Limb>, y: Limb) {
    if limbs_slice_pos_xor_limb_neg_in_place(xs, y) {
        xs.push(1);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise xor of the `Integer` and a negative number whose
/// lowest limb is given by `y` and whose other limbs are full of `true` bits. `xs` may not be empty
/// or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb_neg;
///
/// assert_eq!(limbs_neg_xor_limb_neg(&[0, 2], 3), &[3, 1]);
/// assert_eq!(limbs_neg_xor_limb_neg(&[1, 2, 3], 4), &[4294967291, 2, 3]);
/// ```
pub fn limbs_neg_xor_limb_neg(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut out;
    if xs[0] == 0 {
        let (result, carry) = limbs_sub_limb(xs, 1);
        out = result;
        assert!(!carry);
        out[0] = y;
    } else {
        out = xs.to_vec();
        out[0] = xs[0].wrapping_neg() ^ y;
    }
    out
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise xor of the `Integer` and a negative number whose
/// lowest limb is given by `y` and whose other limbs are full of `true` bits to an output slice.
/// `xs` may not be empty or only contain zeros. The output slice must be at least as long as the
/// input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty or only contains zeros, or if `out` is shorter than `xs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb_neg_to_out;
///
/// let mut out = vec![10, 10];
/// limbs_neg_xor_limb_neg_to_out(&mut out, &[0, 2], 3);
/// assert_eq!(out, &[3, 1]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_neg_xor_limb_neg_to_out(&mut out, &[1, 2, 3], 4);
/// assert_eq!(out, &[4294967291, 2, 3, 10]);
/// ```
pub fn limbs_neg_xor_limb_neg_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    let (head, tail) = xs.split_first().unwrap();
    let (out_head, out_tail) = out[..xs.len()].split_first_mut().unwrap();
    if *head == 0 {
        *out_head = y;
        assert!(!limbs_sub_limb_to_out(out_tail, tail, 1));
    } else {
        *out_head = xs[0].wrapping_neg() ^ y;
        out_tail.copy_from_slice(tail);
    }
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, takes the bitwise xor of the `Integer` and a negative number whose lowest limb is
/// given by `y` and whose other limbs are full of `true` bits, in place. `xs` may not be empty or
/// only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::xor::limbs_neg_xor_limb_neg_in_place;
///
/// let mut xs = vec![0, 2];
/// limbs_neg_xor_limb_neg_in_place(&mut xs, 3);
/// assert_eq!(xs, &[3, 1]);
///
/// let mut xs = vec![1, 2, 3];
/// limbs_neg_xor_limb_neg_in_place(&mut xs, 4);
/// assert_eq!(xs, &[4294967291, 2, 3]);
/// ```
pub fn limbs_neg_xor_limb_neg_in_place(xs: &mut [Limb], y: Limb) {
    let (head, tail) = xs.split_first_mut().unwrap();
    if *head == 0 {
        assert!(!limbs_sub_limb_in_place(tail, 1));
        *head = y;
    } else {
        head.wrapping_neg_assign();
        *head ^= y;
    }
}

fn limbs_xor_pos_neg_helper(x: Limb, boundary_seen: &mut bool) -> Limb {
    if *boundary_seen {
        !x
    } else if x == 0 {
        0
    } else {
        *boundary_seen = true;
        x.wrapping_neg()
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
        let mut out = vec![0; x_i];
        out.push(xs[x_i].wrapping_neg());
        out.extend(xs[x_i + 1..].iter().map(|x| !x));
        out.extend(repeat(Limb::MAX).take(y_i - xs_len));
        out.push(ys[y_i] - 1);
        out.extend_from_slice(&ys[y_i + 1..]);
        return out;
    } else if x_i >= ys_len {
        let mut out = ys.to_vec();
        out.extend_from_slice(&xs[ys_len..]);
        return out;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    let mut out = vec![0; min_i];
    let mut boundary_seen = false;
    let x = match x_i.cmp(&y_i) {
        Ordering::Equal => {
            limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_seen)
        }
        Ordering::Less => {
            boundary_seen = true;
            out.push(xs[x_i].wrapping_neg());
            out.extend(xs[x_i + 1..y_i].iter().map(|x| !x));
            xs[y_i] ^ (ys[y_i] - 1)
        }
        Ordering::Greater => {
            boundary_seen = true;
            out.extend_from_slice(&ys[y_i..x_i]);
            xs[x_i] ^ ys[x_i]
        }
    };
    out.push(x);
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter());
    if boundary_seen {
        out.extend(xys.map(|(x, y)| x ^ y));
    } else {
        for (&x, &y) in xys {
            out.push(limbs_xor_pos_neg_helper(x ^ !y, &mut boundary_seen));
        }
    }
    if xs_len != ys_len {
        let zs = if xs_len > ys_len {
            &xs[ys_len..]
        } else {
            &ys[xs_len..]
        };
        if boundary_seen {
            out.extend_from_slice(zs);
        } else {
            for &z in zs.iter() {
                out.push(limbs_xor_pos_neg_helper(!z, &mut boundary_seen));
            }
        }
    }
    out
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
    let mut boundary_seen = false;
    match x_i.cmp(&y_i) {
        Ordering::Equal => {
            out[x_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_seen);
        }
        Ordering::Less => {
            boundary_seen = true;
            out[x_i] = xs[x_i].wrapping_neg();
            for (out, &x) in out[x_i + 1..y_i].iter_mut().zip(xs[x_i + 1..y_i].iter()) {
                *out = !x;
            }
            out[y_i] = xs[y_i] ^ (ys[y_i] - 1);
        }
        Ordering::Greater => {
            boundary_seen = true;
            out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
            out[x_i] = xs[x_i] ^ ys[x_i];
        }
    }
    let xys = out[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()));
    if boundary_seen {
        for (out, (&x, &y)) in xys {
            *out = x ^ y;
        }
    } else {
        for (out, (&x, &y)) in xys {
            *out = limbs_xor_pos_neg_helper(x ^ !y, &mut boundary_seen);
        }
    }
    if xs_len != ys_len {
        let (min_len, max_len, zs) = if xs_len > ys_len {
            (ys_len, xs_len, &xs[ys_len..])
        } else {
            (xs_len, ys_len, &ys[xs_len..])
        };
        if boundary_seen {
            out[min_len..max_len].copy_from_slice(zs);
        } else {
            for (out, &z) in out[min_len..].iter_mut().zip(zs.iter()) {
                *out = limbs_xor_pos_neg_helper(!z, &mut boundary_seen);
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
    let mut boundary_seen = false;
    match x_i.cmp(&y_i) {
        Ordering::Equal => {
            xs[x_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_seen);
        }
        Ordering::Less => {
            boundary_seen = true;
            xs[x_i].wrapping_neg_assign();
            limbs_not_in_place(&mut xs[x_i + 1..y_i]);
            xs[y_i] ^= ys[y_i] - 1;
        }
        Ordering::Greater => {
            boundary_seen = true;
            xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
            xs[x_i] ^= ys[x_i];
        }
    }
    let xys = xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter());
    if boundary_seen {
        for (x, &y) in xys {
            *x ^= y;
        }
    } else {
        for (x, &y) in xys {
            *x = limbs_xor_pos_neg_helper(*x ^ !y, &mut boundary_seen);
        }
    }
    boundary_seen
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
    let mut boundary_seen = limbs_xor_pos_neg_in_place_left_helper(xs, ys, x_i, y_i);
    match xs_len.cmp(&ys_len) {
        Ordering::Less => {
            if boundary_seen {
                xs.extend_from_slice(&ys[xs_len..]);
            } else {
                for &y in ys[xs_len..].iter() {
                    xs.push(limbs_xor_pos_neg_helper(!y, &mut boundary_seen));
                }
            }
        }
        Ordering::Greater => {
            if !boundary_seen {
                for x in xs[ys_len..].iter_mut() {
                    *x = limbs_xor_pos_neg_helper(!*x, &mut boundary_seen);
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
    let mut boundary_seen = false;
    match x_i.cmp(&y_i) {
        Ordering::Equal => {
            ys[y_i] =
                limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_seen);
        }
        Ordering::Less => {
            boundary_seen = true;
            ys[x_i] = xs[x_i].wrapping_neg();
            for (y, &x) in ys[x_i + 1..].iter_mut().zip(xs[x_i + 1..y_i].iter()) {
                *y = !x;
            }
            ys[y_i] -= 1;
            ys[y_i] ^= xs[y_i];
        }
        Ordering::Greater => {
            boundary_seen = true;
            ys[x_i] ^= xs[x_i];
        }
    }
    let xys = xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter_mut());
    if boundary_seen {
        for (&x, y) in xys {
            *y ^= x;
        }
    } else {
        for (&x, y) in xys {
            *y = limbs_xor_pos_neg_helper(x ^ !*y, &mut boundary_seen);
        }
    }
    boundary_seen
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
    let mut boundary_seen = limbs_xor_pos_neg_in_place_right_helper(xs, ys, x_i, y_i);
    if xs_len > ys_len {
        if boundary_seen {
            ys.extend_from_slice(&xs[ys_len..]);
        } else {
            for &x in xs[ys_len..].iter() {
                ys.push(limbs_xor_pos_neg_helper(!x, &mut boundary_seen));
            }
        }
    } else if xs_len < ys_len && !boundary_seen {
        for y in ys[xs_len..].iter_mut() {
            *y = limbs_xor_pos_neg_helper(!*y, &mut boundary_seen);
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
        let mut boundary_seen = limbs_xor_pos_neg_in_place_left_helper(xs, ys, x_i, y_i);
        if xs_len != ys_len && !boundary_seen {
            for x in xs[ys_len..].iter_mut() {
                *x = limbs_xor_pos_neg_helper(!*x, &mut boundary_seen);
            }
        }
        false
    } else {
        let mut boundary_seen = limbs_xor_pos_neg_in_place_right_helper(xs, ys, x_i, y_i);
        if !boundary_seen {
            for y in ys[xs_len..].iter_mut() {
                *y = limbs_xor_pos_neg_helper(!*y, &mut boundary_seen);
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
    let mut out = vec![0; min_i];
    if x_i == y_i {
        out.push(xs[x_i].wrapping_neg() ^ ys[x_i].wrapping_neg());
    } else {
        let (min_zs, max_zs) = if x_i <= y_i { (xs, ys) } else { (ys, xs) };
        out.push(min_zs[min_i].wrapping_neg());
        out.extend(min_zs[min_i + 1..max_i].iter().map(|z| !z));
        out.push((max_zs[max_i] - 1) ^ min_zs[max_i]);
    }
    out.extend(
        xs[max_i + 1..]
            .iter()
            .zip(ys[max_i + 1..].iter())
            .map(|(x, y)| x ^ y),
    );
    match xs_len.cmp(&ys_len) {
        Ordering::Less => out.extend_from_slice(&ys[xs_len..]),
        Ordering::Greater => out.extend_from_slice(&xs[ys_len..]),
        _ => {}
    }
    out
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
    } else if x_i >= ys_len {
        assert!(!limbs_sub_in_place_left(xs, ys));
    } else {
        limbs_xor_neg_neg_in_place_helper(xs, ys, x_i, y_i);
        if xs_len < ys_len {
            xs.extend_from_slice(&ys[xs_len..]);
        }
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
        true
    } else if x_i >= ys_len {
        assert!(!limbs_sub_in_place_left(xs, ys));
        false
    } else if xs_len >= ys_len {
        limbs_xor_neg_neg_in_place_helper(xs, ys, x_i, y_i);
        false
    } else {
        limbs_xor_neg_neg_in_place_helper(ys, xs, y_i, x_i);
        true
    }
}

impl Natural {
    fn xor_assign_neg_limb_pos(&mut self, other: Limb) {
        match self {
            natural_zero!() => {}
            Natural(Small(ref mut small)) => {
                let result = small.wrapping_neg() ^ other;
                if result == 0 {
                    *self = Natural(Large(vec![0, 1]));
                } else {
                    *small = result.wrapping_neg();
                }
            }
            Natural(Large(ref mut limbs)) => {
                limbs_vec_neg_xor_limb_in_place(limbs, other);
                self.trim();
            }
        }
    }

    fn xor_neg_limb_pos(&self, other: Limb) -> Natural {
        match *self {
            natural_zero!() => self.clone(),
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
        match self {
            Natural(Small(ref mut small)) => {
                let result = *small ^ other;
                if result == 0 {
                    *self = Natural(Large(vec![0, 1]))
                } else {
                    *small = result.wrapping_neg();
                }
            }
            Natural(Large(ref mut limbs)) => {
                limbs_vec_pos_xor_limb_neg_in_place(limbs, other);
                self.trim();
            }
        }
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
            Natural(Large(ref mut limbs)) => {
                limbs_neg_xor_limb_neg_in_place(limbs, other);
                self.trim();
            }
        }
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
        match (&mut *self, &mut other) {
            (Natural(Small(x)), _) => {
                other.xor_assign_neg_limb_pos(*x);
                *self = other;
            }
            (_, Natural(Small(y))) => self.xor_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ys))) => {
                if limbs_xor_pos_neg_in_place_either(xs, ys) {
                    *self = other;
                }
                self.trim();
            }
        }
    }

    fn xor_assign_pos_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), _) => *self = other.xor_neg_limb_pos(*x),
            (_, Natural(Small(y))) => self.xor_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_xor_pos_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn xor_assign_neg_pos(&mut self, mut other: Natural) {
        other.xor_assign_pos_neg_ref(&*self);
        *self = other;
    }

    fn xor_assign_neg_pos_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), _) => *self = other.xor_pos_limb_neg(x.wrapping_neg()),
            (_, Natural(Small(y))) => self.xor_assign_neg_limb_pos(*y),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_xor_pos_neg_in_place_right(ys, xs);
                self.trim();
            }
        }
    }

    fn xor_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (&Natural(Small(x)), _) => other.xor_neg_limb_pos(x),
            (_, &Natural(Small(y))) => self.xor_pos_limb_neg(y.wrapping_neg()),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_xor_pos_neg(xs, ys))
            }
        }
    }

    fn xor_assign_neg_neg(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (Natural(Small(x)), _) => *self = other.xor_neg_limb_neg(x.wrapping_neg()),
            (_, Natural(Small(y))) => self.xor_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_xor_neg_neg_in_place_either(xs, ys) {
                    *self = other;
                }
                self.trim();
            }
        }
    }

    fn xor_assign_neg_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (Natural(Small(x)), _) => *self = other.xor_neg_limb_neg(x.wrapping_neg()),
            (_, Natural(Small(y))) => self.xor_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_xor_neg_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn xor_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (&Natural(Small(x)), _) => other.xor_neg_limb_neg(x.wrapping_neg()),
            (_, &Natural(Small(y))) => self.xor_neg_limb_neg(y.wrapping_neg()),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_xor_neg_neg(xs, ys))
            }
        }
    }
}

impl BitXor<Integer> for Integer {
    type Output = Integer;

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
    /// assert_eq!(
    ///     (-Integer::trillion() ^ -(Integer::trillion() + Integer::ONE)).to_string(),
    ///     "8191"
    /// );
    /// ```
    #[inline]
    fn bitxor(mut self, other: Integer) -> Integer {
        self ^= other;
        self
    }
}

impl<'a> BitXor<&'a Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise xor of two `Integer`s, taking the left `Integer` by value and the right
    /// `Integer` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(m)
    ///
    /// where n = `self.significant_bits() + other.significant_bits()`,
    ///       m = `other.significant_bits()`
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
    /// assert_eq!(
    ///     (-Integer::trillion() ^ &-(Integer::trillion() + Integer::ONE)).to_string(),
    ///     "8191"
    /// );
    /// ```
    #[inline]
    fn bitxor(mut self, other: &'a Integer) -> Integer {
        self ^= other;
        self
    }
}

impl<'a> BitXor<Integer> for &'a Integer {
    type Output = Integer;

    /// Takes the bitwise xor of two `Integer`s, taking the left `Integer` by reference and the
    /// right `Integer` by value.
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
    /// assert_eq!(
    ///     (&-Integer::trillion() ^ -(Integer::trillion() + Integer::ONE)).to_string(),
    ///     "8191"
    /// );
    /// ```
    #[inline]
    fn bitxor(self, mut other: Integer) -> Integer {
        other ^= self;
        other
    }
}

impl<'a, 'b> BitXor<&'a Integer> for &'b Integer {
    type Output = Integer;

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

impl BitXorAssign<Integer> for Integer {
    /// Bitwise-xors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS
    /// by value.
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
    /// let mut x = Integer::from(u32::MAX);
    /// x ^= Integer::from(0x0000_000f);
    /// x ^= Integer::from(0x0000_0f00);
    /// x ^= Integer::from(0x000f_0000);
    /// x ^= Integer::from(0x0f00_0000);
    /// assert_eq!(x, 0xf0f0_f0f0u32);
    /// ```
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

impl<'a> BitXorAssign<&'a Integer> for Integer {
    /// Bitwise-xors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS
    /// by reference.
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
    /// let mut x = Integer::from(u32::MAX);
    /// x ^= &Integer::from(0x0000_000f);
    /// x ^= &Integer::from(0x0000_0f00);
    /// x ^= &Integer::from(0x000f_0000);
    /// x ^= &Integer::from(0x0f00_0000);
    /// assert_eq!(x, 0xf0f0_f0f0u32);
    /// ```
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
