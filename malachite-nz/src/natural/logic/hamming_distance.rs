use malachite_base::num::HammingDistance;
use natural::logic::count_ones::limbs_count_ones;
use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;

fn limbs_hamming_distance_same_length_no_check(xs: &[u32], ys: &[u32]) -> u64 {
    xs.iter()
        .zip(ys.iter())
        .map(|(x, &y)| x.hamming_distance(y))
        .sum()
}

/// Interpreting two equal-length slices of `u32`s as the limbs of `Natural`s in ascending order,
/// returns the Hamming distance between them. Both have infinitely many implicit leading zeros.
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
/// use malachite_nz::natural::logic::hamming_distance::limbs_hamming_distance_same_length;
///
/// assert_eq!(limbs_hamming_distance_same_length(&[2], &[3]), 1);
/// assert_eq!(limbs_hamming_distance_same_length(&[1, 1, 1], &[1, 2, 3]), 3);
/// ```
pub fn limbs_hamming_distance_same_length(xs: &[u32], ys: &[u32]) -> u64 {
    assert_eq!(xs.len(), ys.len());
    limbs_hamming_distance_same_length_no_check(xs, ys)
}

/// Interpreting two slices of `u32`s as the limbs of `Natural`s in ascending order, returns the
/// Hamming distance between them. Both have infinitely many implicit leading zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::hamming_distance::limbs_hamming_distance;
///
/// assert_eq!(limbs_hamming_distance(&[1, 2, 3], &[3]), 4);
/// assert_eq!(limbs_hamming_distance(&[1, 1, 1], &[1, 2, 3]), 3);
/// ```
pub fn limbs_hamming_distance(xs: &[u32], ys: &[u32]) -> u64 {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys_len) {
        Ordering::Equal => limbs_hamming_distance_same_length_no_check(xs, ys),
        Ordering::Less => {
            limbs_hamming_distance_same_length_no_check(xs, &ys[0..xs_len])
                + limbs_count_ones(&ys[xs_len..])
        }
        Ordering::Greater => {
            limbs_hamming_distance_same_length_no_check(&xs[0..ys_len], ys)
                + limbs_count_ones(&xs[ys_len..])
        }
    }
}

impl<'a, 'b> HammingDistance<&'a Natural> for &'b Natural {
    /// Determines the Hamming distance between two `Natural`s. Both have infinitely many implicit
    /// leading zeros.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{HammingDistance, One};
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(123u32).hamming_distance(&Natural::from(123u32)), 0);
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(Natural::from(105u32).hamming_distance(&Natural::from(123u32)), 2);
    ///     let n = Natural::ONE << 100u32;
    ///     assert_eq!(n.hamming_distance(&(&n - 1).unwrap()), 101);
    /// }
    /// ```
    fn hamming_distance(self, other: &'a Natural) -> u64 {
        match (self, other) {
            (&Small(x), _) => x.hamming_distance(other),
            (_, &Small(y)) => self.hamming_distance(y),
            (&Large(ref xs), &Large(ref ys)) => limbs_hamming_distance(xs, ys),
        }
    }
}
