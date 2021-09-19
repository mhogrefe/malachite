use malachite_base::num::arithmetic::traits::{
    DivisibleBy, DivisibleByPowerOf2, EqMod, EqModPowerOf2, Parity, PowerOf2, WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::SplitInHalf;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::slices::slice_trailing_zeros;
use natural::arithmetic::div_exact::limbs_modular_invert_limb;
use natural::arithmetic::divisible_by::{
    limbs_divisible_by, limbs_divisible_by_limb, limbs_divisible_by_val_ref,
};
use natural::arithmetic::mod_op::limbs_mod_limb;
use natural::arithmetic::sub::{
    limbs_sub_greater_in_place_left, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out, limbs_sub_greater_to_out,
};
use natural::comparison::cmp::limbs_cmp;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{DoubleLimb, Limb, BMOD_1_TO_MOD_1_THRESHOLD};
use std::cmp::Ordering;

/// See the description for `_limbs_mod_exact_odd_limb`. divisor must be odd.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_modexact_1c_odd, from mpn/generic/mode1o.c, GMP 6.2.1, where size == 1.
pub const fn _limbs_limb_mod_exact_odd_limb(n: Limb, d: Limb, carry: Limb) -> Limb {
    if n > carry {
        let result = (n - carry) % d;
        if result == 0 {
            0
        } else {
            d - result
        }
    } else {
        (carry - n) % d
    }
}

/// Calculates an r satisfying
///
/// r * B ^ k + n - c == q * d
///
/// where B = 2<sup>`Limb::WIDTH`</sup>, k is either `ns.len()` or `ns.len()` - 1 (the caller won't
/// know which), c is `carry`, and q is the quotient (discarded). `d` must be odd and `carry` can be
/// any limb value.
///
/// If c < d then r will be in the range 0 <= r < d, or if c >= d then 0 <= r <= d.
///
/// This slightly strange function suits the initial N x 1 reduction for GCDs or Jacobi symbols
/// since the factors of 2 in B ^ k can be ignored, leaving -r == a mod d (by passing c = 0). For a
/// GCD the factor of -1 on r can be ignored, or for the Jacobi symbol it can be accounted for. The
/// function also suits divisibility and congruence testing, since if r = 0 (or r = d) is obtained,
/// then a === c mod d.
///
/// ns must be nonempty and divisor must be odd.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_modexact_1c_odd from mpn/generic/mode1o.c, GMP 6.2.1.
pub fn _limbs_mod_exact_odd_limb(ns: &[Limb], d: Limb, mut carry: Limb) -> Limb {
    let len = ns.len();
    if len == 1 {
        return _limbs_limb_mod_exact_odd_limb(ns[0], d, carry);
    }
    let d_inv = limbs_modular_invert_limb(d);
    let d_double = DoubleLimb::from(d);
    let (xs_last, xs_init) = ns.split_last().unwrap();
    let xs_last = *xs_last;
    for x in xs_init {
        let (diff, small_carry) = x.overflowing_sub(carry);
        carry = (DoubleLimb::from(diff.wrapping_mul(d_inv)) * d_double).upper_half();
        if small_carry {
            carry.wrapping_add_assign(1);
        }
    }
    if xs_last <= d {
        if carry >= xs_last {
            carry - xs_last
        } else {
            carry.wrapping_add(d - xs_last)
        }
    } else {
        let (diff, small_carry) = xs_last.overflowing_sub(carry);
        carry = (DoubleLimb::from(diff.wrapping_mul(d_inv)) * d_double).upper_half();
        if small_carry {
            carry.wrapping_add_assign(1);
        }
        carry
    }
}

/// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is equal to a limb mod a given `Limb` `m`.
///
/// This function assumes that `m` is nonzero, `xs` has at least two elements, and the last element
/// of `xs` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if the length of `xs` is less than 2 or `m` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod_limb;
///
/// assert_eq!(limbs_eq_limb_mod_limb(&[6, 7], 3, 2), false);
/// assert_eq!(limbs_eq_limb_mod_limb(&[100, 101, 102], 1238, 10), true);
/// ```
///
/// This is mpz_congruent_ui_p from mpz/cong_ui.c, GMP 6.2.1, where a is positive and the
/// ABOVE_THRESHOLD branch is excluded.
pub fn limbs_eq_limb_mod_limb(xs: &[Limb], y: Limb, m: Limb) -> bool {
    assert_ne!(m, 0);
    assert!(xs.len() > 1);
    let r = if m.even() {
        let twos = TrailingZeros::trailing_zeros(m);
        if !xs[0].wrapping_sub(y).divisible_by_power_of_2(twos) {
            return false;
        }
        _limbs_mod_exact_odd_limb(xs, m >> twos, y)
    } else {
        _limbs_mod_exact_odd_limb(xs, m, y)
    };
    r == 0 || r == m
}

#[allow(clippy::absurd_extreme_comparisons)]
fn limbs_eq_limb_mod_helper(xs: &[Limb], y: Limb, ms: &[Limb]) -> Option<bool> {
    let m_len = ms.len();
    assert!(m_len > 1);
    let x_len = xs.len();
    assert!(x_len > 1);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(y, 0);
    assert_ne!(*ms.last().unwrap(), 0);
    if m_len > x_len {
        // x < m, y < m, and x != y, so x != y mod m
        return Some(false);
    }
    let m_0 = ms[0];
    // Check xs == ys mod low zero bits of m_0.
    let m_0_trailing_zeros = TrailingZeros::trailing_zeros(m_0);
    if !xs[0].eq_mod_power_of_2(y, m_0_trailing_zeros) {
        return Some(false);
    }
    if m_len == 2 && m_0 != 0 {
        let m_1 = ms[1];
        if m_1 < Limb::power_of_2(m_0_trailing_zeros) {
            let m_0 = (m_0 >> m_0_trailing_zeros) | (m_1 << (Limb::WIDTH - m_0_trailing_zeros));
            return Some(if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                let r = limbs_mod_limb(xs, m_0);
                if y < m_0 {
                    r == y
                } else {
                    r == y % m_0
                }
            } else {
                let r = _limbs_mod_exact_odd_limb(xs, m_0, y);
                r == 0 || r == m_0
            });
        }
    }
    None
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `ms` as three
/// numbers x, y, and m, determines whether x === y mod m. Both input slices are immutable.
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
/// Panics if the length of `xs` or `ms` is less than 2, if the last element of either of the slices
/// is zero, or if `y` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod_ref_ref;
///
/// assert_eq!(limbs_eq_limb_mod_ref_ref(&[1, 1], 1, &[0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod_ref_ref(&[0, 1], 1, &[0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive, a and d are
/// longer than one limb, and c is one limb long.
pub fn limbs_eq_limb_mod_ref_ref(xs: &[Limb], y: Limb, ms: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_limb_mod_helper(xs, y, ms) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - y|
    assert!(!limbs_sub_limb_to_out(&mut scratch, xs, y));
    scratch.truncate(scratch.len() - slice_trailing_zeros(&scratch));
    scratch.len() >= ms.len() && limbs_divisible_by_val_ref(&mut scratch, ms)
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `ms` as three
/// numbers x, y, and m, determines whether x === y mod m. The first input slice is immutable and
/// the second is mutable.
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
/// Panics if the length of `xs` or `ms` is less than 2, if the last element of either of the slices
/// is zero, or if `y` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod_ref_val;
///
/// assert_eq!(limbs_eq_limb_mod_ref_val(&[1, 1], 1, &mut [0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod_ref_val(&[0, 1], 1, &mut [0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive, a and d are
/// longer than one limb, and c is one limb long.
pub fn limbs_eq_limb_mod_ref_val(xs: &[Limb], y: Limb, ms: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_limb_mod_helper(xs, y, ms) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - y|
    assert!(!limbs_sub_limb_to_out(&mut scratch, xs, y));
    scratch.truncate(scratch.len() - slice_trailing_zeros(&scratch));
    scratch.len() >= ms.len() && limbs_divisible_by(&mut scratch, ms)
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `ms` as three
/// numbers x, y, and m, determines whether x === y mod m. The first input slice is mutable and the
/// second is immutable.
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
/// Panics if the length of `xs` or `m` is less than 2, if the last element of either of the slices
/// is zero, or if `y` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod_val_ref;
///
/// assert_eq!(limbs_eq_limb_mod_val_ref(&mut [1, 1], 1, &[0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod_val_ref(&mut [0, 1], 1, &[0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive, a and d are
/// longer than one limb, and c is one limb long.
pub fn limbs_eq_limb_mod_val_ref(xs: &mut [Limb], y: Limb, ms: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_limb_mod_helper(xs, y, ms) {
        return equal;
    }
    // calculate |xs - y|
    assert!(!limbs_sub_limb_in_place(xs, y));
    let new_len = xs.len() - slice_trailing_zeros(xs);
    new_len >= ms.len() && limbs_divisible_by_val_ref(&mut xs[..new_len], ms)
}

/// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `ms` as three
/// numbers x, y, and m, determines whether x === y mod m. Both input slices are mutable.
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
/// Panics if the length of `xs` or `ms` is less than 2, if the last element of either of the slices
/// is zero, or if `y` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_limb_mod;
///
/// assert_eq!(limbs_eq_limb_mod(&mut [1, 1], 1, &mut [0, 1]), true);
/// assert_eq!(limbs_eq_limb_mod(&mut [0, 1], 1, &mut [0, 1]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive, a and d are
/// longer than one limb, and c is one limb long.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn limbs_eq_limb_mod(xs: &mut [Limb], y: Limb, ms: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_limb_mod_helper(xs, y, ms) {
        return equal;
    }
    // calculate |xs - y|
    assert!(!limbs_sub_limb_in_place(xs, y));
    let new_len = xs.len() - slice_trailing_zeros(xs);
    new_len >= ms.len() && limbs_divisible_by(&mut xs[..new_len], ms)
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_helper(xs: &[Limb], ys: &[Limb], m: Limb) -> Option<bool> {
    let x_len = xs.len();
    let y_len = ys.len();
    assert!(y_len > 1);
    assert!(x_len >= y_len);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(m, 0);
    if xs == ys {
        Some(true)
    } else if !xs[0].eq_mod_power_of_2(ys[0], u64::from(m.trailing_zeros())) {
        // Check xs == ys mod low zero bits of m.
        Some(false)
    } else {
        None
    }
}

/// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `m` as three numbers x, y, and m,
/// determines whether x === y mod m. Both input slices are immutable.
///
/// This function assumes that each of the two input slices have at least two elements, their last
/// elements are nonzero, and `m` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len`)
///
/// # Panics
/// Panics if the length of `xs` or `ys` is less than 2, if the last element of either of the slices
/// is zero, or if `m` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_limb_ref_ref;
///
/// assert_eq!(limbs_eq_mod_limb_ref_ref(&[1, 1], &[3, 4], 5), true);
/// assert_eq!(limbs_eq_mod_limb_ref_ref(&[0, 1], &[3, 4], 5), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive, a and c are
/// longer than one limb, and m is one limb long.
pub fn limbs_eq_mod_limb_ref_ref(xs: &[Limb], ys: &[Limb], m: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_limb_ref_ref_greater(xs, ys, m)
    } else {
        limbs_eq_mod_limb_ref_ref_greater(ys, xs, m)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_ref_ref_greater(xs: &[Limb], ys: &[Limb], m: Limb) -> bool {
    if let Some(equal) = limbs_eq_mod_limb_helper(xs, ys, m) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_greater_to_out(&mut scratch, xs, ys));
    } else {
        assert!(!limbs_sub_same_length_to_out(&mut scratch, ys, xs));
    }
    scratch.truncate(scratch.len() - slice_trailing_zeros(&scratch));
    // scratch is non-empty here because xs != ys
    if scratch.len() == 1 {
        scratch[0].divisible_by(m)
    } else {
        limbs_divisible_by_limb(&scratch, m)
    }
}

/// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `m` as three numbers x, y, and m,
/// determines whether x === y mod m. The first input slice is immutable and the second is mutable.
///
/// This function assumes that each of the two input slices have at least two elements, their last
/// elements are nonzero, and `m` is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len`)
///
/// # Panics
/// Panics if the length of `xs` or `ys` is less than 2, if the last element of either of the slices
/// is zero, or if `m` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_limb_ref_val;
///
/// assert_eq!(limbs_eq_mod_limb_ref_val(&[1, 1], &mut [3, 4], 5), true);
/// assert_eq!(limbs_eq_mod_limb_ref_val(&[0, 1], &mut [3, 4], 5), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive, a and c are
/// longer than one limb, and m is one limb long.
pub fn limbs_eq_mod_limb_ref_val(xs: &[Limb], ys: &mut [Limb], m: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_limb_ref_val_greater(xs, ys, m)
    } else {
        limbs_eq_mod_limb_val_ref_greater(ys, xs, m)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_ref_val_greater(xs: &[Limb], ys: &mut [Limb], m: Limb) -> bool {
    if let Some(equal) = limbs_eq_mod_limb_helper(xs, ys, m) {
        return equal;
    }
    let mut scratch;
    // calculate |xs - ys|
    let scratch = if limbs_cmp(xs, ys) >= Ordering::Equal {
        scratch = vec![0; xs.len()];
        assert!(!limbs_sub_greater_to_out(&mut scratch, xs, ys));
        &mut scratch
    } else {
        assert!(!limbs_sub_same_length_in_place_left(ys, xs));
        ys
    };
    let new_len = scratch.len() - slice_trailing_zeros(scratch);
    // scratch is non-empty here because xs != ys
    if new_len == 1 {
        scratch[0].divisible_by(m)
    } else {
        limbs_divisible_by_limb(&scratch[..new_len], m)
    }
}

/// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `m` as three numbers x, y, and m,
/// determines whether x === y mod m. The first input slice is mutable and the second is immutable.
///
/// This function assumes that each of the two input slices have at least two elements, their last
/// elements are nonzero, and `m` is nonzero. Both input slices are immutable.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len`)
///
/// # Panics
/// Panics if the length of `xs` or `ys` is less than 2, if the last element of either of the slices
/// is zero, or if `m` is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_limb_val_ref;
///
/// assert_eq!(limbs_eq_mod_limb_val_ref(&mut [1, 1], &[3, 4], 5), true);
/// assert_eq!(limbs_eq_mod_limb_val_ref(&mut [0, 1], &[3, 4], 5), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive, a and c are
/// longer than one limb, and m is one limb long.
pub fn limbs_eq_mod_limb_val_ref(xs: &mut [Limb], ys: &[Limb], m: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_limb_val_ref_greater(xs, ys, m)
    } else {
        limbs_eq_mod_limb_ref_val_greater(ys, xs, m)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_limb_val_ref_greater(xs: &mut [Limb], ys: &[Limb], m: Limb) -> bool {
    if let Some(equal) = limbs_eq_mod_limb_helper(xs, ys, m) {
        return equal;
    }
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_greater_in_place_left(xs, ys));
    } else {
        assert!(!limbs_sub_same_length_in_place_right(ys, xs));
    }
    let new_len = xs.len() - slice_trailing_zeros(xs);
    // xs is non-empty here because xs != ys
    if new_len == 1 {
        xs[0].divisible_by(m)
    } else {
        limbs_divisible_by_limb(&xs[..new_len], m)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_helper(xs: &[Limb], ys: &[Limb], m: &[Limb]) -> Option<bool> {
    let m_len = m.len();
    assert!(m_len > 1);
    let x_len = xs.len();
    let y_len = ys.len();
    assert!(y_len > 1);
    assert!(x_len >= y_len);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(*m.last().unwrap(), 0);
    if xs == ys {
        Some(true)
    } else if m_len > x_len || !xs[0].eq_mod_power_of_2(ys[0], TrailingZeros::trailing_zeros(m[0]))
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
/// Panics if the length of `xs`, `ys`, or `ms` is less than 2, or if the last element of any of the
/// slices is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_ref_ref_ref;
///
/// assert_eq!(limbs_eq_mod_ref_ref_ref(&[1, 1, 1], &[1, 0, 3], &[0, 7]), true);
/// assert_eq!(limbs_eq_mod_ref_ref_ref(&[0, 1, 1], &[1, 0, 3], &[0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive and each is
/// longer than one limb.
pub fn limbs_eq_mod_ref_ref_ref(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater_ref_ref_ref(xs, ys, ms)
    } else {
        limbs_eq_mod_greater_ref_ref_ref(ys, xs, ms)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_ref_ref_ref(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, ms) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_greater_to_out(&mut scratch, xs, ys));
    } else {
        assert!(!limbs_sub_same_length_to_out(&mut scratch, ys, xs));
    }
    scratch.truncate(scratch.len() - slice_trailing_zeros(&scratch));
    scratch.len() >= ms.len() && limbs_divisible_by_val_ref(&mut scratch, ms)
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
/// Panics if the length of `xs`, `ys`, or `ms` is less than 2, or if the last element of any
/// of the slices is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_ref_ref_val;
///
/// assert_eq!(limbs_eq_mod_ref_ref_val(&[1, 1, 1], &[1, 0, 3], &mut [0, 7]), true);
/// assert_eq!(limbs_eq_mod_ref_ref_val(&[0, 1, 1], &[1, 0, 3], &mut [0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive and each is
/// longer than one limb.
pub fn limbs_eq_mod_ref_ref_val(xs: &[Limb], ys: &[Limb], ms: &mut [Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater_ref_ref_val(xs, ys, ms)
    } else {
        limbs_eq_mod_greater_ref_ref_val(ys, xs, ms)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_ref_ref_val(xs: &[Limb], ys: &[Limb], ms: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, ms) {
        return equal;
    }
    let mut scratch = vec![0; xs.len()];
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_greater_to_out(&mut scratch, xs, ys));
    } else {
        assert!(!limbs_sub_same_length_to_out(&mut scratch, ys, xs));
    }
    scratch.truncate(scratch.len() - slice_trailing_zeros(&scratch));
    scratch.len() >= ms.len() && limbs_divisible_by(&mut scratch, ms)
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
/// Panics if the length of `xs`, `ys`, or `ms` is less than 2, or if the last element of any
/// of the slices is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_ref_val_ref;
///
/// assert_eq!(limbs_eq_mod_ref_val_ref(&[1, 1, 1], &mut [1, 0, 3], &[0, 7]), true);
/// assert_eq!(limbs_eq_mod_ref_val_ref(&[0, 1, 1], &mut [1, 0, 3], &[0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive and each is
/// longer than one limb.
pub fn limbs_eq_mod_ref_val_ref(xs: &[Limb], ys: &mut [Limb], ms: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater_ref_val_ref(xs, ys, ms)
    } else {
        limbs_eq_mod_greater_val_ref_ref(ys, xs, ms)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_ref_val_ref(xs: &[Limb], ys: &mut [Limb], ms: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, ms) {
        return equal;
    }
    let mut scratch;
    // calculate |xs - ys|
    let scratch = if limbs_cmp(xs, ys) >= Ordering::Equal {
        scratch = vec![0; xs.len()];
        assert!(!limbs_sub_greater_to_out(&mut scratch, xs, ys));
        &mut scratch
    } else {
        assert!(!limbs_sub_same_length_in_place_left(ys, xs));
        ys
    };
    let new_len = scratch.len() - slice_trailing_zeros(scratch);
    new_len >= ms.len() && limbs_divisible_by_val_ref(&mut scratch[..new_len], ms)
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_val_ref_ref(xs: &mut [Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, ms) {
        return equal;
    }
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_greater_in_place_left(xs, ys));
    } else {
        assert!(!limbs_sub_same_length_in_place_right(ys, xs));
    }
    let new_len = xs.len() - slice_trailing_zeros(xs);
    new_len >= ms.len() && limbs_divisible_by_val_ref(&mut xs[..new_len], ms)
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
/// Panics if the length of `xs`, `ys`, or `ms` is less than 2, or if the last element of any
/// of the slices is zero.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::eq_mod::limbs_eq_mod_ref_val_val;
///
/// assert_eq!(limbs_eq_mod_ref_val_val(&[1, 1, 1], &mut [1, 0, 3], &mut [0, 7]), true);
/// assert_eq!(limbs_eq_mod_ref_val_val(&[0, 1, 1], &mut [1, 0, 3], &mut [0, 7]), false);
/// ```
///
/// This is mpz_congruent_p from mpz/cong.c, GMP 6.2.1, where a, c, and d are positive and each is
/// longer than one limb.
pub fn limbs_eq_mod_ref_val_val(xs: &[Limb], ys: &mut [Limb], ms: &mut [Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_eq_mod_greater_ref_val_val(xs, ys, ms)
    } else {
        limbs_eq_mod_greater_val_ref_val(ys, xs, ms)
    }
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_ref_val_val(xs: &[Limb], ys: &mut [Limb], ms: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, ms) {
        return equal;
    }
    let mut scratch;
    // calculate |xs - ys|
    let scratch = if limbs_cmp(xs, ys) >= Ordering::Equal {
        scratch = vec![0; xs.len()];
        assert!(!limbs_sub_greater_to_out(&mut scratch, xs, ys));
        &mut scratch
    } else {
        assert!(!limbs_sub_same_length_in_place_left(ys, xs));
        ys
    };
    let new_len = scratch.len() - slice_trailing_zeros(scratch);
    new_len >= ms.len() && limbs_divisible_by(&mut scratch[..new_len], ms)
}

// xs.len() >= ys.len()
fn limbs_eq_mod_greater_val_ref_val(xs: &mut [Limb], ys: &[Limb], ms: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_eq_mod_helper(xs, ys, ms) {
        return equal;
    }
    // calculate |xs - ys|
    if limbs_cmp(xs, ys) >= Ordering::Equal {
        assert!(!limbs_sub_greater_in_place_left(xs, ys));
    } else {
        assert!(!limbs_sub_same_length_in_place_right(ys, xs));
    }
    let new_len = xs.len() - slice_trailing_zeros(xs);
    new_len >= ms.len() && limbs_divisible_by(&mut xs[..new_len], ms)
}

impl Natural {
    fn eq_mod_limb(&self, other: Limb, m: Limb) -> bool {
        match *self {
            Natural(Small(small)) => small.eq_mod(other, m),
            Natural(Large(_)) if m == 0 => false,
            Natural(Large(ref limbs)) => limbs_eq_limb_mod_limb(limbs, other, m),
        }
    }
}

impl EqMod<Natural, Natural> for Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural`
    /// `m`; that is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each
    /// other mod 0 iff they are equal. `self`, `other`, and `m` are all taken by value.
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
    fn eq_mod(self, other: Natural, m: Natural) -> bool {
        match (self, other, m) {
            (x, y, natural_zero!()) => x == y,
            (x, natural_zero!(), m) => x.divisible_by(m),
            (natural_zero!(), y, m) => y.divisible_by(m),
            (ref x, Natural(Small(y)), Natural(Small(m))) => x.eq_mod_limb(y, m),
            (Natural(Small(x)), ref y, Natural(Small(m))) => y.eq_mod_limb(x, m),
            (Natural(Small(x)), Natural(Small(y)), _) => x == y,
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), Natural(Small(m))) => {
                limbs_eq_mod_limb_val_ref(xs, ys, m)
            }
            (Natural(Large(ref mut xs)), Natural(Small(y)), Natural(Large(ref mut m))) => {
                limbs_eq_limb_mod(xs, y, m)
            }
            (Natural(Small(x)), Natural(Large(ref mut ys)), Natural(Large(ref mut m))) => {
                limbs_eq_limb_mod(ys, x, m)
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), Natural(Large(ref mut m))) => {
                limbs_eq_mod_ref_val_val(ys, xs, m)
            }
        }
    }
}

impl<'a> EqMod<Natural, &'a Natural> for Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural` `m`;
    /// that is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each other
    /// mod 0 iff they are equal. `self` and `other` are taken by value, and `m` is taken by
    /// reference.
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
    fn eq_mod(self, other: Natural, m: &'a Natural) -> bool {
        match (self, other, m) {
            (x, y, &natural_zero!()) => x == y,
            (x, natural_zero!(), m) => x.divisible_by(m),
            (natural_zero!(), y, m) => y.divisible_by(m),
            (ref x, Natural(Small(y)), &Natural(Small(m))) => x.eq_mod_limb(y, m),
            (Natural(Small(x)), ref y, &Natural(Small(m))) => y.eq_mod_limb(x, m),
            (Natural(Small(x)), Natural(Small(y)), _) => x == y,
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), &Natural(Small(m))) => {
                limbs_eq_mod_limb_val_ref(xs, ys, m)
            }
            (Natural(Large(ref mut xs)), Natural(Small(y)), &Natural(Large(ref m))) => {
                limbs_eq_limb_mod_val_ref(xs, y, m)
            }
            (Natural(Small(x)), Natural(Large(ref mut ys)), &Natural(Large(ref m))) => {
                limbs_eq_limb_mod_val_ref(ys, x, m)
            }
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), &Natural(Large(ref m))) => {
                limbs_eq_mod_ref_val_ref(ys, xs, m)
            }
        }
    }
}

