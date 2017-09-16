use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;

// Compare s1 and s2.
pub fn mpn_cmp(s1: &[u32], s2: &[u32]) -> Ordering {
    s1.len().cmp(&s2.len()).then_with(|| {
        s1.into_iter().rev().cmp(s2.into_iter().rev())
    })
}

/// Compares a `Natural` to another `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`self.significant_bits(), other.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// assert!(Natural::from(123u32) > Natural::from(122u32));
/// assert!(Natural::from(123u32) >= Natural::from(122u32));
/// assert!(Natural::from(123u32) < Natural::from(124u32));
/// assert!(Natural::from(123u32) <= Natural::from(124u32));
/// ```
impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Asserts that `Natural` ordering is a total order.
impl Ord for Natural {
    fn cmp(&self, other: &Natural) -> Ordering {
        if self as *const Natural == other as *const Natural {
            return Ordering::Equal;
        }
        match (self, other) {
            (&Small(ref x), &Small(ref y)) => x.cmp(y),
            (&Small(_), &Large(_)) => Ordering::Less,
            (&Large(_), &Small(_)) => Ordering::Greater,
            (&Large(ref xs), &Large(ref ys)) => mpn_cmp(xs, ys),
        }
    }
}
