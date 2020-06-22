use malachite_base::num::arithmetic::traits::{WrappingAddAssign, WrappingSubAssign, XMulYIsZZ};
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::conversion::traits::{JoinHalves, SplitInHalf};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_nz::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use malachite_nz::natural::arithmetic::div::_div_by_preinversion;
use malachite_nz::natural::arithmetic::div_mod::{_div_mod_by_preinversion, limbs_invert_limb};
use malachite_nz::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use malachite_nz::platform::{DoubleLimb, Limb};

/// The high bit of `d` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c, GMP 6.1.2, with
/// DIV_QR_1N_METHOD == 2, where qp == up, but not computing the remainder.
fn limbs_div_limb_normalized_in_place(ns: &mut [Limb], ns_high: Limb, d: Limb, d_inv: Limb) {
    let len = ns.len();
    if len == 1 {
        ns[0] = _div_by_preinversion(ns_high, ns[0], d, d_inv);
        return;
    }
    let power_of_two = d.wrapping_neg().wrapping_mul(d_inv);
    let (mut q_high, mut q_low) = Limb::x_mul_y_is_zz(d_inv, ns_high);
    q_high.wrapping_add_assign(ns_high);
    let second_highest_limb = ns[len - 1];
    ns[len - 1] = q_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(second_highest_limb, ns[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(ns_high));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (t, r) = Limb::x_mul_y_is_zz(sum_high, d_inv);
        let mut q = DoubleLimb::from(sum_high) + DoubleLimb::from(t) + DoubleLimb::from(q_low);
        q_low = r;
        if big_carry {
            q.wrapping_add_assign(DoubleLimb::join_halves(1, d_inv));
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(d);
                q.wrapping_add_assign(1);
            }
        }
        let (q_higher, q_high) = q.split_in_half();
        ns[j + 1] = q_high;
        assert!(!limbs_slice_add_limb_in_place(&mut ns[j + 2..], q_higher,));
        let (sum, carry) = DoubleLimb::join_halves(sum_low, ns[j])
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut q_high = 0;
    if big_carry {
        q_high += 1;
        sum_high.wrapping_sub_assign(d);
    }
    if sum_high >= d {
        q_high += 1;
        sum_high.wrapping_sub_assign(d);
    }
    let t = _div_by_preinversion(sum_high, sum_low, d, d_inv);
    let (q_high, q_low) = DoubleLimb::join_halves(q_high, q_low)
        .wrapping_add(DoubleLimb::from(t))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(&mut ns[1..], q_high));
    ns[0] = q_low;
}

