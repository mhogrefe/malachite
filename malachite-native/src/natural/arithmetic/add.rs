use natural::Natural::{self, Large, Small};
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
        if other == 0 {
            return;
        }
        if let Small(y) = other {
            *self += y;
        } else if let Small(x) = *self {
            other += x;
            *self = other;
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

fn large_add(xs: &mut Vec<u32>, ys: &[u32]) {
    let mut carry = false;
    let mut past_xs = false;
    let xs_len = xs.len();
    let ys_len = ys.len();
    for (i, y) in ys.iter().enumerate() {
        if !past_xs && i == xs_len {
            past_xs = true;
        }
        if past_xs {
            if carry {
                let (sum, overflow) = y.overflowing_add(1);
                xs.push(sum);
                carry = overflow;
            } else {
                xs.push(*y);
            }
        } else {
            let (sum, overflow) = xs[i].overflowing_add(*y);
            if carry {
                let (sum, overflow) = sum.overflowing_add(1);
                xs[i] = sum;
                carry = overflow;
            } else {
                xs[i] = sum;
                carry = overflow;
            }
        }
    }
    if carry && xs_len > ys_len {
        for x in xs.iter_mut().take(xs_len).skip(ys_len) {
            let (sum, overflow) = x.overflowing_add(1);
            *x = sum;
            if !overflow {
                break;
            }
        }
    }
}
