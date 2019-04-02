use natural::Natural::{self, Large, Small};
use platform::Limb;
use std::cmp::Ordering;

/// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
/// `Natural`s, compares the two `Natural`s.
///
/// Equivalent to GMP's `mpn_cmp`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::comparison::ord::limbs_cmp_same_length;
/// use std::cmp::Ordering;
///
/// assert_eq!(limbs_cmp_same_length(&[3], &[5]), Ordering::Less);
/// assert_eq!(limbs_cmp_same_length(&[3, 0], &[5, 0]), Ordering::Less);
/// assert_eq!(limbs_cmp_same_length(&[1, 2], &[2, 1]), Ordering::Greater);
/// assert_eq!(limbs_cmp_same_length(&[1, 2, 3], &[1, 2, 3]), Ordering::Equal);
/// ```
pub fn limbs_cmp_same_length(xs: &[Limb], ys: &[Limb]) -> Ordering {
    assert_eq!(xs.len(), ys.len());
    xs.iter().rev().cmp(ys.iter().rev())
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, compares
/// the two `Natural`s. It is assumed that neither limb slice contains trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`xs.len`, `ys.len`)
///
/// # Example
/// ```
/// use malachite_nz::natural::comparison::ord::limbs_cmp;
/// use std::cmp::Ordering;
///
/// assert_eq!(limbs_cmp(&[3], &[5]), Ordering::Less);
/// assert_eq!(limbs_cmp(&[3, 1], &[5]), Ordering::Greater);
/// assert_eq!(limbs_cmp(&[1, 2], &[2, 1, 3]), Ordering::Less);
/// assert_eq!(limbs_cmp(&[1, 2, 3], &[1, 2, 3]), Ordering::Equal);
/// ```
pub fn limbs_cmp(xs: &[Limb], ys: &[Limb]) -> Ordering {
    xs.len()
        .cmp(&ys.len())
        .then_with(|| limbs_cmp_same_length(xs, ys))
}

/// Compares a `Natural` to another `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`self.significant_bits()`, `other.significant_bits()`)
///
/// # Examples
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert!(Natural::from(123u32) > Natural::from(122u32));
/// assert!(Natural::from(123u32) >= Natural::from(122u32));
/// assert!(Natural::from(123u32) < Natural::from(124u32));
/// assert!(Natural::from(123u32) <= Natural::from(124u32));
/// ```
impl PartialOrd for Natural {
    #[inline]
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
            (&Large(ref xs), &Large(ref ys)) => limbs_cmp(xs, ys),
        }
    }
}
