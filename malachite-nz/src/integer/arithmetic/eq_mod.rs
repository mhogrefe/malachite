use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, NegMod, Parity};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::One;

use integer::Integer;
use natural::arithmetic::add::limbs_add_to_out;
use natural::arithmetic::divisible_by::limbs_divisible_by_val_ref;
use natural::arithmetic::eq_mod::{limbs_eq_limb_mod_limb, limbs_mod_exact_odd_limb};
use natural::arithmetic::mod_op::limbs_mod_limb;
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
/// This is mpz_congruent_ui_p from mpz/cong_ui.c where a is negative.
pub fn limbs_eq_neg_limb_mod_limb(limbs: &[Limb], limb: Limb, modulus: Limb) -> bool {
    limbs_eq_limb_mod_limb(limbs, limb.neg_mod(modulus), modulus)
}

/// Set r to -n mod d. n >= d is allowed. Can give r > d.
///
/// This is NEG_MOD from gmp-impl.h, where r is returned.
fn quick_neg_mod(n: Limb, d: Limb) -> Limb {
    assert_ne!(d, 0);
    if n <= d {
        // small a is reasonably likely
        d - n
    } else {
        let twos = d.leading_zeros();
        let dnorm = d << twos;
        (if n <= dnorm { dnorm } else { dnorm << 1 }).wrapping_sub(n)
    }
}

pub fn limbs_pos_eq_mod_neg(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_pos_eq_mod_neg_greater(xs, ys, modulus)
    } else {
        limbs_pos_eq_mod_neg_greater(ys, xs, modulus)
    }
}

// ap.len() >= cp.len(), neg case, sign < 0, sign == true, {ap, cp, dp}.len() > 0
// This is mpz_congruent_p from mpz/cong.c.
fn limbs_pos_eq_mod_neg_greater(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    let m_len = modulus.len();
    let mut x_len = xs.len();
    let y_len = ys.len();
    assert_ne!(m_len, 0);
    assert_ne!(x_len, 0);
    assert_ne!(y_len, 0);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(*modulus.last().unwrap(), 0);
    let x_0 = xs[0].wrapping_neg();
    let mut y_0 = ys[0];
    let mut m_0 = modulus[0];
    // Check x == y mod low zero bits of m_0. This might catch a few cases of x != y quickly.
    let dmask = if m_0 == 0 {
        Limb::MAX
    } else {
        (Limb::ONE << m_0.trailing_zeros()).wrapping_sub(1)
    };
    if x_0.wrapping_sub(y_0) & dmask != 0 {
        return false;
    }
    if y_len == 1 {
        if m_len == 1 {
            y_0 = quick_neg_mod(y_0, m_0);
            if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                let r = limbs_mod_limb(xs, m_0);
                if y_0 < m_0 {
                    return r == y_0;
                } else {
                    return r == y_0 % m_0;
                }
            }
            if m_0.even() {
                // Strip low zero bits to get odd modulus required by `limbs_mod_exact_odd_limb`. If
                // modulus == e * 2<sup>n</sup> then x == y mod modulus if and only if both
                // x == y mod e and x == y mod 2<sup>n</sup>, the latter having been done above.
                let twos = m_0.trailing_zeros();
                m_0 >>= twos;
            }
            let r = limbs_mod_exact_odd_limb(xs, m_0, y_0);
            return r == 0 || r == m_0;
        }
        // m_0 == 0 is avoided since we don't want to bother handling extra low zero bits if m_1 is
        // even (would involve borrow if x_0, y_0 != 0).
        if m_len == 2 && m_0 != 0 {
            let m_1 = modulus[1];
            if m_1 <= dmask {
                let twos = m_0.trailing_zeros();
                m_0 = (m_0 >> twos) | (m_1 << (Limb::WIDTH - twos));
                y_0 = quick_neg_mod(y_0, m_0);
                if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                    let r = limbs_mod_limb(xs, m_0);
                    if y_0 < m_0 {
                        return r == y_0;
                    } else {
                        //TODO untested!
                        return r == y_0 % m_0;
                    }
                }
                let r = limbs_mod_exact_odd_limb(xs, m_0, y_0);
                return r == 0 || r == m_0;
            }
        }
    }
    let mut xp = vec![0; x_len + 1];
    // calculate |x - y|. Different signs, add
    let carry = limbs_add_to_out(&mut xp, xs, ys);
    if carry {
        xp[x_len] = 1;
        x_len += 1;
    } else {
        xp[x_len] = 0;
    }
    x_len >= m_len && limbs_divisible_by_val_ref(&mut xp[..x_len], modulus)
}

impl Natural {
    fn pos_eq_neg_mod(self, other: Natural, modulus: Natural) -> bool {
        //TODO
        (Integer::from(self) + Integer::from(other)).divisible_by(Integer::from(modulus))
    }

    fn pos_eq_neg_mod_val_val_ref(self, other: Natural, modulus: &Natural) -> bool {
        //TODO
        (Integer::from(self) + Integer::from(other)).divisible_by(Integer::from(modulus))
    }

    fn pos_eq_neg_mod_val_ref_val(self, other: &Natural, modulus: Natural) -> bool {
        //TODO
        (Integer::from(self) + Integer::from(other)).divisible_by(Integer::from(modulus))
    }

    fn pos_eq_neg_mod_val_ref_ref(self, other: &Natural, modulus: &Natural) -> bool {
        //TODO
        (Integer::from(self) + Integer::from(other)).divisible_by(Integer::from(modulus))
    }

    fn pos_eq_neg_mod_ref_val_val(&self, other: Natural, modulus: Natural) -> bool {
        //TODO
        (Integer::from(self) + Integer::from(other)).divisible_by(Integer::from(modulus))
    }

    fn pos_eq_neg_mod_ref_val_ref(&self, other: Natural, modulus: &Natural) -> bool {
        //TODO
        (Integer::from(self) + Integer::from(other)).divisible_by(Integer::from(modulus))
    }

    fn pos_eq_neg_mod_ref_ref_val(&self, other: &Natural, modulus: Natural) -> bool {
        //TODO
        (Integer::from(self) + Integer::from(other)).divisible_by(Integer::from(modulus))
    }

    fn pos_eq_neg_mod_ref_ref_ref(&self, other: &Natural, modulus: &Natural) -> bool {
        //TODO
        (Integer::from(self) + Integer::from(other)).divisible_by(Integer::from(modulus))
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
            self.abs.pos_eq_neg_mod(other.abs, modulus)
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
            self.abs.pos_eq_neg_mod_val_val_ref(other.abs, modulus)
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
            self.abs.pos_eq_neg_mod_val_ref_val(&other.abs, modulus)
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
            self.abs.pos_eq_neg_mod_val_ref_ref(&other.abs, modulus)
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
            (&self.abs).pos_eq_neg_mod_ref_val_val(other.abs, modulus)
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
            (&self.abs).pos_eq_neg_mod_ref_val_ref(other.abs, modulus)
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
            (&self.abs).pos_eq_neg_mod_ref_ref_val(&other.abs, modulus)
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
            (&self.abs).pos_eq_neg_mod_ref_ref_ref(&other.abs, modulus)
        }
    }
}
