use natural::arithmetic::sub_u32::sub_assign_u32_helper;
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
        if sub_assign_helper(&mut self, other) {
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
        if self as *const Natural == other as *const Natural {
            Some(Small(0))
        } else {
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
        if !sub_assign_helper(self, other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
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

fn sub_assign_helper<'a>(x: &mut Natural, y: &'a Natural) -> bool {
    if *y == 0 {
        true
    } else if x as *const Natural == y as *const Natural {
        *x = Small(0);
        true
    } else if x.limb_count() < y.limb_count() {
        false
    } else if let Small(y) = *y {
        sub_assign_u32_helper(x, y)
    } else if let Small(_) = *x {
        false
    } else {
        match (&mut (*x), y) {
            (&mut Large(ref mut xs), &Large(ref ys)) => {
                if !large_sub_in_place(xs, ys) {
                    return false;
                }
            }
            _ => unreachable!(),
        }
        x.trim();
        true
    }
}
