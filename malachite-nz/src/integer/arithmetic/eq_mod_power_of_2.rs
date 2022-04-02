use integer::Integer;
use malachite_base::num::arithmetic::traits::EqModPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use natural::arithmetic::divisible_by_power_of_2::limbs_divisible_by_power_of_2;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::cmp::Ordering;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns
// whether the negative of the `Natural` is equivalent to a limb mod two to the power of `pow`;
// that is, whether the `pow` least-significant bits of the negative of the `Natural` and the limb
// are equal.
//
// This function assumes that `limbs` has length at least 2 and the last (most significant) limb is
// nonzero.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(1)
//
// where n = `limbs.len()`
pub_test! {limbs_eq_mod_power_of_2_neg_limb(xs: &[Limb], y: Limb, pow: u64) -> bool {
    if y == 0 {
        return limbs_divisible_by_power_of_2(xs, pow);
    }
    let i = usize::exact_from(pow >> Limb::LOG_WIDTH);
    match i.cmp(&xs.len()) {
        Ordering::Greater => false,
        Ordering::Equal => {
            if pow & Limb::WIDTH_MASK == 0 {
                // Check whether the sum of X and y is 0 mod B ^ xs.len().
                let mut carry = y;
                for &x in xs {
                    let sum = x.wrapping_add(carry);
                    if sum != 0 {
                        return false;
                    }
                    carry = 1;
                }
                true
            } else {
                false
            }
        }
        Ordering::Less => {
            if i == 0 {
                xs[0].eq_mod_power_of_2(y.wrapping_neg(), pow)
            } else {
                xs[0] == y.wrapping_neg()
                    && xs[1..i].iter().all(|&x| x == Limb::MAX)
                    && xs[i].eq_mod_power_of_2(Limb::MAX, pow & Limb::WIDTH_MASK)
            }
        }
    }
}}

fn limbs_eq_mod_power_of_2_neg_pos_greater(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    let xs_len = xs.len();
    let i = usize::exact_from(pow >> Limb::LOG_WIDTH);
    let small_pow = pow & Limb::WIDTH_MASK;
    if i > xs_len || i == xs_len && small_pow != 0 {
        false
    } else {
        let ys_len = ys.len();
        let mut y_nonzero_seen = false;
        for j in 0..i {
            let y = if j >= ys_len {
                Limb::MAX
            } else if y_nonzero_seen {
                !ys[j]
            } else if ys[j] == 0 {
                0
            } else {
                y_nonzero_seen = true;
                ys[j].wrapping_neg()
            };
            if xs[j] != y {
                return false;
            }
        }
        if small_pow == 0 {
            true
        } else {
            // i < xs_len
            let y = if i >= ys_len {
                Limb::MAX
            } else if y_nonzero_seen {
                !ys[i]
            } else {
                ys[i].wrapping_neg()
            };
            xs[i].eq_mod_power_of_2(y, small_pow)
        }
    }
}

// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, returns
// whether the first `Natural` and the negative of the second natural (equivalently, the negative
// of the first `Natural` and the second `Natural`) are equivalent mod two to the power of `pow`;
// that is, whether their `pow` least-significant bits are equal.
//
// This function assumes that neither slice is empty and their last elements are nonzero.
//
// Time: worst case O(n)
//
// Additional memory: worst case O(1)
//
// where n = min(pow, max(`xs.len()`, `ys.len()`))
//
// This is mpz_congruent_2exp_p from mpz/cong_2exp.c, GMP 6.2.1, where a is negative and c is
// positive.
pub_test! {limbs_eq_mod_power_of_2_neg_pos(xs: &[Limb], ys: &[Limb], pow: u64) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_power_of_2_neg_pos_greater(xs, ys, pow)
    } else {
        limbs_eq_mod_power_of_2_neg_pos_greater(ys, xs, pow)
    }
}}

impl Natural {
    fn eq_mod_power_of_2_neg_limb(&self, other: Limb, pow: u64) -> bool {
        match *self {
            Natural(Small(ref small)) => {
                pow <= Limb::WIDTH && small.wrapping_neg().eq_mod_power_of_2(other, pow)
            }
            Natural(Large(ref limbs)) => limbs_eq_mod_power_of_2_neg_limb(limbs, other, pow),
        }
    }

    fn eq_mod_power_of_2_neg_pos(&self, other: &Natural, pow: u64) -> bool {
        match (self, other) {
            (_, &Natural(Small(y))) => self.eq_mod_power_of_2_neg_limb(y, pow),
            (&Natural(Small(x)), _) => other.eq_mod_power_of_2_neg_limb(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                limbs_eq_mod_power_of_2_neg_pos(xs, ys, pow)
            }
        }
    }
}

impl<'a, 'b> EqModPowerOf2<&'b Integer> for &'a Integer {
    /// Returns whether two `Integer`s are equivalent mod two to the power of `pow`; that is,
    /// whether their `pow` least-significant bits are equal.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = min(`pow`, max(`self.significant_bits()`, `other.significant_bits()`))
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::EqModPowerOf2;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.eq_mod_power_of_2(&Integer::from(-256), 8), true);
    /// assert_eq!(Integer::from(-0b1101).eq_mod_power_of_2(&Integer::from(0b11011), 3), true);
    /// assert_eq!(Integer::from(-0b1101).eq_mod_power_of_2(&Integer::from(0b11011), 4), false);
    /// ```
    fn eq_mod_power_of_2(self, other: &'b Integer, pow: u64) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod_power_of_2(&other.abs, pow)
        } else {
            self.abs.eq_mod_power_of_2_neg_pos(&other.abs, pow)
        }
    }
}
