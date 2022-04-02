use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::slices::slice_leading_zeros;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::Ordering;
use std::mem::swap;

// Interpreting two equal-length slices of `Limb`s as the limbs (in ascending order) of two
// `Natural`s, compares the two `Natural`s.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(1)
//
// where n = `xs.len()` = `ys.len()`
//
// This is mpn_cmp from gmp.h, GMP 6.2.1.
//
// # Panics
// Panics if `xs` and `ys` have different lengths.
pub_crate_test! {limbs_cmp_same_length(xs: &[Limb], ys: &[Limb]) -> Ordering {
    assert_eq!(xs.len(), ys.len());
    xs.iter().rev().cmp(ys.iter().rev())
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, compares
// the two `Natural`s. Neither limb slice can contain trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// # Panics
// Panics if the last element of `xs` or `ys` is zero.
pub_crate_test! {limbs_cmp(xs: &[Limb], ys: &[Limb]) -> Ordering {
    assert_ne!(xs.last(), Some(&0));
    assert_ne!(ys.last(), Some(&0));
    xs.len()
        .cmp(&ys.len())
        .then_with(|| limbs_cmp_same_length(xs, ys))
}}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
// their normalized comparison. See `Natural::cmp_normalized` for details.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `min(xs.len(), ys.len())`.
//
// # Panics
// Panics if either `xs` or `ys` is empty, or if the last element of `xs` or `ys` is zero.
pub_test! {limbs_cmp_normalized(xs: &[Limb], ys: &[Limb]) -> Ordering {
    let mut xs = &xs[slice_leading_zeros(xs)..];
    let mut ys = &ys[slice_leading_zeros(ys)..];
    let mut xs_leading = LeadingZeros::leading_zeros(*xs.last().unwrap());
    assert_ne!(xs_leading, Limb::WIDTH);
    let mut ys_leading = LeadingZeros::leading_zeros(*ys.last().unwrap());
    assert_ne!(ys_leading, Limb::WIDTH);
    let mut xs_len = xs.len();
    let mut ys_len = ys.len();
    let mut swapped = false;
    match xs_leading.cmp(&ys_leading) {
        Ordering::Equal => {
            return match xs_len.cmp(&ys_len) {
                Ordering::Equal => limbs_cmp_same_length(xs, ys),
                Ordering::Less => {
                    let leading_cmp = limbs_cmp_same_length(xs, &ys[ys_len - xs_len..]);
                    if leading_cmp == Ordering::Greater {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
                Ordering::Greater => {
                    let leading_cmp = limbs_cmp_same_length(&xs[xs_len - ys_len..], ys);
                    if leading_cmp == Ordering::Less {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            };
        }
        Ordering::Less => {
            swap(&mut xs, &mut ys);
            swap(&mut xs_leading, &mut ys_leading);
            swap(&mut xs_len, &mut ys_len);
            swapped = true;
        }
        _ => {}
    }
    let xs_shift = xs_leading - ys_leading;
    let comp_xs_shift = Limb::WIDTH - xs_shift;
    let mut xs_i = xs_len - 1;
    let mut ys_i = ys_len - 1;
    loop {
        let y = ys[ys_i];
        let xs_hi = xs[xs_i];
        let xs_lo = if xs_i == 0 { 0 } else { xs[xs_i - 1] };
        let x = (xs_hi << xs_shift) | (xs_lo >> comp_xs_shift);
        let cmp = x.cmp(&y);
        if cmp != Ordering::Equal {
            return if swapped { cmp.reverse() } else { cmp };
        }
        if xs_i == 0 {
            return if ys_i == 0 {
                Ordering::Equal
            } else if swapped {
                Ordering::Greater
            } else {
                Ordering::Less
            };
        } else if ys_i == 0 {
            return if swapped {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        xs_i -= 1;
        ys_i -= 1;
    }
}}

impl PartialOrd for Natural {
    /// Compares a `Natural` to another `Natural`.
    ///
    /// See the documentation for the `Ord` implementation.
    #[inline]
    fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Natural {
    /// Compares a `Natural` to another `Natural`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
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
    fn cmp(&self, other: &Natural) -> Ordering {
        if std::ptr::eq(self, other) {
            return Ordering::Equal;
        }
        match (self, other) {
            (&Natural(Small(ref x)), &Natural(Small(ref y))) => x.cmp(y),
            (&Natural(Small(_)), &Natural(Large(_))) => Ordering::Less,
            (&Natural(Large(_)), &Natural(Small(_))) => Ordering::Greater,
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => limbs_cmp(xs, ys),
        }
    }
}

impl Natural {
    /// Returns a result of a comparison between two `Natural`s as if each had been multiplied by
    /// some power of 2 to bring it into the interval $[1, 2)$.
    ///
    /// That is, the comparison is equivalent to a comparison between $f(x)$ and $f(y)$, where
    /// $$
    /// f(n) = n2^{\lfloor\log_2 n \rfloor}.
    /// $$
    ///
    /// The multiplication is not actually performed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if either argument is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use std::cmp::Ordering;
    ///
    /// // 1 = 1.0 * 2^0, 4 = 1.0 * 2^2
    /// // 1.0 = 1.0
    /// assert_eq!(Natural::from(1u32).cmp_normalized(&Natural::from(4u32)), Ordering::Equal);
    ///
    /// // 5 = 1.25 * 2^2, 6 = 1.5 * 2^2
    /// // 1.25 < 1.5
    /// assert_eq!(Natural::from(5u32).cmp_normalized(&Natural::from(6u32)), Ordering::Less);
    ///
    /// // 3 = 1.5 * 2^1, 17 = 1.0625 * 2^4
    /// // 1.5 > 1.0625
    /// assert_eq!(Natural::from(3u32).cmp_normalized(&Natural::from(17u32)), Ordering::Greater);
    ///
    /// // 9 = 1.125 * 2^3, 36 = 1.125 * 2^5
    /// // 1.125 = 1.125
    /// assert_eq!(Natural::from(9u32).cmp_normalized(&Natural::from(36u32)), Ordering::Equal);
    /// ```
    pub fn cmp_normalized(&self, other: &Natural) -> Ordering {
        assert_ne!(*self, 0);
        assert_ne!(*other, 0);
        if std::ptr::eq(self, other) {
            return Ordering::Equal;
        }
        match (self, other) {
            (&Natural(Small(x)), &Natural(Small(y))) => {
                let leading_x = x.leading_zeros();
                let leading_y = y.leading_zeros();
                match leading_x.cmp(&leading_y) {
                    Ordering::Equal => x.cmp(&y),
                    Ordering::Less => x.cmp(&(y << (leading_y - leading_x))),
                    Ordering::Greater => (x << (leading_x - leading_y)).cmp(&y),
                }
            }
            (&Natural(Small(x)), &Natural(Large(ref ys))) => limbs_cmp_normalized(&[x], ys),
            (&Natural(Large(ref xs)), &Natural(Small(y))) => limbs_cmp_normalized(xs, &[y]),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => limbs_cmp_normalized(xs, ys),
        }
    }
}
