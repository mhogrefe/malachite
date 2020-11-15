use std::fmt::Display;
use std::ops::{Sub, SubAssign};

use malachite_base::num::arithmetic::traits::{CheckedSub, OverflowingSubAssign};

use natural::Natural;
use platform::Limb;

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
/// `Limb` from the `Natural`. Returns a pair consisting of the limbs of the result, and whether
/// there was a borrow left over; that is, whether the `Limb` was greater than the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_limb;
///
/// assert_eq!(limbs_sub_limb(&[123, 456], 78), (vec![45, 456], false));
/// assert_eq!(limbs_sub_limb(&[123, 456], 789), (vec![4294966630, 455], false));
/// assert_eq!(limbs_sub_limb(&[1], 2), (vec![u32::MAX], true));
/// ```
///
/// This is mpn_sub_1 from gmp.h, GMP 6.1.2, where the result is returned.
pub fn limbs_sub_limb(xs: &[Limb], mut y: Limb) -> (Vec<Limb>, bool) {
    let len = xs.len();
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let (diff, overflow) = xs[i].overflowing_sub(y);
        out.push(diff);
        if overflow {
            y = 1;
        } else {
            y = 0;
            out.extend_from_slice(&xs[i + 1..]);
            break;
        }
    }
    (out, y != 0)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
/// `Limb` from the `Natural`, writing the `xs.len()` limbs of the result to an output slice.
/// Returns whether there was a borrow left over; that is, whether the `Limb` was greater than the
/// `Natural`. The output slice must be at least as long as the input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_limb_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_limb_to_out(&mut out, &[123, 456], 78), false);
/// assert_eq!(out, &[45, 456, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_limb_to_out(&mut out, &[123, 456], 789), false);
/// assert_eq!(out, &[4294966630, 455, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_limb_to_out(&mut out, &[1], 2), true);
/// assert_eq!(out, &[u32::MAX, 0, 0]);
/// ```
///
/// This is mpn_sub_1 from gmp.h, GMP 6.1.2.
pub fn limbs_sub_limb_to_out(out: &mut [Limb], xs: &[Limb], mut y: Limb) -> bool {
    let len = xs.len();
    assert!(out.len() >= len);
    for i in 0..len {
        let (diff, overflow) = xs[i].overflowing_sub(y);
        out[i] = diff;
        if overflow {
            y = 1;
        } else {
            y = 0;
            let copy_index = i + 1;
            out[copy_index..len].copy_from_slice(&xs[copy_index..]);
            break;
        }
    }
    y != 0
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, subtracts the
/// `Limb` from the `Natural` and writes the limbs of the result to the input slice. Returns whether
/// there was a borrow left over; that is, whether the `Limb` was greater than the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_limb_in_place;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_sub_limb_in_place(&mut xs, 78), false);
/// assert_eq!(xs, &[45, 456]);
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_sub_limb_in_place(&mut xs, 789), false);
/// assert_eq!(xs, &[4294966630, 455]);
///
/// let mut xs = vec![1];
/// assert_eq!(limbs_sub_limb_in_place(&mut xs, 2), true);
/// assert_eq!(xs, &[u32::MAX]);
/// ```
///
/// This is mpn_add_1 from gmp.h, GMP 6.1.2, where the result is written to the input slice.
pub fn limbs_sub_limb_in_place(xs: &mut [Limb], mut y: Limb) -> bool {
    for x in xs.iter_mut() {
        if x.overflowing_sub_assign(y) {
            y = 1;
        } else {
            return false;
        }
    }
    y != 0
}

fn sub_and_borrow(x: Limb, y: Limb, borrow: &mut bool) -> Limb {
    let (mut diff, overflow) = x.overflowing_sub(y);
    if *borrow {
        *borrow = overflow;
        *borrow |= diff.overflowing_sub_assign(1);
    } else {
        *borrow = overflow;
    }
    diff
}

