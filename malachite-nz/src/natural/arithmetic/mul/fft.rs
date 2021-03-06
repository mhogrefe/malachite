use fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    DivisibleByPowerOf2, Parity, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::slices::slice_set_zero;
use natural::arithmetic::add::{
    limbs_add_to_out, limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mul::limbs_mul_same_length_to_out;
use natural::arithmetic::mul::mul_mod::{
    _limbs_mul_mod_base_pow_n_minus_1, _limbs_mul_mod_base_pow_n_minus_1_next_size,
    _limbs_mul_mod_base_pow_n_minus_1_scratch_len, MULMOD_BNM1_THRESHOLD, MUL_FFT_MODF_THRESHOLD,
};
use natural::arithmetic::mul::square_mod::_limbs_square_mod_base_pow_n_minus_1_next_size;
use natural::arithmetic::mul::square_mod::{
    _limbs_square_mod_base_pow_n_minus_1, _limbs_square_mod_base_pow_n_minus_1_scratch_len,
};
use natural::arithmetic::shl::{limbs_shl_to_out, limbs_shl_with_complement_to_out};
use natural::arithmetic::square::limbs_square_to_out;
use natural::arithmetic::sub::{
    limbs_sub_in_place_left, limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::comparison::ord::limbs_cmp_same_length;
use natural::logic::not::limbs_not_to_out;
use platform::{Limb, SQR_TOOM3_THRESHOLD};
use std::cmp::{max, Ordering};

//TODO double check this
pub(crate) const SQR_FFT_MODF_THRESHOLD: usize = SQR_TOOM3_THRESHOLD * 3;

pub fn _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs_len: usize, ys_len: usize) -> bool {
    xs_len != 0 && xs_len >= ys_len && {
        let n = _limbs_mul_mod_base_pow_n_minus_1_next_size(xs_len + ys_len);
        n.even() && n >= MULMOD_BNM1_THRESHOLD
    }
}

const fn get_k_by_index(table: &[(usize, usize)], index: usize) -> usize {
    table[index].1
}

const fn get_n(entry: &(usize, usize)) -> usize {
    entry.0
}

const fn get_k(entry: &(usize, usize)) -> usize {
    entry.1
}

const MUL_FFT_TABLE3_SIZE: usize = 208;

//TODO tune!! Format is (n, k)
// from mpn/*/*/gmp-mparam.h
const MUL_FFT_TABLE3: [(usize, usize); MUL_FFT_TABLE3_SIZE] = [
    (396, 5),
    (19, 6),
    (10, 5),
    (21, 6),
    (11, 5),
    (23, 6),
    (21, 7),
    (11, 6),
    (25, 7),
    (13, 6),
    (27, 7),
    (21, 8),
    (11, 7),
    (25, 8),
    (13, 7),
    (27, 8),
    (15, 7),
    (31, 8),
    (17, 7),
    (35, 8),
    (21, 9),
    (11, 8),
    (27, 9),
    (15, 8),
    (33, 9),
    (19, 8),
    (39, 9),
    (23, 8),
    (47, 9),
    (27, 10),
    (15, 9),
    (39, 10),
    (23, 9),
    (51, 11),
    (15, 10),
    (31, 9),
    (67, 10),
    (39, 9),
    (79, 10),
    (47, 9),
    (95, 10),
    (55, 11),
    (31, 10),
    (79, 11),
    (47, 10),
    (95, 12),
    (31, 11),
    (63, 10),
    (135, 11),
    (79, 10),
    (159, 11),
    (95, 10),
    (191, 9),
    (383, 12),
    (63, 11),
    (127, 10),
    (255, 9),
    (511, 11),
    (143, 10),
    (287, 9),
    (575, 10),
    (303, 11),
    (159, 10),
    (319, 12),
    (95, 11),
    (191, 10),
    (383, 11),
    (207, 13),
    (63, 12),
    (127, 11),
    (255, 10),
    (511, 11),
    (271, 10),
    (543, 11),
    (287, 10),
    (575, 11),
    (303, 12),
    (159, 11),
    (319, 10),
    (639, 11),
    (351, 10),
    (703, 11),
    (367, 10),
    (735, 11),
    (383, 10),
    (767, 11),
    (415, 10),
    (831, 12),
    (223, 11),
    (479, 13),
    (127, 12),
    (255, 11),
    (511, 10),
    (1023, 11),
    (543, 12),
    (287, 11),
    (575, 10),
    (1151, 11),
    (607, 12),
    (319, 11),
    (671, 12),
    (351, 11),
    (735, 12),
    (383, 11),
    (767, 12),
    (415, 11),
    (831, 12),
    (447, 11),
    (895, 12),
    (479, 14),
    (127, 13),
    (255, 12),
    (511, 11),
    (1023, 12),
    (543, 11),
    (1087, 12),
    (575, 11),
    (1151, 12),
    (607, 13),
    (319, 12),
    (735, 13),
    (383, 12),
    (831, 13),
    (447, 12),
    (959, 14),
    (255, 13),
    (511, 12),
    (1087, 13),
    (575, 12),
    (1215, 13),
    (639, 12),
    (1279, 13),
    (703, 12),
    (1407, 14),
    (383, 13),
    (767, 12),
    (1535, 13),
    (831, 12),
    (1663, 13),
    (959, 14),
    (511, 13),
    (1087, 12),
    (2175, 13),
    (1215, 14),
    (639, 13),
    (1343, 12),
    (2687, 13),
    (1407, 12),
    (2815, 13),
    (1471, 14),
    (767, 13),
    (1663, 14),
    (895, 13),
    (1919, 15),
    (511, 14),
    (1023, 13),
    (2175, 14),
    (1151, 13),
    (2431, 12),
    (4863, 14),
    (1279, 13),
    (2687, 14),
    (1407, 13),
    (2815, 15),
    (767, 14),
    (1535, 13),
    (3_071, 14),
    (1663, 13),
    (3455, 14),
    (1919, 16),
    (511, 15),
    (1023, 14),
    (2431, 13),
    (4863, 15),
    (1279, 14),
    (2943, 13),
    (5887, 15),
    (1535, 14),
    (3455, 15),
    (1791, 14),
    (3839, 13),
    (7679, 16),
    (1023, 15),
    (2047, 14),
    (4223, 15),
    (2303, 14),
    (4863, 15),
    (2815, 14),
    (5887, 16),
    (1535, 15),
    (3327, 14),
    (6911, 15),
    (3839, 14),
    (7679, 17),
    (1023, 16),
    (2047, 15),
    (4863, 16),
    (2559, 15),
    (5887, 14),
    (11775, 16),
    (0x10000, 17),
    (0x20000, 18),
    (0x40000, 19),
    (0x80000, 20),
    (0x100000, 21),
    (0x200000, 22),
    (0x400000, 23),
    (0x800000, 24),
];

const SQR_FFT_TABLE3_SIZE: usize = 203;

//TODO tune!! Format is (n, k)
// from mpn/*/*/gmp-mparam.h
const SQR_FFT_TABLE3: [(usize, usize); SQR_FFT_TABLE3_SIZE] = [
    (340, 5),
    (11, 4),
    (23, 5),
    (21, 6),
    (11, 5),
    (23, 6),
    (25, 7),
    (13, 6),
    (27, 7),
    (25, 8),
    (13, 7),
    (28, 8),
    (15, 7),
    (31, 8),
    (21, 9),
    (11, 8),
    (27, 9),
    (15, 8),
    (35, 9),
    (19, 8),
    (41, 9),
    (23, 8),
    (47, 9),
    (27, 10),
    (15, 9),
    (39, 10),
    (23, 9),
    (51, 11),
    (15, 10),
    (31, 9),
    (63, 10),
    (39, 9),
    (79, 10),
    (55, 11),
    (31, 10),
    (79, 11),
    (47, 10),
    (95, 12),
    (31, 11),
    (63, 10),
    (127, 9),
    (255, 10),
    (135, 11),
    (79, 10),
    (159, 9),
    (319, 11),
    (95, 10),
    (191, 9),
    (383, 12),
    (63, 11),
    (127, 10),
    (255, 9),
    (511, 10),
    (271, 9),
    (543, 11),
    (143, 10),
    (287, 9),
    (575, 10),
    (303, 9),
    (607, 11),
    (159, 10),
    (319, 9),
    (639, 12),
    (95, 11),
    (191, 10),
    (383, 11),
    (207, 13),
    (63, 12),
    (127, 11),
    (255, 10),
    (511, 11),
    (271, 10),
    (543, 11),
    (287, 10),
    (575, 11),
    (303, 10),
    (607, 11),
    (319, 10),
    (639, 11),
    (335, 10),
    (671, 11),
    (351, 10),
    (703, 11),
    (367, 12),
    (191, 11),
    (383, 10),
    (767, 11),
    (415, 10),
    (831, 12),
    (223, 11),
    (479, 13),
    (127, 12),
    (255, 11),
    (543, 12),
    (287, 11),
    (607, 12),
    (319, 11),
    (671, 12),
    (351, 11),
    (703, 13),
    (191, 12),
    (383, 11),
    (767, 12),
    (415, 11),
    (831, 12),
    (447, 11),
    (895, 12),
    (479, 14),
    (127, 13),
    (255, 12),
    (543, 11),
    (1087, 12),
    (607, 13),
    (319, 12),
    (735, 13),
    (383, 12),
    (831, 13),
    (447, 12),
    (959, 13),
    (511, 12),
    (1087, 13),
    (575, 12),
    (1215, 13),
    (639, 12),
    (1343, 13),
    (703, 12),
    (1407, 14),
    (383, 13),
    (767, 12),
    (1535, 13),
    (831, 12),
    (1663, 13),
    (959, 14),
    (511, 13),
    (1087, 12),
    (2175, 13),
    (1215, 14),
    (639, 13),
    (1343, 12),
    (2687, 13),
    (1407, 12),
    (2815, 13),
    (1471, 14),
    (767, 13),
    (1599, 12),
    (3199, 13),
    (1663, 14),
    (895, 13),
    (1791, 15),
    (511, 14),
    (1023, 13),
    (2175, 14),
    (1151, 13),
    (2431, 12),
    (4863, 14),
    (1279, 13),
    (2687, 14),
    (1407, 13),
    (2815, 15),
    (767, 14),
    (1535, 13),
    (3199, 14),
    (1663, 13),
    (3455, 12),
    (6911, 14),
    (1791, 16),
    (511, 15),
    (1023, 14),
    (2431, 13),
    (4863, 15),
    (1279, 14),
    (2943, 13),
    (5887, 15),
    (1535, 14),
    (3455, 13),
    (6911, 15),
    (1791, 14),
    (3839, 16),
    (1023, 15),
    (2047, 14),
    (4223, 15),
    (2303, 14),
    (4863, 15),
    (2815, 14),
    (5887, 16),
    (1535, 15),
    (3327, 14),
    (6911, 15),
    (3839, 17),
    (1023, 16),
    (2047, 15),
    (4863, 16),
    (2559, 15),
    (5887, 14),
    (11775, 16),
    (0x10000, 17),
    (0x20000, 18),
    (0x40000, 19),
    (0x80000, 20),
    (0x100000, 21),
    (0x200000, 22),
    (0x400000, 23),
    (0x800000, 24),
];

/// Find the best k to use for a mod 2<sup>m * Limb::WIDTH</sup> + 1 FFT for m >= n.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_fft_best_k from mpn/generic/mul_fft.c, GMP 6.1.2, the mpn_fft_table3 variant.
pub(crate) fn _limbs_mul_fft_best_k(n: usize, square: bool) -> usize {
    let fft_table: &[(usize, usize)] = if square {
        &SQR_FFT_TABLE3
    } else {
        &MUL_FFT_TABLE3
    };
    let mut last_k = get_k_by_index(fft_table, 0);
    for entry in &fft_table[1..] {
        let threshold = get_n(entry) << last_k;
        if n <= usize::exact_from(threshold) {
            break;
        }
        last_k = get_k(entry);
    }
    last_k
}

/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = `xs.len()`
///
/// This is mpn_fft_mul from gmp-impl.h, GMP 6.2.1, and mpn_nussbaumer_mul from
/// mpn/generic/mpn_nussbaumer_mul.c, GMP 6.1.2.
#[inline]
pub fn _limbs_mul_greater_to_out_fft(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert_ne!(ys_len, 0);
    if std::ptr::eq(xs, ys) {
        let n = _limbs_square_mod_base_pow_n_minus_1_next_size(xs_len << 1);
        let mut scratch = vec![0; _limbs_square_mod_base_pow_n_minus_1_scratch_len(n, xs_len)];
        _limbs_square_mod_base_pow_n_minus_1(out, n, xs, &mut scratch);
    } else {
        let n = _limbs_mul_mod_base_pow_n_minus_1_next_size(xs_len + ys_len);
        let mut scratch = vec![0; _limbs_mul_mod_base_pow_n_minus_1_scratch_len(n, xs_len, ys_len)];
        _limbs_mul_mod_base_pow_n_minus_1(out, n, xs, ys, &mut scratch);
    }
}

/// Initialize table[i][j] with bitrev(j).
///
/// Time: worst case O(2<sup>k</sup>)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_fft_initl from mpn/generic/mul_fft.c, GMP 6.1.2.
fn _limbs_mul_fft_bit_reverse_table(mut scratch: &mut [usize], k: usize) -> Vec<&[usize]> {
    let mut table = Vec::with_capacity(k + 1);
    for i in 0..=k {
        // force scratch to move rather than be borrowed
        let (scratch_lo, scratch_hi) = { scratch }.split_at_mut(1 << i);
        table.push(scratch_lo);
        scratch = scratch_hi;
    }
    table[0][0] = 0;
    let mut shift = 1;
    for i in 1..=k {
        for j in 0..shift {
            table[i][j] = table[i - 1][j] << 1;
            table[i][shift + j] = table[i][j] + 1;
        }
        shift <<= 1;
    }
    table.into_iter().map(|row| &*row).collect()
}

/// Return the LCM of a and 2<sup>k</sup>.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// This is mpn_mul_fft_lcm from mpn/generic/mul_fft.c, GMP 6.1.2.
fn _limbs_mul_fft_lcm_of_a_and_two_pow_k(a: usize, k: usize) -> usize {
    a << k.saturating_sub(usize::exact_from(a.trailing_zeros()))
}

/// Given xs with xs[n] <= 1, reduce it modulo 2<sup>n * Limb::WIDTH</sup> + 1, by subtracting that
/// modulus if necessary.
///
/// If xs is exactly 2<sup>n * Limb::WIDTH</sup> then limbs_sub_limb_in_place produces a borrow and
/// the limbs must be zeroed out again. This will occur very infrequently.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_fft_normalize from mpn/generic/mul_fft.c, GMP 6.1.2.
fn _limbs_mul_fft_normalize(xs: &mut [Limb]) {
    if *xs.last().unwrap() != 0 {
        assert!(!limbs_sub_limb_in_place(xs, 1));
        let (xs_last, xs_init) = xs.split_last_mut().unwrap();
        if *xs_last == 0 {
            slice_set_zero(xs_init);
            *xs_last = 1;
        } else {
            *xs_last = 0;
        }
    }
}

/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_fft_add_modF from mpn/generic/mul_fft.c, GMP 6.1.2, where r == a.
fn _limbs_mul_fft_add_mod_f_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    let (xs_last, xs_init) = xs.split_last_mut().unwrap();
    let (ys_last, ys_init) = ys.split_last().unwrap();
    let mut carry = xs_last.wrapping_add(*ys_last);
    if limbs_slice_add_same_length_in_place_left(xs_init, ys_init) {
        carry.wrapping_add_assign(1);
    }
    // 0 <= carry <= 3
    let sub = carry > 1;
    *xs_last = if sub {
        1 // r[n] - carry = 1
    } else {
        carry
    };
    if sub {
        assert!(!limbs_sub_limb_in_place(xs, carry - 1));
    }
}

