use malachite_base::num::Zero;
use natural::arithmetic::sub_u32::{mpn_sub_1, mpn_sub_1_in_place, sub_assign_u32_helper};
use natural::Natural::{self, Large, Small};
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

// Subtract s2 from s1 (which must both have length n), and write the n least significant limbs of
// the result to r. Return borrow. r must have size at least n.
pub fn mpn_sub_n(r: &mut [u32], s1: &[u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    assert_eq!(s1_len, s2.len());
    assert!(r.len() >= s1_len);
    let mut borrow = false;
    for i in 0..s1_len {
        r[i] = sub_and_borrow(s1[i], s2[i], &mut borrow);
    }
    borrow
}

// Subtract s2 from s1 (which must both have length n), and write the n least significant limbs of
// the result to s1. Return borrow.
pub fn mpn_sub_n_in_place(s1: &mut [u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    assert_eq!(s1_len, s2.len());
    let mut borrow = false;
    for i in 0..s1_len {
        s1[i] = sub_and_borrow(s1[i], s2[i], &mut borrow);
    }
    borrow
}

//TODO docs
// s1 = s2 - s1
pub fn mpn_sub_n_aba(s1: &mut [u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    assert_eq!(s1_len, s2.len());
    let mut borrow = false;
    for i in 0..s1_len {
        s1[i] = sub_and_borrow(s2[i], s1[i], &mut borrow);
    }
    borrow
}

// Subtract s2 from s1, and write the s1.len() least significant limbs of the result to r. Return
// borrow. This function requires that s1.len() >= s2.len() and r.len() >= s1.len().
pub fn mpn_sub(r: &mut [u32], s1: &[u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    let s2_len = s2.len();
    assert!(s1_len >= s2_len);
    assert!(r.len() >= s1_len);
    let borrow = mpn_sub_n(r, &s1[0..s2_len], s2);
    if s1_len == s2_len {
        borrow
    } else if borrow {
        mpn_sub_1(&mut r[s2_len..], &s1[s2_len..], 1)
    } else {
        r[s2_len..s1_len].copy_from_slice(&s1[s2_len..]);
        false
    }
}

// Subtract s2 from s1, and write the s1.len() least significant limbs of the result to s1. Return
// borrow. This function requires that s1.len() >= s2.len().
pub fn mpn_sub_in_place(s1: &mut [u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    let s2_len = s2.len();
    assert!(s1_len >= s2_len);
    let borrow = mpn_sub_n_in_place(&mut s1[0..s2_len], s2);
    if s1_len == s2_len {
        borrow
    } else if borrow {
        mpn_sub_1_in_place(&mut s1[s2_len..], 1)
    } else {
        false
    }
}

//TODO docs
pub fn mpn_sub_aba(a: &mut [u32], b: &[u32], len: usize) -> bool {
    let s1_len = b.len();
    assert!(s1_len >= len);
    assert!(a.len() >= s1_len);
    let borrow = mpn_sub_n_aba(&mut a[0..len], &b[0..len]);
    if s1_len == len {
        borrow
    } else if borrow {
        mpn_sub_1(&mut a[len..], &b[len..], 1)
    } else {
        a[len..s1_len].copy_from_slice(&b[len..]);
        false
    }
}

// x -= y, return borrow
fn sub_assign_helper<'a>(x: &mut Natural, y: &'a Natural) -> bool {
    if *y == 0 {
        false
    } else if x as *const Natural == y as *const Natural {
        *x = Small(0);
        false
    } else if x.limb_count() < y.limb_count() {
        true
    } else if let Small(y) = *y {
        sub_assign_u32_helper(x, y)
    } else {
        match (&mut (*x), y) {
            (&mut Large(ref mut xs), &Large(ref ys)) => {
                if mpn_sub_in_place(xs, ys) {
                    return true;
                }
            }
            _ => unreachable!(),
        }
        x.trim();
        false
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
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", Natural::ZERO - &Natural::from(123u32)), "None");
///     assert_eq!(format!("{:?}", Natural::from(123u32) - &Natural::ZERO), "Some(123)");
///     assert_eq!(format!("{:?}", Natural::from(456u32) - &Natural::from(123u32)), "Some(333)");
///     assert_eq!(format!("{:?}", Natural::trillion() * 3 - &Natural::trillion()),
///         "Some(2000000000000)");
/// }
/// ```
impl<'a> Sub<&'a Natural> for Natural {
    type Output = Option<Natural>;

    fn sub(mut self, other: &'a Natural) -> Option<Natural> {
        if sub_assign_helper(&mut self, other) {
            None
        } else {
            Some(self)
        }
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
///     assert_eq!(format!("{:?}", &Natural::ZERO - &Natural::from(123u32)), "None");
///     assert_eq!(format!("{:?}", &Natural::from(123u32) - &Natural::ZERO), "Some(123)");
///     assert_eq!(format!("{:?}", &Natural::from(456u32) - &Natural::from(123u32)), "Some(333)");
///     assert_eq!(format!("{:?}", &(Natural::trillion() * 3) - &Natural::trillion()),
///         "Some(2000000000000)");
/// }
/// ```
impl<'a, 'b> Sub<&'a Natural> for &'b Natural {
    type Output = Option<Natural>;

    fn sub(self, other: &'a Natural) -> Option<Natural> {
        if self as *const Natural == other as *const Natural {
            Some(Natural::ZERO)
        } else {
            match (self, other) {
                (x, &Small(0)) => Some(x.clone()),
                (x, &Small(y)) => x - y,
                (&Small(_), _) => None,
                (&Large(ref xs), &Large(ref ys)) => {
                    let xs_len = xs.len();
                    if xs_len < ys.len() {
                        None
                    } else {
                        let mut difference_limbs = vec![0; xs_len];
                        if mpn_sub(&mut difference_limbs, xs, ys) {
                            None
                        } else {
                            let mut difference = Large(difference_limbs);
                            difference.trim();
                            Some(difference)
                        }
                    }
                }
            }
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
        if sub_assign_helper(self, other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
        }
        self.trim();
    }
}
