use malachite_base::traits::Assign;
use natural::Natural::{self, Large, Small};
use std::ops::{Add, AddAssign};

// Add s1 and s2limb, and write the s1.len() least significant limbs of the result to r. Return
// carry. r must have size at least s1.len().
pub fn mpn_add_1(r: &mut [u32], s1: &[u32], mut s2limb: u32) -> bool {
    let s1_len = s1.len();
    assert!(r.len() >= s1_len);
    for i in 0..s1_len {
        let (sum, overflow) = s1[i].overflowing_add(s2limb);
        r[i] = sum;
        if overflow {
            s2limb = 1;
        } else {
            s2limb = 0;
            let copy_index = i + 1;
            r[copy_index..s1_len].copy_from_slice(&s1[copy_index..]);
            break;
        }
    }
    s2limb != 0
}

// Add s1 and s2limb, and write the s1.len() least significant limbs of the result to s1. Return
// carry.
pub fn mpn_add_1_in_place(s1: &mut [u32], mut s2limb: u32) -> bool {
    for limb in s1.iter_mut() {
        let (sum, overflow) = limb.overflowing_add(s2limb);
        *limb = sum;
        if overflow {
            s2limb = 1;
        } else {
            return false;
        }
    }
    true
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
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
            Large(ref limbs) => {
                let mut sum_limbs = vec![0; limbs.len()];
                if mpn_add_1(&mut sum_limbs[..], limbs, other) {
                    sum_limbs.push(1);
                }
                Large(sum_limbs)
            }
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
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
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
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
            if mpn_add_1_in_place(&mut limbs[..], other) {
                limbs.push(1);
            }
        });
    }
}
