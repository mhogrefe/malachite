use integer::Integer;
use std::cmp::Ordering;

/// Compares an `Integer` to another `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`self.significant_bits(), other.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(Integer::from(-123) < Integer::from(-122));
/// assert!(Integer::from(-123) <= Integer::from(-122));
/// assert!(Integer::from(-123) > Integer::from(-124));
/// assert!(Integer::from(-123) >= Integer::from(-124));
/// ```
impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Asserts that `Integer` ordering is a total order.
impl Ord for Integer {
    fn cmp(&self, other: &Integer) -> Ordering {
        if self as *const Integer == other as *const Integer {
            return Ordering::Equal;
        }
        match (self.sign, other.sign) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (true, true) => self.abs.cmp(&other.abs),
            (false, false) => other.abs.cmp(&self.abs),
        }
    }
}
