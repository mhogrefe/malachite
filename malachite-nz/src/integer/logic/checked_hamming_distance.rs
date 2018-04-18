use integer::Integer;
use malachite_base::num::{CheckedHammingDistance, HammingDistance};
use natural::Natural::{self, Large, Small};

fn limbs_hamming_distance_neg_helper(limbs: &[u32], i: usize, done: bool, nonzero_seen: &mut bool) -> u32 {
    let x = if done {0} else {limbs[i]};
    if *nonzero_seen {
        !x
    } else if x == 0 {
        0
    } else {
        *nonzero_seen = true;
        x.wrapping_neg()
    }
}

/// Interpreting two equal-length slices of `u32`s as the limbs of `Natural`s in ascending order,
/// returns the Hamming distance between their negatives (two's complement). Both have infinitely
/// many implicit leading ones. Neither slice may be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::integer::logic::checked_hamming_distance::limbs_hamming_distance_neg;
///
/// assert_eq!(limbs_hamming_distance_neg(&[2], &[3]), 2);
/// assert_eq!(limbs_hamming_distance_neg(&[1, 1, 1], &[1, 2, 3]), 3);
/// ```
pub fn limbs_hamming_distance_neg(xs: &[u32], ys: &[u32]) -> u64 {
    let mut distance = 0u64;
    let xs_len = xs.len();
    let ys_len = ys.len();
    let mut xs_nonzero_seen = false;
    let mut ys_nonzero_seen = false;
    let mut xs_done = false;
    let mut ys_done = false;
    let mut i = 0;
    loop {
        if !xs_done && i == xs_len {
            if ys_done {
                break;
            }
            xs_done = true;
        }
        if !ys_done && i == ys_len {
            if xs_done {
                break;
            }
            ys_done = true;
        }
        let x = limbs_hamming_distance_neg_helper(xs, i, xs_done, &mut xs_nonzero_seen);
        let y = limbs_hamming_distance_neg_helper(ys, i, ys_done, &mut ys_nonzero_seen);
        distance += x.hamming_distance(y);
        i += 1;
    }
    distance
}

impl<'a, 'b> CheckedHammingDistance<&'a Integer> for &'b Integer {
    /// Determines the Hamming distance between two `Integer`s. The two `Integer`s have infinitely
    /// many leading zeros or infinitely many leading ones, depending on their signs. If they are
    /// both non-negative or both negative, the Hamming distance is finite. If one is non-negative
    /// and the other is negative, the Hamming distance is infinite, so `None` is returned.
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
    /// use malachite_base::num::CheckedHammingDistance;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::from(123).checked_hamming_distance(&Integer::from(123)), Some(0));
    ///     // 105 = 1101001b, 123 = 1111011
    ///     assert_eq!(Integer::from(-105).checked_hamming_distance(&Integer::from(-123)), Some(2));
    ///     assert_eq!(Integer::from(-105).checked_hamming_distance(&Integer::from(123)), None);
    /// }
    /// ```
    fn checked_hamming_distance(self, other: &Integer) -> Option<u64> {
        match (self.sign, other.sign) {
            (true, true) => Some(self.abs.hamming_distance(&other.abs)),
            (false, false) => Some(self.abs.hamming_distance_neg(&other.abs)),
            _ => None,
        }
    }
}

impl Natural {
    fn hamming_distance_neg(&self, other: &Natural) -> u64 {
        match (self, other) {
            (&Small(x), _) => other.hamming_distance_neg_u32(x),
            (_, &Small(y)) => self.hamming_distance_neg_u32(y),
            (&Large(ref xs), &Large(ref ys)) => limbs_hamming_distance_neg(xs, ys),
        }
    }
}
