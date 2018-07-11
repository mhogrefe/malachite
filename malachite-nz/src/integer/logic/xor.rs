use integer::Integer;
use malachite_base::limbs::{limbs_leading_zero_limbs, limbs_set_zero};
use malachite_base::num::WrappingNegAssign;
use natural::arithmetic::sub::{
    limbs_sub, limbs_sub_in_place_left, limbs_sub_in_place_right, limbs_sub_to_out,
};
use natural::logic::not::limbs_not_in_place;
use natural::Natural::{self, Large, Small};
use std::cmp::max;
use std::iter::repeat;
use std::ops::{BitXor, BitXorAssign};
use std::u32;

fn limbs_xor_pos_neg_helper(input: u32, boundary_limb_seen: &mut bool) -> u32 {
    if *boundary_limb_seen {
        !input
    } else if input == 0 {
        0
    } else {
        *boundary_limb_seen = true;
        input.wrapping_neg()
    }
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of one `Integer` and the
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
pub fn limbs_xor_pos_neg(xs: &[u32], ys: &[u32]) -> Vec<u32> {
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

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of one `Integer` and the
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
/// Panics if `xs` or `ys` are empty or contain only zeros, or if `out_limbs` is shorter than the
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
pub fn limbs_xor_pos_neg_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out_limbs.len() >= xs_len);
    assert!(out_limbs.len() >= ys_len);
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        limbs_set_zero(&mut out_limbs[..x_i]);
        out_limbs[x_i] = xs[x_i].wrapping_neg();
        for (out, &x) in out_limbs[x_i + 1..xs_len]
            .iter_mut()
            .zip(xs[x_i + 1..].iter())
        {
            *out = !x;
        }
        for out in out_limbs[xs_len..y_i].iter_mut() {
            *out = u32::MAX;
        }
        out_limbs[y_i] = ys[y_i] - 1;
        out_limbs[y_i + 1..ys_len].copy_from_slice(&ys[y_i + 1..]);
        return;
    } else if x_i >= ys_len {
        out_limbs[..ys_len].copy_from_slice(ys);
        out_limbs[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
        return;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    limbs_set_zero(&mut out_limbs[..min_i]);
    let mut boundary_limb_seen = false;
    if x_i == y_i {
        out_limbs[x_i] =
            limbs_xor_pos_neg_helper(xs[x_i] ^ ys[y_i].wrapping_neg(), &mut boundary_limb_seen);
    } else if x_i > y_i {
        boundary_limb_seen = true;
        out_limbs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        out_limbs[x_i] = xs[x_i] ^ ys[x_i];
    } else {
        boundary_limb_seen = true;
        out_limbs[x_i] = xs[x_i].wrapping_neg();
        for (out, &x) in out_limbs[x_i + 1..y_i]
            .iter_mut()
            .zip(xs[x_i + 1..y_i].iter())
        {
            *out = !x;
        }
        out_limbs[y_i] = xs[y_i] ^ (ys[y_i] - 1);
    }
    {
        let xys = out_limbs[max_i + 1..]
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
            out_limbs[min_len..max_len].copy_from_slice(zs);
        } else {
            for (out, &z) in out_limbs[min_len..].iter_mut().zip(zs.iter()) {
                *out = limbs_xor_pos_neg_helper(!z, &mut boundary_limb_seen);
            }
        }
    }
}

fn limbs_xor_pos_neg_in_place_left_helper(
    xs: &mut [u32],
    ys: &[u32],
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

/// Interpreting a `Vec` of `u32`s and a slice of `u32`s as the limbs (in ascending order) of one
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
pub fn limbs_xor_pos_neg_in_place_left(xs: &mut Vec<u32>, ys: &[u32]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..]);
        xs.extend(repeat(u32::MAX).take(y_i - xs_len));
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
    xs: &[u32],
    ys: &mut [u32],
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

