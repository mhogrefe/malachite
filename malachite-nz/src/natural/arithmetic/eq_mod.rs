use std::cmp::Ordering;

use malachite_base::limbs::limbs_trailing_zero_limbs;
use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, EqModPowerOfTwo};
use malachite_base::num::basic::integers::PrimitiveInteger;

use natural::arithmetic::divisible_by::limbs_divisible_by_val_ref;
use natural::arithmetic::divisible_by_limb::limbs_divisible_by_limb;
use natural::arithmetic::eq_limb_mod_limb::limbs_mod_exact_odd_limb;
use natural::arithmetic::mod_limb::limbs_mod_limb;
use natural::arithmetic::mod_op::limbs_mod;
use natural::arithmetic::sub::{limbs_sub, limbs_sub_same_length_to_out, limbs_sub_to_out};
use natural::arithmetic::sub_limb::{limbs_sub_limb, limbs_sub_limb_to_out};
use natural::comparison::ord::limbs_cmp;
use natural::Natural;
use platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `modulus` as
/// three numbers x, y, and m, determines whether x === y mod m.
///
/// This function assumes that each of the two input slices have at least two elements, their last
/// elements are nonzero, and `y` is nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if the length of `xs` or `modulus` is less than 2, if the last element of either of the
/// slices is zero, or if `y` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod;
///
/// assert_eq!(limbs_eq_limb_mod(&[1, 1], 1, &[0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod(&[0, 1], 1, &[0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and d are longer than
/// one limb, and c is one limb long.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn limbs_eq_limb_mod(xs: &[Limb], y: Limb, modulus: &[Limb]) -> bool {
    let m_len = modulus.len();
    assert!(m_len > 1);
    let x_len = xs.len();
    assert!(x_len > 1);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(y, 0);
    assert_ne!(*modulus.last().unwrap(), 0);
    if m_len > x_len {
        // x < m, y < m, and x != y, so x != y mod m
        return false;
    }
    let m_0 = modulus[0];
    // Check xs == ys mod low zero bits of m_0.
    let m_trailing_zeros = m_0.trailing_zeros();
    if !xs[0].eq_mod_power_of_two(y, u64::from(m_trailing_zeros)) {
        return false;
    }
    if m_len == 2 && m_0 != 0 {
        let m_1 = modulus[1];
        if m_1 < 1 << m_trailing_zeros {
            let m_0_trailing_zeros = m_0.trailing_zeros();
            let m_0 = (m_0 >> m_0_trailing_zeros) | (m_1 << (Limb::WIDTH - m_0_trailing_zeros));
            return if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                let r = limbs_mod_limb(xs, m_0);
                if y < m_0 {
                    r == y
                } else {
                    r == y % m_0
                }
            } else {
                let r = limbs_mod_exact_odd_limb(xs, m_0, y);
                r == 0 || r == m_0
            };
        }
    }
    let mut scratch = vec![0; x_len + 1];
    // calculate |xs - y|
    assert!(!limbs_sub_limb_to_out(&mut scratch, xs, y));
    scratch.truncate(scratch.len() - limbs_trailing_zero_limbs(&scratch));
    scratch.len() >= modulus.len() && limbs_divisible_by_val_ref(&mut scratch, modulus)
}

/// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `modulus` as three numbers x, y, and
/// m, determines whether x === y mod m.
///
/// This function assumes that each of the two input slices have at least two elements, their last
/// elements are nonzero, and `modulus` is nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if the length of `xs` or `ys` is less than 2, if the last element of either of the slices
/// is zero, or if `modulus` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_limb;
///
/// assert_eq!(limbs_eq_mod_limb(&[1, 1], &[3, 4], 5), true);
/// assert_eq!(limbs_eq_mod_limb(&[0, 1], &[3, 4], 5), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and c are longer than
/// one limb, and m is one limb long.
pub fn limbs_eq_mod_limb(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_limb_greater(xs, ys, modulus)
    } else {
        limbs_eq_mod_limb_greater(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_greater(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    let x_len = xs.len();
    let y_len = ys.len();
    assert!(y_len > 1);
    assert!(x_len >= y_len);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(modulus, 0);
    if xs == ys {
        return true;
    }
    // Check xs == ys mod low zero bits of m.
    if !xs[0].eq_mod_power_of_two(ys[0], u64::from(modulus.trailing_zeros())) {
        return false;
    }
    let mut scratch = vec![0; x_len + 1];
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_to_out(&mut scratch, xs, ys));
    } else {
        assert!(!limbs_sub_same_length_to_out(&mut scratch, ys, xs));
    }
    scratch.truncate(scratch.len() - limbs_trailing_zero_limbs(&scratch));
    // scratch is non-empty here because xs != ys
    if scratch.len() == 1 {
        scratch[0].divisible_by(modulus)
    } else {
        limbs_divisible_by_limb(&scratch, modulus)
    }
}

/// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
/// first `Natural` is equal to the second `Natural` mod the third `Natural`.
///
/// This function assumes that each of the three input slices have at least two elements, and their
/// last elements are nonzero.
///
/// Time: Worst case O(n * log(n) * log(log(n)))
///
/// Additional memory: Worst case O(n * log(n))
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Panics
/// Panics if the length of `xs`, `ys`, or `modulus` is less than 2, or if the last element of any
/// of the slices is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod;
///
/// assert_eq!(limbs_eq_mod(&[1, 1, 1], &[1, 0, 3], &[0, 7]), true);
/// assert_eq!(limbs_eq_mod(&[0, 1, 1], &[1, 0, 3], &[0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive and each is longer than
/// one limb.
pub fn limbs_eq_mod(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater(xs, ys, modulus)
    } else {
        limbs_eq_mod_greater(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    let m_len = modulus.len();
    assert!(m_len > 1);
    let x_len = xs.len();
    let y_len = ys.len();
    assert!(y_len > 1);
    assert!(x_len >= y_len);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(*modulus.last().unwrap(), 0);
    if xs == ys {
        return true;
    }
    if m_len > x_len {
        // x < m, y < m, and x != y, so x != y mod m
        return false;
    }
    // Check xs == ys mod low zero bits of m_0. This helps the y_len == 1 special cases below.
    if !xs[0].eq_mod_power_of_two(ys[0], u64::from(modulus[0].trailing_zeros())) {
        return false;
    }
    let mut scratch = vec![0; x_len + 1];
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_to_out(&mut scratch, xs, ys));
    } else {
        assert!(!limbs_sub_same_length_to_out(&mut scratch, ys, xs));
    }
    scratch.truncate(scratch.len() - limbs_trailing_zero_limbs(&scratch));
    scratch.len() >= modulus.len() && limbs_divisible_by_val_ref(&mut scratch, modulus)
}

pub fn _limbs_eq_limb_mod_naive_1(xs: &[Limb], y: Limb, modulus: &[Limb]) -> bool {
    assert!(xs.len() > 1);
    assert!(modulus.len() > 1);
    let mut xs_mod = if xs.len() >= modulus.len() {
        limbs_mod(xs, modulus)
    } else {
        xs.to_vec()
    };
    xs_mod.truncate(xs_mod.len() - limbs_trailing_zero_limbs(&xs_mod));
    xs_mod == [y]
}

pub fn _limbs_eq_limb_mod_naive_2(xs: &[Limb], y: Limb, modulus: &[Limb]) -> bool {
    let mut difference = limbs_sub_limb(xs, y).0;
    difference.truncate(difference.len() - limbs_trailing_zero_limbs(&difference));
    difference.len() >= modulus.len() && limbs_divisible_by_val_ref(&mut difference, modulus)
}

pub fn _limbs_eq_mod_limb_naive_1(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    assert!(xs.len() > 1);
    assert!(ys.len() > 1);
    limbs_mod_limb(xs, modulus) == limbs_mod_limb(ys, modulus)
}