impl<'a> EqMod<&'a Natural, Natural> for Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural` `m`;
    /// that is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each other
    /// mod 0 iff they are equal. `self` and `m` are taken by value, and `other` is taken by
    /// reference.
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
    fn eq_mod(self, other: &'a Natural, m: Natural) -> bool {
        match (self, other, m) {
            (x, y, natural_zero!()) => x == *y,
            (x, &natural_zero!(), m) => x.divisible_by(m),
            (natural_zero!(), y, m) => y.divisible_by(m),
            (ref x, &Natural(Small(y)), Natural(Small(m))) => x.eq_mod_limb(y, m),
            (Natural(Small(x)), y, Natural(Small(m))) => y.eq_mod_limb(x, m),
            (Natural(Small(x)), &Natural(Small(y)), _) => x == y,
            (Natural(Large(ref mut xs)), &Natural(Large(ref ys)), Natural(Small(m))) => {
                limbs_eq_mod_limb_val_ref(xs, ys, m)
            }
            (Natural(Large(ref mut xs)), &Natural(Small(y)), Natural(Large(ref mut m))) => {
                limbs_eq_limb_mod(xs, y, m)
            }
            (Natural(Small(x)), &Natural(Large(ref ys)), Natural(Large(ref mut m))) => {
                limbs_eq_limb_mod_ref_val(ys, x, m)
            }
            (Natural(Large(ref mut xs)), &Natural(Large(ref ys)), Natural(Large(ref mut m))) => {
                limbs_eq_mod_ref_val_val(ys, xs, m)
            }
        }
    }
}

impl<'a, 'b> EqMod<&'a Natural, &'b Natural> for Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural` `m`;
    /// that is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each other
    /// mod 0 iff they are equal. `other` and `m` are taken by reference, and `self` is taken by
    /// value.
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
    fn eq_mod(self, other: &'a Natural, m: &'b Natural) -> bool {
        match (self, other, m) {
            (x, y, &natural_zero!()) => x == *y,
            (x, &natural_zero!(), m) => x.divisible_by(m),
            (natural_zero!(), y, m) => y.divisible_by(m),
            (ref x, &Natural(Small(y)), &Natural(Small(m))) => x.eq_mod_limb(y, m),
            (Natural(Small(x)), y, &Natural(Small(m))) => y.eq_mod_limb(x, m),
            (Natural(Small(x)), &Natural(Small(y)), _) => x == y,
            (Natural(Large(ref mut xs)), &Natural(Large(ref ys)), &Natural(Small(m))) => {
                limbs_eq_mod_limb_val_ref(xs, ys, m)
            }
            (Natural(Large(ref mut xs)), &Natural(Small(y)), &Natural(Large(ref m))) => {
                limbs_eq_limb_mod_val_ref(xs, y, m)
            }
            (Natural(Small(x)), &Natural(Large(ref ys)), &Natural(Large(ref m))) => {
                limbs_eq_limb_mod_ref_ref(ys, x, m)
            }
            (Natural(Large(ref mut xs)), &Natural(Large(ref ys)), &Natural(Large(ref m))) => {
                limbs_eq_mod_ref_val_ref(ys, xs, m)
            }
        }
    }
}