/// Interpreting a two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second from the first. Returns a pair consisting of the limbs of the result, and
/// whether there was a borrow left over; that is, whether the second `Natural` was greater than the
/// first `Natural`. The first slice must be at least as long as the second.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub;
///
/// assert_eq!(limbs_sub(&[123, 456], &[789]), (vec![4294966630, 455], false));
/// assert_eq!(limbs_sub(&[123, 456], &[456, 789]), (vec![4294966963, 4294966962], true));
/// ```
///
/// This is mpn_sub from gmp.h, GMP 6.1.2, where the output is returned.
pub fn limbs_sub(xs: &[Limb], ys: &[Limb]) -> (Vec<Limb>, bool) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let mut out = Vec::with_capacity(xs_len);
    let mut borrow = false;
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        out.push(sub_and_borrow(x, y, &mut borrow));
    }
    if xs_len != ys_len {
        out.extend_from_slice(&xs[ys_len..]);
        if borrow {
            borrow = limbs_sub_limb_in_place(&mut out[ys_len..], 1);
        }
    }
    (out, borrow)
}

/// Interpreting a two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
/// an output slice. Returns whether there was a borrow left over; that is, whether the second
/// `Natural` was greater than the first `Natural`. The output slice must be at least as long as
/// either input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs` or if `xs` and `ys` have different lengths.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_same_length_to_out(&mut out, &[123, 456], &[789, 123]), false);
/// assert_eq!(out, &[4294966630, 332, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_same_length_to_out(&mut out, &[123, 456], &[456, 789]), true);
/// assert_eq!(out, &[4294966963, 4294966962, 0]);
/// ```
///
/// This is mpn_sub_n from gmp.h, GMP 6.1.2.
pub fn limbs_sub_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    let mut borrow = false;
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        *out = sub_and_borrow(x, y, &mut borrow);
    }
    borrow
}

/// Interpreting a two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second from the first, writing the `xs.len()` limbs of the result to an output
/// slice. Returns whether there was a borrow left over; that is, whether the second `Natural` was
/// greater than the first `Natural`. The output slice must be at least as long as the first input
/// slice and the first input slice must be at least as long as the second.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs` or if `xs` is shorter than `ys`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_to_out(&mut out, &[123, 456], &[789]), false);
/// assert_eq!(out, &[4294966630, 455, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_to_out(&mut out, &[123, 456], &[456, 789]), true);
/// assert_eq!(out, &[4294966963, 4294966962, 0]);
/// ```
///
/// This is mpn_sub from gmp.h, GMP 6.1.2.
pub fn limbs_sub_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(out.len() >= xs_len);
    let (xs_lo, xs_hi) = xs.split_at(ys_len);
    let borrow = limbs_sub_same_length_to_out(out, xs_lo, ys);
    if xs_len == ys_len {
        borrow
    } else if borrow {
        limbs_sub_limb_to_out(&mut out[ys_len..], xs_hi, 1)
    } else {
        out[ys_len..xs_len].copy_from_slice(xs_hi);
        false
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
/// the first (left) slice. Returns whether there was a borrow left over; that is, whether the
/// second `Natural` was greater than the first `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_same_length_in_place_left(xs, &[789, 123]), false);
/// assert_eq!(xs, &[4294966630, 332]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_same_length_in_place_left(xs, &[456, 789]), true);
/// assert_eq!(xs, &[4294966963, 4294966962]);
/// ```
///
/// This is mpn_sub_n from gmp.h, GMP 6.1.2, where the output is written to the first input.
pub fn limbs_sub_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    assert_eq!(xs.len(), ys.len());
    let mut borrow = false;
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        *x = sub_and_borrow(*x, y, &mut borrow);
    }
    borrow
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
/// subtracts the second from the first, writing the `xs.len()` limbs of the result to the first
/// (left) slice. Returns whether there was a borrow left over; that is, whether the second
/// `Natural` was greater than the first `Natural`. The first slice must be at least as long as the
/// second.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_in_place_left;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_in_place_left(xs, &[789]), false);
/// assert_eq!(xs, &[4294966630, 455]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_in_place_left(xs, &[456, 789]), true);
/// assert_eq!(xs, &[4294966963, 4294966962]);
/// ```
///
/// This is mpn_sub from gmp.h, GMP 6.1.2, where the output is written to the first input.
pub fn limbs_sub_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let (xs_lo, xs_hi) = xs.split_at_mut(ys_len);
    let borrow = limbs_sub_same_length_in_place_left(xs_lo, ys);
    if xs_len == ys_len {
        borrow
    } else if borrow {
        limbs_sub_limb_in_place(xs_hi, 1)
    } else {
        false
    }
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
/// the second (right) slice. Returns whether there was a borrow left over; that is, whether the
/// second `Natural` was greater than the first `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_in_place_right;
///
/// let ys = &mut [789, 123];
/// assert_eq!(limbs_sub_same_length_in_place_right(&[123, 456], ys), false);
/// assert_eq!(ys, &[4294966630, 332]);
///
/// let ys = &mut [456, 789];
/// assert_eq!(limbs_sub_same_length_in_place_right(&[123, 456], ys), true);
/// assert_eq!(ys, &[4294966963, 4294966962]);
/// ```
///
/// This is mpn_sub_n from gmp.h, GMP 6.1.2, where the output is written to the second input.
pub fn limbs_sub_same_length_in_place_right(xs: &[Limb], ys: &mut [Limb]) -> bool {
    assert_eq!(xs.len(), ys.len());
    let mut borrow = false;
    for (&x, y) in xs.iter().zip(ys.iter_mut()) {
        *y = sub_and_borrow(x, *y, &mut borrow);
    }
    borrow
}

