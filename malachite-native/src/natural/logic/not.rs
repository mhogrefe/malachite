use integer::Integer;
use natural::Natural;
use std::ops::Not;

//TODO use
/*
pub(crate) fn mpn_com(r: &mut [u32], s: &[u32]) {
    let s_len = s.len();
    assert!(r.len() >= s_len);
    for i in 0..s_len {
        r[i] = !s[i];
    }
}*/

pub(crate) fn mpn_com_in_place(s: &mut [u32]) {
    for limb in s.iter_mut() {
        *limb = !*limb;
    }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     assert_eq!((!Natural::ZERO).to_string(), "-1");
///     assert_eq!((!Natural::from(123u32)).to_string(), "-124");
/// }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     assert_eq!((!&Natural::ZERO).to_string(), "-1");
///     assert_eq!((!&Natural::from(123u32)).to_string(), "-124");
/// }
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