/// out <- xs - ys mod 2<sup>n * Limb::WIDTH</sup> + 1. Assumes xs and ys are semi-normalized.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_fft_sub_modF from mpn/generic/mul_fft.c, GMP 6.1.2.
fn _limbs_mul_fft_sub_mod_f_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let n = xs.len() - 1;
    let mut carry = xs[n].wrapping_sub(ys[n]);
    if limbs_sub_same_length_to_out(out, &xs[..n], &ys[..n]) {
        carry.wrapping_sub_assign(1);
    }
    // -2 <= carry <= 1
    if carry.get_highest_bit() {
        out[n] = 0;
        assert!(!limbs_slice_add_limb_in_place(out, carry.wrapping_neg()));
    } else {
        out[n] = carry;
    }
}

/// out <- xs * 2<sup>bits</sup> mod 2<sup>n * Limb::WIDTH</sup> + 1. Assumes xs is semi-normalized,
/// i.e. xs[n] <= 1. out and xs must have n + 1 limbs.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_fft_mul_2exp_modF from mpn/generic/mul_fft.c, GMP 6.1.2.
fn _limbs_mul_fft_shl_mod_f_to_out(out: &mut [Limb], xs: &[Limb], bits: usize) {
    let n = xs.len() - 1;
    let small_bits = u64::exact_from(bits) & Limb::WIDTH_MASK;
    let mut shift = bits >> Limb::LOG_WIDTH;
    // negate
    if shift >= n {
        // out[0..shift - 1]  <-- xs[n - shift..n - 1] << small_bits
        // out[shift..n - 1]  <-- -xs[0..n - shift-1] << small_bits
        shift -= n;
        let (xs_lo, xs_hi) = xs.split_at(n - shift);
        let mut carry2;
        let mut carry = if small_bits == 0 {
            out[..shift].copy_from_slice(&xs_hi[..shift]);
            carry2 = xs[n];
            limbs_not_to_out(&mut out[shift..], xs_lo);
            0
        } else {
            // no out shift below since xs[n] <= 1
            limbs_shl_to_out(out, &xs_hi[..shift + 1], small_bits);
            carry2 = out[shift];
            limbs_shl_with_complement_to_out(&mut out[shift..], xs_lo, small_bits)
        };

        // add carry to out[0], and add carry2 to out[shift]
        // now add 1 in out[shift], subtract 1 in out[n], i.e. add 1 in out[0]
        out[n] = 0;
        // carry < 2 ^ small_bits <= 2 ^ (Limb::WIDTH - 1) thus no overflow here
        carry += 1;
        limbs_slice_add_limb_in_place(out, carry);
        carry2.wrapping_add_assign(1);
        // carry2 might overflow when small_bits = Limb::WIDTH - 1
        if carry2 == 0 {
            limbs_slice_add_limb_in_place(&mut out[shift + 1..], 1);
        } else {
            limbs_slice_add_limb_in_place(&mut out[shift..], carry2);
        }
    } else {
        let carry2;
        // out[0..shift - 1]  <-- -xs[n - shift..n - 1] << small_bits
        // out[shift..n - 1]  <-- xs[0..n - shift - 1] << small_bits
        let (xs_lo, xs_hi) = xs.split_at(n - shift);
        let mut carry = if small_bits != 0 {
            // no out bits below since xs[n] <= 1
            limbs_shl_with_complement_to_out(out, &xs_hi[..shift + 1], small_bits);
            carry2 = !out[shift];
            // out[..shift + 1] = xs[n - shift, n + 1] << small_bits
            // out[shift..n] = xs[n - shift..] << small_bits
            limbs_shl_to_out(&mut out[shift..], xs_lo, small_bits)
        } else {
            // out[shift] is not used below, but we save xs test for shift = 0
            limbs_not_to_out(out, &xs_hi[..shift + 1]);
            carry2 = xs[n];
            out[shift..n].copy_from_slice(xs_lo);
            0
        };

        // now complement out[..shift], subtract carry from out[0], subtract carry2 from out[shift].
        // If shift == 0 we just have out[0] = xs[n] << small_bits.
        if shift != 0 {
            // now add 1 in out[0], subtract 1 in out[shift], then add 1 to out[0]
            if carry == 0 {
                if limbs_slice_add_limb_in_place(&mut out[..n], 1) {
                    carry = 1;
                }
            } else {
                carry -= 1;
            }
            // add 1 to carry instead of carry2 since carry2 might overflow
            carry = Limb::iverson(limbs_sub_limb_in_place(&mut out[..shift], carry)) + 1;
        }

        // now subtract carry and carry2 from out[shift..n]
        let (out_last, out_init) = out[..n + 1].split_last_mut().unwrap();
        let out_init_hi = &mut out_init[shift..];
        *out_last = Limb::iverson(limbs_sub_limb_in_place(out_init_hi, carry)).wrapping_neg();
        if limbs_sub_limb_in_place(out_init_hi, carry2) {
            out_last.wrapping_sub_assign(1);
        }
        if out_last.get_highest_bit() {
            *out_last = Limb::iverson(limbs_slice_add_limb_in_place(out_init, 1));
        }
    }
}