pub fn _limbs_eq_mod_limb_naive_2(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    if xs == ys {
        return true;
    }
    let mut difference = if limbs_cmp(xs, ys) >= Ordering::Equal {
        limbs_sub(xs, ys)
    } else {
        limbs_sub(ys, xs)
    }
    .0;
    difference.truncate(difference.len() - limbs_trailing_zero_limbs(&difference));
    if difference.len() == 1 {
        difference[0].divisible_by(modulus)
    } else {
        limbs_divisible_by_limb(&difference, modulus)
    }
}

pub fn _limbs_eq_mod_naive_1(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    let mut xs_mod = if xs.len() >= modulus.len() {
        limbs_mod(xs, modulus)
    } else {
        xs.to_vec()
    };
    let mut ys_mod = if ys.len() >= modulus.len() {
        limbs_mod(ys, modulus)
    } else {
        ys.to_vec()
    };
    xs_mod.truncate(xs_mod.len() - limbs_trailing_zero_limbs(&xs_mod));
    ys_mod.truncate(ys_mod.len() - limbs_trailing_zero_limbs(&ys_mod));
    limbs_cmp(&xs_mod, &ys_mod) == Ordering::Equal
}

pub fn _limbs_eq_mod_naive_2(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if xs == ys {
        return true;
    }
    let mut difference = if limbs_cmp(xs, ys) >= Ordering::Equal {
        limbs_sub(xs, ys)
    } else {
        limbs_sub(ys, xs)
    }
    .0;
    difference.truncate(difference.len() - limbs_trailing_zero_limbs(&difference));
    difference.len() >= modulus.len() && limbs_divisible_by_val_ref(&mut difference, modulus)
}

impl EqMod<Natural, Natural> for Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `modulus`; that is, whether `self` - `other` is a multiple of `modulus`. Two numbers are
    /// equal to each other mod 0 iff they are equal. `self`, `other`, and `modulus` are all taken
    /// by value.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(123u32).eq_mod(Natural::from(223u32), Natural::from(100u32)),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         Natural::from_str("1000000987654").unwrap().eq_mod(
    ///                 Natural::from_str("2000000987654").unwrap(),
    ///                 Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         Natural::from_str("1000000987654").unwrap().eq_mod(
    ///                 Natural::from_str("2000000987655").unwrap(),
    ///                 Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: Natural, modulus: Natural) -> bool {
        //TODO
        (&self).eq_mod(&other, &modulus)
    }
}

impl<'a> EqMod<Natural, &'a Natural> for Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `modulus`; that is, whether `self` - `other` is a multiple of `modulus`. Two numbers are
    /// equal to each other mod 0 iff they are equal. `self` and `other` are taken by value, and
    /// `modulus` is taken by reference.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(123u32).eq_mod(Natural::from(223u32), &Natural::from(100u32)),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         Natural::from_str("1000000987654").unwrap().eq_mod(
    ///                 Natural::from_str("2000000987654").unwrap(),
    ///                 &Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         Natural::from_str("1000000987654").unwrap().eq_mod(
    ///                 Natural::from_str("2000000987655").unwrap(),
    ///                 &Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: Natural, modulus: &'a Natural) -> bool {
        //TODO
        (&self).eq_mod(&other, modulus)
    }
}

impl<'a> EqMod<&'a Natural, Natural> for Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `modulus`; that is, whether `self` - `other` is a multiple of `modulus`. Two numbers are
    /// equal to each other mod 0 iff they are equal. `self` and `modulus` are taken by value, and
    /// `other` is taken by reference.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(123u32).eq_mod(&Natural::from(223u32), Natural::from(100u32)),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         Natural::from_str("1000000987654").unwrap().eq_mod(
    ///                 &Natural::from_str("2000000987654").unwrap(),
    ///                 Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         Natural::from_str("1000000987654").unwrap().eq_mod(
    ///                 &Natural::from_str("2000000987655").unwrap(),
    ///                 Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: &'a Natural, modulus: Natural) -> bool {
        //TODO
        (&self).eq_mod(other, &modulus)
    }
}