/// Given two equal-length slices `xs` and `ys`, computes the difference between the `Natural`s
/// whose limbs are `xs` and `&ys[..len]`, and writes the limbs of the result to `ys`. Returns
/// whether there was a borrow left over; that is, whether the second `Natural` was greater than the
/// first `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths or if `len` is greater than `xs.len()`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_slice_sub_in_place_right;
///
/// let ys = &mut [789, 123];
/// assert_eq!(limbs_slice_sub_in_place_right(&[123, 456], ys, 2), false);
/// assert_eq!(ys, &[4294966630, 332]);
///
/// let ys = &mut [789, 123];
/// assert_eq!(limbs_slice_sub_in_place_right(&[123, 456], ys, 1), false);
/// assert_eq!(ys, &[4294966630, 455]);
/// ```
///
/// This is mpn_sub_n from gmp.h, GMP 6.1.2, where the output is written to the second input (which
/// has `len` limbs) and the second input has enough space past `len` to accomodate the output.
pub fn limbs_slice_sub_in_place_right(xs: &[Limb], ys: &mut [Limb], len: usize) -> bool {
    let xs_len = xs.len();
    assert_eq!(xs_len, ys.len());
    let (xs_lo, xs_hi) = xs.split_at(len);
    let (ys_lo, ys_hi) = ys.split_at_mut(len);
    let borrow = limbs_sub_same_length_in_place_right(xs_lo, ys_lo);
    if xs_len == len {
        borrow
    } else if borrow {
        limbs_sub_limb_to_out(ys_hi, xs_hi, 1)
    } else {
        ys_hi.copy_from_slice(xs_hi);
        false
    }
}

/// Interpreting a of `Limb`s and a `Vec` of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, writing the `xs.len()` limbs of the result to
/// the `Vec`, possibly extending the `Vec`'s length. Returns whether there was a borrow left over;
/// that is, whether the second `Natural` was greater than the first `Natural`. The first slice must
/// be at least as long as the second.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len()`, m = `xs.len()` - `ys.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_vec_sub_in_place_right;
///
/// let mut ys = vec![789];
/// assert_eq!(limbs_vec_sub_in_place_right(&[123, 456], &mut ys), false);
/// assert_eq!(ys, &[4294966630, 455]);
///
/// let mut ys = vec![456, 789];
/// assert_eq!(limbs_vec_sub_in_place_right(&[123, 456], &mut ys), true);
/// assert_eq!(ys, &[4294966963, 4294966962]);
/// ```
pub fn limbs_vec_sub_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let (xs_lo, xs_hi) = xs.split_at(ys_len);
    let borrow = limbs_sub_same_length_in_place_right(xs_lo, ys);
    if xs_len == ys_len {
        borrow
    } else {
        ys.extend_from_slice(xs_hi);
        if borrow {
            limbs_sub_limb_in_place(&mut ys[ys_len..], 1)
        } else {
            false
        }
    }
}