/// out <- xs / 2<sup>bits</sup> mod 2<sup>n * Limb::WIDTH</sup> + 1
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// This is mpn_fft_div_2exp_modF from mpn/generic/mul_fft.c, GMP 6.1.2.
fn _limbs_mul_fft_shr_mod_f_to_out(out: &mut [Limb], xs: &[Limb], bits: usize) {
    _limbs_mul_fft_shl_mod_f_to_out(out, xs, ((xs.len() - 1) << (Limb::LOG_WIDTH + 1)) - bits);
    // 1 / 2 ^ bits = 2 ^ (2 * n' * L - bits) mod 2 ^ (n' * Limb::WIDTH) + 1 where n' = xs.len() - 1
    // normalize so that out < 2 ^ (n' * Limb::WIDTH) + 1
    _limbs_mul_fft_normalize(out);
}

/// store in A[0..width - 1] the first m bits from xs[..len], in A[width..] the following m bits.
/// Assumes m is a multiple of Limb::WIDTH (M = n * Limb::WIDTH). T must have space for at least
/// `width` limbs. We must have xs.len() <= 2 * k * n.
///
/// Time: worst case O(a)
///
/// Additional memory: worst case O(a)
///
/// where a = `xs.len()`
///
/// This is mpn_mul_fft_decompose from mpn/generic/mul_fft.c, GMP 6.1.2. nl is omitted as it is just
/// the length of n (here, xs). nprime is `width` - 1.
fn _limbs_mul_fft_decompose<'a>(
    a_s: &'a mut [Limb],
    k: usize,
    width: usize,
    xs: &[Limb],
    n: usize,
    m: usize,
    scratch: &mut [Limb],
) -> Vec<&'a mut [Limb]> {
    let mut len = xs.len();
    let k_times_n = k * n;
    // normalize xs mod 2 ^ (k * n * Limb::WIDTH) + 1
    let mut scratch2;
    let mut source: &[Limb] = if len > k_times_n {
        let diff = len - k_times_n;
        scratch2 = vec![0; k_times_n + 1];
        // difference > k_times_n -> len - k * n > k * n -> len > 2 * k * n, which violates the
        // precondition len <= 2 * k * n. So difference > k_times_n cannot happen.
        assert!(diff <= k_times_n);
        // difference <= k_times_n, i.e. len <= 2 * k_times_n
        let (xs_lo, xs_hi) = xs.split_at(k_times_n);
        scratch2[k_times_n] = Limb::iverson(
            limbs_sub_to_out(&mut scratch2, xs_lo, &xs_hi[..diff])
                && limbs_slice_add_limb_in_place(&mut scratch2[..k_times_n], 1),
        );
        len = k_times_n + 1;
        &scratch2
    } else {
        xs
    };
    let mut a_table = Vec::with_capacity(k);
    let mut remainder: &mut [Limb] = a_s;
    for i in 0..k {
        // force remainder to move rather than be borrowed
        let (a_lo, a_hi) = { remainder }.split_at_mut(width);
        remainder = a_hi;
        // store the next m bits of xs into a[0..width_minus_one]
        // len is the number of remaining limbs
        if len == 0 {
            slice_set_zero(a_lo);
        } else {
            let j = if n <= len && i < k - 1 { n } else { len }; // store j next limbs
            len -= j;
            if i != 0 {
                source = &source[n..];
            }
            let scratch = &mut scratch[..width];
            let (scratch_lo, scratch_hi) = scratch.split_at_mut(j);
            scratch_lo.copy_from_slice(&source[..j]);
            slice_set_zero(scratch_hi);
            _limbs_mul_fft_shl_mod_f_to_out(a_lo, scratch, i * m);
        }
        a_table.push(a_lo);
    }
    assert_eq!(len, 0);
    a_table
}

