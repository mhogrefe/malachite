use natural::Natural::{self, Large, Small};
use std::ops::{Sub, SubAssign};

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
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", Natural::from(0u32) - &Natural::from(123u32)), "None");
/// assert_eq!(format!("{:?}", Natural::from(123u32) - &Natural::from(0u32)), "Some(123)");
/// assert_eq!(format!("{:?}", Natural::from(456u32) - &Natural::from(123u32)), "Some(333)");
/// assert_eq!(format!("{:?}", Natural::from_str("3000000000000").unwrap() -
///                            &Natural::from_str("1000000000000").unwrap()),
///                    "Some(2000000000000)");
/// ```
impl<'a> Sub<&'a Natural> for Natural {
    type Output = Option<Natural>;

    fn sub(mut self, other: &'a Natural) -> Option<Natural> {
        if self >= *other {
            self -= other;
            Some(self)
        } else {
            None
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
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", &Natural::from(0u32) - &Natural::from(123u32)), "None");
/// assert_eq!(format!("{:?}", &Natural::from(123u32) - &Natural::from(0u32)), "Some(123)");
/// assert_eq!(format!("{:?}", &Natural::from(456u32) - &Natural::from(123u32)), "Some(333)");
/// assert_eq!(format!("{:?}", &Natural::from_str("3000000000000").unwrap() -
///                            &Natural::from_str("1000000000000").unwrap()),
///                    "Some(2000000000000)");
/// ```
impl<'a, 'b> Sub<&'a Natural> for &'b Natural {
    type Output = Option<Natural>;

    fn sub(self, other: &'a Natural) -> Option<Natural> {
        match (self, other) {
            (x, &Small(0)) => Some(x.clone()),
            (x, &Small(y)) => x - y,
            (&Small(_), _) => None,
            (&Large(ref xs), &Large(ref ys)) => {
                large_sub(xs, ys).map(|limbs| {
                                          let mut result = Large(limbs);
                                          result.trim();
                                          result
                                      })
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
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::from_str("10000000000000").unwrap();
/// x -= &Natural::from_str("1000000000000").unwrap();
/// x -= &Natural::from_str("2000000000000").unwrap();
/// x -= &Natural::from_str("3000000000000").unwrap();
/// x -= &Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "0");
/// ```
impl<'a> SubAssign<&'a Natural> for Natural {
    fn sub_assign(&mut self, other: &'a Natural) {
        if *other == 0 {
            return;
        }
        let mut panic = false;
        if let Small(y) = *other {
            *self -= y;
        } else if let Small(_) = *self {
            panic = true;
        } else if *self < *other {
            panic = true;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    panic = !large_sub_in_place(xs, ys);
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

fn large_sub_in_place(xs: &mut Vec<u32>, ys: &[u32]) -> bool {
    let mut borrow = false;
    let mut xs_iter = xs.iter_mut();
    for y in ys.iter() {
        let x = match xs_iter.next() {
            Some(x) => x,
            None => return false,
        };
        *x = sub_and_borrow(*x, *y, &mut borrow);
    }
    for x in xs_iter {
        if !borrow {
            break;
        }
        *x = sub_and_borrow(*x, 0, &mut borrow);
    }
    !borrow
}

fn large_sub(xs: &[u32], ys: &[u32]) -> Option<Vec<u32>> {
    let mut difference_limbs = Vec::with_capacity(xs.len());
    let mut borrow = false;
    let mut xs_iter = xs.iter();
    for y in ys.iter() {
        let x = match xs_iter.next() {
            Some(x) => x,
            None => return None,
        };
        difference_limbs.push(sub_and_borrow(*x, *y, &mut borrow));
    }
    for x in xs_iter {
        if borrow {
            difference_limbs.push(sub_and_borrow(*x, 0, &mut borrow));
        } else {
            difference_limbs.push(*x);
        }
    }
    if borrow { None } else { Some(difference_limbs) }
}
