use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, NegMod};

use integer::Integer;
use natural::arithmetic::eq_mod::limbs_eq_limb_mod_limb;
use natural::Natural;
use platform::Limb;

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