/// input: xss\[0\] ... xss[increment * (k - 1)] are residues mod 2 ^ N + 1 where
/// N = n * Limb::WIDTH, and 2 ^ omega is a primitive root mod 2 ^ N + 1.
/// output: xss[increment * bit_reverse_table[k]\[i\]] <-
///      sum (2 ^ omega) ^ (i * j) xss[increment * j] mod 2 ^ N + 1
///
/// Time: O(n * log(n) * log(log(n))), assuming k = O(log(n))
///
/// Additional memory: worst case O(1)
///
/// where n = `xss[0].len()`
///
/// This is mpn_fft_fft from mpn/generic/mul_fft.c, GMP 6.1.2.
pub fn _limbs_mul_fft_fft(
    xss: &mut [&mut [Limb]],
    k: usize,
    bit_reverse_table: &[&[usize]],
    bit_reverse_table_offset: usize,
    omega: usize,
    increment: usize,
    scratch: &mut [Limb],
) {
    assert_ne!(increment, 0);
    if k == 2 {
        let (xss_first, xss_tail) = xss.split_first_mut().unwrap();
        scratch.copy_from_slice(xss_first);
        limbs_slice_add_same_length_in_place_left(xss_first, xss_tail[increment - 1]);
        let (xss_0_last, xss_0_init) = xss_first.split_last_mut().unwrap();
        // can be 2 or 3
        if *xss_0_last > 1 {
            *xss_0_last = Limb::iverson(!limbs_sub_limb_in_place(xss_0_init, *xss_0_last - 1));
        }
        // xss[increment][n] can be -1 or -2
        if limbs_sub_same_length_in_place_right(scratch, &mut xss[increment]) {
            let (xss_increment_last, xss_increment_init) = xss[increment].split_last_mut().unwrap();
            *xss_increment_last = Limb::iverson(limbs_slice_add_limb_in_place(
                xss_increment_init,
                xss_increment_last.wrapping_neg(),
            ));
        }
    } else {
        let half_k = k >> 1;
        let twice_omega = omega << 1;
        let twice_increment = increment << 1;
        let offset_minus_one = bit_reverse_table_offset - 1;
        _limbs_mul_fft_fft(
            xss,
            half_k,
            bit_reverse_table,
            offset_minus_one,
            twice_omega,
            twice_increment,
            scratch,
        );
        _limbs_mul_fft_fft(
            &mut xss[increment..],
            half_k,
            bit_reverse_table,
            offset_minus_one,
            twice_omega,
            twice_increment,
            scratch,
        );
        // xss[2 * i * increment] <- xss[2 * j * increment] +
        //      omega ^ bit_reverse_table[k][2 * j * increment] * xss[(2 * j + 1) * increment]
        // xss[(2 * j + 1) * increment] <- xss[2 * j * increment] +
        //      omega ^ l[k][(2 * j + 1) * increment] * xss[(2 * j + 1) * increment]
        let bit_reverse_row = bit_reverse_table[bit_reverse_table_offset];
        let mut xss_offset = 0;
        for i in 0..half_k {
            // xss[increment] <- xss[0] + xss[increment] * 2 ^ (bit_reverse_row[1] * omega)
            // xss[0] <- xss[0] + xss[increment] * 2 ^ (bit_reverse_row[0] * omega)
            let (xss_lo, xss_hi) = xss.split_at_mut(xss_offset + increment);
            _limbs_mul_fft_shl_mod_f_to_out(
                scratch,
                xss_hi[0],
                bit_reverse_row[i << 1].wrapping_mul(omega),
            );
            _limbs_mul_fft_sub_mod_f_to_out(xss_hi[0], xss_lo[xss_offset], scratch);
            _limbs_mul_fft_add_mod_f_in_place_left(xss_lo[xss_offset], scratch);
            xss_offset += increment << 1;
        }
    }
}

