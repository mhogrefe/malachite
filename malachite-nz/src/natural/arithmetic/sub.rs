use malachite_base::num::CheckedSub;
use natural::arithmetic::sub_u32::{limbs_sub_limb_in_place, limbs_sub_limb_to_out};
use natural::Natural;
use std::fmt::Display;
use std::ops::{Sub, SubAssign};

fn sub_and_borrow(x: u32, y: u32, borrow: &mut bool) -> u32 {
    let (difference, overflow) = x.overflowing_sub(y);
    if *borrow {
        *borrow = overflow;
        let (difference, overflow) = difference.overflowing_sub(1);
        *borrow |= overflow;
        difference
    } else {
        *borrow = overflow;
        difference
    }
}

pub fn limbs_sub(xs: &[u32], ys: &[u32]) -> (Vec<u32>, bool) {
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

// Subtract s2 from s1 (which must both have length n), and write the n least significant limbs of
// the result to r. Return borrow. r must have size at least n.
pub fn limbs_sub_same_length_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    assert_eq!(xs_len, ys.len());
    assert!(out_limbs.len() >= xs_len);
    let mut borrow = false;
    for i in 0..xs_len {
        out_limbs[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
    }
    borrow
}

// Subtract s2 from s1, and write the s1.len() least significant limbs of the result to r. Return
// borrow. This function requires that s1.len() >= s2.len() and r.len() >= s1.len().
pub fn limbs_sub_to_out(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert!(out_limbs.len() >= xs_len);
    let borrow = limbs_sub_same_length_to_out(out_limbs, &xs[..ys_len], ys);
    if xs_len == ys_len {
        borrow
    } else if borrow {
        limbs_sub_limb_to_out(&mut out_limbs[ys_len..], &xs[ys_len..], 1)
    } else {
        out_limbs[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
        false
    }
}

// Subtract s2 from s1 (which must both have length n), and write the n least significant limbs of
// the result to s1. Return borrow.
pub fn limbs_sub_same_length_in_place_left(xs: &mut [u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    assert_eq!(xs_len, ys.len());
    let mut borrow = false;
    for i in 0..xs_len {
        xs[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
    }
    borrow
}

// Subtract s2 from s1, and write the s1.len() least significant limbs of the result to s1. Return
// borrow. This function requires that s1.len() >= s2.len().
pub fn limbs_sub_in_place_left(xs: &mut [u32], ys: &[u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let borrow = limbs_sub_same_length_in_place_left(&mut xs[..ys_len], ys);
    if xs_len == ys_len {
        borrow
    } else if borrow {
        limbs_sub_limb_in_place(&mut xs[ys_len..], 1)
    } else {
        false
    }
}

pub fn limbs_sub_same_length_in_place_right(xs: &[u32], ys: &mut [u32]) -> bool {
    let ys_len = ys.len();
    assert_eq!(xs.len(), ys_len);
    let mut borrow = false;
    for i in 0..ys_len {
        ys[i] = sub_and_borrow(xs[i], ys[i], &mut borrow);
    }
    borrow
}

pub fn limbs_sub_in_place_right(xs: &[u32], ys: &mut Vec<u32>) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let borrow = limbs_sub_same_length_in_place_right(&xs[..ys_len], ys);
    if xs_len == ys_len {
        borrow
    } else {
        ys.extend_from_slice(&xs[ys_len..]);
        if borrow {
            limbs_sub_limb_in_place(&mut ys[ys_len..], 1)
        } else {
            false
        }
    }
}

fn sub_panic<S: Display, T: Display>(x: S, y: T) {
    panic!(
        "Cannot subtract a Natural from a smaller Natural. self: {}, other: {}",
        x, y
    );
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
/// use malachite_base::num::Zero;
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
/// use malachite_base::num::Zero;
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
            unreachable!();
        })
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
        if self.sub_assign_no_panic(other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
        }
    }
}