impl<'a> EqMod<Natural, Natural> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural` `m`;
    /// that is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each other
    /// mod 0 iff they are equal. `other` and `m` are taken by value, and `self` is taken by
    /// reference.
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
    fn eq_mod(self, other: Natural, m: Natural) -> bool {
        match (self, other, m) {
            (x, y, natural_zero!()) => *x == y,
            (x, natural_zero!(), m) => x.divisible_by(m),
            (&natural_zero!(), y, m) => y.divisible_by(m),
            (x, Natural(Small(y)), Natural(Small(m))) => x.eq_mod_limb(y, m),
            (&Natural(Small(x)), ref y, Natural(Small(m))) => y.eq_mod_limb(x, m),
            (&Natural(Small(x)), Natural(Small(y)), _) => x == y,
            (&Natural(Large(ref xs)), Natural(Large(ref mut ys)), Natural(Small(m))) => {
                limbs_eq_mod_limb_ref_val(xs, ys, m)
            }
            (&Natural(Large(ref xs)), Natural(Small(y)), Natural(Large(ref mut m))) => {
                limbs_eq_limb_mod_ref_val(xs, y, m)
            }
            (&Natural(Small(x)), Natural(Large(ref mut ys)), Natural(Large(ref mut m))) => {
                limbs_eq_limb_mod(ys, x, m)
            }
            (&Natural(Large(ref xs)), Natural(Large(ref mut ys)), Natural(Large(ref mut m))) => {
                limbs_eq_mod_ref_val_val(xs, ys, m)
            }
        }
    }
}