/// input: xs ^ bit_reverse_table\[k\]\[0\], xs ^ bit_reverse_table\[k\]\[1\], ...,
///      xs ^ bit_reverse_table[k][K-1]
/// output: k * xss\[0\], k * xss[k - 1], ..., k * xss\[1\].
/// Assumes the xss are pseudo-normalized, i.e. 0 <= xss[]\[n\] <= 1. This condition is also
/// fulfilled at exit.
///
/// Time: O(n * log(n) * log(log(n))), assuming k = O(log(n))
///
/// Additional memory: worst case O(1)
///
/// where n = `xss[0].len()`
///
/// This is mpn_fft_fftinv from mpn/generic/mul_fft.c, GMP 6.1.2.
pub fn _limbs_mul_fft_inverse(
    xss: &mut [&mut [Limb]],
    k: usize,
    omega: usize,
    scratch: &mut [Limb],
) {
    if k == 2 {
        let (xss_first, xss_tail) = xss.split_first_mut().unwrap();
        scratch.copy_from_slice(xss_first);
        limbs_slice_add_same_length_in_place_left(xss_first, xss_tail[0]);
        // can be 2 or 3
        let (xss_0_last, xss_0_init) = xss_first.split_last_mut().unwrap();
        if *xss_0_last > 1 {
            *xss_0_last = Limb::iverson(!limbs_sub_limb_in_place(xss_0_init, *xss_0_last - 1));
        }
        // Ap[1][n] can be -1 or -2
        if limbs_sub_same_length_in_place_right(scratch, &mut xss_tail[0]) {
            let (xss_1_last, xss_1_init) = xss_tail[0].split_last_mut().unwrap();
            *xss_1_last = Limb::iverson(limbs_slice_add_limb_in_place(
                xss_1_init,
                xss_1_last.wrapping_neg(),
            ));
        }
    } else {
        let half_k = k >> 1;
        let twice_omega = omega << 1;
        _limbs_mul_fft_inverse(xss, half_k, twice_omega, scratch);
        _limbs_mul_fft_inverse(&mut xss[half_k..], half_k, twice_omega, scratch);
        // xss[i] <- xss[i] + omega ^ i * xss[i + k / 2]
        // xss[i + k / 2] <- xss[i] + omega ^ (i + k / 2) * xss[i + k / 2]
        for i in 0..half_k {
            // xss[k / 2] <- Ap[0] + Ap[k / 2] * 2 ^ ((i + k / 2) * omega)
            // xss[0] <- xss[0] + xss[K2] * 2 ^ (i * omega)
            let (xss_lo, xss_hi) = xss.split_at_mut(half_k + i);
            _limbs_mul_fft_shl_mod_f_to_out(scratch, xss_hi[0], i * omega);
            _limbs_mul_fft_sub_mod_f_to_out(xss_hi[0], xss_lo[i], scratch);
            _limbs_mul_fft_add_mod_f_in_place_left(xss_lo[i], scratch);
        }
    }
}

