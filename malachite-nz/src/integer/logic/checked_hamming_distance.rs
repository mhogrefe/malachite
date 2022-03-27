use integer::logic::checked_count_zeros::limbs_count_zeros_neg;
use integer::Integer;
use malachite_base::num::logic::traits::{
    CheckedHammingDistance, CountOnes, CountZeros, HammingDistance,
};
use malachite_base::slices::slice_leading_zeros;
use natural::logic::count_ones::limbs_count_ones;
use natural::logic::hamming_distance::limbs_hamming_distance_same_length;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::Ordering;

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, returns the
/// Hamming distance between the negative of that `Natural` (two's complement) and the negative of a
/// `Limb`. Both have infinitely many implicit leading ones. `xs` cannot be empty or only contain
/// zeros; `y` cannot be zero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is empty.
#[doc(hidden)]
pub fn limbs_hamming_distance_limb_neg(xs: &[Limb], y: Limb) -> u64 {
    let x_lo = xs[0].wrapping_neg();
    limbs_count_zeros_neg(xs) - CountZeros::count_zeros(x_lo)
        + x_lo.hamming_distance(y.wrapping_neg())
}

fn limbs_count_zeros(xs: &[Limb]) -> u64 {
    xs.iter().map(|&limb| CountZeros::count_zeros(limb)).sum()
}

fn limbs_hamming_distance_neg_leading_limbs_helper(xs: &[Limb], ys: &[Limb], i: usize) -> u64 {
    let xs_len = xs.len();
    let ys_len = ys.len();
    match xs_len.cmp(&ys_len) {
        Ordering::Equal => limbs_hamming_distance_same_length(&xs[i + 1..], &ys[i + 1..]),
        Ordering::Less => {
            let (ys_lo, ys_hi) = ys.split_at(xs_len);
            limbs_hamming_distance_same_length(&ys_lo[i + 1..], &xs[i + 1..])
                + limbs_count_ones(ys_hi)
        }
        Ordering::Greater => {
            let (xs_lo, xs_hi) = xs.split_at(ys_len);
            limbs_hamming_distance_same_length(&xs_lo[i + 1..], &ys[i + 1..])
                + limbs_count_ones(xs_hi)
        }
    }
}

/// xs: nnnnnnnb000
/// ys:   nnb000000
///
/// or
///
/// xs:   nnnnnb000
/// ys: nnnnb000000
///
/// where 0 is a zero limb, n is a nonzero limb, and b is the boundary (least-significant) nonzero
/// limb. xs_i and ys_i are the indices of the boundary limbs in xs and ys. xs_i < ys_i but xs may
/// be shorter, longer, or the same length as ys.
fn limbs_hamming_distance_neg_helper(xs: &[Limb], ys: &[Limb], xs_i: usize, ys_i: usize) -> u64 {
    let mut distance = CountOnes::count_ones(xs[xs_i].wrapping_neg());
    let xs_len = xs.len();
    if xs_i == xs_len - 1 {
        return distance + limbs_count_zeros_neg(&ys[xs_len..]);
    }
    if xs_len < ys_i {
        return distance
            + limbs_count_zeros(&xs[xs_i + 1..])
            + limbs_count_zeros_neg(&ys[xs_len..]);
    }
    distance += limbs_count_zeros(&xs[xs_i + 1..ys_i]);
    if xs_len == ys_i {
        return distance + limbs_count_zeros_neg(&ys[xs_len..]);
    }
    distance += ys[ys_i].wrapping_neg().hamming_distance(!xs[ys_i]);
    if xs_len == ys_i + 1 {
        return distance + limbs_count_ones(&ys[xs_len..]);
    }
    distance + limbs_hamming_distance_neg_leading_limbs_helper(xs, ys, ys_i)
}

/// Interpreting two equal-length slices of `Limb`s as the limbs of `Natural`s in ascending order,
/// returns the Hamming distance between their negatives (two's complement). Both have infinitely
/// many implicit leading ones. Neither slice may be empty or only contain zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// May panic if `xs` or `ys` only contain zeros.
///
/// This is mpz_hamdist from mpz/hamdist.c, GMP 6.2.1, where both arguments are negative and have
/// the same length.
#[doc(hidden)]
pub fn limbs_hamming_distance_neg(xs: &[Limb], ys: &[Limb]) -> u64 {
    let xs_i = slice_leading_zeros(xs);
    let ys_i = slice_leading_zeros(ys);
    match xs_i.cmp(&ys_i) {
        Ordering::Equal => {
            xs[xs_i]
                .wrapping_neg()
                .hamming_distance(ys[ys_i].wrapping_neg())
                + limbs_hamming_distance_neg_leading_limbs_helper(xs, ys, xs_i)
        }
        Ordering::Less => limbs_hamming_distance_neg_helper(xs, ys, xs_i, ys_i),
        Ordering::Greater => limbs_hamming_distance_neg_helper(ys, xs, ys_i, xs_i),
    }
}

impl Natural {
    fn hamming_distance_neg_limb(&self, other: Limb) -> u64 {
        match *self {
            Natural(Small(small)) => small.wrapping_neg().hamming_distance(other.wrapping_neg()),
            Natural(Large(ref limbs)) => limbs_hamming_distance_limb_neg(limbs, other),
        }
    }

    fn hamming_distance_neg(&self, other: &Natural) -> u64 {
        match (self, other) {
            (&Natural(Small(x)), _) => other.hamming_distance_neg_limb(x),
            (_, &Natural(Small(y))) => self.hamming_distance_neg_limb(y),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                limbs_hamming_distance_neg(xs, ys)
            }
        }
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
    /// use malachite_base::num::logic::traits::CheckedHammingDistance;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(123).checked_hamming_distance(&Integer::from(123)), Some(0));
    /// // 105 = 1101001b, 123 = 1111011
    /// assert_eq!(Integer::from(-105).checked_hamming_distance(&Integer::from(-123)), Some(2));
    /// assert_eq!(Integer::from(-105).checked_hamming_distance(&Integer::from(123)), None);
    /// ```
    fn checked_hamming_distance(self, other: &Integer) -> Option<u64> {
        match (self.sign, other.sign) {
            (true, true) => Some(self.abs.hamming_distance(&other.abs)),
            (false, false) => Some(self.abs.hamming_distance_neg(&other.abs)),
            _ => None,
        }
    }
}
