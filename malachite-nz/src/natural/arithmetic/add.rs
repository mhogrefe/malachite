use natural::arithmetic::add_u32::{mpn_add_1, mpn_add_1_in_place};
use natural::Natural::{self, Large, Small};
use std::mem::swap;
use std::ops::{Add, AddAssign};

fn add_and_carry(x: u32, y: u32, carry: &mut bool) -> u32 {
    let (sum, overflow) = x.overflowing_add(y);
    if *carry {
        *carry = overflow;
        let (sum, overflow) = sum.overflowing_add(1);
        *carry |= overflow;
        sum
    } else {
        *carry = overflow;
        sum
    }
}

// Add s1 and s2 (which must both have length n), and write the n least significant limbs of the
// result to r. Return carry. r must have size at least n.
pub fn mpn_add_n(r: &mut [u32], s1: &[u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    assert_eq!(s1_len, s2.len());
    assert!(r.len() >= s1_len);
    let mut carry = false;
    for i in 0..s1_len {
        r[i] = add_and_carry(s1[i], s2[i], &mut carry);
    }
    carry
}

// Add s1 and s2 (which must both have length n), and write the n least significant limbs of the
// result to s1. Return carry.
pub fn mpn_add_n_in_place(s1: &mut [u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    assert_eq!(s1_len, s2.len());
    let mut carry = false;
    for i in 0..s1_len {
        s1[i] = add_and_carry(s1[i], s2[i], &mut carry);
    }
    carry
}

// Add s1 and s2, and write the s1.len() least significant limbs of the result to r. Return carry.
// This function requires that s1.len() >= s2.len() and r.len() >= s1.len().
pub fn mpn_add(r: &mut [u32], s1: &[u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    let s2_len = s2.len();
    assert!(s1_len >= s2_len);
    assert!(r.len() >= s1_len);
    let carry = mpn_add_n(r, &s1[0..s2_len], s2);
    if s1_len == s2_len {
        carry
    } else if carry {
        mpn_add_1(&mut r[s2_len..], &s1[s2_len..], 1)
    } else {
        r[s2_len..s1_len].copy_from_slice(&s1[s2_len..]);
        false
    }
}

// Add s1 and s2, and write the s1.len() least significant limbs of the result to s1. Return carry.
// This function requires that s1.len() >= s2.len().
pub fn mpn_add_in_place(s1: &mut [u32], s2: &[u32]) -> bool {
    let s1_len = s1.len();
    let s2_len = s2.len();
    assert!(s1_len >= s2_len);
    let carry = mpn_add_n_in_place(&mut s1[0..s2_len], s2);
    if s1_len == s2_len {
        carry
    } else if carry {
        mpn_add_1_in_place(&mut s1[s2_len..], 1)
    } else {
        false
    }
}

/// Adds a `Natural` to a `Natural`, taking both `Natural`s by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO + Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) + Natural::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
///     assert_eq!((Natural::trillion() + Natural::trillion() * 2).to_string(), "3000000000000");
/// }
/// ```
impl Add<Natural> for Natural {
    type Output = Natural;

    fn add(mut self, other: Natural) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `Natural`, taking the left `Natural` by value and the right `Natural` by
/// reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO + &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) +&Natural::ZERO).to_string(), "123");
///     assert_eq!((Natural::from(123u32) +&Natural::from(456u32)).to_string(), "579");
///     assert_eq!((Natural::trillion() + &(Natural::trillion() * 2)).to_string(), "3000000000000");
/// }
/// ```
impl<'a> Add<&'a Natural> for Natural {
    type Output = Natural;

    fn add(mut self, other: &'a Natural) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `Natural`, taking the left `Natural` by reference and the right `Natural`
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO + Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
///     assert_eq!((&Natural::trillion() + Natural::trillion() * 2).to_string(), "3000000000000");
/// }
/// ```
impl<'a> Add<Natural> for &'a Natural {
    type Output = Natural;

    fn add(self, mut other: Natural) -> Natural {
        other += self;
        other
    }
}

// xs.len() >= ys.len()
fn add_helper(xs: &[u32], ys: &[u32]) -> Natural {
    let mut sum_limbs = vec![0; xs.len()];
    if mpn_add(&mut sum_limbs, xs, ys) {
        sum_limbs.push(1);
    }
    Large(sum_limbs)
}

/// Adds a `Natural` to a `Natural`, taking both `Natural`s by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO + &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + &Natural::ZERO).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) + &Natural::from(456u32)).to_string(), "579");
///     assert_eq!((&Natural::trillion() + &(Natural::trillion() * 2)).to_string(),
///         "3000000000000");
/// }
/// ```
impl<'a, 'b> Add<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn add(self, other: &'a Natural) -> Natural {
        if self as *const Natural == other as *const Natural {
            self << 1
        } else {
            match (self, other) {
                (&Small(0), _) => other.clone(),
                (_, &Small(0)) => self.clone(),
                (x, &Small(y)) => x + y,
                (&Small(x), y) => x + y,
                (&Large(ref xs), &Large(ref ys)) => {
                    if xs.len() >= ys.len() {
                        add_helper(xs, ys)
                    } else {
                        add_helper(ys, xs)
                    }
                }
            }
        }
    }
}

/// Adds a `Natural` to a `Natural` in place, taking the `Natural` on the RHS by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), other.significant_bits)`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ZERO;
///     x += Natural::trillion();
///     x += Natural::trillion() * 2;
///     x += Natural::trillion() * 3;
///     x += Natural::trillion() * 4;
///     assert_eq!(x.to_string(), "10000000000000");
/// }
/// ```
impl AddAssign<Natural> for Natural {
    fn add_assign(&mut self, mut other: Natural) {
        if *self == 0 {
            *self = other;
        } else if other != 0 {
            if self.limb_count() < other.limb_count() {
                swap(self, &mut other);
            }
            match other {
                Small(y) => *self += y,
                Large(ref ys) => match *self {
                    Large(ref mut xs) => {
                        if mpn_add_in_place(xs, ys) {
                            xs.push(1);
                        }
                    }
                    _ => unreachable!(),
                },
            }
        }
    }
}

/// Adds a `Natural` to a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::traits::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ZERO;
///     x += &Natural::trillion();
///     x += &(Natural::trillion() * 2);
///     x += &(Natural::trillion() * 3);
///     x += &(Natural::trillion() * 4);
///     assert_eq!(x.to_string(), "10000000000000");
/// }
/// ```
impl<'a> AddAssign<&'a Natural> for Natural {
    fn add_assign(&mut self, other: &'a Natural) {
        if *self == 0 {
            self.clone_from(other);
        } else if self as *const Natural == other as *const Natural {
            *self <<= 1;
        } else if *other != 0 {
            match *other {
                Small(y) => *self += y,
                Large(ref ys) => match *self {
                    Small(x) => *self = other + x,
                    Large(ref mut xs) => {
                        let ys_len = ys.len();
                        if xs.len() < ys_len {
                            xs.resize(ys_len, 0);
                        }
                        if mpn_add_in_place(xs, ys) {
                            xs.push(1);
                        }
                    }
                },
            }
        }
    }
}
