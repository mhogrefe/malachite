use std::cmp::{max, Ordering};
use std::iter::repeat;
use std::ops::{BitOr, BitOrAssign};

use malachite_base::num::arithmetic::traits::WrappingNegAssign;
use malachite_base::slices::{slice_leading_zeros, slice_set_zero};

use integer::Integer;
use natural::logic::not::{limbs_not_in_place, limbs_not_to_out};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the limbs of the bitwise or of the `Integer` and a `Limb`. `xs` cannot be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// May panic if `xs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_neg_or_limb;
///
/// assert_eq!(limbs_neg_or_limb(&[123, 456], 789), &[107, 456]);
/// assert_eq!(limbs_neg_or_limb(&[0, 0, 456], 789), &[0xffff_fceb, u32::MAX, 455]);
/// ```
pub fn limbs_neg_or_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    if y == 0 {
        return xs.to_vec();
    }
    let mut out = vec![0; xs.len()];
    let i = slice_leading_zeros(xs);
    if i == 0 {
        out[0] = (xs[0].wrapping_neg() | y).wrapping_neg();
        out[1..].copy_from_slice(&xs[1..]);
    } else {
        out[0] = y.wrapping_neg();
        for x in out[1..i].iter_mut() {
            *x = Limb::MAX;
        }
        out[i] = xs[i] - 1;
        out[i + 1..].copy_from_slice(&xs[i + 1..]);
    }
    out
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise or of the `Integer` and a `Limb` to an output slice.
/// The output slice must be at least as long as the input slice. `xs` cannot be empty or only
/// contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// May panic if `xs` is empty or only contains zeros, or if `out` is shorter than `xs`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_neg_or_limb_to_out;
///
/// let mut xs = vec![0, 0, 0, 0];
/// limbs_neg_or_limb_to_out(&mut xs, &[123, 456], 789);
/// assert_eq!(xs, &[107, 456, 0, 0]);
///
/// let mut xs = vec![0, 0, 0, 0];
/// limbs_neg_or_limb_to_out(&mut xs, &[0, 0, 456], 789);
/// assert_eq!(xs, &[0xffff_fceb, u32::MAX, 455, 0]);
/// ```
pub fn limbs_neg_or_limb_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) {
    let len = xs.len();
    assert!(out.len() >= len);
    if y == 0 {
        out[..len].copy_from_slice(xs);
        return;
    }
    let i = slice_leading_zeros(xs);
    if i == 0 {
        out[0] = (xs[0].wrapping_neg() | y).wrapping_neg();
        out[1..len].copy_from_slice(&xs[1..]);
    } else {
        out[0] = y.wrapping_neg();
        for x in out[1..i].iter_mut() {
            *x = Limb::MAX;
        }
        out[i] = xs[i] - 1;
        out[i + 1..len].copy_from_slice(&xs[i + 1..]);
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, writes the limbs of the bitwise or of the `Integer`, writes the limbs of the bitwise
/// or of the `Integer` and a `Limb` to the input slice. `xs` cannot be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// May panic if `xs` is empty or only contains zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_neg_or_limb_in_place;
///
/// let mut xs = vec![123, 456];
/// limbs_neg_or_limb_in_place(&mut xs, 789);
/// assert_eq!(xs, &[107, 456]);
///
/// let mut xs = vec![0, 0, 456];
/// limbs_neg_or_limb_in_place(&mut xs, 789);
/// assert_eq!(xs, &[0xffff_fceb, u32::MAX, 455]);
/// ```
pub fn limbs_neg_or_limb_in_place(xs: &mut [Limb], y: Limb) {
    if y == 0 {
        return;
    }
    let i = slice_leading_zeros(xs);
    if i == 0 {
        xs[0] = (xs[0].wrapping_neg() | y).wrapping_neg();
    } else {
        xs[0] = y.wrapping_neg();
        for x in xs[1..i].iter_mut() {
            *x = Limb::MAX;
        }
        xs[i] -= 1;
    }
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of an `Integer`, returns the
/// negative of the bitwise or of the `Integer` and a negative number whose lowest limb is given by
/// `y` and whose other limbs are full of `true` bits. The slice cannot be empty or only contain
/// zeros.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_pos_or_neg_limb;
///
/// assert_eq!(limbs_pos_or_neg_limb(&[6, 7], 3), 4294967289);
/// assert_eq!(limbs_pos_or_neg_limb(&[100, 101, 102], 10), 4294967186);
/// ```
pub const fn limbs_pos_or_neg_limb(xs: &[Limb], y: Limb) -> Limb {
    (xs[0] | y).wrapping_neg()
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of the negative of an
/// `Integer`, returns the negative of the bitwise or of the `Integer` and a negative number whose
/// lowest limb is given by `y` and whose other limbs are full of `true` bits. The slice cannot be
/// empty or only contain zeros.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `xs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_neg_or_neg_limb;
///
/// assert_eq!(limbs_neg_or_neg_limb(&[6, 7], 3), 5);
/// assert_eq!(limbs_neg_or_neg_limb(&[100, 101, 102], 10), 98);
/// ```
pub const fn limbs_neg_or_neg_limb(xs: &[Limb], y: Limb) -> Limb {
    (xs[0].wrapping_neg() | y).wrapping_neg()
}

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
/// use malachite_nz::integer::logic::or::limbs_or_pos_neg;
///
/// assert_eq!(limbs_or_pos_neg(&[1, 2], &[100, 200]), &[99, 200]);
/// assert_eq!(limbs_or_pos_neg(&[1, 2], &[100, 200, 300]), &[99, 200, 300]);
/// ```
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res is returned, the first input is positive,
/// and the second is negative.
pub fn limbs_or_pos_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
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
        out
    } else if x_i >= ys_len {
        ys.to_vec()
    } else {
        let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
        let mut out = vec![0; min_i];
        match x_i.cmp(&y_i) {
            Ordering::Equal => {
                out.push((!xs[x_i] & (ys[y_i] - 1)) + 1);
            }
            Ordering::Less => {
                out.push(xs[x_i].wrapping_neg());
                out.extend(xs[x_i + 1..y_i].iter().map(|x| !x));
                out.push(!xs[y_i] & (ys[y_i] - 1));
            }
            Ordering::Greater => {
                out.extend_from_slice(&ys[y_i..x_i]);
                out.push(!xs[x_i] & ys[x_i]);
            }
        }
        out.extend(
            xs[max_i + 1..]
                .iter()
                .zip(ys[max_i + 1..].iter())
                .map(|(x, y)| !x & y),
        );
        if xs_len < ys_len {
            out.extend_from_slice(&ys[xs_len..]);
        }
        out
    }
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
/// use malachite_nz::integer::logic::or::limbs_or_pos_neg_to_out;
///
/// let mut out = vec![0, 0];
/// limbs_or_pos_neg_to_out(&mut out, &[1, 2], &[100, 200]);
/// assert_eq!(out, &[99, 200]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_or_pos_neg_to_out(&mut out, &[1, 2], &[100, 200, 300]);
/// assert_eq!(out, &[99, 200, 300, 10]);
/// ```
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where the first input is positive and the second is
/// negative.
pub fn limbs_or_pos_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        slice_set_zero(&mut out[..x_i]);
        out[x_i] = xs[x_i].wrapping_neg();
        limbs_not_to_out(&mut out[x_i + 1..xs_len], &xs[x_i + 1..]);
        for x in out[xs_len..y_i].iter_mut() {
            *x = Limb::MAX;
        }
        out[y_i] = ys[y_i] - 1;
        out[y_i + 1..ys_len].copy_from_slice(&ys[y_i + 1..]);
    } else if x_i >= ys_len {
        out[..ys_len].copy_from_slice(ys);
    } else {
        let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
        slice_set_zero(&mut out[..min_i]);
        match x_i.cmp(&y_i) {
            Ordering::Equal => {
                out[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
            }
            Ordering::Less => {
                out[x_i] = xs[x_i].wrapping_neg();
                limbs_not_to_out(&mut out[x_i + 1..y_i], &xs[x_i + 1..y_i]);
                out[y_i] = !xs[y_i] & (ys[y_i] - 1);
            }
            Ordering::Greater => {
                out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
                out[x_i] = !xs[x_i] & ys[x_i];
            }
        }
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
/// use malachite_nz::integer::logic::or::limbs_slice_or_pos_neg_in_place_left;
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
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res == op1, the first input is positive and the
/// second is negative, and the length of op1 is not changed; instead, a carry is returned.
pub fn limbs_slice_or_pos_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs[x_i].wrapping_neg_assign();
        limbs_not_in_place(&mut xs[x_i + 1..]);
        true
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        slice_set_zero(&mut xs[ys_len..]);
        false
    } else {
        let max_i = max(x_i, y_i);
        match x_i.cmp(&y_i) {
            Ordering::Equal => {
                xs[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
            }
            Ordering::Less => {
                xs[x_i].wrapping_neg_assign();
                limbs_not_in_place(&mut xs[x_i + 1..y_i]);
                xs[y_i] = !xs[y_i] & (ys[y_i] - 1);
            }
            Ordering::Greater => {
                xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
                xs[x_i] = !xs[x_i] & ys[x_i];
            }
        }
        if xs_len < ys_len {
            for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..xs_len].iter()) {
                *x = !*x & y
            }
            true
        } else {
            for (x, y) in xs[max_i + 1..ys_len].iter_mut().zip(ys[max_i + 1..].iter()) {
                *x = !*x & y
            }
            slice_set_zero(&mut xs[ys_len..]);
            false
        }
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
/// use malachite_nz::integer::logic::or::limbs_vec_or_pos_neg_in_place_left;
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
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res == op1 and the first input is positive and
/// the second is negative.
pub fn limbs_vec_or_pos_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
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
    } else if x_i >= ys_len {
        *xs = ys.to_vec();
    } else {
        let max_i = max(x_i, y_i);
        match x_i.cmp(&y_i) {
            Ordering::Equal => {
                xs[x_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
            }
            Ordering::Less => {
                xs[x_i].wrapping_neg_assign();
                limbs_not_in_place(&mut xs[x_i + 1..y_i]);
                xs[y_i] = !xs[y_i] & (ys[y_i] - 1);
            }
            Ordering::Greater => {
                xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
                xs[x_i] = !xs[x_i] & ys[x_i];
            }
        }
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
/// use malachite_nz::integer::logic::or::limbs_or_pos_neg_in_place_right;
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
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res == op2 and the first input is positive and
/// the second is negative.
pub fn limbs_or_pos_neg_in_place_right(xs: &[Limb], ys: &mut [Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        ys[x_i] = xs[x_i].wrapping_neg();
        limbs_not_to_out(&mut ys[x_i + 1..xs_len], &xs[x_i + 1..]);
        for y in ys[xs_len..y_i].iter_mut() {
            *y = Limb::MAX;
        }
        ys[y_i] -= 1;
    } else if x_i < ys_len {
        let max_i = max(x_i, y_i);
        match x_i.cmp(&y_i) {
            Ordering::Equal => {
                ys[y_i] = (!xs[x_i] & (ys[y_i] - 1)) + 1;
            }
            Ordering::Less => {
                ys[x_i] = xs[x_i].wrapping_neg();
                limbs_not_to_out(&mut ys[x_i + 1..y_i], &xs[x_i + 1..y_i]);
                ys[y_i] = !xs[y_i] & (ys[y_i] - 1);
            }
            Ordering::Greater => {
                ys[x_i] &= !xs[x_i];
            }
        }
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
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, returns the limbs of the bitwise or of the `Integer`s. `xs` and `ys` may not be
/// empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len() + ys.len()`, m = min(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_or_neg_neg;
///
/// assert_eq!(limbs_or_neg_neg(&[1, 2], &[100, 200]), &[1, 0]);
/// assert_eq!(limbs_or_neg_neg(&[1, 2, 5], &[100, 200]), &[1, 0]);
/// ```
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res is returned and both inputs are negative.
pub fn limbs_or_neg_neg(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        xs.to_vec()
    } else if x_i >= ys_len {
        ys.to_vec()
    } else {
        let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
        let mut out = vec![0; min_i];
        let x = match x_i.cmp(&y_i) {
            Ordering::Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Ordering::Less => {
                out.extend_from_slice(&xs[x_i..y_i]);
                xs[y_i] & (ys[y_i] - 1)
            }
            Ordering::Greater => {
                out.extend_from_slice(&ys[y_i..x_i]);
                (xs[x_i] - 1) & ys[x_i]
            }
        };
        out.push(x);
        out.extend(
            xs[max_i + 1..]
                .iter()
                .zip(ys[max_i + 1..].iter())
                .map(|(x, y)| x & y),
        );
        out
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, writes the max(`xs.len()`, `ys.len()`) limbs of the bitwise or of the `Integer`s to
/// an output slice. `xs` and `ys` may not be empty or only contain zeros. The output slice must be
/// at least as long as the shorter input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len() + ys.len()`
///
/// # Panics
/// Panics if `xs` or `ys` are empty or contain only zeros, or if `out` is shorter than the
/// shorter of `xs` and `ys`.
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::or::limbs_or_neg_neg_to_out;
///
/// let mut out = vec![10, 10];
/// limbs_or_neg_neg_to_out(&mut out, &[1, 2], &[100, 200]);
/// assert_eq!(out, &[1, 0]);
///
/// let mut out = vec![10, 10, 10, 10];
/// limbs_or_neg_neg_to_out(&mut out, &[1, 2, 5], &[100, 200]);
/// assert_eq!(out, &[1, 0, 10, 10]);
/// ```
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where both inputs are negative.
pub fn limbs_or_neg_neg_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len || out.len() >= ys_len);
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        out[..xs_len].copy_from_slice(xs);
    } else if x_i >= ys_len {
        out[..ys_len].copy_from_slice(ys);
    } else {
        let (min_i, max_i) = if x_i <= y_i { (x_i, y_i) } else { (y_i, x_i) };
        slice_set_zero(&mut out[..min_i]);
        let x = match x_i.cmp(&y_i) {
            Ordering::Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Ordering::Less => {
                out[x_i..y_i].copy_from_slice(&xs[x_i..y_i]);
                xs[y_i] & (ys[y_i] - 1)
            }
            Ordering::Greater => {
                out[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
                (xs[x_i] - 1) & ys[x_i]
            }
        };
        out[max_i] = x;
        for (out, (x, y)) in out[max_i + 1..]
            .iter_mut()
            .zip(xs[max_i + 1..].iter().zip(ys[max_i + 1..].iter()))
        {
            *out = x & y;
        }
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, writes the limbs of the bitwise or of the `Integer`s to the first (left) slice. `xs`
/// and `ys` may not be empty or only contain zeros. If the result has fewer significant limbs than
/// the left slice, the remaining limbs in the left slice are set to `Limb::MAX`.
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
/// use malachite_nz::integer::logic::or::limbs_slice_or_neg_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// limbs_slice_or_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[1, 0]);
///
/// let mut xs = vec![1, 2, 5];
/// limbs_slice_or_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[1, 0, 0]);
/// ```
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res == op1, both inputs are negative, and the
/// length of op1 is not changed.
pub fn limbs_slice_or_neg_neg_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
    } else if x_i >= ys_len {
        xs[..ys_len].copy_from_slice(ys);
        slice_set_zero(&mut xs[ys_len..]);
    } else {
        let max_i = max(x_i, y_i);
        if x_i > y_i {
            xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        }
        xs[max_i] = match x_i.cmp(&y_i) {
            Ordering::Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Ordering::Less => xs[y_i] & (ys[y_i] - 1),
            Ordering::Greater => (xs[x_i] - 1) & ys[x_i],
        };
        for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
            *x &= y;
        }
        if xs_len > ys_len {
            slice_set_zero(&mut xs[ys_len..]);
        }
    }
}

/// Interpreting a slice of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of the
/// negatives of two `Integer`s, writes the limbs of the bitwise or of the `Integer`s to the `Vec`.
/// `xs` and `ys` may not be empty or only contain zeros.
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
/// use malachite_nz::integer::logic::or::limbs_vec_or_neg_neg_in_place_left;
///
/// let mut xs = vec![1, 2];
/// limbs_vec_or_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[1, 0]);
///
/// let mut xs = vec![100, 200];
/// limbs_vec_or_neg_neg_in_place_left(&mut xs, &[1, 2, 5]);
/// assert_eq!(xs, &[1, 0]);
///
/// let mut xs = vec![1, 2, 5];
/// limbs_vec_or_neg_neg_in_place_left(&mut xs, &[100, 200]);
/// assert_eq!(xs, &[1, 0]);
/// ```
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where res == op1 and both inputs are negative.
pub fn limbs_vec_or_neg_neg_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
    } else if x_i >= ys_len {
        xs.truncate(ys_len);
        xs.copy_from_slice(ys);
    } else {
        let max_i = max(x_i, y_i);
        if x_i > y_i {
            xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
        }
        xs[max_i] = match x_i.cmp(&y_i) {
            Ordering::Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Ordering::Less => xs[y_i] & (ys[y_i] - 1),
            Ordering::Greater => (xs[x_i] - 1) & ys[x_i],
        };
        for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
            *x &= y;
        }
        xs.truncate(ys_len);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of the negatives of two
/// `Integer`s, writes the lower min(`xs.len()`, `ys.len()`) limbs of the bitwise or of the
/// `Integer`s to the shorter slice (or the first one, if they are equally long). `xs` and `ys` may
/// not be empty or only contain zeros. Returns a `bool` which is `false` when the output is to the
/// first slice and `true` when it's to the second slice.
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
/// use malachite_nz::integer::logic::or::limbs_or_neg_neg_in_place_either;
///
/// let mut xs = vec![1, 2];
/// let mut ys = vec![100, 200];
/// assert_eq!(limbs_or_neg_neg_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[1, 0]);
/// assert_eq!(ys, &[100, 200]);
///
/// let mut xs = vec![1, 2, 5];
/// let mut ys = vec![100, 200];
/// assert_eq!(limbs_or_neg_neg_in_place_either(&mut xs, &mut ys), true);
/// assert_eq!(xs, &[1, 2, 5]);
/// assert_eq!(ys, &[1, 0]);
///
/// let mut xs = vec![100, 200];
/// let mut ys = vec![1, 2, 5];
/// assert_eq!(limbs_or_neg_neg_in_place_either(&mut xs, &mut ys), false);
/// assert_eq!(xs, &[1, 0]);
/// assert_eq!(ys, &[1, 2, 5]);
/// ```
///
/// This is mpz_ior from mpz/ior.c, GMP 6.1.2, where both inputs are negative and the result is
/// written to the shorter input slice.
pub fn limbs_or_neg_neg_in_place_either(xs: &mut [Limb], ys: &mut [Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let x_i = slice_leading_zeros(xs);
    let y_i = slice_leading_zeros(ys);
    assert!(x_i < xs_len);
    assert!(y_i < ys_len);
    if y_i >= xs_len {
        false
    } else if x_i >= ys_len {
        true
    } else {
        let max_i = max(x_i, y_i);
        let boundary = match x_i.cmp(&y_i) {
            Ordering::Equal => ((xs[x_i] - 1) & (ys[y_i] - 1)) + 1,
            Ordering::Less => xs[y_i] & (ys[y_i] - 1),
            Ordering::Greater => (xs[x_i] - 1) & ys[x_i],
        };
        if xs_len > ys_len {
            if y_i > x_i {
                ys[x_i..y_i].copy_from_slice(&xs[x_i..y_i]);
            }
            ys[max_i] = boundary;
            for (y, x) in ys[max_i + 1..].iter_mut().zip(xs[max_i + 1..].iter()) {
                *y &= x;
            }
            true
        } else {
            if x_i > y_i {
                xs[y_i..x_i].copy_from_slice(&ys[y_i..x_i]);
            }
            xs[max_i] = boundary;
            for (x, y) in xs[max_i + 1..].iter_mut().zip(ys[max_i + 1..].iter()) {
                *x &= y;
            }
            false
        }
    }
}

impl Natural {
    fn or_assign_pos_limb_neg(&mut self, other: Limb) {
        *self = self.or_pos_limb_neg(other);
    }

    fn or_pos_limb_neg(&self, other: Limb) -> Natural {
        Natural(Small(match *self {
            Natural(Small(small)) => (small | other).wrapping_neg(),
            Natural(Large(ref limbs)) => limbs_pos_or_neg_limb(limbs, other),
        }))
    }

    fn or_assign_neg_limb_neg(&mut self, other: Limb) {
        *self = self.or_neg_limb_neg(other);
    }

    fn or_neg_limb_neg(&self, other: Limb) -> Natural {
        Natural(Small(match *self {
            Natural(Small(small)) => (small.wrapping_neg() | other).wrapping_neg(),
            Natural(Large(ref limbs)) => limbs_neg_or_neg_limb(limbs, other),
        }))
    }

    fn or_assign_neg_limb_pos(&mut self, other: Limb) {
        match *self {
            Natural(Small(ref mut small)) => {
                *small = (small.wrapping_neg() | other).wrapping_neg();
            }
            Natural(Large(ref mut limbs)) => {
                limbs_neg_or_limb_in_place(limbs, other);
                self.trim();
            }
        }
    }

    fn or_neg_limb_pos(&self, other: Limb) -> Natural {
        match *self {
            Natural(Small(ref small)) => {
                Natural(Small((small.wrapping_neg() | other).wrapping_neg()))
            }
            Natural(Large(ref limbs)) => {
                Natural::from_owned_limbs_asc(limbs_neg_or_limb(limbs, other))
            }
        }
    }

    fn or_assign_pos_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.or_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Small(x)), _) => *self = other.or_neg_limb_pos(*x),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_vec_or_pos_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn or_assign_pos_neg(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.or_assign_pos_limb_neg(y.wrapping_neg()),
            (Natural(Small(x)), _) => {
                other.or_assign_neg_limb_pos(*x);
                *self = other;
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                limbs_or_pos_neg_in_place_right(xs, ys);
                *self = other;
                self.trim();
            }
        }
    }

    fn or_assign_neg_pos_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.or_assign_neg_limb_pos(*y),
            (Natural(Small(x)), _) => *self = other.or_pos_limb_neg(x.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_or_pos_neg_in_place_right(ys, xs);
                self.trim();
            }
        }
    }

    fn or_assign_neg_pos(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.or_assign_neg_limb_pos(*y),
            (Natural(Small(x)), _) => {
                other.or_assign_pos_limb_neg(x.wrapping_neg());
                *self = other;
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                limbs_or_pos_neg_in_place_right(ys, xs);
                self.trim();
            }
        }
    }

    fn or_pos_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.or_pos_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), _) => other.or_neg_limb_pos(x),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_or_pos_neg(xs, ys))
            }
        }
    }

    fn or_assign_neg_neg_ref(&mut self, other: &Natural) {
        match (&mut *self, other) {
            (_, Natural(Small(y))) => self.or_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Small(x)), _) => *self = other.or_neg_limb_neg(x.wrapping_neg()),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys))) => {
                limbs_vec_or_neg_neg_in_place_left(xs, ys);
                self.trim();
            }
        }
    }

    fn or_assign_neg_neg(&mut self, mut other: Natural) {
        match (&mut *self, &mut other) {
            (_, Natural(Small(y))) => self.or_assign_neg_limb_neg(y.wrapping_neg()),
            (Natural(Small(x)), _) => {
                other.or_assign_neg_limb_neg(x.wrapping_neg());
                *self = other;
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys))) => {
                if limbs_or_neg_neg_in_place_either(xs, ys) {
                    *self = other;
                }
                self.trim();
            }
        }
    }

    fn or_neg_neg(&self, other: &Natural) -> Natural {
        match (self, other) {
            (_, &Natural(Small(y))) => self.or_neg_limb_neg(y.wrapping_neg()),
            (&Natural(Small(x)), _) => other.or_neg_limb_neg(x.wrapping_neg()),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_or_neg_neg(xs, ys))
            }
        }
    }
}

