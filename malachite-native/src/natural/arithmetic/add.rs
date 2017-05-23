use natural::Natural::{self, Large, Small};
use std::mem::swap;
use std::ops::{Add, AddAssign};

/// Adds a `Natural` to a `Natural`, taking ownership of both `Natural`s.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0) + Natural::from(123)).to_string(), "123");
/// assert_eq!((Natural::from(123) + Natural::from(0)).to_string(), "123");
/// assert_eq!((Natural::from(123) + Natural::from(456)).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl Add<Natural> for Natural {
    type Output = Natural;

    fn add(mut self, other: Natural) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `Natural` in place.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::new();
/// x += Natural::from_str("1000000000000").unwrap();
/// x += Natural::from_str("2000000000000").unwrap();
/// x += Natural::from_str("3000000000000").unwrap();
/// x += Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "10000000000000");
/// ```
impl AddAssign<Natural> for Natural {
    fn add_assign(&mut self, mut other: Natural) {
        if self.limb_count() < other.limb_count() {
            swap(self, &mut other);
        }
        if other == 0 {
            return;
        }
        if let Small(y) = other {
            *self += y;
        } else {
            match (self, other) {
                (&mut Large(ref mut xs), Large(ref ys)) => {
                    large_add(xs, ys);
                }
                _ => unreachable!(),
            }
        }
    }
}

// assumes that xs.len() >= ys.len()
fn large_add(xs: &mut Vec<u32>, ys: &[u32]) {
    let mut carry = false;
    for (i, x) in xs.iter_mut().enumerate() {
        let (sum, overflow) = x.overflowing_add(*ys.get(i).unwrap_or(&0));
        if carry {
            carry = overflow;
            let (sum, overflow) = sum.overflowing_add(1);
            *x = sum;
            carry |= overflow;
        } else {
            *x = sum;
            carry = overflow;
        }
    }
    if carry {
        xs.push(1);
    }
}