/// out[..n] <- xs[..an] mod 2 ^ (n * Limb::WIDTH) + 1, n <= an <= 3 * n.
/// Returns carry out, i.e. 1 iff xs[..an] == -1 mod 2 ^ (n * Limb::WIDTH) + 1, then out[..n] = 0.
///
/// Time: O(n)
///
/// Additional memory: O(1)
///
/// where n = `n`
///
/// This is mpn_fft_norm_modF from mpn/generic/mul_fft.c, GMP 6.1.2.
pub fn _limbs_mul_fft_normalize_mod_f(out: &mut [Limb], n: usize, xs: &[Limb]) -> bool {
    let xs_len = xs.len();
    assert!(n <= xs_len && xs_len <= 3 * n);
    let out = &mut out[..n];
    if xs_len >= 2 * n {
        // add xs[..m] and xs[2 * n..2 * n + m] in out[..m]
        // copy xs[m..n] to out[m..n]
        split_into_chunks!(xs, n, [xs_0, xs_1], xs_2);
        if limbs_add_to_out(out, xs_0, xs_2) {
            limbs_sub_same_length_in_place_left(out, xs_1);
            true
        } else {
            limbs_sub_in_place_left(out, xs_1) && limbs_slice_add_limb_in_place(out, 1)
        }
    } else {
        let (xs_lo, xs_hi) = xs.split_at(n);
        out.copy_from_slice(xs_lo);
        limbs_sub_in_place_left(out, xs_hi) && limbs_slice_add_limb_in_place(out, 1)
    }
}

/// Time: TODO
///
/// Additional memory: O(n * log(n))
///
/// where n = `xss[i].len()` (each slice in `xss` should have the same length)
///
/// This is mpn_fft_mul_modF_K from mpn/generic/mul_fft.c, GMP 6.1.2, where ap != bp. K is omitted
/// because it is unused; it is just the length of `xss` and `yss`.
fn _limbs_mul_fft_mul_mod_f_k(xss: &mut [&mut [Limb]], yss: &mut [&mut [Limb]]) {
    let n = xss[0].len() - 1;
    if n >= MUL_FFT_MODF_THRESHOLD {
        let k = _limbs_mul_fft_best_k(n, false);
        let two_pow_k = 1 << k;
        assert_eq!(n & (two_pow_k - 1), 0);
        let max_two_pow_k_width = max(two_pow_k, usize::wrapping_from(Limb::WIDTH));
        let m = n << Limb::LOG_WIDTH >> k;
        let p = n >> k;
        let mut q =
            (2 * m + k + 2 + max_two_pow_k_width) / max_two_pow_k_width * max_two_pow_k_width;
        // r = ceil((2 * m + k + 3) / max_two_pow_k_width) * max_two_pow_k_width
        let mut r = q >> Limb::LOG_WIDTH;
        // we should ensure that r is a multiple of the next k
        if r >= MUL_FFT_MODF_THRESHOLD {
            loop {
                let two_pow_best_k = 1 << _limbs_mul_fft_best_k(r, false);
                if r & (two_pow_best_k - 1) == 0 {
                    break;
                }
                r = (r + two_pow_best_k - 1) & two_pow_best_k.wrapping_neg();
                q = r << Limb::LOG_WIDTH;
                // warning: since r changed, two_pow_best_k may change too!
            }
        }
        assert!(r < n); // otherwise we'll loop
        let q_shifted = q >> k;
        let s = r + 1;
        // a.len() is n * log(n)
        let mut a = vec![0; s << (k + 1)];
        let (a, b) = a.split_at_mut(s << k);
        let mut scratch = vec![0; s << 1];
        let mut scratch2 = vec![0; 2 << k];
        let bit_reverse_table = _limbs_mul_fft_bit_reverse_table(&mut scratch2, k);
        for (xs, ys) in xss.iter_mut().zip(yss.iter_mut()) {
            _limbs_mul_fft_normalize(xs);
            _limbs_mul_fft_normalize(ys);
            let residues =
                _limbs_mul_fft_decompose(a, two_pow_k, s, xs, p, q_shifted, &mut scratch);
            _limbs_mul_fft_decompose(b, two_pow_k, s, ys, p, q_shifted, &mut scratch);
            xs[n] = Limb::iverson(_limbs_mul_fft_internal(
                xs,
                n,
                k,
                residues,
                b,
                p,
                q_shifted,
                &bit_reverse_table,
                &mut scratch,
                false,
            ));
        }
    } else {
        let mut scratch = vec![0; n << 1];
        for (xs, ys) in xss.iter_mut().zip(yss.iter_mut()) {
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            let (ys_last, ys_init) = ys.split_last_mut().unwrap();
            limbs_mul_same_length_to_out(&mut scratch, ys_init, xs_init);
            let scratch_hi = &mut scratch[n..];
            let mut carry = Limb::iverson(
                *xs_last != 0 && limbs_slice_add_same_length_in_place_left(scratch_hi, ys_init),
            );
            if *ys_last != 0 {
                if limbs_slice_add_same_length_in_place_left(scratch_hi, xs_init) {
                    carry += 1;
                }
                carry += *xs_last;
            }
            if carry != 0 {
                assert!(!limbs_slice_add_limb_in_place(&mut scratch, carry));
            }
            let (scratch_lo, scratch_hi) = scratch.split_at(n);
            *xs_last = Limb::iverson(
                limbs_sub_same_length_to_out(xs_init, scratch_lo, scratch_hi)
                    && limbs_slice_add_limb_in_place(xs_init, 1),
            );
        }
    }
}