impl<'a, 'b> EqMod<Natural, &'b Natural> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural` `m`;
    /// that is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each other
    /// mod 0 iff they are equal. `self` and `m` are taken by reference, and `other` is taken by
    /// value.
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
    fn eq_mod(self, other: Natural, m: &'b Natural) -> bool {
        match (self, other, m) {
            (x, y, &natural_zero!()) => *x == y,
            (x, natural_zero!(), m) => x.divisible_by(m),
            (&natural_zero!(), y, m) => y.divisible_by(m),
            (x, Natural(Small(y)), &Natural(Small(m))) => x.eq_mod_limb(y, m),
            (&Natural(Small(x)), ref y, &Natural(Small(m))) => y.eq_mod_limb(x, m),
            (&Natural(Small(x)), Natural(Small(y)), _) => x == y,
            (&Natural(Large(ref xs)), Natural(Large(ref mut ys)), &Natural(Small(m))) => {
                limbs_eq_mod_limb_ref_val(xs, ys, m)
            }
            (&Natural(Large(ref xs)), Natural(Small(y)), &Natural(Large(ref m))) => {
                limbs_eq_limb_mod_ref_ref(xs, y, m)
            }
            (&Natural(Small(x)), Natural(Large(ref mut ys)), &Natural(Large(ref m))) => {
                limbs_eq_limb_mod_val_ref(ys, x, m)
            }
            (&Natural(Large(ref xs)), Natural(Large(ref mut ys)), &Natural(Large(ref m))) => {
                limbs_eq_mod_ref_val_ref(xs, ys, m)
            }
        }
    }
}