impl BitOr<Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise or of two `Integer`s, taking both by value.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((Integer::from(-123) | Integer::from(-456)).to_string(), "-67");
    /// assert_eq!(
    ///     (-Integer::trillion() | -(Integer::trillion() + Integer::ONE)).to_string(),
    ///     "-999999995905"
    /// );
    /// ```
    #[inline]
    fn bitor(mut self, other: Integer) -> Integer {
        self |= other;
        self
    }
}

impl<'a> BitOr<&'a Integer> for Integer {
    type Output = Integer;

    /// Takes the bitwise or of two `Integer`s, taking the left `Integer` by value and the right
    /// `Integer` by reference.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((Integer::from(-123) | &Integer::from(-456)).to_string(), "-67");
    /// assert_eq!((-Integer::trillion() | &-(Integer::trillion() + Integer::ONE)).to_string(),
    ///     "-999999995905");
    /// ```
    #[inline]
    fn bitor(mut self, other: &'a Integer) -> Integer {
        self |= other;
        self
    }
}

impl<'a> BitOr<Integer> for &'a Integer {
    type Output = Integer;

    /// Takes the bitwise or of two `Integer`s, taking the left `Integer` by reference and the right
    /// `Integer` by value.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-123) | Integer::from(-456)).to_string(), "-67");
    /// assert_eq!((&-Integer::trillion() | -(Integer::trillion() + Integer::ONE)).to_string(),
    ///     "-999999995905");
    /// ```
    #[inline]
    fn bitor(self, mut other: Integer) -> Integer {
        other |= self;
        other
    }
}

