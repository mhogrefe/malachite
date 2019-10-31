use std::fmt::Display;
use std::ops::{Sub, SubAssign};

use malachite_base::num::arithmetic::traits::{CheckedSub, OverflowingSubAssign};

use natural::arithmetic::sub_limb::{limbs_sub_limb_in_place, limbs_sub_limb_to_out};
use natural::Natural;
use platform::Limb;

fn sub_and_borrow(x: Limb, y: Limb, borrow: &mut bool) -> Limb {
    let (mut difference, overflow) = x.overflowing_sub(y);
    if *borrow {
        *borrow = overflow;
        *borrow |= difference.overflowing_sub_assign(1);
    } else {
        *borrow = overflow;
    }
    difference
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub;
///
/// assert_eq!(limbs_sub(&[123, 456], &[789]), (vec![4_294_966_630, 455], false));
/// assert_eq!(limbs_sub(&[123, 456], &[456, 789]), (vec![4_294_966_963, 4_294_966_962], true));
/// ```
///
/// This is mpn_sub from gmp.h, where the output is returned.
pub fn limbs_sub(xs: &[Limb], ys: &[Limb]) -> (Vec<Limb>, bool) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let mut difference_limbs = Vec::with_capacity(xs_len);
    let mut borrow = false;
    for i in 0..ys_len {
        difference_limbs.push(sub_and_borrow(xs[i], ys[i], &mut borrow));
    }
    if xs_len != ys_len {
        difference_limbs.extend_from_slice(&xs[ys_len..]);
        if borrow {
            borrow = limbs_sub_limb_in_place(&mut difference_limbs[ys_len..], 1);
        }
    }
    (difference_limbs, borrow)
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_same_length_to_out(&mut out, &[123, 456], &[789, 123]), false);
/// assert_eq!(out, &[4_294_966_630, 332, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_same_length_to_out(&mut out, &[123, 456], &[456, 789]), true);
/// assert_eq!(out, &[4_294_966_963, 4_294_966_962, 0]);
/// ```
///
/// This is mpn_sub_n from gmp.h.
pub fn limbs_sub_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    let mut borrow = false;
    for i in 0..len {
        out[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_to_out(&mut out, &[123, 456], &[789]), false);
/// assert_eq!(out, &[4_294_966_630, 455, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_sub_to_out(&mut out, &[123, 456], &[456, 789]), true);
/// assert_eq!(out, &[4_294_966_963, 4_294_966_962, 0]);
/// ```
///
/// This is mpn_sub from gmp.h.
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_same_length_in_place_left(xs, &[789, 123]), false);
/// assert_eq!(xs, &[4_294_966_630, 332]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_same_length_in_place_left(xs, &[456, 789]), true);
/// assert_eq!(xs, &[4_294_966_963, 4_294_966_962]);
/// ```
///
/// This is mpn_sub_n from gmp.h, where the output is written to the first input.
pub fn limbs_sub_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let len = xs.len();
    assert_eq!(len, ys.len());
    let mut borrow = false;
    for i in 0..len {
        xs[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_in_place_left;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_in_place_left(xs, &[789]), false);
/// assert_eq!(xs, &[4_294_966_630, 455]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_in_place_left(xs, &[456, 789]), true);
/// assert_eq!(xs, &[4_294_966_963, 4_294_966_962]);
/// ```
///
/// This is mpn_sub from gmp.h, where the output is written to the first input.
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_in_place_right;
///
/// let ys = &mut [789, 123];
/// assert_eq!(limbs_sub_same_length_in_place_right(&[123, 456], ys), false);
/// assert_eq!(ys, &[4_294_966_630, 332]);
///
/// let ys = &mut [456, 789];
/// assert_eq!(limbs_sub_same_length_in_place_right(&[123, 456], ys), true);
/// assert_eq!(ys, &[4_294_966_963, 4_294_966_962]);
/// ```
///
/// This is mpn_sub_n from gmp.h, where the output is written to the second input.
pub fn limbs_sub_same_length_in_place_right(xs: &[Limb], ys: &mut [Limb]) -> bool {
    let len = ys.len();
    assert_eq!(xs.len(), len);
    let mut borrow = false;
    for i in 0..len {
        ys[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_slice_sub_in_place_right;
///
/// let ys = &mut [789, 123];
/// assert_eq!(limbs_slice_sub_in_place_right(&[123, 456], ys, 2), false);
/// assert_eq!(ys, &[4_294_966_630, 332]);
///
/// let ys = &mut [789, 123];
/// assert_eq!(limbs_slice_sub_in_place_right(&[123, 456], ys, 1), false);
/// assert_eq!(ys, &[4_294_966_630, 455]);
/// ```
///
/// This is mpn_sub_n from gmp.h, where the output is written to the second input (which has `len`
/// limbs) and the second input has enough space past `len` to accomodate the output.
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub::limbs_vec_sub_in_place_right;
///
/// let mut ys = vec![789];
/// assert_eq!(limbs_vec_sub_in_place_right(&[123, 456], &mut ys), false);
/// assert_eq!(ys, &[4_294_966_630, 455]);
///
/// let mut ys = vec![456, 789];
/// assert_eq!(limbs_vec_sub_in_place_right(&[123, 456], &mut ys), true);
/// assert_eq!(ys, &[4_294_966_963, 4_294_966_962]);
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
/// # Example
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
/// This is mpn_sub_n from gmp.h, where the output is written to the first input, and the two inputs
/// are possibly-overlapping subslices of a single slice.
pub fn limbs_sub_same_length_in_place_with_overlap(xs: &mut [Limb], right_start: usize) -> bool {
    let len = xs.len() - right_start;
    let mut borrow = false;
    for i in 0..len {
        xs[i] = sub_and_borrow(xs[i], xs[i + right_start], &mut borrow);
    }
    borrow
}

//TODO test
// `&xs[..xs.len() - right_start]` <- `&xs[right_start..]` - `ys`
pub fn limbs_sub_same_length_to_out_with_overlap(
    xs: &mut [Limb],
    right_start: usize,
    ys: &[Limb],
) -> bool {
    let len = xs.len() - right_start;
    assert_eq!(ys.len(), len);
    let mut borrow = false;
    for i in 0..len {
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
/// This is mpn_sub_nc from gmp-impl.h, where rp and up are disjoint.
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
/// This is mpn_sub_nc from gmp-impl.h, where rp is the same as up.
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
/// This is mpn_sub_nc from gmp-impl.h, where rp is the same as vp.
pub fn _limbs_sub_same_length_with_borrow_in_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    borrow_in: bool,
) -> bool {
    let mut borrow = limbs_sub_same_length_in_place_right(xs, ys);
    if borrow_in {
        borrow |= limbs_sub_limb_in_place(&mut ys[..xs.len()], 1);
    }
    borrow
}

pub(crate) fn sub_panic<S: Display, T: Display>(x: S, y: T) -> ! {
    panic!(
        "Cannot subtract a number from a smaller number. self: {}, other: {}",
        x, y
    );
}

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
/// fn main() {
///     assert_eq!((Natural::from(123u32) - Natural::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(456u32) - Natural::from(123u32)).to_string(), "333");
///     assert_eq!((Natural::trillion() * 3 - Natural::trillion()).to_string(), "2000000000000");
/// }
/// ```
impl Sub<Natural> for Natural {
    type Output = Natural;

    fn sub(self, other: Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

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
/// fn main() {
///     assert_eq!((Natural::from(123u32) - &Natural::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(456u32) - &Natural::from(123u32)).to_string(), "333");
///     assert_eq!((Natural::trillion() * 3 - &Natural::trillion()).to_string(), "2000000000000");
/// }
/// ```
impl<'a> Sub<&'a Natural> for Natural {
    type Output = Natural;

    fn sub(self, other: &'a Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

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
/// fn main() {
///     assert_eq!((&Natural::from(123u32) - Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(456u32) - Natural::from(123u32)).to_string(), "333");
///     assert_eq!((&(Natural::trillion() * 3) - Natural::trillion()).to_string(), "2000000000000");
/// }
/// ```
impl<'a> Sub<Natural> for &'a Natural {
    type Output = Natural;

    fn sub(self, other: Natural) -> Natural {
        self.checked_sub(other)
            .expect("Cannot subtract a Natural from a smaller Natural")
    }
}

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
/// fn main() {
///     assert_eq!((&Natural::from(123u32) - &Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(456u32) - &Natural::from(123u32)).to_string(), "333");
///     assert_eq!((&(Natural::trillion() * 3) - &Natural::trillion()).to_string(),
///         "2000000000000");
/// }
/// ```
impl<'a, 'b> Sub<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn sub(self, other: &'a Natural) -> Natural {
        self.checked_sub(other).unwrap_or_else(|| {
            sub_panic(self, other);
        })
    }
}

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
/// let mut x = Natural::trillion() * 10;
/// x -= Natural::trillion();
/// x -= (Natural::trillion() * 2);
/// x -= (Natural::trillion() * 3);
/// x -= (Natural::trillion() * 4);
/// assert_eq!(x.to_string(), "0");
/// ```
impl SubAssign<Natural> for Natural {
    fn sub_assign(&mut self, other: Natural) {
        if self.sub_assign_no_panic(other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
        }
    }
}

/// Subtracts a `Natural` from a `Natural` in place, taking the `Natural` on the RHS by reference.
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
/// let mut x = Natural::trillion() * 10;
/// x -= &Natural::trillion();
/// x -= &(Natural::trillion() * 2);
/// x -= &(Natural::trillion() * 3);
/// x -= &(Natural::trillion() * 4);
/// assert_eq!(x.to_string(), "0");
/// ```
impl<'a> SubAssign<&'a Natural> for Natural {
    fn sub_assign(&mut self, other: &'a Natural) {
        if self.sub_assign_ref_no_panic(other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
        }
    }
}
