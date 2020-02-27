use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, EqModPowerOfTwo, NegMod};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::logic::traits::TrailingZeros;

use integer::Integer;
use natural::arithmetic::add::{limbs_add, limbs_add_limb};
use natural::arithmetic::divisible_by::{
    limbs_divisible_by, limbs_divisible_by_limb, limbs_divisible_by_val_ref,
};
use natural::arithmetic::eq_mod::{limbs_eq_limb_mod_limb, limbs_mod_exact_odd_limb};
use natural::arithmetic::mod_op::limbs_mod_limb;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is equal to the negative of a limb mod a given `Limb` modulus.
///
/// This function assumes that `modulus` is nonzero, `limbs` has at least two elements, and the last
/// element of `limbs` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if the length of `limbs` is less than 2.
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::eq_mod::limbs_eq_neg_limb_mod_limb;
///
/// assert_eq!(limbs_eq_neg_limb_mod_limb(&[6, 7], 3, 2), false);
/// assert_eq!(limbs_eq_neg_limb_mod_limb(&[100, 101, 102], 1_232, 10), true);
/// ```
///
/// This is mpz_congruent_ui_p from mpz/cong_ui.c, GMP 6.1.2, where a is negative.
pub fn limbs_eq_neg_limb_mod_limb(limbs: &[Limb], limb: Limb, modulus: Limb) -> bool {
    limbs_eq_limb_mod_limb(limbs, limb.neg_mod(modulus), modulus)
}

/// Set r to -n mod d. n >= d is allowed. Can give r > d. d cannot equal 0.
///
/// This is NEG_MOD from gmp-impl.h, GMP 6.1.2, where r is returned.
fn quick_neg_mod(n: Limb, d: Limb) -> Limb {
    if n <= d {
        d - n
    } else {
        let d = d << d.leading_zeros();
        (if n <= d { d } else { d << 1 }).wrapping_sub(n)
    }
}

/// Interpreting two limbs `x` and `y` and slice of `Limb`s `modulus` as three numbers x, y, and m,
/// determines whether x === -y mod m.
///
/// This function assumes that the input slice has at least two elements, its last element is
/// nonzero, and `x` and `y` are nonzero.
///
/// Time: Worst case O(1)
///
/// Additional memory: Worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::eq_mod::limbs_pos_limb_eq_neg_limb_mod;
///
/// assert_eq!(limbs_pos_limb_eq_neg_limb_mod(0xffff_ffff, 0xffff_ffff, &[0xffff_fffe, 1]), true);
/// assert_eq!(limbs_pos_limb_eq_neg_limb_mod(1, 1, &[1, 0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.1.2, where a and d are positive, c is negative, a
/// and d are one limb long, and c is longer than one limb.
pub fn limbs_pos_limb_eq_neg_limb_mod(x: Limb, y: Limb, modulus: &[Limb]) -> bool {
    // We are checking whether x === -y mod m; that is, whether x + y = k * m for some k in Z. But
    // because of the preconditions on m, the lowest possible value of m is 2<sup>Limb::WIDTH</sup>,
    // while the highest possible value of x + y is 2<sup>Limb::WIDTH + 1</sup> - 2, so we have
    // x + y < 2 * m. This means that k can only be 1, so we're actually checking whether x + y = m.
    modulus.len() == 2 && modulus[1] == 1 && {
        let (sum, overflow) = x.overflowing_add(y);
        overflow && sum == modulus[0]
    }
}