/// Interpreting a slice of `u32`s and a `Vec` of `u32`s as the limbs (in ascending order) of one
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
pub fn limbs_xor_pos_neg_in_place_right(xs: &[u32], ys: &mut Vec<u32>) {
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
            *y = u32::MAX;
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

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of one `Integer` and the
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
pub fn limbs_xor_pos_neg_in_place_either(xs: &mut [u32], ys: &mut [u32]) -> bool {
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
            *y = u32::MAX;
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

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of the negatives of two
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
pub fn limbs_xor_neg_neg(xs: &[u32], ys: &[u32]) -> Vec<u32> {
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

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of the negatives of two
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
/// Panics if `xs` or `ys` are empty or contain only zeros, or if `out_limbs` is shorter than the
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
pub fn limbs_xor_neg_neg_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out_limbs.len() >= xs_len);
    assert!(out_limbs.len() >= ys_len);
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        assert!(!limbs_sub_to_out(out_limbs, ys, xs));
        return;
    } else if x_i >= ys_len {
        assert!(!limbs_sub_to_out(out_limbs, xs, ys));
        return;
    }
    let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
    limbs_set_zero(&mut out_limbs[..min_i]);
    if x_i == y_i {
        out_limbs[x_i] = xs[x_i].wrapping_neg() ^ ys[x_i].wrapping_neg();
    } else {
        let (min_zs, max_zs) = if x_i <= y_i { (xs, ys) } else { (ys, xs) };
        out_limbs[min_i] = min_zs[min_i].wrapping_neg();
        for (out, &z) in out_limbs[min_i + 1..max_i]
            .iter_mut()
            .zip(min_zs[min_i + 1..max_i].iter())
        {
            *out = !z;
        }
        out_limbs[max_i] = (max_zs[max_i] - 1) ^ min_zs[max_i];
    }
    for (out, (&x, &y)) in out_limbs[max_i + 1..]
        .iter_mut()
        .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
    {
        *out = x ^ y;
    }
    if xs_len > ys_len {
        out_limbs[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
    } else if xs_len < ys_len {
        out_limbs[xs_len..ys_len].copy_from_slice(&ys[xs_len..]);
    }
}

fn limbs_xor_neg_neg_in_place_helper(xs: &mut [u32], ys: &[u32], x_i: usize, y_i: usize) {
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

/// Interpreting a `Vec` of `u32`s and a slice of `u32`s as the limbs (in ascending order) of the
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
pub fn limbs_xor_neg_neg_in_place_left(xs: &mut Vec<u32>, ys: &[u32]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = limbs_leading_zero_limbs(xs);
    let y_i = limbs_leading_zero_limbs(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        assert!(!limbs_sub_in_place_right(ys, xs));
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

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of the negatives of two
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
pub fn limbs_xor_neg_neg_in_place_either(xs: &mut [u32], ys: &mut [u32]) -> bool {
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
/// use malachite_base::num::NegativeOne;
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
/// use malachite_base::num::NegativeOne;
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
    fn xor_assign_pos_neg(&mut self, mut other: Natural) {
        if let Small(y) = other {
            self.xor_assign_pos_u32_neg(y.wrapping_neg());
            return;
        } else if let Small(x) = *self {
            if let Large(_) = other {
                *self = other;
                self.xor_assign_neg_u32_pos(x);
            }
            return;
        } else if let Large(ref mut ys) = other {
            let right = if let Large(ref mut xs) = *self {
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
        if let Small(y) = *other {
            self.xor_assign_pos_u32_neg(y.wrapping_neg());
        } else if let Small(x) = *self {
            if let Large(_) = *other {
                *self = other.clone();
                self.xor_assign_neg_u32_pos(x);
            }
        } else if let Large(ref ys) = *other {
            if let Large(ref mut xs) = *self {
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
        let new_self_value = if let Small(x) = *self {
            let mut new_self_value = other.clone();
            new_self_value.xor_assign_pos_u32_neg(x.wrapping_neg());
            Some(new_self_value)
        } else if let Small(y) = *other {
            if let Large(_) = *self {
                self.xor_assign_neg_u32_pos(y);
            } else {
                unreachable!()
            };
            None
        } else if let Large(ref ys) = *other {
            if let Large(ref mut xs) = *self {
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
            (_, &Small(y)) => self.xor_pos_u32_neg(y.wrapping_neg()),
            (&Small(x), _) => other.xor_neg_u32_pos(x),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_xor_pos_neg(xs, ys));
                result.trim();
                result
            }
        }
    }

    fn xor_assign_neg_neg(&mut self, other: Natural) {
        let new_self_value = if let Small(y) = other {
            self.xor_assign_neg_u32_neg(y.wrapping_neg());
            None
        } else if let Small(ref mut x) = *self {
            let mut new_self_value = other.clone();
            new_self_value.xor_assign_neg_u32_neg(x.wrapping_neg());
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
            self.xor_assign_neg_u32_neg(y.wrapping_neg());
            None
        } else if let Small(ref mut x) = *self {
            let mut new_self_value = other.clone();
            new_self_value.xor_assign_neg_u32_neg(x.wrapping_neg());
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
            (_, &Small(y)) => self.xor_neg_u32_neg(y.wrapping_neg()),
            (&Small(x), _) => other.xor_neg_u32_neg(x.wrapping_neg()),
            (&Large(ref xs), &Large(ref ys)) => {
                let mut result = Large(limbs_xor_neg_neg(xs, ys));
                result.trim();
                result
            }
        }
    }
}