impl<'a, 'b> BitOr<&'a Integer> for &'b Integer {
    type Output = Integer;

    /// Takes the bitwise or of an `Integer` and a `Natural`, taking both by reference.
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
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-123) | &Integer::from(-456)).to_string(), "-67");
    /// assert_eq!((&-Integer::trillion() | &-(Integer::trillion() + Integer::ONE)).to_string(),
    ///     "-999999995905");
    /// ```
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

impl BitOrAssign<Integer> for Integer {
    /// Bitwise-ors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
    /// value.
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
    ///
    /// let mut x = Integer::ZERO;
    /// x |= Integer::from(0x0000_000f);
    /// x |= Integer::from(0x0000_0f00);
    /// x |= Integer::from(0x000f_0000);
    /// x |= Integer::from(0x0f00_0000);
    /// assert_eq!(x, 0x0f0f_0f0f);
    /// ```
    fn bitor_assign(&mut self, other: Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitor_assign(other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.or_assign_pos_neg(other.abs)
            }
            (false, true) => self.abs.or_assign_neg_pos(other.abs),
            (false, false) => self.abs.or_assign_neg_neg(other.abs),
        }
    }
}

impl<'a> BitOrAssign<&'a Integer> for Integer {
    /// Bitwise-ors an `Integer` with another `Integer` in place, taking the `Integer` on the RHS by
    /// reference.
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
    ///
    /// let mut x = Integer::ZERO;
    /// x |= &Integer::from(0x0000_000f);
    /// x |= &Integer::from(0x0000_0f00);
    /// x |= &Integer::from(0x000f_0000);
    /// x |= &Integer::from(0x0f00_0000);
    /// assert_eq!(x, 0x0f0f_0f0f);
    /// ```
    fn bitor_assign(&mut self, other: &'a Integer) {
        match (self.sign, other.sign) {
            (true, true) => self.abs.bitor_assign(&other.abs),
            (true, false) => {
                self.sign = false;
                self.abs.or_assign_pos_neg_ref(&other.abs)
            }
            (false, true) => self.abs.or_assign_neg_pos_ref(&other.abs),
            (false, false) => self.abs.or_assign_neg_neg_ref(&other.abs),
        }
    }
}
