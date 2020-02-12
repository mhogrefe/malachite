use std::cmp::Ordering;

use malachite_base::limbs::limbs_trailing_zero_limbs;
use malachite_base::num::arithmetic::traits::{
    DivisibleBy, DivisibleByPowerOfTwo, EqMod, EqModPowerOfTwo, Parity, WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::SplitInHalf;

use natural::arithmetic::div_exact::limbs_modular_invert_limb;
use natural::arithmetic::divisible_by::{
    limbs_divisible_by, limbs_divisible_by_limb, limbs_divisible_by_val_ref,
};
use natural::arithmetic::mod_op::{limbs_mod, limbs_mod_limb};
use natural::arithmetic::sub::{
    limbs_sub, limbs_sub_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place,
    limbs_sub_limb_to_out, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::comparison::ord::limbs_cmp;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{DoubleLimb, Limb, BMOD_1_TO_MOD_1_THRESHOLD};

/// divisor must be odd. //TODO test
pub fn limbs_limb_mod_exact_odd_limb(limb: Limb, divisor: Limb, carry: Limb) -> Limb {
    if limb > carry {
        let result = (limb - carry) % divisor;
        if result == 0 {
            0
        } else {
            divisor - result
        }
    } else {
        (carry - limb) % divisor
    }
}

/// divisor must be odd. //TODO test
///
/// This is mpn_modexact_1c_odd from mpn/generic/mode1o.c.
pub fn limbs_mod_exact_odd_limb(limbs: &[Limb], divisor: Limb, mut carry: Limb) -> Limb {
    let len = limbs.len();
    if len == 1 {
        return limbs_limb_mod_exact_odd_limb(limbs[0], divisor, carry);
    }
    let inverse = limbs_modular_invert_limb(divisor);
    let divisor_double = DoubleLimb::from(divisor);
    let last_index = len - 1;
    for &limb in &limbs[..last_index] {
        let (difference, small_carry) = limb.overflowing_sub(carry);
        carry = (DoubleLimb::from(difference.wrapping_mul(inverse)) * divisor_double).upper_half();
        if small_carry {
            carry.wrapping_add_assign(1);
        }
    }
    let last = limbs[last_index];
    if last <= divisor {
        if carry >= last {
            carry - last
        } else {
            carry.wrapping_add(divisor - last)
        }
    } else {
        let (difference, small_carry) = last.overflowing_sub(carry);
        carry = (DoubleLimb::from(difference.wrapping_mul(inverse)) * divisor_double).upper_half();
        if small_carry {
            carry.wrapping_add_assign(1);
        }
        carry
    }
}

/// Benchmarks show that this is never faster than just calling `limbs_eq_limb_mod_limb`.
///
/// limbs.len() must be greater than 1; modulus must be nonzero.
///
/// This is mpz_congruent_ui_p from mpz/cong_ui.c where a is non-negative.
pub fn _combined_limbs_eq_limb_mod_limb(limbs: &[Limb], limb: Limb, modulus: Limb) -> bool {
    if limbs.len() < BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_mod_limb(limbs, modulus) == limb % modulus
    } else {
        limbs_eq_limb_mod_limb(limbs, limb, modulus)
    }
}

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is equal to a limb mod a given `Limb` modulus.
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
/// Panics if the length of `limbs` is less than 2 or `modulus` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod_limb;
///
/// assert_eq!(limbs_eq_limb_mod_limb(&[6, 7], 3, 2), false);
/// assert_eq!(limbs_eq_limb_mod_limb(&[100, 101, 102], 1_238, 10), true);
/// ```
///
/// This is mpz_congruent_ui_p from mpz/cong_ui.c where a is positive and the ABOVE_THRESHOLD branch
/// is excluded.
pub fn limbs_eq_limb_mod_limb(limbs: &[Limb], limb: Limb, modulus: Limb) -> bool {
    assert_ne!(modulus, 0);
    assert!(limbs.len() > 1);
    let remainder = if modulus.even() {
        let twos = modulus.trailing_zeros();
        if !limbs[0]
            .wrapping_sub(limb)
            .divisible_by_power_of_two(u64::from(twos))
        {
            return false;
        }
        limbs_mod_exact_odd_limb(limbs, modulus >> twos, limb)
    } else {
        limbs_mod_exact_odd_limb(limbs, modulus, limb)
    };
    remainder == 0 || remainder == modulus
}

#[allow(clippy::absurd_extreme_comparisons)]
fn limbs_eq_limb_mod_helper(xs: &[Limb], y: Limb, modulus: &[Limb]) -> Option<bool> {
    let m_len = modulus.len();
    assert!(m_len > 1);
    let x_len = xs.len();
    assert!(x_len > 1);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(y, 0);
    assert_ne!(*modulus.last().unwrap(), 0);
    if m_len > x_len {
        // x < m, y < m, and x != y, so x != y mod m
        return Some(false);
    }
    let m_0 = modulus[0];
    // Check xs == ys mod low zero bits of m_0.
    let m_trailing_zeros = m_0.trailing_zeros();
    if !xs[0].eq_mod_power_of_two(y, u64::from(m_trailing_zeros)) {
        return Some(false);
    }
    if m_len == 2 && m_0 != 0 {
        let m_1 = modulus[1];
        if m_1 < 1 << m_trailing_zeros {
            let m_0_trailing_zeros = m_0.trailing_zeros();
            let m_0 = (m_0 >> m_0_trailing_zeros)
                | (m_1 << (Limb::WIDTH - u64::from(m_0_trailing_zeros)));
            return Some(if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                let r = limbs_mod_limb(xs, m_0);
                if y < m_0 {
                    r == y
                } else {
                    r == y % m_0
                }
            } else {
                let r = limbs_mod_exact_odd_limb(xs, m_0, y);
                r == 0 || r == m_0
            });
        }
    }
    None
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `modulus` as
/// three numbers x, y, and m, determines whether x === y mod m. Both input slices are immutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod_ref_ref;
///
/// assert_eq!(limbs_eq_limb_mod_ref_ref(&[1, 1], 1, &[0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod_ref_ref(&[0, 1], 1, &[0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and d are longer than
/// one limb, and c is one limb long.
pub fn limbs_eq_limb_mod_ref_ref(xs: &[Limb], y: Limb, modulus: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_limb_mod_helper(xs, y, modulus) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - y|
    assert!(!limbs_sub_limb_to_out(&mut scratch, xs, y));
    scratch.truncate(scratch.len() - limbs_trailing_zero_limbs(&scratch));
    scratch.len() >= modulus.len() && limbs_divisible_by_val_ref(&mut scratch, modulus)
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `modulus` as
/// three numbers x, y, and m, determines whether x === y mod m. The first input slice is immutable
/// and the second is mutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod_ref_val;
///
/// assert_eq!(limbs_eq_limb_mod_ref_val(&[1, 1], 1, &mut [0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod_ref_val(&[0, 1], 1, &mut [0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and d are longer than
/// one limb, and c is one limb long.
pub fn limbs_eq_limb_mod_ref_val(xs: &[Limb], y: Limb, modulus: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_limb_mod_helper(xs, y, modulus) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - y|
    assert!(!limbs_sub_limb_to_out(&mut scratch, xs, y));
    scratch.truncate(scratch.len() - limbs_trailing_zero_limbs(&scratch));
    scratch.len() >= modulus.len() && limbs_divisible_by(&mut scratch, modulus)
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `modulus` as
/// three numbers x, y, and m, determines whether x === y mod m. The first input slice is mutable
/// and the second is immutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod_val_ref;
///
/// assert_eq!(limbs_eq_limb_mod_val_ref(&mut [1, 1], 1, &[0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod_val_ref(&mut [0, 1], 1, &[0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and d are longer than
/// one limb, and c is one limb long.
pub fn limbs_eq_limb_mod_val_ref(xs: &mut [Limb], y: Limb, modulus: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_limb_mod_helper(xs, y, modulus) {
        return equal;
    }
    // calculate |xs - y|
    assert!(!limbs_sub_limb_in_place(xs, y));
    let new_len = xs.len() - limbs_trailing_zero_limbs(xs);
    new_len >= modulus.len() && limbs_divisible_by_val_ref(&mut xs[..new_len], modulus)
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `modulus` as
/// three numbers x, y, and m, determines whether x === y mod m. Both input slices are mutable.
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
/// assert_eq!(limbs_eq_limb_mod(&mut [1, 1], 1, &mut [0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod(&mut [0, 1], 1, &mut [0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and d are longer than
/// one limb, and c is one limb long.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn limbs_eq_limb_mod(xs: &mut [Limb], y: Limb, modulus: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_limb_mod_helper(xs, y, modulus) {
        return equal;
    }
    // calculate |xs - y|
    assert!(!limbs_sub_limb_in_place(xs, y));
    let new_len = xs.len() - limbs_trailing_zero_limbs(xs);
    new_len >= modulus.len() && limbs_divisible_by(&mut xs[..new_len], modulus)
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_helper(xs: &[Limb], ys: &[Limb], modulus: Limb) -> Option<bool> {
    let x_len = xs.len();
    let y_len = ys.len();
    assert!(y_len > 1);
    assert!(x_len >= y_len);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(modulus, 0);
    if xs == ys {
        Some(true)
    } else if !xs[0].eq_mod_power_of_two(ys[0], u64::from(modulus.trailing_zeros())) {
        // Check xs == ys mod low zero bits of m.
        Some(false)
    } else {
        None
    }
}

/// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `modulus` as three numbers x, y, and
/// m, determines whether x === y mod m. Both input slices are immutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_limb_ref_ref;
///
/// assert_eq!(limbs_eq_mod_limb_ref_ref(&[1, 1], &[3, 4], 5), true);
/// assert_eq!(limbs_eq_mod_limb_ref_ref(&[0, 1], &[3, 4], 5), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and c are longer than
/// one limb, and m is one limb long.
pub fn limbs_eq_mod_limb_ref_ref(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_limb_ref_ref_greater(xs, ys, modulus)
    } else {
        limbs_eq_mod_limb_ref_ref_greater(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_ref_ref_greater(xs: &[Limb], ys: &[Limb], modulus: Limb) -> bool {
    if let Some(equal) = limbs_eq_mod_limb_helper(xs, ys, modulus) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
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

/// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `modulus` as three numbers x, y, and
/// m, determines whether x === y mod m. The first input slice is immutable and the second is
/// mutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_limb_ref_val;
///
/// assert_eq!(limbs_eq_mod_limb_ref_val(&[1, 1], &mut [3, 4], 5), true);
/// assert_eq!(limbs_eq_mod_limb_ref_val(&[0, 1], &mut [3, 4], 5), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and c are longer than
/// one limb, and m is one limb long.
pub fn limbs_eq_mod_limb_ref_val(xs: &[Limb], ys: &mut [Limb], modulus: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_limb_ref_val_greater(xs, ys, modulus)
    } else {
        limbs_eq_mod_limb_val_ref_greater(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_ref_val_greater(xs: &[Limb], ys: &mut [Limb], modulus: Limb) -> bool {
    if let Some(equal) = limbs_eq_mod_limb_helper(xs, ys, modulus) {
        return equal;
    }
    let mut scratch;
    // calculate |xs - ys|
    let scratch = if limbs_cmp(xs, ys) >= Ordering::Equal {
        scratch = vec![0; xs.len()];
        assert!(!limbs_sub_to_out(&mut scratch, xs, ys));
        &mut scratch
    } else {
        assert!(!limbs_sub_same_length_in_place_left(ys, xs));
        ys
    };
    let new_len = scratch.len() - limbs_trailing_zero_limbs(scratch);
    // scratch is non-empty here because xs != ys
    if new_len == 1 {
        scratch[0].divisible_by(modulus)
    } else {
        limbs_divisible_by_limb(&scratch[..new_len], modulus)
    }
}

/// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `modulus` as three numbers x, y, and
/// m, determines whether x === y mod m. The first input slice is mutable and the second is
/// immutable.
///
/// This function assumes that each of the two input slices have at least two elements, their last
/// elements are nonzero, and `modulus` is nonzero. Both input slices are immutable.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len`)
///
/// # Panics
/// Panics if the length of `xs` or `ys` is less than 2, if the last element of either of the slices
/// is zero, or if `modulus` is zero.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_limb_val_ref;
///
/// assert_eq!(limbs_eq_mod_limb_val_ref(&mut [1, 1], &[3, 4], 5), true);
/// assert_eq!(limbs_eq_mod_limb_val_ref(&mut [0, 1], &[3, 4], 5), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive, a and c are longer than
/// one limb, and m is one limb long.
pub fn limbs_eq_mod_limb_val_ref(xs: &mut [Limb], ys: &[Limb], modulus: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_limb_val_ref_greater(xs, ys, modulus)
    } else {
        limbs_eq_mod_limb_ref_val_greater(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_val_ref_greater(xs: &mut [Limb], ys: &[Limb], modulus: Limb) -> bool {
    if let Some(equal) = limbs_eq_mod_limb_helper(xs, ys, modulus) {
        return equal;
    }
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_in_place_left(xs, ys));
    } else {
        assert!(!limbs_sub_same_length_in_place_right(ys, xs));
    }
    let new_len = xs.len() - limbs_trailing_zero_limbs(xs);
    // xs is non-empty here because xs != ys
    if new_len == 1 {
        xs[0].divisible_by(modulus)
    } else {
        limbs_divisible_by_limb(&xs[..new_len], modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_helper(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> Option<bool> {
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
        Some(true)
    } else if m_len > x_len
        || !xs[0].eq_mod_power_of_two(ys[0], u64::from(modulus[0].trailing_zeros()))
    {
        // Either: x < m, y < m, and x != y, so x != y mod m
        // Or: xs != ys mod low zero bits of m_0
        Some(false)
    } else {
        None
    }
}

/// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
/// first `Natural` is equal to the second `Natural` mod the third `Natural`. All input slices are
/// immutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_ref_ref_ref;
///
/// assert_eq!(limbs_eq_mod_ref_ref_ref(&[1, 1, 1], &[1, 0, 3], &[0, 7]), true);
/// assert_eq!(limbs_eq_mod_ref_ref_ref(&[0, 1, 1], &[1, 0, 3], &[0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive and each is longer than
/// one limb.
pub fn limbs_eq_mod_ref_ref_ref(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater_ref_ref_ref(xs, ys, modulus)
    } else {
        limbs_eq_mod_greater_ref_ref_ref(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_ref_ref_ref(xs: &[Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, modulus) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_to_out(&mut scratch, xs, ys));
    } else {
        assert!(!limbs_sub_same_length_to_out(&mut scratch, ys, xs));
    }
    scratch.truncate(scratch.len() - limbs_trailing_zero_limbs(&scratch));
    scratch.len() >= modulus.len() && limbs_divisible_by_val_ref(&mut scratch, modulus)
}

/// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
/// first `Natural` is equal to the second `Natural` mod the third `Natural`. The first two input
/// slices are immutable, and the third is mutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_ref_ref_val;
///
/// assert_eq!(limbs_eq_mod_ref_ref_val(&[1, 1, 1], &[1, 0, 3], &mut [0, 7]), true);
/// assert_eq!(limbs_eq_mod_ref_ref_val(&[0, 1, 1], &[1, 0, 3], &mut [0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive and each is longer than
/// one limb.
pub fn limbs_eq_mod_ref_ref_val(xs: &[Limb], ys: &[Limb], modulus: &mut [Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater_ref_ref_val(xs, ys, modulus)
    } else {
        limbs_eq_mod_greater_ref_ref_val(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_ref_ref_val(xs: &[Limb], ys: &[Limb], modulus: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, modulus) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_to_out(&mut scratch, xs, ys));
    } else {
        assert!(!limbs_sub_same_length_to_out(&mut scratch, ys, xs));
    }
    scratch.truncate(scratch.len() - limbs_trailing_zero_limbs(&scratch));
    scratch.len() >= modulus.len() && limbs_divisible_by(&mut scratch, modulus)
}

/// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
/// first `Natural` is equal to the second `Natural` mod the third `Natural`. The first and third
/// input slices are immutable, and the second is mutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_ref_val_ref;
///
/// assert_eq!(limbs_eq_mod_ref_val_ref(&[1, 1, 1], &mut [1, 0, 3], &[0, 7]), true);
/// assert_eq!(limbs_eq_mod_ref_val_ref(&[0, 1, 1], &mut [1, 0, 3], &[0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive and each is longer than
/// one limb.
pub fn limbs_eq_mod_ref_val_ref(xs: &[Limb], ys: &mut [Limb], modulus: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater_ref_val_ref(xs, ys, modulus)
    } else {
        limbs_eq_mod_greater_val_ref_ref(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_ref_val_ref(xs: &[Limb], ys: &mut [Limb], modulus: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, modulus) {
        return equal;
    }
    let mut scratch;
    // calculate |xs - ys|
    let scratch = if limbs_cmp(xs, ys) >= Ordering::Equal {
        scratch = vec![0; xs.len()];
        assert!(!limbs_sub_to_out(&mut scratch, xs, ys));
        &mut scratch
    } else {
        assert!(!limbs_sub_same_length_in_place_left(ys, xs));
        ys
    };
    let new_len = scratch.len() - limbs_trailing_zero_limbs(scratch);
    new_len >= modulus.len() && limbs_divisible_by_val_ref(&mut scratch[..new_len], modulus)
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_val_ref_ref(xs: &mut [Limb], ys: &[Limb], modulus: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, modulus) {
        return equal;
    }
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_in_place_left(xs, ys));
    } else {
        assert!(!limbs_sub_same_length_in_place_right(ys, xs));
    }
    let new_len = xs.len() - limbs_trailing_zero_limbs(xs);
    new_len >= modulus.len() && limbs_divisible_by_val_ref(&mut xs[..new_len], modulus)
}

/// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
/// first `Natural` is equal to the second `Natural` mod the third `Natural`. The first input slice
/// is immutable, and the second and third are mutable.
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
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_ref_val_val;
///
/// assert_eq!(limbs_eq_mod_ref_val_val(&[1, 1, 1], &mut [1, 0, 3], &mut [0, 7]), true);
/// assert_eq!(limbs_eq_mod_ref_val_val(&[0, 1, 1], &mut [1, 0, 3], &mut [0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, where a, c, and d are positive and each is longer than
/// one limb.
pub fn limbs_eq_mod_ref_val_val(xs: &[Limb], ys: &mut [Limb], modulus: &mut [Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater_ref_val_val(xs, ys, modulus)
    } else {
        limbs_eq_mod_greater_val_ref_val(ys, xs, modulus)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_ref_val_val(xs: &[Limb], ys: &mut [Limb], modulus: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, modulus) {
        return equal;
    }
    let mut scratch;
    // calculate |xs - ys|
    let scratch = if limbs_cmp(xs, ys) >= Ordering::Equal {
        scratch = vec![0; xs.len()];
        assert!(!limbs_sub_to_out(&mut scratch, xs, ys));
        &mut scratch
    } else {
        assert!(!limbs_sub_same_length_in_place_left(ys, xs));
        ys
    };
    let new_len = scratch.len() - limbs_trailing_zero_limbs(scratch);
    new_len >= modulus.len() && limbs_divisible_by(&mut scratch[..new_len], modulus)
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_val_ref_val(xs: &mut [Limb], ys: &[Limb], modulus: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, modulus) {
        return equal;
    }
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_in_place_left(xs, ys));
    } else {
        assert!(!limbs_sub_same_length_in_place_right(ys, xs));
    }
    let new_len = xs.len() - limbs_trailing_zero_limbs(xs);
    new_len >= modulus.len() && limbs_divisible_by(&mut xs[..new_len], modulus)
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

impl Natural {
    fn eq_mod_limb(&self, other: Limb, modulus: Limb) -> bool {
        match *self {
            Natural(Small(small)) => small.eq_mod(other, modulus),
            Natural(Large(_)) if modulus == 0 => false,
            Natural(Large(ref limbs)) => limbs_eq_limb_mod_limb(limbs, other, modulus),
        }
    }
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
    /// assert_eq!(
    ///     Natural::from(123u32).eq_mod(Natural::from(223u32), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000987654").unwrap().eq_mod(
    ///             Natural::from_str("2000000987654").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000987654").unwrap().eq_mod(
    ///             Natural::from_str("2000000987655").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Natural, modulus: Natural) -> bool {
        match (self, other, modulus) {
            (x, y, Natural(Small(0))) => x == y,
            (x, Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (ref x, Natural(Small(y)), Natural(Small(modulus))) => x.eq_mod_limb(y, modulus),
            (Natural(Small(x)), ref y, Natural(Small(modulus))) => y.eq_mod_limb(x, modulus),
            (Natural(Small(x)), Natural(Small(y)), _) => x == y,
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), Natural(Small(modulus))) => {
                limbs_eq_mod_limb_val_ref(xs, ys, modulus)
            }
            (Natural(Large(ref mut xs)), Natural(Small(y)), Natural(Large(ref mut modulus))) => {
                limbs_eq_limb_mod(xs, y, modulus)
            }
            (Natural(Small(x)), Natural(Large(ref mut ys)), Natural(Large(ref mut modulus))) => {
                limbs_eq_limb_mod(ys, x, modulus)
            }
            (
                Natural(Large(ref mut xs)),
                Natural(Large(ref ys)),
                Natural(Large(ref mut modulus)),
            ) => limbs_eq_mod_ref_val_val(ys, xs, modulus),
        }
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
    /// assert_eq!(
    ///     Natural::from(123u32).eq_mod(Natural::from(223u32), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000987654").unwrap().eq_mod(
    ///             Natural::from_str("2000000987654").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000987654").unwrap().eq_mod(
    ///             Natural::from_str("2000000987655").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Natural, modulus: &'a Natural) -> bool {
        match (self, other, modulus) {
            (x, y, &Natural(Small(0))) => x == y,
            (x, Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (ref x, Natural(Small(y)), &Natural(Small(modulus))) => x.eq_mod_limb(y, modulus),
            (Natural(Small(x)), ref y, &Natural(Small(modulus))) => y.eq_mod_limb(x, modulus),
            (Natural(Small(x)), Natural(Small(y)), _) => x == y,
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), &Natural(Small(modulus))) => {
                limbs_eq_mod_limb_val_ref(xs, ys, modulus)
            }
            (Natural(Large(ref mut xs)), Natural(Small(y)), &Natural(Large(ref modulus))) => {
                limbs_eq_limb_mod_val_ref(xs, y, modulus)
            }
            (Natural(Small(x)), Natural(Large(ref mut ys)), &Natural(Large(ref modulus))) => {
                limbs_eq_limb_mod_val_ref(ys, x, modulus)
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), &Natural(Large(ref modulus))) => {
                limbs_eq_mod_ref_val_ref(ys, xs, modulus)
            }
        }
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
    /// assert_eq!(
    ///     Natural::from(123u32).eq_mod(&Natural::from(223u32), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000987654").unwrap().eq_mod(
    ///             &Natural::from_str("2000000987654").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000987654").unwrap().eq_mod(
    ///             &Natural::from_str("2000000987655").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &'a Natural, modulus: Natural) -> bool {
        match (self, other, modulus) {
            (x, y, Natural(Small(0))) => x == *y,
            (x, &Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (ref x, &Natural(Small(y)), Natural(Small(modulus))) => x.eq_mod_limb(y, modulus),
            (Natural(Small(x)), y, Natural(Small(modulus))) => y.eq_mod_limb(x, modulus),
            (Natural(Small(x)), &Natural(Small(y)), _) => x == y,
            (Natural(Large(ref mut xs)), &Natural(Large(ref ys)), Natural(Small(modulus))) => {
                limbs_eq_mod_limb_val_ref(xs, ys, modulus)
            }
            (Natural(Large(ref mut xs)), &Natural(Small(y)), Natural(Large(ref mut modulus))) => {
                limbs_eq_limb_mod(xs, y, modulus)
            }
            (Natural(Small(x)), &Natural(Large(ref ys)), Natural(Large(ref mut modulus))) => {
                limbs_eq_limb_mod_ref_val(ys, x, modulus)
            }
            (
                Natural(Large(ref mut xs)),
                &Natural(Large(ref ys)),
                Natural(Large(ref mut modulus)),
            ) => limbs_eq_mod_ref_val_val(ys, xs, modulus),
        }
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
    /// assert_eq!(
    ///     Natural::from(123u32).eq_mod(&Natural::from(223u32), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000987654").unwrap().eq_mod(
    ///             &Natural::from_str("2000000987654").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Natural::from_str("1000000987654").unwrap().eq_mod(
    ///             &Natural::from_str("2000000987655").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &'a Natural, modulus: &'b Natural) -> bool {
        match (self, other, modulus) {
            (x, y, &Natural(Small(0))) => x == *y,
            (x, &Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (ref x, &Natural(Small(y)), &Natural(Small(modulus))) => x.eq_mod_limb(y, modulus),
            (Natural(Small(x)), y, &Natural(Small(modulus))) => y.eq_mod_limb(x, modulus),
            (Natural(Small(x)), &Natural(Small(y)), _) => x == y,
            (Natural(Large(ref mut xs)), &Natural(Large(ref ys)), &Natural(Small(modulus))) => {
                limbs_eq_mod_limb_val_ref(xs, ys, modulus)
            }
            (Natural(Large(ref mut xs)), &Natural(Small(y)), &Natural(Large(ref modulus))) => {
                limbs_eq_limb_mod_val_ref(xs, y, modulus)
            }
            (Natural(Small(x)), &Natural(Large(ref ys)), &Natural(Large(ref modulus))) => {
                limbs_eq_limb_mod_ref_ref(ys, x, modulus)
            }
            (Natural(Large(ref mut xs)), &Natural(Large(ref ys)), &Natural(Large(ref modulus))) => {
                limbs_eq_mod_ref_val_ref(ys, xs, modulus)
            }
        }
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
    /// assert_eq!(
    ///     (&Natural::from(123u32)).eq_mod(Natural::from(223u32), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///             Natural::from_str("2000000987654").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///             Natural::from_str("2000000987655").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Natural, modulus: Natural) -> bool {
        match (self, other, modulus) {
            (x, y, Natural(Small(0))) => *x == y,
            (x, Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (&Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (x, Natural(Small(y)), Natural(Small(modulus))) => x.eq_mod_limb(y, modulus),
            (&Natural(Small(x)), ref y, Natural(Small(modulus))) => y.eq_mod_limb(x, modulus),
            (&Natural(Small(x)), Natural(Small(y)), _) => x == y,
            (&Natural(Large(ref xs)), Natural(Large(ref mut ys)), Natural(Small(modulus))) => {
                limbs_eq_mod_limb_ref_val(xs, ys, modulus)
            }
            (&Natural(Large(ref xs)), Natural(Small(y)), Natural(Large(ref mut modulus))) => {
                limbs_eq_limb_mod_ref_val(xs, y, modulus)
            }
            (&Natural(Small(x)), Natural(Large(ref mut ys)), Natural(Large(ref mut modulus))) => {
                limbs_eq_limb_mod(ys, x, modulus)
            }
            (
                &Natural(Large(ref xs)),
                Natural(Large(ref mut ys)),
                Natural(Large(ref mut modulus)),
            ) => limbs_eq_mod_ref_val_val(xs, ys, modulus),
        }
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
    /// assert_eq!(
    ///     (&Natural::from(123u32)).eq_mod(Natural::from(223u32), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///             Natural::from_str("2000000987654").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///             Natural::from_str("2000000987655").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Natural, modulus: &'b Natural) -> bool {
        match (self, other, modulus) {
            (x, y, &Natural(Small(0))) => *x == y,
            (x, Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (&Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (x, Natural(Small(y)), &Natural(Small(modulus))) => x.eq_mod_limb(y, modulus),
            (&Natural(Small(x)), ref y, &Natural(Small(modulus))) => y.eq_mod_limb(x, modulus),
            (&Natural(Small(x)), Natural(Small(y)), _) => x == y,
            (&Natural(Large(ref xs)), Natural(Large(ref mut ys)), &Natural(Small(modulus))) => {
                limbs_eq_mod_limb_ref_val(xs, ys, modulus)
            }
            (&Natural(Large(ref xs)), Natural(Small(y)), &Natural(Large(ref modulus))) => {
                limbs_eq_limb_mod_ref_ref(xs, y, modulus)
            }
            (&Natural(Small(x)), Natural(Large(ref mut ys)), &Natural(Large(ref modulus))) => {
                limbs_eq_limb_mod_val_ref(ys, x, modulus)
            }
            (&Natural(Large(ref xs)), Natural(Large(ref mut ys)), &Natural(Large(ref modulus))) => {
                limbs_eq_mod_ref_val_ref(xs, ys, modulus)
            }
        }
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
    /// assert_eq!(
    ///     (&Natural::from(123u32)).eq_mod(&Natural::from(223u32), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///             &Natural::from_str("2000000987654").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///             &Natural::from_str("2000000987655").unwrap(),
    ///             Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &'b Natural, modulus: Natural) -> bool {
        match (self, other, modulus) {
            (x, y, Natural(Small(0))) => x == y,
            (x, &Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (&Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (x, &Natural(Small(y)), Natural(Small(modulus))) => x.eq_mod_limb(y, modulus),
            (&Natural(Small(x)), y, Natural(Small(modulus))) => y.eq_mod_limb(x, modulus),
            (&Natural(Small(x)), &Natural(Small(y)), _) => x == y,
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), Natural(Small(modulus))) => {
                limbs_eq_mod_limb_ref_ref(xs, ys, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), Natural(Large(ref mut modulus))) => {
                limbs_eq_limb_mod_ref_val(xs, y, modulus)
            }
            (&Natural(Small(x)), &Natural(Large(ref ys)), Natural(Large(ref mut modulus))) => {
                limbs_eq_limb_mod_ref_val(ys, x, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), Natural(Large(ref mut modulus))) => {
                limbs_eq_mod_ref_ref_val(xs, ys, modulus)
            }
        }
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
    /// assert_eq!(
    ///     (&Natural::from(123u32)).eq_mod(&Natural::from(223u32), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///             &Natural::from_str("2000000987654").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Natural::from_str("1000000987654").unwrap()).eq_mod(
    ///             &Natural::from_str("2000000987655").unwrap(),
    ///             &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &'b Natural, modulus: &'c Natural) -> bool {
        match (self, other, modulus) {
            (x, y, &Natural(Small(0))) => x == y,
            (x, &Natural(Small(0)), modulus) => x.divisible_by(modulus),
            (&Natural(Small(0)), y, modulus) => y.divisible_by(modulus),
            (x, &Natural(Small(y)), &Natural(Small(modulus))) => x.eq_mod_limb(y, modulus),
            (&Natural(Small(x)), y, &Natural(Small(modulus))) => y.eq_mod_limb(x, modulus),
            (&Natural(Small(x)), &Natural(Small(y)), _) => x == y,
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Small(modulus))) => {
                limbs_eq_mod_limb_ref_ref(xs, ys, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), &Natural(Large(ref modulus))) => {
                limbs_eq_limb_mod_ref_ref(xs, y, modulus)
            }
            (&Natural(Small(x)), &Natural(Large(ref ys)), &Natural(Large(ref modulus))) => {
                limbs_eq_limb_mod_ref_ref(ys, x, modulus)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Large(ref modulus))) => {
                limbs_eq_mod_ref_ref_ref(xs, ys, modulus)
            }
        }
    }
}