/// Time: TODO
///
/// Additional memory: O(n * log(n))
///
/// where n = `xss[i].len()` (each slice in `xss` should have the same length)
///
/// This is mpn_fft_mul_modF_K from mpn/generic/mul_fft.c, GMP 6.1.2, where ap == bp. K is omitted
/// because it is unused; it is just the length of `xss`.
fn _limbs_mul_fft_mul_mod_f_k_square(xss: &mut [&mut [Limb]]) {
    let n = xss[0].len() - 1;
    if n >= SQR_FFT_MODF_THRESHOLD {
        let k = _limbs_mul_fft_best_k(n, false);
        let two_pow_k = 1 << k;
        assert_eq!(n & (two_pow_k - 1), 0);
        let max_k_pow_2_width = max(two_pow_k, usize::wrapping_from(Limb::WIDTH));
        let m = n << Limb::LOG_WIDTH >> k;
        let p = n >> k;
        let mut q = (2 * m + k + 2 + max_k_pow_2_width) / max_k_pow_2_width * max_k_pow_2_width;
        // r = ceil((2 * m + k + 3) / max_k_pow_2_width) * max_k_pow_2_width
        let mut r = q >> Limb::LOG_WIDTH;

        // we should ensure that r is a multiple of the next k
        if r >= SQR_FFT_MODF_THRESHOLD {
            loop {
                let two_pow_best_k = 1 << _limbs_mul_fft_best_k(r, true);
                if r & (two_pow_best_k - 1) == 0 {
                    break;
                }
                r = (r + two_pow_best_k - 1) & two_pow_best_k.wrapping_neg();
                q = r << Limb::LOG_WIDTH;
                // warning: since r changed, two_pow_best_k may change too!
            }
        }
        assert!(r < n); // otherwise we'll loop
        let q_shifted = q >> k;
        let s = r + 1;
        // a.len() is O(n * log(n))
        let mut a = vec![0; s << (k + 1)];
        let (a_lo, a_hi) = a.split_at_mut(s << k);
        let mut scratch = vec![0; s << 1];
        let mut scratch2 = vec![0; 2 << k];
        let bit_reverse_table = _limbs_mul_fft_bit_reverse_table(&mut scratch2, k);
        for xs in xss.iter_mut() {
            _limbs_mul_fft_normalize(xs);
            let residues =
                _limbs_mul_fft_decompose(a_lo, two_pow_k, s, xs, p, q_shifted, &mut scratch);
            xs[n] = Limb::iverson(_limbs_mul_fft_internal(
                xs,
                n,
                k,
                residues,
                a_hi,
                p,
                q_shifted,
                &bit_reverse_table,
                &mut scratch,
                true,
            ));
        }
    } else {
        let mut scratch = vec![0; n << 1];
        for xs in xss.iter_mut() {
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            limbs_square_to_out(&mut scratch, xs_init);
            if *xs_last != 0 {
                let carry = limbs_slice_add_mul_limb_same_length_in_place_left(
                    &mut scratch[n..],
                    xs_init,
                    2,
                )
                .wrapping_add(*xs_last);
                if carry != 0 {
                    assert!(!limbs_slice_add_limb_in_place(&mut scratch, carry));
                }
            }
            let (scratch_lo, scratch_hi) = scratch.split_at(n);
            *xs_last = Limb::iverson(
                limbs_sub_same_length_to_out(xs_init, scratch_lo, scratch_hi)
                    && limbs_slice_add_limb_in_place(xs_init, 1),
            );
        }
    }
}

/// Time: TODO
///
/// Additional memory: O(n * log(n))
///
/// where n = `xss[i].len()` (each slice in `xss` should have the same length)
///
/// This is mpn_mul_fft_internal from mpn/generic/mul_fft.c, GMP 6.1.2. A is excluded as it is
/// unused. nprime is `xss[0].len()` - 1.
pub fn _limbs_mul_fft_internal(
    out: &mut [Limb],
    p: usize,
    k: usize,
    mut xss: Vec<&mut [Limb]>,
    ys: &mut [Limb],
    a: usize,
    omega: usize,
    bit_reverse_table: &[&[usize]],
    scratch: &mut [Limb],
    square: bool,
) -> bool {
    let two_pow_k: usize = 1 << k;
    let twice_omega = omega << 1;
    let width = xss[0].len();
    let width_minus_one = width - 1;
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(width);
    // direct FFTs
    _limbs_mul_fft_fft(
        &mut xss,
        two_pow_k,
        bit_reverse_table,
        k,
        twice_omega,
        1,
        scratch_lo,
    );
    if square {
        // term to term multiplications
        _limbs_mul_fft_mul_mod_f_k_square(&mut xss);
    } else {
        let mut yss = Vec::with_capacity(two_pow_k);
        let mut remainder: &mut [Limb] = ys;
        for _ in 0..two_pow_k {
            // force remainder to move rather than be borrowed
            let (ys_lo, ys_hi) = { remainder }.split_at_mut(width);
            yss.push(ys_lo);
            remainder = ys_hi;
        }
        _limbs_mul_fft_fft(
            &mut yss,
            two_pow_k,
            bit_reverse_table,
            k,
            twice_omega,
            1,
            scratch_lo,
        );
        // term to term multiplications
        _limbs_mul_fft_mul_mod_f_k(&mut xss, &mut yss);
    }
    // inverse FFTs
    _limbs_mul_fft_inverse(&mut xss, two_pow_k, twice_omega, scratch_lo);
    // division of terms after inverse fft
    let mut yss = Vec::with_capacity(two_pow_k);
    yss.push(scratch_hi);
    _limbs_mul_fft_shr_mod_f_to_out(&mut yss[0], xss[0], k);
    for i in 1..two_pow_k {
        let (xss_lo, xss_hi) = xss.split_at_mut(i);
        _limbs_mul_fft_shr_mod_f_to_out(&mut xss_lo[i - 1], xss_hi[0], k + (two_pow_k - i) * omega);
    }
    yss.extend(xss.drain(..two_pow_k - 1));

    // addition of terms in result p
    slice_set_zero(scratch_lo);
    let q = a * (two_pow_k - 1) + width; // number of required limbs for p
    let ys = &mut ys[..q];
    // B has k * width limbs, which is >= q, i.e. enough
    slice_set_zero(ys);
    let mut carry = 0i64; // will accumulate the (signed) carry at p[q]
    let mut sh = a * two_pow_k;
    let mut lo = sh + width_minus_one;
    for i in (0..two_pow_k).rev() {
        lo -= a;
        sh -= a;
        let j = (two_pow_k - i) & (two_pow_k - 1);
        if limbs_slice_add_same_length_in_place_left(&mut ys[sh..sh + width], yss[j])
            && limbs_slice_add_limb_in_place(&mut ys[sh + width..], 1)
        {
            carry += 1;
        }
        // scratch = (i + 1) * 2 ^ (2 * M)
        let i_plus_one = Limb::exact_from(i) + 1;
        if 2 * a < width {
            scratch_lo[2 * a] = i_plus_one;
        } else {
            yss[0][2 * a - width] = i_plus_one;
        }
        if limbs_cmp_same_length(yss[j], scratch_lo) == Ordering::Greater {
            // subtract 2 ^ N' + 1
            if limbs_sub_limb_in_place(&mut ys[sh..], 1) {
                carry -= 1;
            }
            if limbs_sub_limb_in_place(&mut ys[lo..], 1) {
                carry -= 1;
            }
        }
    }
    match carry {
        -1 => {
            let ys = &mut ys[q - p - 1..];
            if limbs_slice_add_limb_in_place(&mut ys[1..], 1) {
                // p[q - p], ..., p[q - 1] are all zero
                limbs_sub_limb_in_place(ys, 1);
                limbs_sub_limb_in_place(&mut ys[p..], 1);
            }
        }
        1 => {
            fail_on_untested_path("_limbs_mul_fft_internal, carry == 1");
            if q >= p << 1 {
                let ys = &mut ys[q - (p << 1)..];
                if limbs_slice_add_limb_in_place(ys, 1) {
                    limbs_slice_add_limb_in_place(ys, 1);
                }
            } else {
                assert!(!limbs_sub_limb_in_place(&mut ys[q - p..], 1));
            }
        }
        _ => assert_eq!(carry, 0),
    }
    // here p < 2 ^ (2 * m) * [k * 2 ^ (m(k - 1)) + (k - 1) * 2 ^ (m * (k - 2)) + ...]
    // < k * 2 ^ (2 * m) * [2 ^ (m * (k - 1)) + 2 ^ (m * (k - 2)) + ...]
    // < k * 2 ^ (2 * m) * 2 ^ (m * (k - 1)) * 2 = 2 ^ (m * k + m + k + 1)
    _limbs_mul_fft_normalize_mod_f(out, p, ys)
}