impl<'a, 'b> EqMod<&'b Natural, Natural> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural` `m`;
    /// that is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each other
    /// mod 0 iff they are equal. `self` and `other` are taken by reference, and `m` is taken by
    /// value.
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
    fn eq_mod(self, other: &'b Natural, m: Natural) -> bool {
        match (self, other, m) {
            (x, y, natural_zero!()) => x == y,
            (x, &natural_zero!(), m) => x.divisible_by(m),
            (&natural_zero!(), y, m) => y.divisible_by(m),
            (x, &Natural(Small(y)), Natural(Small(m))) => x.eq_mod_limb(y, m),
            (&Natural(Small(x)), y, Natural(Small(m))) => y.eq_mod_limb(x, m),
            (&Natural(Small(x)), &Natural(Small(y)), _) => x == y,
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), Natural(Small(m))) => {
                limbs_eq_mod_limb_ref_ref(xs, ys, m)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), Natural(Large(ref mut m))) => {
                limbs_eq_limb_mod_ref_val(xs, y, m)
            }
            (&Natural(Small(x)), &Natural(Large(ref ys)), Natural(Large(ref mut m))) => {
                limbs_eq_limb_mod_ref_val(ys, x, m)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), Natural(Large(ref mut m))) => {
                limbs_eq_mod_ref_ref_val(xs, ys, m)
            }
        }
    }
}

