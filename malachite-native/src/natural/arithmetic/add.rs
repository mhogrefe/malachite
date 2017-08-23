use natural::Natural::{self, Large, Small};
use std::mem::swap;
use std::ops::{Add, AddAssign};

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
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) + Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + Natural::from(0u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
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
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) + &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) +&Natural::from(0u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) +&Natural::from(456u32)).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + &Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
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
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(0u32) + Natural::from(123u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + Natural::from(0u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
/// assert_eq!((&Natural::from_str("1000000000000").unwrap() + Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl<'a> Add<Natural> for &'a Natural {
    type Output = Natural;

    fn add(self, mut other: Natural) -> Natural {
        other += self;
        other
    }
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
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(0u32) + &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + &Natural::from(0u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + &Natural::from(456u32)).to_string(), "579");
/// assert_eq!((&Natural::from_str("1000000000000").unwrap() + &Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl<'a, 'b> Add<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn add(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (&Small(0), _) => other.clone(),
            (_, &Small(0)) => self.clone(),
            (x, &Small(y)) => x + y,
            (&Small(x), y) => x + y,
            (&Large(ref xs), &Large(ref ys)) => Large(large_add(xs, ys)),
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
        if *self == 0 {
            *self = other;
            return;
        } else if other == 0 {
            return;
        }
        if self.limb_count() < other.limb_count() {
            swap(self, &mut other);
        }
        match other {
            Small(y) => *self += y,
            Large(ref ys) => {
                match *self {
                    Large(ref mut xs) => large_add_in_place(xs, ys),
                    _ => unreachable!(),
                }
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
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::new();
/// x += &Natural::from_str("1000000000000").unwrap();
/// x += &Natural::from_str("2000000000000").unwrap();
/// x += &Natural::from_str("3000000000000").unwrap();
/// x += &Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "10000000000000");
/// ```
impl<'a> AddAssign<&'a Natural> for Natural {
    fn add_assign(&mut self, other: &'a Natural) {
        if *self == 0 {
            self.clone_from(other);
            return;
        } else if *other == 0 {
            return;
        }
        match *other {
            Small(y) => *self += y,
            Large(ref ys) => {
                match *self {
                    Small(x) => *self = other + x,
                    Large(ref mut xs) => large_add_in_place(xs, ys),
                }
            }
        }
    }
}

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

fn large_add_in_place(xs: &mut Vec<u32>, ys: &[u32]) {
    let mut carry = false;
    let mut ys_iter = ys.iter();
    for x in xs.iter_mut() {
        match ys_iter.next() {
            Some(y) => *x = add_and_carry(*x, *y, &mut carry),
            None if carry => *x = add_and_carry(*x, 0, &mut carry),
            None => break,
        }
    }
    for y in ys_iter {
        if carry {
            xs.push(add_and_carry(0, *y, &mut carry));
        } else {
            xs.push(*y);
        }
    }
    if carry {
        xs.push(1);
    }
}

fn large_add(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut sum_limbs = Vec::with_capacity(xs.len());
    let mut carry = false;
    let mut ys_iter = ys.iter();
    for x in xs.iter() {
        sum_limbs.push(match ys_iter.next() {
            Some(y) => add_and_carry(*x, *y, &mut carry),
            None if carry => add_and_carry(*x, 0, &mut carry),
            None => *x,
        });
    }
    for y in ys_iter {
        if carry {
            sum_limbs.push(add_and_carry(0, *y, &mut carry));
        } else {
            sum_limbs.push(*y);
        }
    }
    if carry {
        sum_limbs.push(1);
    }
    sum_limbs
}
