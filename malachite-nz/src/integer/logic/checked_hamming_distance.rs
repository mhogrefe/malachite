use integer::logic::checked_count_zeros::limbs_count_zeros_neg;
use integer::Integer;
use malachite_base::num::{CheckedHammingDistance, HammingDistance};
use natural::logic::count_ones::limbs_count_ones;
use natural::logic::hamming_distance::limbs_hamming_distance_same_length;
use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;

fn limbs_count_zeros(limbs: &[u32]) -> u64 {
    limbs.iter().map(|limb| u64::from(limb.count_zeros())).sum()
}

fn limbs_hamming_distance_neg_helper_2(xs: &[u32], ys: &[u32], i: usize) -> u64 {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys_len) {
        Ordering::Equal => limbs_hamming_distance_same_length(&xs[i + 1..], &ys[i + 1..]),
        Ordering::Less => {
            limbs_hamming_distance_same_length(&ys[i + 1..xs_len], &xs[i + 1..])
                + limbs_count_ones(&ys[xs_len..])
        }
        Ordering::Greater => {
            limbs_hamming_distance_same_length(&xs[i + 1..ys_len], &ys[i + 1..])
                + limbs_count_ones(&xs[ys_len..])
        }
    }
}

// xs: nnnnnnnb000
// ys:   nnb000000
//
// or
//
// xs:   nnnnnb000
// ys: nnnnb000000
//
// where 0 is a zero limb, n is a nonzero limb, and b is the boundary (least-significant) nonzero
// limb. xs_i and ys_i are the indices of the boundary limbs in xs and ys. xs_i < ys_i but xs may be
// shorter, longer, or the same length as ys.
fn limbs_hamming_distance_neg_helper(xs: &[u32], ys: &[u32], xs_i: usize, ys_i: usize) -> u64 {
    let mut distance = u64::from(xs[xs_i].wrapping_neg().count_ones());
    let xs_len = xs.len();
    if xs_i == xs_len - 1 {
        return distance + limbs_count_zeros_neg(&ys[xs_len..]);
    }
    if xs_len < ys_i {
        return distance + limbs_count_zeros(&xs[xs_i + 1..]) + limbs_count_zeros_neg(&ys[xs_len..]);
    }
    distance += limbs_count_zeros(&xs[xs_i + 1..ys_i]);
    if xs_len == ys_i {
        return distance + limbs_count_zeros_neg(&ys[xs_len..]);
    }
    distance += ys[ys_i].wrapping_neg().hamming_distance(!xs[ys_i]);
    if xs_len == ys_i + 1 {
        return distance + limbs_count_ones(&ys[xs_len..]);
    }
    distance + limbs_hamming_distance_neg_helper_2(xs, ys, ys_i)
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
    let xs_i = xs.iter().cloned().take_while(|&x| x == 0).count();
    let ys_i = ys.iter().cloned().take_while(|&y| y == 0).count();
    match xs_i.cmp(&ys_i) {
        Ordering::Equal => {
            xs[xs_i]
                .wrapping_neg()
                .hamming_distance(ys[ys_i].wrapping_neg())
                + limbs_hamming_distance_neg_helper_2(xs, ys, xs_i)
        }
        Ordering::Less => limbs_hamming_distance_neg_helper(xs, ys, xs_i, ys_i),
        Ordering::Greater => limbs_hamming_distance_neg_helper(ys, xs, ys_i, xs_i),
    }
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
