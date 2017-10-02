use integer::Integer;
use natural::Natural;
use std::ops::Not;

fn neg_and_borrow(x: u32, borrow: &mut bool) -> u32 {
    let (neg, overflow) = 0u32.overflowing_sub(x);
    if *borrow {
        *borrow = overflow;
        let (difference, overflow) = neg.overflowing_sub(1);
        *borrow |= overflow;
        difference
    } else {
        *borrow = overflow;
        neg
    }
}

// Perform the negation of s and write the result to r. This is equivalent to calling mpn_sub_n with
// a n-limb zero minuend and passing s as subtrahend. Return borrow. r.len() >= s.len().
pub fn mpn_neg(r: &mut [u32], s: &[u32]) -> bool {
    let s_len = s.len();
    assert!(r.len() >= s_len);
    let mut borrow = false;
    for i in 0..s_len {
        r[i] = neg_and_borrow(s[i], &mut borrow);
    }
    borrow
}

// Perform the negation of s and write the result to s. This is equivalent to calling
// mpn_sub_n_in_place with a n-limb zero minuend and passing s as subtrahend.
pub fn mpn_neg_in_place(s: &mut [u32]) -> bool {
    let mut borrow = false;
    for limb in s.iter_mut() {
        *limb = neg_and_borrow(*limb, &mut borrow);
    }
    borrow
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by value and returning an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!((!Natural::from(0u32)).to_string(), "-1");
/// assert_eq!((!Natural::from(123u32)).to_string(), "-124");
/// ```
impl Not for Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self + 1,
        }
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by reference and returning an `Integer`.
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
///
/// assert_eq!((!&Natural::from(0u32)).to_string(), "-1");
/// assert_eq!((!&Natural::from(123u32)).to_string(), "-124");
/// ```
impl<'a> Not for &'a Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self + 1,
        }
    }
}
