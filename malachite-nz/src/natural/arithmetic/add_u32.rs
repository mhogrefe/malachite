use malachite_base::num::Assign;
use natural::Natural::{self, Large, Small};
use std::ops::{Add, AddAssign};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the sum of the `Natural` and a `u32`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_u32::limbs_add_limb;
///
/// assert_eq!(limbs_add_limb(&[123, 456], 789), &[912, 456]);
/// assert_eq!(limbs_add_limb(&[0xffff_ffff, 5], 2), &[1, 6]);
/// assert_eq!(limbs_add_limb(&[0xffff_ffff], 2), &[1, 1]);
/// ```
pub fn limbs_add_limb(limbs: &[u32], mut limb: u32) -> Vec<u32> {
    let len = limbs.len();
    let mut result_limbs = Vec::with_capacity(len);
    for i in 0..len {
        let (sum, overflow) = limbs[i].overflowing_add(limb);
        result_limbs.push(sum);
        if overflow {
            limb = 1;
        } else {
            limb = 0;
            result_limbs.extend_from_slice(&limbs[i + 1..]);
            break;
        }
    }
    if limb != 0 {
        result_limbs.push(limb);
    }
    result_limbs
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the sum of the `Natural` and a `u32` to an output slice. The output slice must be at
/// least as long as the input slice. Returns whether there is a carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_u32::limbs_add_limb_to_out;
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_add_limb_to_out(&mut out_limbs, &[123, 456], 789), false);
/// assert_eq!(out_limbs, &[912, 456, 0]);
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_add_limb_to_out(&mut out_limbs, &[0xffff_ffff], 2), true);
/// assert_eq!(out_limbs, &[1, 0, 0]);
/// ```
pub fn limbs_add_limb_to_out(out_limbs: &mut [u32], in_limbs: &[u32], mut limb: u32) -> bool {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    for i in 0..len {
        let (sum, overflow) = in_limbs[i].overflowing_add(limb);
        out_limbs[i] = sum;
        if overflow {
            limb = 1;
        } else {
            limb = 0;
            let copy_index = i + 1;
            out_limbs[copy_index..len].copy_from_slice(&in_limbs[copy_index..]);
            break;
        }
    }
    limb != 0
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the sum of the `Natural` and a `u32` to the input slice. Returns whether there is a
/// carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_u32::limbs_slice_add_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_add_limb_in_place(&mut limbs, 789), false);
/// assert_eq!(limbs, &[912, 456]);
///
/// let mut limbs = vec![0xffff_ffff];
/// assert_eq!(limbs_slice_add_limb_in_place(&mut limbs, 2), true);
/// assert_eq!(limbs, &[1]);
/// ```
pub fn limbs_slice_add_limb_in_place(limbs: &mut [u32], mut limb: u32) -> bool {
    for x in limbs.iter_mut() {
        let (sum, overflow) = x.overflowing_add(limb);
        *x = sum;
        if overflow {
            limb = 1;
        } else {
            return false;
        }
    }
    limb != 0
}

/// Interpreting a nonempty `Vec` of `u32`s as the limbs (in ascending order) of a `Natural`, writes
/// the limbs of the sum of the `Natural` and a `u32` to the input `Vec`.
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
/// use malachite_nz::natural::arithmetic::add_u32::limbs_vec_add_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_add_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[912, 456]);
///
/// let mut limbs = vec![0xffff_ffff];
/// limbs_vec_add_limb_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[1, 1]);
/// ```
pub fn limbs_vec_add_limb_in_place(limbs: &mut Vec<u32>, limb: u32) {
    assert!(!limbs.is_empty());
    if limbs_slice_add_limb_in_place(limbs, limb) {
        limbs.push(1);
    }
}

/// Adds a `u32` to a `Natural`, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO + 123).to_string(), "123");
///     assert_eq!((Natural::from(123u32) + 0).to_string(), "123");
///     assert_eq!((Natural::from(123u32) + 456).to_string(), "579");
///     assert_eq!((Natural::trillion() + 123).to_string(), "1000000000123");
/// }
/// ```
impl Add<u32> for Natural {
    type Output = Natural;

    fn add(mut self, other: u32) -> Natural {
        self += other;
        self
    }
}

/// Adds a `u32` to a `Natural`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO + 123).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + 0).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + 456).to_string(), "579");
///     assert_eq!((&Natural::trillion() + 123).to_string(), "1000000000123");
/// }
/// ```
impl<'a> Add<u32> for &'a Natural {
    type Output = Natural;

    fn add(self, other: u32) -> Natural {
        if other == 0 {
            return self.clone();
        }
        match *self {
            Small(small) => match small.overflowing_add(other) {
                (sum, false) => Small(sum),
                (sum, true) => Large(vec![sum, 1]),
            },
            Large(ref limbs) => Large(limbs_add_limb(limbs, other)),
        }
    }
}

/// Adds a `Natural` to a `u32`, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((123 + Natural::ZERO).to_string(), "123");
///     assert_eq!((0 + Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 + Natural::from(123u32)).to_string(), "579");
///     assert_eq!((123 + Natural::trillion()).to_string(), "1000000000123");
/// }
/// ```
impl Add<Natural> for u32 {
    type Output = Natural;

    fn add(self, mut other: Natural) -> Natural {
        other += self;
        other
    }
}

/// Adds a `Natural` to a `u32`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((123 + &Natural::ZERO).to_string(), "123");
///     assert_eq!((0 + &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 + &Natural::from(123u32)).to_string(), "579");
///     assert_eq!((123 + &Natural::trillion()).to_string(), "1000000000123");
/// }
/// ```
impl<'a> Add<&'a Natural> for u32 {
    type Output = Natural;

    fn add(self, other: &'a Natural) -> Natural {
        other + self
    }
}

/// Adds a `u32` to a `Natural` in place.
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
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ZERO;
///     x += 1;
///     x += 2;
///     x += 3;
///     x += 4;
///     assert_eq!(x.to_string(), "10");
/// }
/// ```
impl AddAssign<u32> for Natural {
    fn add_assign(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        if *self == 0 {
            self.assign(other);
            return;
        }
        mutate_with_possible_promotion!(self, small, limbs, { small.checked_add(other) }, {
            limbs_vec_add_limb_in_place(limbs, other);
        });
    }
}
