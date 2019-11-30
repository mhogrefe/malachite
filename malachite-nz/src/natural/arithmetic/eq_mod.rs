use std::cmp::Ordering;

use malachite_base::limbs::limbs_trailing_zero_limbs;
use malachite_base::num::arithmetic::traits::{DivisibleBy, EqModPowerOfTwo};
use malachite_base::num::basic::integers::PrimitiveInteger;

use natural::arithmetic::divisible_by::{limbs_divisible_by_ref_ref, limbs_divisible_by_val_ref};
use natural::arithmetic::divisible_by_limb::limbs_divisible_by_limb;
use natural::arithmetic::eq_limb_mod_limb::limbs_mod_exact_odd_limb;
use natural::arithmetic::mod_limb::limbs_mod_limb;
use natural::arithmetic::mod_op::limbs_mod;
use natural::arithmetic::sub::{limbs_sub_same_length_to_out, limbs_sub_to_out};
use natural::comparison::ord::limbs_cmp;
use platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};

/// all slices must be empty or have last limb nonzero; d must be nonempty, and a and c cannot both
/// be empty.
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive.
pub fn limbs_eq_mod(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater(xs, ys, modulus)
    } else {
        limbs_eq_mod_greater(ys, xs, modulus)
    }
}

#[allow(clippy::absurd_extreme_comparisons)]
fn limbs_eq_mod_greater(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    let m_len = modulus.len();
    assert_ne!(m_len, 0);
    let x_len = xs.len();
    let y_len = ys.len();
    assert!(x_len >= y_len);
    assert!(x_len != 0 || y_len != 0);
    if y_len == 0 {
        return xs.len() >= modulus.len() && limbs_divisible_by_ref_ref(xs, modulus);
    }
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(*modulus.last().unwrap(), 0);
    if xs == ys {
        return true;
    }
    let x_0 = xs[0];
    let y_0 = ys[0];
    let m_0 = modulus[0];
    // Check xs == ys mod low zero bits of m_0. This helps the y_len == 1 special cases below.
    let m_trailing_zeros = m_0.trailing_zeros(); // 0
    if !x_0.eq_mod_power_of_two(y_0, u64::from(m_trailing_zeros)) {
        return false;
    }
    if y_len == 1 {
        if m_len == 1 {
            return if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                let r = limbs_mod_limb(xs, m_0);
                if y_0 < m_0 {
                    r == y_0
                } else {
                    r == y_0 % m_0
                }
            } else {
                // Strip low zero bits to get odd modulus required by modexact. If
                // modulus == e * 2 ^ n, then xs == ys mod modulus iff both xs == ys mod e and
                // xs == ys mod 2 ^ n, the latter having been done above.
                let m_0 = m_0 >> m_trailing_zeros;
                let r = limbs_mod_exact_odd_limb(xs, m_0, y_0);
                r == 0 || r == m_0
            };
        }
        if m_len == 2 && m_0 != 0 {
            let m_1 = modulus[1];
            if m_1 < 1 << m_trailing_zeros {
                let m_0_trailing_zeros = m_0.trailing_zeros();
                let m_0 = (m_0 >> m_0_trailing_zeros) | (m_1 << (Limb::WIDTH - m_0_trailing_zeros));
                return if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                    let r = limbs_mod_limb(xs, m_0);
                    if y_0 < m_0 {
                        r == y_0
                    } else {
                        r == y_0 % m_0
                    }
                } else {
                    let r = limbs_mod_exact_odd_limb(xs, m_0, y_0);
                    r == 0 || r == m_0
                };
            }
        }
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

pub fn _limbs_eq_mod_naive(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if modulus.len() == 1 {
        let modulus = modulus[0];
        if xs.is_empty() {
            if ys.len() == 1 {
                ys[0].divisible_by(modulus)
            } else {
                limbs_divisible_by_limb(ys, modulus)
            }
        } else if ys.is_empty() {
            if xs.len() == 1 {
                xs[0].divisible_by(modulus)
            } else {
                limbs_divisible_by_limb(xs, modulus)
            }
        } else {
            let xs_mod = if xs.len() == 1 {
                xs[0] % modulus
            } else {
                limbs_mod_limb(xs, modulus)
            };
            let ys_mod = if ys.len() == 1 {
                ys[0] % modulus
            } else {
                limbs_mod_limb(ys, modulus)
            };
            xs_mod == ys_mod
        }
    } else {
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
}