#[allow(clippy::absurd_extreme_comparisons)]
fn limbs_pos_eq_neg_limb_mod_helper(xs: &[Limb], y: Limb, modulus: &[Limb]) -> Option<bool> {
    let m_len = modulus.len();
    let x_len = xs.len();
    assert!(m_len > 1);
    assert!(x_len > 1);
    assert_ne!(y, 0);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*modulus.last().unwrap(), 0);
    let m_0 = modulus[0];
    // Check x == y mod low zero bits of m_0. This might catch a few cases of x != y quickly.
    let twos = TrailingZeros::trailing_zeros(m_0);
    if !xs[0].wrapping_neg().eq_mod_power_of_two(y, twos) {
        return Some(false);
    }
    // m_0 == 0 is avoided since we don't want to bother handling extra low zero bits if m_1 is even
    // (would involve borrow if x_0, y_0 != 0).
    if m_len == 2 && m_0 != 0 {
        let m_1 = modulus[1];
        if m_1 < 1 << twos {
            let m_0 = (m_0 >> twos) | (m_1 << (Limb::WIDTH - twos));
            let y = quick_neg_mod(y, m_0);
            return Some(if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                //TODO else untested!
                limbs_mod_limb(xs, m_0) == if y < m_0 { y } else { y % m_0 }
            } else {
                let r = limbs_mod_exact_odd_limb(xs, m_0, y);
                r == 0 || r == m_0
            });
        }
    }
    None
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `modulus` as
/// three numbers x, y, and m, determines whether x === -y mod m. The second input slice is
/// immutable.
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
/// use malachite_nz::integer::arithmetic::eq_mod::limbs_pos_eq_neg_limb_mod_ref;
///
/// assert_eq!(limbs_pos_eq_neg_limb_mod_ref(&[2, 2], 2, &[2, 1]), true);
/// assert_eq!(limbs_pos_eq_neg_limb_mod_ref(&[0, 1], 1, &[1, 0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.1.2, where a and d are positive, c is negative, a
/// and d are longer than one limb, and c is one limb long.
pub fn limbs_pos_eq_neg_limb_mod_ref(xs: &[Limb], y: Limb, modulus: &[Limb]) -> bool {
    if let Some(equal) = limbs_pos_eq_neg_limb_mod_helper(xs, y, modulus) {
        return equal;
    }
    // calculate |x - y|. Different signs, add
    let mut scratch = limbs_add_limb(xs, y);
    scratch.len() >= modulus.len() && limbs_divisible_by_val_ref(&mut scratch, modulus)
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `modulus` as
/// three numbers x, y, and m, determines whether x === -y mod m. The second input slice is mutable.
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
/// use malachite_nz::integer::arithmetic::eq_mod::limbs_pos_eq_neg_limb_mod;
///
/// assert_eq!(limbs_pos_eq_neg_limb_mod(&[2, 2], 2, &mut [2, 1]), true);
/// assert_eq!(limbs_pos_eq_neg_limb_mod(&[0, 1], 1, &mut [1, 0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.1.2, where a and d are positive, c is negative, a
/// and d are longer than one limb, and c is one limb long.
pub fn limbs_pos_eq_neg_limb_mod(xs: &[Limb], y: Limb, modulus: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_pos_eq_neg_limb_mod_helper(xs, y, modulus) {
        return equal;
    }
    // calculate |x - y|. Different signs, add
    let mut scratch = limbs_add_limb(xs, y);
    scratch.len() >= modulus.len() && limbs_divisible_by(&mut scratch, modulus)
}

/// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `modulus` as three numbers x, y, and
/// m, determines whether x === -y mod m.
///
/// This function assumes that each of the two input slices have at least two elements, their last
/// elements are nonzero, and `modulus` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len`)
///
/// # Panics
/// Panics if the length of `xs` or `ys` is less than 2, if the last element of either of the slices
/// is zero, or if `modulus` is zero.
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::eq_mod::limbs_pos_eq_neg_mod_limb;
///
/// assert_eq!(limbs_pos_eq_neg_mod_limb(&[0, 1], &[6, 1], 2), true);
/// assert_eq!(limbs_pos_eq_neg_mod_limb(&[0, 1], &[7, 1], 2), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.1.2, where a and d are positive, c is negative, a
/// and c are longer than one limb, and m is one limb long.
pub fn limbs_pos_eq_neg_mod_limb(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_pos_eq_mod_neg_limb_greater(xs, ys, modulus)
    } else {
        limbs_pos_eq_mod_neg_limb_greater(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_pos_eq_mod_neg_limb_greater(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    assert!(xs.len() > 1);
    assert!(ys.len() > 1);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(modulus, 0);
    // Check x == y mod low zero bits of m_0. This might catch a few cases of x != y quickly.
    if !xs[0]
        .wrapping_neg()
        .eq_mod_power_of_two(ys[0], TrailingZeros::trailing_zeros(modulus))
    {
        return false;
    }
    // calculate |x - y|. Different signs, add
    limbs_divisible_by_limb(&limbs_add(xs, ys), modulus)
}

fn limbs_pos_eq_neg_mod_greater_helper(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> Option<bool> {
    assert!(modulus.len() > 1);
    assert!(xs.len() > 1);
    assert!(ys.len() > 1);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(*modulus.last().unwrap(), 0);
    // Check x == y mod low zero bits of m_0. This might catch a few cases of x != y quickly.
    if !xs[0]
        .wrapping_neg()
        .eq_mod_power_of_two(ys[0], TrailingZeros::trailing_zeros(modulus[0]))
    {
        Some(false)
    } else {
        None
    }
}

/// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
/// first `Natural` is equal to the negative of the second `Natural` mod the third `Natural`. The
/// second input slice is immutable.
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
/// use malachite_nz::integer::arithmetic::eq_mod::limbs_pos_eq_neg_mod_ref;
///
/// assert_eq!(limbs_pos_eq_neg_mod_ref(&[0, 0, 1], &[0, 1], &[1, 1]), true);
/// assert_eq!(limbs_pos_eq_neg_mod_ref(&[1, 2], &[3, 4], &[0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.1.2, where a and d are positive, c is negative,
/// and each is longer than one limb.
pub fn limbs_pos_eq_neg_mod_ref(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_pos_eq_neg_mod_greater_ref(xs, ys, modulus)
    } else {
        limbs_pos_eq_neg_mod_greater_ref(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_pos_eq_neg_mod_greater_ref(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if let Some(equal) = limbs_pos_eq_neg_mod_greater_helper(xs, ys, modulus) {
        return equal;
    }
    // calculate |x - y|. Different signs, add
    let mut scratch = limbs_add(xs, ys);
    scratch.len() >= modulus.len() && limbs_divisible_by_val_ref(&mut scratch, modulus)
}

/// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
/// first `Natural` is equal to the negative of the second `Natural` mod the third `Natural`. The
/// second input slice is mutable.
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
/// use malachite_nz::integer::arithmetic::eq_mod::limbs_pos_eq_neg_mod;
///
/// assert_eq!(limbs_pos_eq_neg_mod(&[0, 0, 1], &[0, 1], &mut [1, 1]), true);
/// assert_eq!(limbs_pos_eq_neg_mod(&[1, 2], &[3, 4], &mut [0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.1.2, where a and d are positive, c is negative,
/// and each is longer than one limb.
pub fn limbs_pos_eq_neg_mod(xs: &[Limb], ys: &[Limb], modulus: &mut [Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_pos_eq_neg_mod_greater(xs, ys, modulus)
    } else {
        limbs_pos_eq_neg_mod_greater(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_pos_eq_neg_mod_greater(xs: &[Limb], ys: &[Limb], modulus: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_pos_eq_neg_mod_greater_helper(xs, ys, modulus) {
        return equal;
    }
    // calculate |x - y|. Different signs, add
    let mut scratch = limbs_add(xs, ys);
    scratch.len() >= modulus.len() && limbs_divisible_by(&mut scratch, modulus)
}

impl Natural {
    fn eq_neg_limb_mod_limb(&self, other: Limb, modulus: Limb) -> bool {
        modulus != 0
            && match *self {
                Natural(Small(small)) => small % modulus == other.neg_mod(modulus),
                Natural(Large(ref limbs)) => limbs_eq_neg_limb_mod_limb(limbs, other, modulus),
            }
    }

    fn pos_eq_neg_mod(&self, other: &Natural, modulus: Natural) -> bool {
        match (self, other, modulus) {
            (_, _, Natural(Small(0))) => false,
            (x, &Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (&Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (x, &Natural(Small(y)), Natural(Small(modulus))) => x.eq_neg_limb_mod_limb(y, modulus),
            (&Natural(Small(x)), y, Natural(Small(modulus))) => y.eq_neg_limb_mod_limb(x, modulus),
            (&Natural(Small(x)), &Natural(Small(y)), Natural(Large(ref modulus))) => {
                limbs_pos_limb_eq_neg_limb_mod(x, y, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), Natural(Small(modulus))) => {
                limbs_pos_eq_neg_mod_limb(xs, ys, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), Natural(Large(ref mut modulus))) => {
                limbs_pos_eq_neg_limb_mod(xs, y, modulus)
            }
            (&Natural(Small(x)), &Natural(Large(ref ys)), Natural(Large(ref mut modulus))) => {
                limbs_pos_eq_neg_limb_mod(ys, x, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), Natural(Large(ref mut modulus))) => {
                limbs_pos_eq_neg_mod(xs, ys, modulus)
            }
        }
    }

    fn pos_eq_neg_mod_ref(&self, other: &Natural, modulus: &Natural) -> bool {
        match (self, other, modulus) {
            (_, _, &Natural(Small(0))) => false,
            (x, &Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (&Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (x, &Natural(Small(y)), &Natural(Small(modulus))) => x.eq_neg_limb_mod_limb(y, modulus),
            (&Natural(Small(x)), y, &Natural(Small(modulus))) => y.eq_neg_limb_mod_limb(x, modulus),
            (&Natural(Small(x)), &Natural(Small(y)), &Natural(Large(ref modulus))) => {
                limbs_pos_limb_eq_neg_limb_mod(x, y, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Small(modulus))) => {
                limbs_pos_eq_neg_mod_limb(xs, ys, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), &Natural(Large(ref modulus))) => {
                limbs_pos_eq_neg_limb_mod_ref(xs, y, modulus)
            }
            (&Natural(Small(x)), &Natural(Large(ref ys)), &Natural(Large(ref modulus))) => {
                limbs_pos_eq_neg_limb_mod_ref(ys, x, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Large(ref modulus))) => {
                limbs_pos_eq_neg_mod_ref(xs, ys, modulus)
            }
        }
    }
}

impl EqMod<Integer, Natural> for Integer {
    /// Returns whether this `Integer` is equivalent to another `Integer` mod a third `Natural`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Integer::from(123).eq_mod(Integer::from(223), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///             Integer::from_str("-999999012346").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///             Integer::from_str("2000000987655").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Integer, modulus: Natural) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod(other.abs, modulus)
        } else {
            (&self.abs).pos_eq_neg_mod(&other.abs, modulus)
        }
    }
}

impl<'a> EqMod<Integer, &'a Natural> for Integer {
    /// Returns whether this `Integer` is equivalent to another `Integer` mod a third `Natural`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Integer::from(123).eq_mod(Integer::from(223), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///             Integer::from_str("-999999012346").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///             Integer::from_str("2000000987655").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Integer, modulus: &'a Natural) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod(other.abs, modulus)
        } else {
            (&self.abs).pos_eq_neg_mod_ref(&other.abs, modulus)
        }
    }
}

impl<'a> EqMod<&'a Integer, Natural> for Integer {
    /// Returns whether this `Integer` is equivalent to another `Integer` mod a third `Natural`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Integer::from(123).eq_mod(&Integer::from(223), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///             &Integer::from_str("-999999012346").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///             &Integer::from_str("2000000987655").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &'a Integer, modulus: Natural) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod(&other.abs, modulus)
        } else {
            (&self.abs).pos_eq_neg_mod(&other.abs, modulus)
        }
    }
}

impl<'a, 'b> EqMod<&'a Integer, &'b Natural> for Integer {
    /// Returns whether this `Integer` is equivalent to another `Integer` mod a third `Natural`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     Integer::from(123).eq_mod(&Integer::from(223), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///             &Integer::from_str("-999999012346").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///             &Integer::from_str("2000000987655").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &'a Integer, modulus: &'b Natural) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod(&other.abs, modulus)
        } else {
            (&self.abs).pos_eq_neg_mod_ref(&other.abs, modulus)
        }
    }
}

impl<'a> EqMod<Integer, Natural> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to another `Integer` mod a third `Natural`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&Integer::from(123)).eq_mod(Integer::from(223), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///             Integer::from_str("-999999012346").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///             Integer::from_str("2000000987655").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Integer, modulus: Natural) -> bool {
        if self.sign == other.sign {
            (&self.abs).eq_mod(other.abs, modulus)
        } else {
            (&self.abs).pos_eq_neg_mod(&other.abs, modulus)
        }
    }
}

impl<'a, 'b> EqMod<Integer, &'b Natural> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to another `Integer` mod a third `Natural`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&Integer::from(123)).eq_mod(Integer::from(223), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///             Integer::from_str("-999999012346").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///             Integer::from_str("2000000987655").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Integer, modulus: &'b Natural) -> bool {
        if self.sign == other.sign {
            (&self.abs).eq_mod(other.abs, modulus)
        } else {
            (&self.abs).pos_eq_neg_mod_ref(&other.abs, modulus)
        }
    }
}

impl<'a, 'b> EqMod<&'b Integer, Natural> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to another `Integer` mod a third `Natural`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&Integer::from(123)).eq_mod(&Integer::from(223), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///             &Integer::from_str("-999999012346").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///             &Integer::from_str("2000000987655").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &'b Integer, modulus: Natural) -> bool {
        if self.sign == other.sign {
            (&self.abs).eq_mod(&other.abs, modulus)
        } else {
            (&self.abs).pos_eq_neg_mod(&other.abs, modulus)
        }
    }
}

impl<'a, 'b, 'c> EqMod<&'b Integer, &'c Natural> for &'a Integer {
    /// Returns whether this `Integer` is equivalent to another `Integer` mod a third `Natural`
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
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     (&Integer::from(123)).eq_mod(&Integer::from(223), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///             &Integer::from_str("-999999012346").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///             &Integer::from_str("2000000987655").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &'b Integer, modulus: &'c Natural) -> bool {
        if self.sign == other.sign {
            (&self.abs).eq_mod(&other.abs, modulus)
        } else {
            (&self.abs).pos_eq_neg_mod_ref(&other.abs, modulus)
        }
    }
}