impl<'a, 'b> EqMod<&'a Natural, &'b Natural> for Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `modulus`; that is, whether `self` - `other` is a multiple of `modulus`. Two numbers are
    /// equal to each other mod 0 iff they are equal. `other` and `modulus` are taken by reference,
    /// and `self` is taken by value.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         Natural::from(123u32).eq_mod(&Natural::from(223u32), &Natural::from(100u32)),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         Natural::from_str("1000000987654").unwrap().eq_mod(
    ///                 &Natural::from_str("2000000987654").unwrap(),
    ///                 &Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         Natural::from_str("1000000987654").unwrap().eq_mod(
    ///                 &Natural::from_str("2000000987655").unwrap(),
    ///                 &Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: &'a Natural, modulus: &'b Natural) -> bool {
        //TODO
        (&self).eq_mod(other, modulus)
    }
}

impl<'a> EqMod<Natural, Natural> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `modulus`; that is, whether `self` - `other` is a multiple of `modulus`. Two numbers are
    /// equal to each other mod 0 iff they are equal. `other` and `modulus` are taken by value, and
    /// `self` is taken by reference.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(123u32)).eq_mod(Natural::from(223u32), Natural::from(100u32)),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///                 Natural::from_str("2000000987654").unwrap(),
    ///                 Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///                 Natural::from_str("2000000987655").unwrap(),
    ///                 Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: Natural, modulus: Natural) -> bool {
        //TODO
        self.eq_mod(&other, &modulus)
    }
}

impl<'a, 'b> EqMod<Natural, &'b Natural> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `modulus`; that is, whether `self` - `other` is a multiple of `modulus`. Two numbers are
    /// equal to each other mod 0 iff they are equal. `self` and `modulus` are taken by reference,
    /// and `other` is taken by value.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(123u32)).eq_mod(Natural::from(223u32), &Natural::from(100u32)),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///                 Natural::from_str("2000000987654").unwrap(),
    ///                 &Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///                 Natural::from_str("2000000987655").unwrap(),
    ///                 &Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: Natural, modulus: &'b Natural) -> bool {
        //TODO
        self.eq_mod(&other, modulus)
    }
}

impl<'a, 'b> EqMod<&'b Natural, Natural> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `modulus`; that is, whether `self` - `other` is a multiple of `modulus`. Two numbers are
    /// equal to each other mod 0 iff they are equal. `self` and `other` are taken by reference,
    /// and `modulus` is taken by value.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(123u32)).eq_mod(&Natural::from(223u32), Natural::from(100u32)),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///                 &Natural::from_str("2000000987654").unwrap(),
    ///                 Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///                 &Natural::from_str("2000000987655").unwrap(),
    ///                 Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: &'b Natural, modulus: Natural) -> bool {
        //TODO
        self.eq_mod(other, &modulus)
    }
}

impl<'a, 'b, 'c> EqMod<&'b Natural, &'c Natural> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `modulus`; that is, whether `self` - `other` is a multiple of `modulus`. Two numbers are
    /// equal to each other mod 0 iff they are equal. `self`, `other`, and `modulus` are all taken
    /// by reference.
    ///
    /// Time: Worst case O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: Worst case O(n * log(n))
    ///
    /// where n = max(`self.significant_bits()`, `other.significant_bits()`)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(
    ///         (&Natural::from(123u32)).eq_mod(&Natural::from(223u32), &Natural::from(100u32)),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///                 &Natural::from_str("2000000987654").unwrap(),
    ///                 &Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         true
    ///     );
    ///     assert_eq!(
    ///         (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///                 &Natural::from_str("2000000987655").unwrap(),
    ///                 &Natural::from_str("1000000000000").unwrap()
    ///         ),
    ///         false
    ///     );
    /// }
    /// ```
    fn eq_mod(self, other: &'b Natural, modulus: &'c Natural) -> bool {
        //TODO
        if *modulus == 0 as Limb {
            self == other
        } else {
            self % modulus == other % modulus
        }
    }
}