impl<'a, 'b, 'c> EqMod<&'b Natural, &'c Natural> for &'a Natural {
    /// Returns whether this `Natural` is equivalent to another `Natural` mod a third `Natural` `m`;
    /// that is, whether `self` - `other` is a multiple of `m`. Two numbers are equal to each other
    /// mod 0 iff they are equal. `self`, `other`, and `m` are all taken by reference.
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
    fn eq_mod(self, other: &'b Natural, m: &'c Natural) -> bool {
        match (self, other, m) {
            (x, y, &natural_zero!()) => x == y,
            (x, &natural_zero!(), m) => x.divisible_by(m),
            (&natural_zero!(), y, m) => y.divisible_by(m),
            (x, &Natural(Small(y)), &Natural(Small(m))) => x.eq_mod_limb(y, m),
            (&Natural(Small(x)), y, &Natural(Small(m))) => y.eq_mod_limb(x, m),
            (&Natural(Small(x)), &Natural(Small(y)), _) => x == y,
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Small(m))) => {
                limbs_eq_mod_limb_ref_ref(xs, ys, m)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), &Natural(Large(ref m))) => {
                limbs_eq_limb_mod_ref_ref(xs, y, m)
            }
            (&Natural(Small(x)), &Natural(Large(ref ys)), &Natural(Large(ref m))) => {
                limbs_eq_limb_mod_ref_ref(ys, x, m)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Large(ref m))) => {
                limbs_eq_mod_ref_ref_ref(xs, ys, m)
            }
        }
    }
}