/// Given a slice `xs`, computes the difference between the `Natural`s whose limbs are
/// `&xs[..xs.len() - right_start]` and `&xs[right_start..]`, and writes the limbs of the result to
/// `&xs[..xs.len() - right_start]`. Returns whether there was a borrow left over; that is, whether
/// the second `Natural` was greater than the first `Natural`. As implied by the name, the input
/// slices may overlap.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` - `right_start`
///
/// # Panics
/// Panics if `right_start` is greater than `xs.len()`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_in_place_with_overlap;
///
/// let xs: &mut [u32] = &mut [4, 3, 2, 1];
/// assert_eq!(limbs_sub_same_length_in_place_with_overlap(xs, 1), false);
/// assert_eq!(xs, &[1, 1, 1, 1]);
///
/// let xs: &mut [u32] = &mut [4, 3, 2, 1];
/// assert_eq!(limbs_sub_same_length_in_place_with_overlap(xs, 3), false);
/// assert_eq!(xs, &[3, 3, 2, 1]);
/// ```
///
/// This is mpn_sub_n from gmp.h, GMP 6.1.2, where the output is written to the first input, and the
/// two inputs are possibly-overlapping subslices of a single slice.
pub fn limbs_sub_same_length_in_place_with_overlap(xs: &mut [Limb], right_start: usize) -> bool {
    let len = xs.len() - right_start;
    let mut borrow = false;
    for i in 0..len {
        xs[i] = sub_and_borrow(xs[i], xs[i + right_start], &mut borrow);
    }
    borrow
}

/// Given two slices `xs` and `ys`, computes the difference between the `Natural`s whose limbs are
/// `&xs[xs.len() - ys.len()..]` and `&ys`, and writes the limbs of the result to `&xs[..ys.len()]`.
/// Returns whether there was a borrow left over; that is, whether the second `Natural` was greater
/// than the first `Natural`. As implied by the name, the input and output ranges may overlap. `xs`
/// must be at least as long as `ys`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `ys.len()`
///
/// # Panics
/// Panics if `xs.len()` is shorter than `ys.len()`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_to_out_with_overlap;
///
/// let xs = &mut [1, 2, 3, 4];
/// assert_eq!(limbs_sub_same_length_to_out_with_overlap(xs, &[2, 2, 2]), false);
/// assert_eq!(xs, &[0, 1, 2, 4]);
/// ```
///
/// This is mpn_sub_n from gmp.h, GMP 6.1.2, where the output is a prefix of a slice and the left
/// operand of the subtraction is a suffix of the same slice, and the prefix and suffix may overlap.
pub fn limbs_sub_same_length_to_out_with_overlap(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let right_start = xs_len - ys_len;
    let mut borrow = false;
    for i in 0..ys_len {
        xs[i] = sub_and_borrow(xs[i + right_start], ys[i], &mut borrow);
    }
    borrow
}

