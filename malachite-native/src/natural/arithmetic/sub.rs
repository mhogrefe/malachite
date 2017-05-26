use natural::Natural::{self, Large, Small};
use std::ops::{Sub, SubAssign};

/// Subtracts a `Natural` from a `Natural`, taking ownership of both `Natural`s.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", Natural::from(0u32) - Natural::from(123u32)), "None");
/// assert_eq!(format!("{:?}", Natural::from(123u32) - Natural::from(0u32)), "Some(123)");
/// assert_eq!(format!("{:?}", Natural::from(456u32) - Natural::from(123u32)), "Some(333)");
/// assert_eq!(format!("{:?}", Natural::from_str("3000000000000").unwrap() -
///                            Natural::from_str("1000000000000").unwrap()), "Some(2000000000000)");
/// ```
impl Sub<Natural> for Natural {
    type Output = Option<Natural>;

    fn sub(mut self, other: Natural) -> Option<Natural> {
        if self >= other {
            self -= other;
            Some(self)
        } else {
            None
        }
    }
}

/// Subtracts a `Natural` from a `Natural` in place.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::from_str("10000000000000").unwrap();
/// x -= Natural::from_str("1000000000000").unwrap();
/// x -= Natural::from_str("2000000000000").unwrap();
/// x -= Natural::from_str("3000000000000").unwrap();
/// x -= Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "0");
/// ```
impl SubAssign<Natural> for Natural {
    fn sub_assign(&mut self, other: Natural) {
        if other == 0 {
            return;
        }
        let mut panic = false;
        if let Small(y) = other {
            *self -= y;
        } else if let Small(_) = *self {
            panic = true;
        } else if *self < other {
            panic = true;
        } else {
            match (&mut (*self), &other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    panic = !large_sub(xs, ys);
                }
                _ => unreachable!(),
            }
        }
        if panic {
            panic!("Cannot subtract a u32 from a smaller Natural. self: {}, other: {}",
                   *self,
                   other);
        }
        self.trim();
    }
}

fn large_sub(xs: &mut Vec<u32>, ys: &[u32]) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if ys_len > xs_len {
        return false;
    }
    let mut borrow = false;
    for (i, y) in ys.iter().enumerate() {
        if borrow {
            let (difference, overflow) = xs[i].overflowing_sub(1);
            borrow = overflow;
            let (difference, overflow) = difference.overflowing_sub(*y);
            xs[i] = difference;
            borrow |= overflow;
        } else {
            let (difference, overflow) = xs[i].overflowing_sub(*y);
            xs[i] = difference;
            borrow = overflow;
        }
    }
    if borrow && xs_len > ys_len {
        for x in xs.iter_mut().skip(ys_len) {
            let (difference, overflow) = x.overflowing_sub(1);
            *x = difference;
            if !overflow {
                borrow = false;
                break;
            }
        }
    }
    !borrow
}