/// The high bit of `d` must be set.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// This is mpn_div_qr_1n_pi1 from mpn/generic/div_qr_1n_pi1.c, GMP 6.1.2, with
/// DIV_QR_1N_METHOD == 2, but not computing the remainder.
fn limbs_div_limb_normalized_to_out(
    out: &mut [Limb],
    ns: &[Limb],
    ns_high: Limb,
    d: Limb,
    d_inv: Limb,
) {
    let len = ns.len();
    if len == 1 {
        out[0] = _div_by_preinversion(ns_high, ns[0], d, d_inv);
        return;
    }
    let power_of_two = d.wrapping_neg().wrapping_mul(d_inv);
    let (mut q_high, mut q_low) = Limb::x_mul_y_is_zz(d_inv, ns_high);
    q_high.wrapping_add_assign(ns_high);
    out[len - 1] = q_high;
    let (sum, mut big_carry) = DoubleLimb::join_halves(ns[len - 1], ns[len - 2])
        .overflowing_add(DoubleLimb::from(power_of_two) * DoubleLimb::from(ns_high));
    let (mut sum_high, mut sum_low) = sum.split_in_half();
    for j in (0..len - 2).rev() {
        let (t, r) = Limb::x_mul_y_is_zz(sum_high, d_inv);
        let mut q = DoubleLimb::from(sum_high) + DoubleLimb::from(t) + DoubleLimb::from(q_low);
        q_low = r;
        if big_carry {
            q.wrapping_add_assign(DoubleLimb::join_halves(1, d_inv));
            let (sum, carry) = sum_low.overflowing_add(power_of_two);
            sum_low = sum;
            if carry {
                sum_low.wrapping_sub_assign(d);
                q.wrapping_add_assign(1);
            }
        }
        let (q_higher, q_high) = q.split_in_half();
        out[j + 1] = q_high;
        assert!(!limbs_slice_add_limb_in_place(&mut out[j + 2..], q_higher,));
        let (sum, carry) = DoubleLimb::join_halves(sum_low, ns[j])
            .overflowing_add(DoubleLimb::from(sum_high) * DoubleLimb::from(power_of_two));
        sum_high = sum.upper_half();
        sum_low = sum.lower_half();
        big_carry = carry;
    }
    let mut q_high = 0;
    if big_carry {
        q_high += 1;
        sum_high.wrapping_sub_assign(d);
    }
    if sum_high >= d {
        q_high += 1;
        sum_high.wrapping_sub_assign(d);
    }
    let t = _div_by_preinversion(sum_high, sum_low, d, d_inv);
    let (q_high, q_low) = DoubleLimb::join_halves(q_high, q_low)
        .wrapping_add(DoubleLimb::from(t))
        .split_in_half();
    assert!(!limbs_slice_add_limb_in_place(&mut out[1..], q_high));
    out[0] = q_low;
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c, GMP 6.1.2, but not computing the remainder.
/// Experiments show that this is always slower than `_limbs_div_limb_to_out`.
pub fn limbs_div_limb_to_out_alt(out: &mut [Limb], ns: &[Limb], d: Limb) {
    assert_ne!(d, 0);
    let len = ns.len();
    assert!(len > 1);
    assert!(out.len() >= len);
    let len_minus_1 = len - 1;
    let mut highest_limb = ns[len_minus_1];
    let bits = LeadingZeros::leading_zeros(d);
    if bits == 0 {
        let adjust = highest_limb >= d;
        if adjust {
            highest_limb -= d;
        }
        out[len_minus_1] = Limb::iverson(adjust);
        let d_inv = limbs_invert_limb(d);
        limbs_div_limb_normalized_to_out(out, &ns[..len_minus_1], highest_limb, d, d_inv)
    } else {
        let d = d << bits;
        let ns_high = limbs_shl_to_out(out, ns, bits);
        let d_inv = limbs_invert_limb(d);
        let (q, r) = _div_mod_by_preinversion(ns_high, out[len_minus_1], d, d_inv);
        out[len_minus_1] = q;
        limbs_div_limb_normalized_in_place(&mut out[..len_minus_1], r, d, d_inv)
    }
}

/// This is mpn_div_qr_1 from mpn/generic/div_qr_1.c, GMP 6.1.2, where qp == up, but not computing
/// the remainder. Experiments show that this is always slower than `_limbs_div_limb_in_place`.
pub fn limbs_div_limb_in_place_alt(ns: &mut [Limb], d: Limb) {
    assert_ne!(d, 0);
    let len = ns.len();
    assert!(len > 1);
    let len_minus_1 = len - 1;
    let mut highest_limb = ns[len_minus_1];
    let bits = LeadingZeros::leading_zeros(d);
    if bits == 0 {
        let adjust = highest_limb >= d;
        if adjust {
            highest_limb -= d;
        }
        ns[len_minus_1] = Limb::iverson(adjust);
        let d_inv = limbs_invert_limb(d);
        limbs_div_limb_normalized_in_place(&mut ns[..len_minus_1], highest_limb, d, d_inv)
    } else {
        let d = d << bits;
        let ns_high = limbs_slice_shl_in_place(ns, bits);
        let d_inv = limbs_invert_limb(d);
        let (q, r) = _div_mod_by_preinversion(ns_high, ns[len_minus_1], d, d_inv);
        ns[len_minus_1] = q;
        limbs_div_limb_normalized_in_place(&mut ns[..len_minus_1], r, d, d_inv)
    }
}
