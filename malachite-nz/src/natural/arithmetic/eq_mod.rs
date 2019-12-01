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
use natural::arithmetic::sub_limb::limbs_sub_limb_to_out;
use natural::comparison::ord::limbs_cmp;
use platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};

// xs.len() > 1, modulus.len() > 1
#[allow(clippy::absurd_extreme_comparisons)]
pub fn limbs_eq_limb_mod(xs: &[Limb], y: Limb, modulus: &[Limb]) -> bool {
    let m_len = modulus.len();
    assert!(m_len > 1);
    let x_len = xs.len();
    assert!(x_len > 1);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(y, 0);
    assert_ne!(*modulus.last().unwrap(), 0);
    let x_0 = xs[0];
    let m_0 = modulus[0];
    // Check xs == ys mod low zero bits of m_0.
    let m_trailing_zeros = m_0.trailing_zeros();
    if !x_0.eq_mod_power_of_two(y, u64::from(m_trailing_zeros)) {
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

pub fn limbs_eq_mod_limb(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_limb_greater(xs, ys, modulus)
    } else {
        limbs_eq_mod_limb_greater(ys, xs, modulus)
    }
}

// xs.len() > 1, ys.len() > 1
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
    let x_0 = xs[0];
    let y_0 = ys[0];
    // Check xs == ys mod low zero bits of m.
    if !x_0.eq_mod_power_of_two(y_0, u64::from(modulus.trailing_zeros())) {
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
    match scratch.len() {
        0 => true,
        1 => scratch[0].divisible_by(modulus),
        _ => limbs_divisible_by_limb(&scratch, modulus),
    }
}

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

// xs.len() > 1, ys.len() > 1, modulus.len() > 1
fn limbs_eq_mod_greater(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    let m_len = modulus.len();
    assert!(m_len > 1);
    let x_len = xs.len();
    let y_len = ys.len();
    assert!(y_len > 1);
    assert!(x_len >= y_len);
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
    let m_trailing_zeros = m_0.trailing_zeros();
    if !x_0.eq_mod_power_of_two(y_0, u64::from(m_trailing_zeros)) {
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

pub fn _limbs_eq_limb_mod_naive(xs: &[Limb], y: Limb, modulus: &[Limb]) -> bool {
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

pub fn _limbs_eq_mod_limb_naive(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    assert!(xs.len() > 1);
    assert!(ys.len() > 1);
    limbs_mod_limb(xs, modulus) == limbs_mod_limb(ys, modulus)
}

pub fn _limbs_eq_mod_naive(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
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