/// Time: TODO
///
/// Additional memory: O(p * log(p))
///
/// assuming k = O(log(p)), xs.len() = O(p).
///
// This is mpn_mul_fft from mpn/generic/mul_fft.c, GMP 6.1.2.
pub(crate) fn _limbs_mul_fft(
    out: &mut [Limb],
    p: usize,
    xs: &[Limb],
    ys: &[Limb],
    k: usize,
) -> bool {
    let square = std::ptr::eq(xs, ys);
    assert!(p.divisible_by_power_of_2(u64::exact_from(k)));
    let n = p << Limb::LOG_WIDTH;
    let two_pow_k = 1 << k;
    let m = n >> k; // n == 2 ^ k * m
    let a = 1 + ((m - 1) >> Limb::LOG_WIDTH);
    // LCM(Limb::WIDTH, 2 ^ k)
    let lcm_with_two_pow_k =
        _limbs_mul_fft_lcm_of_a_and_two_pow_k(usize::wrapping_from(Limb::WIDTH), k);
    let mut big_width_minus_one = (1 + (2 * m + k + 2) / lcm_with_two_pow_k) * lcm_with_two_pow_k;
    let mut width_minus_one = big_width_minus_one >> Limb::LOG_WIDTH;
    // We should ensure that recursively, width_minus_one is a multiple of the next two_pow_k.
    if width_minus_one
        >= if square {
            SQR_FFT_MODF_THRESHOLD
        } else {
            MUL_FFT_MODF_THRESHOLD
        }
    {
        loop {
            let shifted_k = 1 << _limbs_mul_fft_best_k(width_minus_one, square);
            if (width_minus_one & (shifted_k - 1)) == 0 {
                break;
            }
            width_minus_one = (width_minus_one + shifted_k - 1) & shifted_k.wrapping_neg();
            big_width_minus_one = width_minus_one << Limb::LOG_WIDTH;
            // warning: since width_minus_one changed, shifted_k may change too!
        }
    }
    // width = O(log(p))
    let width = width_minus_one + 1;
    assert!(width_minus_one < p); // otherwise we'll loop

    // O(log(p)) memory
    let mut scratch = vec![0; width << 1];
    let mp = big_width_minus_one >> k;
    // O(p * log(p)) memory
    let mut xss_scratch = vec![0; two_pow_k * width];
    // O(p) memory
    let xss = _limbs_mul_fft_decompose(&mut xss_scratch, two_pow_k, width, xs, a, mp, &mut scratch);
    // O(p) memory
    let mut table_scratch = vec![0; 2 << k];
    // O(p) memory
    let bit_reverse_table = _limbs_mul_fft_bit_reverse_table(&mut table_scratch, k);
    if square {
        let q = a * (two_pow_k - 1) + width; // number of required limbs for p
        let mut ys_residues = vec![0; q];
        // O(log(p) * log(log(p))) memory
        _limbs_mul_fft_internal(
            out,
            p,
            k,
            xss,
            &mut ys_residues,
            a,
            mp,
            &bit_reverse_table,
            &mut scratch,
            square,
        )
    } else {
        let mut ys_residues = vec![0; two_pow_k * width];
        // O(p) memory
        _limbs_mul_fft_decompose(&mut ys_residues, two_pow_k, width, ys, a, mp, &mut scratch);
        // O(log(p) * log(log(p))) memory
        _limbs_mul_fft_internal(
            out,
            p,
            k,
            xss,
            &mut ys_residues,
            a,
            mp,
            &bit_reverse_table,
            &mut scratch,
            square,
        )
    }
}