/// Interpreting a two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, and then subtracts a borrow (`false` is 0,
/// `true` is 1), writing the `xs.len()` limbs of the result to an output slice. Returns whether
/// there was a borrow left over. The output slice must be at least as long as either input slice.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs` or if `xs` and `ys` have different lengths.
///
/// This is mpn_sub_nc from gmp-impl.h, GMP 6.1.2, where rp, up, and vp are disjoint.
pub fn _limbs_sub_same_length_with_borrow_in_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    borrow_in: bool,
) -> bool {
    let mut borrow = limbs_sub_same_length_to_out(out, xs, ys);
    if borrow_in {
        borrow |= limbs_sub_limb_in_place(&mut out[..xs.len()], 1);
    }
    borrow
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, and then subtracts a borrow (`false` is 0,
/// `true` is 1), writing the `xs.len()` limbs of the result to the first (left) slice. Return
/// whether there was a borrow left over.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// This is mpn_sub_nc from gmp-impl.h, GMP 6.1.2, where rp is the same as up.
pub fn _limbs_sub_same_length_with_borrow_in_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    borrow_in: bool,
) -> bool {
    let mut borrow = limbs_sub_same_length_in_place_left(xs, ys);
    if borrow_in {
        borrow |= limbs_sub_limb_in_place(xs, 1);
    }
    borrow
}

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, subtracts the second from the first, and then subtracts a borrow (`false` is 0,
/// `true` is 1), writing the `xs.len()` limbs of the result to the second (right) slice. Returns
/// whether there was a borrow left over.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// This is mpn_sub_nc from gmp-impl.h, GMP 6.1.2, where rp is the same as vp.
pub fn _limbs_sub_same_length_with_borrow_in_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    borrow_in: bool,
) -> bool {
    let mut borrow = limbs_sub_same_length_in_place_right(xs, ys);
    if borrow_in {
        borrow |= limbs_sub_limb_in_place(ys, 1);
    }
    borrow
}

fn sub_panic<S: Display, T: Display>(x: S, y: T) -> ! {
    panic!(
        "Cannot subtract a number from a smaller number. self: {}, other: {}",
        x, y
    );
}

impl Natural {
    pub(crate) fn sub_limb(self, other: Limb) -> Natural {
        self.checked_sub_limb(other)
            .expect("Cannot subtract a Limb from a smaller Natural")
    }

    pub(crate) fn sub_limb_ref(&self, other: Limb) -> Natural {
        self.checked_sub_limb_ref(other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }
}

impl Sub<Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((Natural::from(123u32) - Natural::ZERO).to_string(), "123");
    /// assert_eq!((Natural::from(456u32) - Natural::from(123u32)).to_string(), "333");
    /// assert_eq!(
    ///     (Natural::trillion() * Natural::from(3u32) - Natural::trillion()).to_string(),
    ///     "2000000000000"
    /// );
    /// ```
    fn sub(self, other: Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

impl<'a> Sub<&'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by value and the right
    /// `Natural` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((Natural::from(123u32) - &Natural::ZERO).to_string(), "123");
    /// assert_eq!((Natural::from(456u32) - &Natural::from(123u32)).to_string(), "333");
    /// assert_eq!(
    ///     (Natural::trillion() * Natural::from(3u32) - &Natural::trillion()).to_string(),
    ///     "2000000000000"
    /// );
    /// ```
    fn sub(self, other: &'a Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

impl<'a> Sub<Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by reference and the right
    /// `Natural` by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(123u32) - Natural::ZERO).to_string(), "123");
    /// assert_eq!((&Natural::from(456u32) - Natural::from(123u32)).to_string(), "333");
    /// assert_eq!(
    ///     (&(Natural::trillion() * Natural::from(3u32)) - Natural::trillion()).to_string(),
    ///     "2000000000000"
    /// );
    /// ```
    fn sub(self, other: Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

impl<'a, 'b> Sub<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(123u32) - &Natural::ZERO).to_string(), "123");
    /// assert_eq!((&Natural::from(456u32) - &Natural::from(123u32)).to_string(), "333");
    /// assert_eq!(
    ///     (&(Natural::trillion() * Natural::from(3u32)) - &Natural::trillion()).to_string(),
    ///     "2000000000000"
    /// );
    /// ```
    fn sub(self, other: &'a Natural) -> Natural {
        self.checked_sub(other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }
}

impl SubAssign<Natural> for Natural {
    /// Subtracts a `Natural` from a `Natural` in place, taking the `Natural` on the RHS by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::trillion() * Natural::from(10u32);
    /// x -= Natural::trillion();
    /// x -= (Natural::trillion() * Natural::from(2u32));
    /// x -= (Natural::trillion() * Natural::from(3u32));
    /// x -= (Natural::trillion() * Natural::from(4u32));
    /// assert_eq!(x.to_string(), "0");
    /// ```
    fn sub_assign(&mut self, other: Natural) {
        if self.sub_assign_no_panic(other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
        }
    }
}

impl<'a> SubAssign<&'a Natural> for Natural {
    /// Subtracts a `Natural` from a `Natural` in place, taking the `Natural` on the RHS by
    /// reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::trillion() * Natural::from(10u32);
    /// x -= &Natural::trillion();
    /// x -= &(Natural::trillion() * Natural::from(2u32));
    /// x -= &(Natural::trillion() * Natural::from(3u32));
    /// x -= &(Natural::trillion() * Natural::from(4u32));
    /// assert_eq!(x.to_string(), "0");
    /// ```
    fn sub_assign(&mut self, other: &'a Natural) {
        if self.sub_assign_ref_no_panic(other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
        }
    }
}
