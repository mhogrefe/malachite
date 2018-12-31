use malachite_base::limbs::{limbs_set_zero, limbs_test_zero};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{
    NotAssign, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned, WrappingAddAssign,
    WrappingSubAssign,
};
use natural::arithmetic::add::{
    _limbs_add_same_length_with_carry_in_in_place_left, _limbs_add_to_out_special,
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_mul_u32::mpn_addmul_1;
use natural::arithmetic::add_u32::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::arithmetic::div_exact_u32::limbs_div_exact_3_in_place;
use natural::arithmetic::mul_u32::limbs_mul_limb_to_out;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_in_place_with_overlap,
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_sub_in_place_left,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::arithmetic::sub_u32::limbs_sub_limb_in_place;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural::{self, Large, Small};
use std::cmp::Ordering;
use std::ops::{Mul, MulAssign};
use std::u32;

//TODO use better algorithms

//TODO test
// docs preserved
// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, computation is mod
// B ^ rn - 1, and values are semi-normalised; zero is represented as either 0 or B ^ n - 1. Needs a
// scratch of 2rn limbs at tp.
// mpn_bc_mulmod_bnm1 from mpn/generic/mulmod_bnm1.c
pub fn mpn_bc_mulmod_bnm1(rp: &mut [u32], ap: &[u32], bp: &[u32], tp: &mut [u32]) {
    let rn = ap.len();
    assert_ne!(rn, 0);
    mpn_mul_n(tp, ap, bp);
    let cy = if limbs_add_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
        1
    } else {
        0
    };
    // If cy == 1, then the value of rp is at most B ^ rn - 2, so there can be no overflow when
    // adding in the carry.
    limbs_slice_add_limb_in_place(&mut rp[..rn], cy);
}

//TODO test
// docs preserved
// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, in semi-normalised
// representation, computation is mod B ^ rn + 1. Needs a scratch area of 2rn + 2 limbs at tp.
// Output is normalised.
// mpn_bc_mulmod_bnp1 from mpn/generic/mulmod_bnm1.c
pub fn mpn_bc_mulmod_bnp1(rp: &mut [u32], ap: &[u32], bp: &[u32], tp: &mut [u32]) {
    let rn = ap.len() - 1;
    assert_ne!(rn, 0);
    mpn_mul_n(tp, ap, bp);
    assert_eq!(tp[2 * rn + 1], 0);
    assert!(tp[2 * rn] < u32::MAX);
    let cy = tp[2 * rn]
        + if limbs_sub_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
            1
        } else {
            0
        };
    rp[rn] = 0;
    limbs_slice_add_limb_in_place(&mut rp[..rn + 1], cy);
}

//TODO PASTE A

//TODO test
// checked
// docs preserved
// Returns smallest possible number of limbs >= pl for a fft of size 2 ^ k, i.e. smallest multiple
// of 2 ^ k >= pl.
// mpn_fft_next_size from mpn/generic/mul-fft.c
pub fn mpn_fft_next_size(mut pl: usize, k: u32) -> usize {
    pl = 1 + ((pl - 1) >> k); // ceil(pl / 2 ^ k)
    pl << k
}

struct FFTTableNK {
    n: u32,
    k: u32,
}

const FFT_TABLE3_SIZE: usize = 193;

//TODO tune!!
// from mpn/*/*/gmp-mparam.h
const MUL_FFT_TABLE3: [FFTTableNK; FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 372, k: 5 },
    FFTTableNK { n: 17, k: 6 },
    FFTTableNK { n: 9, k: 5 },
    FFTTableNK { n: 19, k: 6 },
    FFTTableNK { n: 21, k: 7 },
    FFTTableNK { n: 11, k: 6 },
    FFTTableNK { n: 23, k: 7 },
    FFTTableNK { n: 12, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 21, k: 8 },
    FFTTableNK { n: 11, k: 7 },
    FFTTableNK { n: 24, k: 8 },
    FFTTableNK { n: 13, k: 7 },
    FFTTableNK { n: 27, k: 8 },
    FFTTableNK { n: 15, k: 7 },
    FFTTableNK { n: 31, k: 8 },
    FFTTableNK { n: 17, k: 7 },
    FFTTableNK { n: 35, k: 8 },
    FFTTableNK { n: 19, k: 7 },
    FFTTableNK { n: 39, k: 8 },
    FFTTableNK { n: 21, k: 9 },
    FFTTableNK { n: 11, k: 8 },
    FFTTableNK { n: 27, k: 9 },
    FFTTableNK { n: 15, k: 8 },
    FFTTableNK { n: 35, k: 9 },
    FFTTableNK { n: 19, k: 8 },
    FFTTableNK { n: 41, k: 9 },
    FFTTableNK { n: 23, k: 8 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 27, k: 10 },
    FFTTableNK { n: 15, k: 9 },
    FFTTableNK { n: 39, k: 10 },
    FFTTableNK { n: 23, k: 9 },
    FFTTableNK { n: 51, k: 11 },
    FFTTableNK { n: 15, k: 10 },
    FFTTableNK { n: 31, k: 9 },
    FFTTableNK { n: 67, k: 10 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 83, k: 10 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 55, k: 11 },
    FFTTableNK { n: 31, k: 10 },
    FFTTableNK { n: 79, k: 11 },
    FFTTableNK { n: 47, k: 10 },
    FFTTableNK { n: 95, k: 12 },
    FFTTableNK { n: 31, k: 11 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 127, k: 9 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 135, k: 9 },
    FFTTableNK { n: 271, k: 11 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 159, k: 9 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 167, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 207, k: 11 },
    FFTTableNK { n: 111, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 271, k: 9 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 143, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 11 },
    FFTTableNK { n: 159, k: 10 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 10 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 223, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 12 },
    FFTTableNK { n: 159, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 351, k: 12 },
    FFTTableNK { n: 191, k: 11 },
    FFTTableNK { n: 415, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 351, k: 11 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 191, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 575, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 255, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 703, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 959, k: 15 },
    FFTTableNK { n: 255, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 1215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1343, k: 12 },
    FFTTableNK { n: 2687, k: 13 },
    FFTTableNK { n: 1407, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1535, k: 12 },
    FFTTableNK { n: 3199, k: 13 },
    FFTTableNK { n: 1663, k: 14 },
    FFTTableNK { n: 895, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 2175, k: 14 },
    FFTTableNK { n: 1151, k: 13 },
    FFTTableNK { n: 2303, k: 12 },
    FFTTableNK { n: 4607, k: 13 },
    FFTTableNK { n: 2431, k: 12 },
    FFTTableNK { n: 4863, k: 14 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 2687, k: 14 },
    FFTTableNK { n: 1407, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 3199, k: 14 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 3327, k: 12 },
    FFTTableNK { n: 6655, k: 13 },
    FFTTableNK { n: 3455, k: 12 },
    FFTTableNK { n: 6911, k: 14 },
    FFTTableNK { n: 1791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1023, k: 14 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 4351, k: 12 },
    FFTTableNK { n: 8703, k: 14 },
    FFTTableNK { n: 2303, k: 13 },
    FFTTableNK { n: 4607, k: 14 },
    FFTTableNK { n: 2431, k: 13 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 1279, k: 14 },
    FFTTableNK { n: 2815, k: 13 },
    FFTTableNK { n: 5631, k: 14 },
    FFTTableNK { n: 2943, k: 13 },
    FFTTableNK { n: 5887, k: 12 },
    FFTTableNK { n: 11775, k: 15 },
    FFTTableNK { n: 1535, k: 14 },
    FFTTableNK { n: 3199, k: 13 },
    FFTTableNK { n: 6399, k: 14 },
    FFTTableNK { n: 3327, k: 13 },
    FFTTableNK { n: 6655, k: 14 },
    FFTTableNK { n: 3455, k: 13 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 1791, k: 14 },
    FFTTableNK { n: 3583, k: 13 },
    FFTTableNK { n: 7167, k: 14 },
    FFTTableNK { n: 3839, k: 13 },
    FFTTableNK { n: 7679, k: 16 },
    FFTTableNK { n: 65536, k: 17 },
    FFTTableNK { n: 131072, k: 18 },
    FFTTableNK { n: 262144, k: 19 },
    FFTTableNK { n: 524288, k: 20 },
    FFTTableNK { n: 1048576, k: 21 },
    FFTTableNK { n: 2097152, k: 22 },
    FFTTableNK { n: 4194304, k: 23 },
    FFTTableNK { n: 8388608, k: 24 },
];

// from mpn/*/*/gmp-mparam.h
const SQR_FFT_TABLE3: [FFTTableNK; FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 340, k: 5 },
    FFTTableNK { n: 15, k: 6 },
    FFTTableNK { n: 8, k: 5 },
    FFTTableNK { n: 17, k: 6 },
    FFTTableNK { n: 9, k: 5 },
    FFTTableNK { n: 19, k: 6 },
    FFTTableNK { n: 21, k: 7 },
    FFTTableNK { n: 11, k: 6 },
    FFTTableNK { n: 23, k: 7 },
    FFTTableNK { n: 12, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 21, k: 8 },
    FFTTableNK { n: 11, k: 7 },
    FFTTableNK { n: 24, k: 8 },
    FFTTableNK { n: 13, k: 7 },
    FFTTableNK { n: 27, k: 8 },
    FFTTableNK { n: 15, k: 7 },
    FFTTableNK { n: 31, k: 8 },
    FFTTableNK { n: 17, k: 7 },
    FFTTableNK { n: 35, k: 8 },
    FFTTableNK { n: 21, k: 9 },
    FFTTableNK { n: 11, k: 8 },
    FFTTableNK { n: 27, k: 9 },
    FFTTableNK { n: 15, k: 8 },
    FFTTableNK { n: 35, k: 9 },
    FFTTableNK { n: 19, k: 8 },
    FFTTableNK { n: 41, k: 9 },
    FFTTableNK { n: 23, k: 8 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 27, k: 10 },
    FFTTableNK { n: 15, k: 9 },
    FFTTableNK { n: 39, k: 10 },
    FFTTableNK { n: 23, k: 9 },
    FFTTableNK { n: 51, k: 11 },
    FFTTableNK { n: 15, k: 10 },
    FFTTableNK { n: 31, k: 9 },
    FFTTableNK { n: 67, k: 10 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 55, k: 11 },
    FFTTableNK { n: 31, k: 10 },
    FFTTableNK { n: 79, k: 11 },
    FFTTableNK { n: 47, k: 10 },
    FFTTableNK { n: 95, k: 12 },
    FFTTableNK { n: 31, k: 11 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 127, k: 9 },
    FFTTableNK { n: 255, k: 8 },
    FFTTableNK { n: 511, k: 9 },
    FFTTableNK { n: 271, k: 8 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 79, k: 9 },
    FFTTableNK { n: 319, k: 8 },
    FFTTableNK { n: 639, k: 10 },
    FFTTableNK { n: 175, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 207, k: 9 },
    FFTTableNK { n: 415, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 271, k: 9 },
    FFTTableNK { n: 543, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 9 },
    FFTTableNK { n: 607, k: 10 },
    FFTTableNK { n: 319, k: 9 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 175, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 10 },
    FFTTableNK { n: 415, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 10 },
    FFTTableNK { n: 607, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 351, k: 12 },
    FFTTableNK { n: 191, k: 11 },
    FFTTableNK { n: 415, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 607, k: 12 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 351, k: 13 },
    FFTTableNK { n: 191, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 607, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 255, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 703, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 959, k: 15 },
    FFTTableNK { n: 255, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 1215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1343, k: 12 },
    FFTTableNK { n: 2687, k: 13 },
    FFTTableNK { n: 1407, k: 12 },
    FFTTableNK { n: 2815, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1535, k: 12 },
    FFTTableNK { n: 3199, k: 13 },
    FFTTableNK { n: 1663, k: 14 },
    FFTTableNK { n: 895, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 2175, k: 14 },
    FFTTableNK { n: 1151, k: 13 },
    FFTTableNK { n: 2303, k: 12 },
    FFTTableNK { n: 4607, k: 13 },
    FFTTableNK { n: 2431, k: 12 },
    FFTTableNK { n: 4863, k: 14 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 2687, k: 14 },
    FFTTableNK { n: 1407, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 3199, k: 14 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 3327, k: 12 },
    FFTTableNK { n: 6655, k: 13 },
    FFTTableNK { n: 3455, k: 14 },
    FFTTableNK { n: 1791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1023, k: 14 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 4351, k: 12 },
    FFTTableNK { n: 8703, k: 14 },
    FFTTableNK { n: 2303, k: 13 },
    FFTTableNK { n: 4607, k: 14 },
    FFTTableNK { n: 2431, k: 13 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 1279, k: 14 },
    FFTTableNK { n: 2815, k: 13 },
    FFTTableNK { n: 5631, k: 14 },
    FFTTableNK { n: 2943, k: 13 },
    FFTTableNK { n: 5887, k: 12 },
    FFTTableNK { n: 11775, k: 15 },
    FFTTableNK { n: 1535, k: 14 },
    FFTTableNK { n: 3199, k: 13 },
    FFTTableNK { n: 6399, k: 14 },
    FFTTableNK { n: 3327, k: 13 },
    FFTTableNK { n: 6655, k: 14 },
    FFTTableNK { n: 3455, k: 15 },
    FFTTableNK { n: 1791, k: 14 },
    FFTTableNK { n: 3583, k: 13 },
    FFTTableNK { n: 7167, k: 14 },
    FFTTableNK { n: 3839, k: 16 },
    FFTTableNK { n: 65536, k: 17 },
    FFTTableNK { n: 131072, k: 18 },
    FFTTableNK { n: 262144, k: 19 },
    FFTTableNK { n: 524288, k: 20 },
    FFTTableNK { n: 1048576, k: 21 },
    FFTTableNK { n: 2097152, k: 22 },
    FFTTableNK { n: 4194304, k: 23 },
    FFTTableNK { n: 8388608, k: 24 },
];

const MPN_FFT_TABLE_3: [[FFTTableNK; FFT_TABLE3_SIZE]; 2] = [MUL_FFT_TABLE3, SQR_FFT_TABLE3];

//TODO test
// checked
// docs preserved
// Find the best k to use for a mod 2 ^ (m * u32::WIDTH) + 1 FFT for m >= n. We have sqr = 0 if for
// a multiply, sqr = 1 for a square.
// mpn_fft_best_k from mpn/generic/mul-fft.c, mpn_fft_table3 variant
pub fn mpn_fft_best_k(n: usize, sqr: usize) -> u32 {
    let fft_tab = &MPN_FFT_TABLE_3[sqr];
    let mut last_k = fft_tab[0].k;
    let mut tab = 1;
    loop {
        let tab_n = fft_tab[tab].n;
        let thres = tab_n << last_k;
        if n <= thres as usize {
            break;
        }
        last_k = fft_tab[tab].k;
        tab += 1;
    }
    last_k
}

//TODO tune
const MULMOD_BNM1_THRESHOLD: usize = 16;
const MUL_FFT_MODF_THRESHOLD: usize = MUL_TOOM33_THRESHOLD * 3;

//TODO test
// checked
// docs preserved
// mpn_mulmod_bnm1_next_size from mpn/generic/mulmod_bnm1.c
pub fn mpn_mulmod_bnm1_next_size(n: usize) -> usize {
    if n < MULMOD_BNM1_THRESHOLD {
        return n;
    } else if n < 4 * (MULMOD_BNM1_THRESHOLD - 1) + 1 {
        return (n + (2 - 1)) & 2_usize.wrapping_neg();
    } else if n < 8 * (MULMOD_BNM1_THRESHOLD - 1) + 1 {
        return (n + (4 - 1)) & 4_usize.wrapping_neg();
    }
    let nh = (n + 1) >> 1;
    if nh < MUL_FFT_MODF_THRESHOLD {
        (n + (8 - 1)) & 8_usize.wrapping_neg()
    } else {
        2 * mpn_fft_next_size(nh, mpn_fft_best_k(nh, 0))
    }
}

//TODO test
// checked
// docs preserved
// mpn_mulmod_bnm1_itch from gmp-impl.h
pub fn mpn_mulmod_bnm1_itch(rn: usize, an: usize, bn: usize) -> usize {
    let n = rn >> 1;
    rn + 4
        + if an > n {
            if bn > n {
                rn
            } else {
                n
            }
        } else {
            0
        }
}

//TODO tune
pub const MUL_BASECASE_MAX_UN: usize = 500;
pub const MUL_TOOM22_THRESHOLD: usize = 30;
pub const MUL_TOOM33_THRESHOLD: usize = 100;
pub const MUL_TOOM44_THRESHOLD: usize = 300;
pub const MUL_TOOM6H_THRESHOLD: usize = 350;
pub const MUL_TOOM8H_THRESHOLD: usize = 450;
pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 100;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 110;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 100;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 110;
pub const MUL_TOOM33_THRESHOLD_LIMIT: usize = MUL_TOOM33_THRESHOLD;

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. The output must be at least as long as `xs.len() + ys.len()`, `xs` must be as least as
/// long as `ys`, and `ys` cannot be empty. Returns the result limb at index
/// `xs.len() + ys.len() - 1` (which may be zero).
///
/// This uses the basecase, quadratic, schoolbook algorithm, and it is most critical code for
/// multiplication. All multiplies rely on this, both small and huge. Small ones arrive here
/// immediately, and huge ones arrive here as this is the base case for Karatsuba's recursive
/// algorithm.
///
/// Time: worst case O(n<sup>2</sup>)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// Panics if `out_limbs` is too short, `xs` is shorter than `ys`, or `ys` is empty.
///
/// This is mpn_mul_basecase from mpn/generic/mul_basecase.c.
pub fn _limbs_mul_to_out_basecase(out_limbs: &mut [u32], xs: &[u32], ys: &[u32]) -> u32 {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(out_limbs.len() >= xs_len + ys_len);

    // We first multiply by the low order limb. This result can be stored, not added, to out_limbs.
    // We also avoid a loop for zeroing this way.
    out_limbs[xs_len] = limbs_mul_limb_to_out(out_limbs, xs, ys[0]);

    // Now accumulate the product of xs and the next higher limb from ys.
    for i in 1..ys_len {
        out_limbs[xs_len + i] = mpn_addmul_1(&mut out_limbs[i..], xs, ys[i]);
    }
    // TODO maybe remove return once mpn_mul is ready
    out_limbs[xs_len + ys_len - 1]
}

// toom6_flags from gmp-impl.h
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Toom6Flags {
    Toom6AllPos,
    Toom6Vm1Neg,
    Toom6Vm2Neg,
}

impl Toom6Flags {
    pub fn to_u32(&self) -> u32 {
        match *self {
            Toom6Flags::Toom6AllPos => 0,
            Toom6Flags::Toom6Vm1Neg => 1,
            Toom6Flags::Toom6Vm2Neg => 2,
        }
    }

    pub fn from_u32(id: u32) -> Toom6Flags {
        match id {
            0 => Toom6Flags::Toom6AllPos,
            1 => Toom6Flags::Toom6Vm1Neg,
            2 => Toom6Flags::Toom6Vm2Neg,
            _ => panic!(),
        }
    }
}

//TODO test
// docs preserved
// Interpolate for toom43, 52
// Interpolation for Toom-3.5, using the evaluation points: infinity, 1, -1, 2, -2. More precisely,
// we want to compute f(2^(GMP_NUMB_BITS * n)) for a polynomial f of degree 5, given the six values
//
//   w5 = f(0),
//   w4 = f(-1),
//   w3 = f(1)
//   w2 = f(-2),
//   w1 = f(2),
//   w0 = limit at infinity of f(x) / x^5,
//
// The result is stored in {pp, 5*n + w0n}. At entry, w5 is stored at
// {pp, 2n}, w3 is stored at {pp + 2n, 2n+1}, and w0 is stored at
// {pp + 5n, w0n}. The other values are 2n + 1 limbs each (with most
// significant limbs small). f(-1) and f(-2) may be negative, signs
// determined by the flag bits. All intermediate results are positive.
// Inputs are destroyed.
//
// Interpolation sequence was taken from the paper: "Integer and
// Polynomial Multiplication: Towards Optimal Toom-Cook Matrices".
// Some slight variations were introduced: adaptation to "gmp
// instruction set", and a final saving of an operation by interlacing
// interpolation and recomposition phases.
//
// mpn_toom_interpolate_6pts from mpn/generic/mpn_toom_interpolate_6pts.c
pub fn mpn_toom_interpolate_6pts(
    pp: &mut [u32],
    n: usize,
    flags: Toom6Flags,
    w4: &mut [u32],
    w2: &mut [u32],
    w1: &mut [u32],
    w0n: usize,
) {
    assert_ne!(n, 0);
    assert!(2 * n >= w0n && w0n > 0);

    {
        let (w5, remainder) = pp.split_at_mut(2 * n);
        let (w3, _) = remainder.split_at_mut(3 * n);

        // Interpolate with sequence:
        // W2 =(W1 - W2)>>2
        // W1 =(W1 - W5)>>1
        // W1 =(W1 - W2)>>1
        // W4 =(W3 - W4)>>1
        // W2 =(W2 - W4)/3
        // W3 = W3 - W4 - W5
        // W1 =(W1 - W3)/3
        // // Last steps are mixed with recomposition...
        // W2 = W2 - W0<<2
        // W4 = W4 - W2
        // W3 = W3 - W1
        // W2 = W2 - W0
        //
        // W2 =(W1 - W2)>>2
        if (flags.to_u32() & Toom6Flags::Toom6Vm2Neg.to_u32()) != 0 {
            limbs_slice_add_same_length_in_place_left(&mut w2[..2 * n + 1], &w1[..2 * n + 1]);
        } else {
            limbs_sub_same_length_in_place_right(&w1[..2 * n + 1], &mut w2[..2 * n + 1]);
        }
        limbs_slice_shr_in_place(&mut w2[..2 * n + 1], 2);

        // W1 =(W1 - W5)>>1
        let local_borrow = if limbs_sub_same_length_in_place_left(&mut w1[..2 * n], &w5[..2 * n]) {
            1
        } else {
            0
        };
        w1[2 * n].wrapping_sub_assign(local_borrow);
        limbs_slice_shr_in_place(&mut w1[..2 * n + 1], 1);

        // W1 =(W1 - W2)>>1
        limbs_sub_same_length_in_place_left(&mut w1[..2 * n + 1], &w2[..2 * n + 1]);
        limbs_slice_shr_in_place(&mut w1[..2 * n + 1], 1);

        //W4 =(W3 - W4)>>1
        if (flags.to_u32() & Toom6Flags::Toom6Vm1Neg.to_u32()) != 0 {
            limbs_slice_add_same_length_in_place_left(&mut w4[..2 * n + 1], &w3[..2 * n + 1]);
            limbs_slice_shr_in_place(&mut w4[..2 * n + 1], 1);
        } else {
            limbs_sub_same_length_in_place_right(&w3[..2 * n + 1], &mut w4[..2 * n + 1]);
            limbs_slice_shr_in_place(&mut w4[..2 * n + 1], 1);
        }

        // W2 =(W2 - W4)/3
        limbs_sub_same_length_in_place_left(&mut w2[..2 * n + 1], &w4[..2 * n + 1]);
        limbs_div_exact_3_in_place(&mut w2[..2 * n + 1]);

        // W3 = W3 - W4 - W5
        limbs_sub_same_length_in_place_left(&mut w3[..2 * n + 1], &w4[..2 * n + 1]);
        let local_borrow = if limbs_sub_same_length_in_place_left(&mut w3[..2 * n], &w5[..2 * n]) {
            1
        } else {
            0
        };
        w3[2 * n].wrapping_sub_assign(local_borrow);

        // W1 =(W1 - W3)/3
        limbs_sub_same_length_in_place_left(&mut w1[..2 * n + 1], &w3[..2 * n + 1]);
        limbs_div_exact_3_in_place(&mut w1[..2 * n + 1]);
    }
    // [1 0 0 0 0 0;
    //  0 1 0 0 0 0;
    //  1 0 1 0 0 0;
    //  0 1 0 1 0 0;
    //  1 0 1 0 1 0;
    //  0 0 0 0 0 1]
    //
    // pp[] prior to operations:
    //  |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //
    // summation scheme for remaining operations:
    //  |______________5|n_____4|n_____3|n_____2|n______|n______|pp
    //  |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //                 || H w4  | L w4  |
    //         || H w2  | L w2  |
    //     || H w1  | L w1  |
    //             ||-H w1  |-L w1  |
    //          |-H w0  |-L w0 ||-H w2  |-L w2  |
    //
    let mut cy =
        if limbs_slice_add_same_length_in_place_left(&mut pp[n..3 * n + 1], &w4[..2 * n + 1]) {
            1
        } else {
            0
        };
    assert!(!limbs_slice_add_limb_in_place(
        &mut pp[3 * n + 1..4 * n + 1],
        cy
    ));

    {
        let (_, remainder) = pp.split_at_mut(2 * n);
        let (_, w0) = remainder.split_at_mut(3 * n);
        // W2 -= W0<<2
        // {W4,2*n+1} is now free and can be overwritten.
        cy = limbs_shl_to_out(w4, &w0[..w0n], 2);
        cy += if limbs_sub_same_length_in_place_left(&mut w2[..w0n], &w4[..w0n]) {
            1
        } else {
            0
        };
        assert!(!limbs_sub_limb_in_place(&mut w2[w0n..2 * n + 1], cy));
    }

    // W4L = W4L - W2L
    cy = if limbs_sub_same_length_in_place_left(&mut pp[n..2 * n], &w2[..n]) {
        1
    } else {
        0
    };
    {
        let (_, remainder) = pp.split_at_mut(2 * n);
        let (w3, _) = remainder.split_at_mut(3 * n);
        assert!(!limbs_sub_limb_in_place(&mut w3[..2 * n + 1], cy));
    }

    let local_carry = if limbs_slice_add_same_length_in_place_left(&mut pp[3 * n..4 * n], &w2[..n])
    {
        1
    } else {
        0
    };
    let cy4;
    {
        let (_, remainder) = pp.split_at_mut(2 * n);
        let (w3, _) = remainder.split_at_mut(3 * n);
        // W3H = W3H + W2L
        cy4 = w3[2 * n] + local_carry;
    }
    // W1L + W2H
    cy = w2[2 * n]
        + if limbs_add_same_length_to_out(&mut pp[4 * n..], &w1[..n], &w2[n..2 * n]) {
            1
        } else {
            0
        };
    assert!(!limbs_slice_add_limb_in_place(&mut w1[n..2 * n + 1], cy));
    let cy6;
    {
        let (_, remainder) = pp.split_at_mut(2 * n);
        let (_, w0) = remainder.split_at_mut(3 * n);
        // W0 = W0 + W1H
        cy6 = if w0n > n {
            w1[2 * n]
                + if limbs_slice_add_same_length_in_place_left(&mut w0[..n], &w1[n..2 * n]) {
                    1
                } else {
                    0
                }
        } else {
            if limbs_slice_add_same_length_in_place_left(&mut w0[..w0n], &w1[n..n + w0n]) {
                1
            } else {
                0
            }
        };
    }

    // summation scheme for the next operation:
    //  |...____5|n_____4|n_____3|n_____2|n______|n______|pp
    //  |...w0___|_w1_w2_|_H w3__|_L w3__|_H w5__|_L w5__|
    //          ...-w0___|-w1_w2 |
    //
    // if(LIKELY(w0n>n)) the two operands below DO overlap!
    cy = if _limbs_sub_same_length_in_place_with_overlap(&mut pp[2 * n..5 * n + w0n], 2 * n) {
        1
    } else {
        0
    };

    // embankment is a "dirty trick" to avoid carry/borrow propagation beyond allocated memory
    let embankment;
    {
        let (_, remainder) = pp.split_at_mut(2 * n);
        let (_, w0) = remainder.split_at_mut(3 * n);
        embankment = w0[w0n - 1] - 1;
        w0[w0n - 1] = 1;
    }
    if w0n > n {
        if cy4 > cy6 {
            assert!(!limbs_slice_add_limb_in_place(
                &mut pp[4 * n..5 * n + w0n],
                cy4 - cy6
            ));
        } else {
            assert!(!limbs_sub_limb_in_place(
                &mut pp[4 * n..5 * n + w0n],
                cy6 - cy4
            ));
        }
        assert!(!limbs_sub_limb_in_place(
            &mut pp[3 * n + w0n..5 * n + w0n],
            cy
        ));
        let (_, remainder) = pp.split_at_mut(2 * n);
        let (_, w0) = remainder.split_at_mut(3 * n);
        assert!(!limbs_slice_add_limb_in_place(&mut w0[n..w0n], cy6));
    } else {
        assert!(!limbs_slice_add_limb_in_place(
            &mut pp[4 * n..5 * n + w0n],
            cy4
        ));
        assert!(!limbs_sub_limb_in_place(
            &mut pp[3 * n + w0n..5 * n + w0n],
            cy + cy6
        ));
    }
    let (_, remainder) = pp.split_at_mut(2 * n);
    let (_, w0) = remainder.split_at_mut(3 * n);
    w0[w0n - 1].wrapping_add_assign(embankment);
}

// docs preserved
// DO_addlsh2(d,a,b,n,cy) computes cy,{d,n} <- {a,n} + 4*(cy,{b,n}), it can be used as
// DO_addlsh2(d,a,d,n,d[n]), for accumulation on {d,n+1}.
// The following is not a general substitute for addlsh2. It is correct if d == b, but it is not if
// d == a.
// DO_addlsh2 from mpn/generic/toom_eval_pm2.c
fn do_addlsh2(d: &mut [u32], a: &[u32], b: &[u32], cy: &mut u32) {
    *cy <<= 2;
    *cy += limbs_shl_to_out(d, b, 2);
    *cy += if limbs_slice_add_same_length_in_place_left(d, a) {
        1
    } else {
        0
    };
}

// docs preserved
// DO_addlsh2(d,a,b,n,cy) computes cy,{d,n} <- {a,n} + 4*(cy,{b,n}), it can be used as
// DO_addlsh2(d,a,d,n,d[n]), for accumulation on {d,n+1}.
// The following is not a general substitute for addlsh2. It is correct if d == b, but it is not if
// d == a.
// DO_addlsh2 from mpn/generic/toom_eval_pm2.c, when d == b
fn do_addlsh2_in_place_right(a: &[u32], b: &mut [u32], cy: &mut u32) {
    *cy <<= 2;
    *cy += limbs_slice_shl_in_place(b, 2);
    *cy += if limbs_slice_add_same_length_in_place_left(b, a) {
        1
    } else {
        0
    };
}

//TODO test
// docs preserved
// Evaluate a polynomial in +2 and -2
// Evaluates a polynomial of degree 2 < k < GMP_NUMB_BITS, in the points +2 and -2.
// mpn_toom_eval_pm2 from mpn/generic/toom_eval_pm2.c
pub fn mpn_toom_eval_pm2(
    xp2: &mut [u32],
    xm2: &mut [u32],
    mut k: u32,
    xp: &[u32],
    n: usize,
    hn: usize,
    tp: &mut [u32],
) -> u32 {
    assert!(k >= 3);
    assert!(k < u32::WIDTH);
    assert!(hn > 0);
    assert!(hn <= n);

    // The degree k is also the number of full-size coefficients, so that last coefficient, of size
    // hn, starts at xp + k*n.

    let k_u = k as usize;
    let mut cy = 0;
    do_addlsh2(
        xp2,
        &xp[(k_u - 2) * n..(k_u - 2) * n + hn],
        &xp[k_u * n..k_u * n + hn],
        &mut cy,
    );
    if hn != n {
        cy = if limbs_add_limb_to_out(&mut xp2[hn..], &xp[(k_u - 2) * n + hn..(k_u - 1) * n], cy) {
            1
        } else {
            0
        };
    }
    let mut i = k_u - 4;
    loop {
        do_addlsh2_in_place_right(&xp[i * n..(i + 1) * n], &mut xp2[..n], &mut cy);
        if i <= 2 {
            break;
        }
        i -= 2;
    }
    xp2[n] = cy;

    k.wrapping_add_assign(1);

    cy = 0;
    do_addlsh2(
        tp,
        &xp[(k_u - 2) * n..(k_u - 1) * n],
        &xp[k_u * n..(k_u + 1) * n],
        &mut cy,
    );
    let mut i = k_u - 4;
    loop {
        do_addlsh2_in_place_right(&xp[i * n..(i + 1) * n], &mut tp[..n], &mut cy);
        if i <= 2 {
            break;
        }
        i -= 2;
    }
    tp[n] = cy;

    if (k & 1) != 0 {
        assert_eq!(limbs_slice_shl_in_place(&mut tp[..n + 1], 1), 0);
    } else {
        assert_eq!(limbs_slice_shl_in_place(&mut xp2[..n + 1], 1), 0);
    }

    let mut neg = if limbs_cmp_same_length(&xp2[..n + 1], &tp[..n + 1]) == Ordering::Less {
        u32::MAX
    } else {
        0
    };

    if neg != 0 {
        limbs_sub_same_length_to_out(xm2, &tp[..n + 1], &xp2[..n + 1]);
    } else {
        limbs_sub_same_length_to_out(xm2, &xp2[..n + 1], &tp[..n + 1]);
    }

    limbs_slice_add_same_length_in_place_left(&mut xp2[..n + 1], &tp[..n + 1]);

    assert!(xp2[n] < (1 << (k + 2)) - 1);
    assert!(xm2[n] < ((1 << (k + 3)) - 1 - (1 ^ k & 1)) / 3);

    neg ^= (k & 1) - 1;
    neg
}

//TODO test
// docs preserved
// Evaluate a degree 3 polynomial in +2 and -2
// Needs n+1 limbs of temporary storage.
// mpn_toom_eval_dgr3_pm2 from mpn/generic/toom_eval_dg3_pm2.c
pub fn mpn_toom_eval_dgr3_pm2(
    xp2: &mut [u32],
    xm2: &mut [u32],
    xp: &[u32],
    n: usize,
    x3n: usize,
    tp: &mut [u32],
) -> u32 {
    assert!(x3n > 0);
    assert!(x3n <= n);

    // (x0 + 4 * x2) +/- (2 x1 + 8 x_3)
    let cy = limbs_shl_to_out(tp, &xp[2 * n..3 * n], 2);
    xp2[n] = cy
        + if limbs_add_same_length_to_out(xp2, &tp[..n], &xp[..n]) {
            1
        } else {
            0
        };

    tp[x3n] = limbs_shl_to_out(tp, &xp[3 * n..3 * n + x3n], 2);
    if x3n < n {
        tp[n] = if _limbs_add_to_out_special(tp, x3n + 1, &xp[n..2 * n]) {
            1
        } else {
            0
        };
    } else {
        tp[n] += if limbs_slice_add_same_length_in_place_left(&mut tp[..n], &xp[n..2 * n]) {
            1
        } else {
            0
        };
    }
    limbs_slice_shl_in_place(&mut tp[..n + 1], 1);

    let neg = if limbs_cmp_same_length(&xp2[..n + 1], &tp[..n + 1]) == Ordering::Less {
        u32::MAX
    } else {
        0
    };
    if neg != 0 {
        limbs_sub_same_length_to_out(xm2, &tp[..n + 1], &xp2[..n + 1]);
    } else {
        limbs_sub_same_length_to_out(xm2, &xp2[..n + 1], &tp[..n + 1]);
    }
    limbs_slice_add_same_length_in_place_left(&mut xp2[..n + 1], &tp[..n + 1]);
    assert!(xp2[n] < 15);
    assert!(xm2[n] < 10);
    neg
}

//TODO test
// docs preserved
// mpn_toom43_mul -- Multiply {ap,an} and {bp,bn} where an is nominally 4/3 times as large as bn. Or
// more accurately, bn < an < 2 bn.
//  Evaluate in: -2, -1, 0, +1, +2, +inf
//
// <-s-><--n--><--n--><--n-->
//  ___ ______ ______ ______
// |a3_|___a2_|___a1_|___a0_|
//   |_b2_|___b1_|___b0_|
//   <-t--><--n--><--n-->
//
// v0  =  a0             * b0          #   A(0)*B(0)
// v1  = (a0+ a1+ a2+ a3)*(b0+ b1+ b2) #   A(1)*B(1)      ah  <= 3  bh <= 2
// vm1 = (a0- a1+ a2- a3)*(b0- b1+ b2) #  A(-1)*B(-1)    |ah| <= 1 |bh|<= 1
// v2  = (a0+2a1+4a2+8a3)*(b0+2b1+4b2) #   A(2)*B(2)      ah  <= 14 bh <= 6
// vm2 = (a0-2a1+4a2-8a3)*(b0-2b1+4b2) #  A(-2)*B(-2)    |ah| <= 9 |bh|<= 4
// vinf=              a3 *         b2  # A(inf)*B(inf)
//
// mpn_toom43_mul from mpn/generic/toom43_mul.c
pub fn mpn_toom43_mul(pp: &mut [u32], ap: &[u32], bp: &[u32], scratch: &mut [u32]) {
    let an = ap.len();
    let bn = bp.len();
    let n = 1 + if 3 * an >= 4 * bn {
        (an - 1) >> 2
    } else {
        (bn - 1) / 3
    };
    let a3 = &ap[3 * n..];
    let b0 = bp;
    let b1 = &bp[n..];
    let b2 = &bp[2 * n..];

    let s = an - 3 * n;
    let t = bn - 2 * n;

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    // This is true whenever an >= 25 or bn >= 19, I think. It guarantees that we can fit 5 values
    // of size n+1 in the product area.
    assert!(s + t >= 5);

    // Total scratch need is 6 * n + 3 + 1; we allocate one extra limb, because products will
    // overwrite 2n+2 limbs.

    let mut flags;
    {
        let (bs1, remainder) = pp.split_at_mut(n + 1);
        let (bsm2, remainder) = remainder.split_at_mut(n + 1);
        let (bs2, remainder) = remainder.split_at_mut(n + 1);
        let (as2, as1) = remainder.split_at_mut(n + 1);
        {
            let (_, remainder) = scratch.split_at_mut(2 * n + 2);
            let (bsm1, remainder) = remainder.split_at_mut(n + 1);
            let (asm1, asm2) = remainder.split_at_mut(n + 1);
            // Compute as2 and asm2.
            flags = Toom6Flags::from_u32(
                Toom6Flags::Toom6Vm2Neg.to_u32()
                    & mpn_toom_eval_dgr3_pm2(as2, asm2, ap, n, s, asm1),
            );

            // Compute bs2 and bsm2.
            bsm1[n] = limbs_shl_to_out(bsm1, &b1[..n], 1); // 2b1
        }
        let mut cy = limbs_shl_to_out(scratch, &b2[..t], 2); //4b2
        cy += if limbs_slice_add_same_length_in_place_left(&mut scratch[..t], &b0[..t]) {
            1
        } else {
            0
        }; // 4b2 + b0
        if t != n {
            cy = if limbs_add_limb_to_out(&mut scratch[t..], &b0[t..n], cy) {
                1
            } else {
                0
            };
        }
        scratch[n] = cy;

        let (small_scratch, remainder) = scratch.split_at_mut(2 * n + 2);
        let (bsm1, remainder) = remainder.split_at_mut(n + 1);
        let (asm1, asm2) = remainder.split_at_mut(n + 1);
        limbs_add_same_length_to_out(bs2, &small_scratch[..n + 1], &bsm1[..n + 1]);
        if limbs_cmp_same_length(&small_scratch[..n + 1], &bsm1[..n + 1]) == Ordering::Less {
            limbs_sub_same_length_to_out(bsm2, &bsm1[..n + 1], &small_scratch[..n + 1]);
            flags = Toom6Flags::from_u32(flags.to_u32() ^ Toom6Flags::Toom6Vm2Neg.to_u32());
        } else {
            limbs_sub_same_length_to_out(bsm2, &small_scratch[..n + 1], &bsm1[..n + 1]);
        }

        // Compute as1 and asm1.
        flags = Toom6Flags::from_u32(
            flags.to_u32()
                ^ (Toom6Flags::Toom6Vm1Neg.to_u32()
                    & mpn_toom_eval_dgr3_pm1(as1, asm1, ap, n, s, small_scratch)),
        );

        // Compute bs1 and bsm1.
        bsm1[n] = if limbs_add_to_out(bsm1, &b0[..n], &b2[..t]) {
            1
        } else {
            0
        };
        bs1[n] = bsm1[n]
            + if limbs_add_same_length_to_out(bs1, &bsm1[..n], &b1[..n]) {
                1
            } else {
                0
            };
        if bsm1[n] == 0 && limbs_cmp_same_length(&bsm1[..n], &b1[..n]) == Ordering::Less {
            limbs_sub_same_length_in_place_right(&b1[..n], &mut bsm1[..n]);
            flags = Toom6Flags::from_u32(flags.to_u32() ^ Toom6Flags::Toom6Vm1Neg.to_u32());
        } else {
            bsm1[n] -= if limbs_sub_same_length_in_place_left(&mut bsm1[..n], &b1[..n]) {
                1
            } else {
                0
            };
        }

        assert!(as1[n] <= 3);
        assert!(bs1[n] <= 2);
        assert!(asm1[n] <= 1);
        assert!(bsm1[n] <= 1);
        assert!(as2[n] <= 14);
        assert!(bs2[n] <= 6);
        assert!(asm2[n] <= 9);
        assert!(bsm2[n] <= 4);
    }

    {
        let (vm1, remainder) = scratch.split_at_mut(2 * n + 2);
        let (bsm1, asm1) = remainder.split_at_mut(n + 1);
        // vm1, 2n+1 limbs
        mpn_mul_n(vm1, &asm1[..n + 1], &bsm1[..n + 1]); // W4
    }

    {
        let (_, remainder) = scratch.split_at_mut(2 * n + 1);
        let (vm2, asm2) = remainder.split_at_mut(2 * n + 3);
        let bsm2 = &mut pp[n + 1..];
        // vm2, 2n+1 limbs
        mpn_mul_n(vm2, &asm2[..n + 1], &bsm2[..n + 1]); // W2
    }
    {
        let (_, remainder) = scratch.split_at_mut(2 * n + 1);
        let (_, v2) = remainder.split_at_mut(2 * n + 1);

        let (_, remainder) = pp.split_at_mut(n + 1);
        let (_, remainder) = remainder.split_at_mut(n + 1);
        let (bs2, remainder) = remainder.split_at_mut(n + 1);
        let (as2, _) = remainder.split_at_mut(n + 1);

        // v2, 2n+1 limbs
        mpn_mul_n(v2, &as2[..n + 1], &bs2[..n + 1]); // W1
    }
    {
        let (bs1, remainder) = pp.split_at_mut(2 * n);
        let (v1, as1) = remainder.split_at_mut(2 * n + 4);

        // v1, 2n+1 limbs
        mpn_mul_n(v1, &as1[..n + 1], &bs1[..n + 1]); // W3
    }
    {
        let vinf = &mut pp[5 * n..];
        // vinf, s+t limbs   // W0
        if s > t {
            mpn_mul(vinf, &a3[..s], &b2[..t]);
        } else {
            mpn_mul(vinf, &b2[..t], &a3[..s]);
        }
    }

    // v0, 2n limbs
    mpn_mul_n(pp, &ap[..n], &bp[..n]); // W5
    let (vm1, remainder) = scratch.split_at_mut(2 * n + 1);
    let (vm2, v2) = remainder.split_at_mut(2 * n + 1);
    mpn_toom_interpolate_6pts(pp, n, flags, vm1, vm2, v2, t + s);
}

pub const MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD < 3 * MUL_TOOM22_THRESHOLD;
pub const MAYBE_MUL_TOOM33: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM44_THRESHOLD >= 3 * MUL_TOOM33_THRESHOLD;

//TODO test
// docs preserved
// TOOM33_MUL_N_REC from mpn/generic/toom33_mul.c
pub fn toom33_mul_n_rec(p: &mut [u32], a: &[u32], b: &[u32], ws: &mut [u32]) {
    let n = a.len();
    assert_eq!(a.len(), n);
    if MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(p, a, b);
    } else if !MAYBE_MUL_TOOM33 || n < MUL_TOOM33_THRESHOLD {
        _limbs_mul_to_out_toom_22(p, a, b, ws);
    } else {
        mpn_toom33_mul(p, a, b, ws);
    }
}

pub const SMALLER_RECURSION: bool = false;

//TODO test
// Multiply {ap,an} and {p,bn} where an and bn are close in size. Or more accurately,
// bn <= an < (3/2)bn.
//
//  Evaluate in: -1, 0, +1, +2, +inf
//
// <-s--><--n--><--n-->
//  ____ ______ ______
// |_a2_|___a1_|___a0_|
//  |b2_|___b1_|___b0_|
//  <-t-><--n--><--n-->
//
// v0  =  a0         * b0          #   A(0)*B(0)
// v1  = (a0+ a1+ a2)*(b0+ b1+ b2) #   A(1)*B(1)      ah  <= 2  bh <= 2
// vm1 = (a0- a1+ a2)*(b0- b1+ b2) #  A(-1)*B(-1)    |ah| <= 1  bh <= 1
// v2  = (a0+2a1+4a2)*(b0+2b1+4b2) #   A(2)*B(2)      ah  <= 6  bh <= 6
// vinf=          a2 *         b2  # A(inf)*B(inf)
//
// docs preserved
// mpn_toom33_mul from mpn/generic/toom33_mul.c
pub fn mpn_toom33_mul(pp: &mut [u32], ap: &[u32], bp: &[u32], scratch: &mut [u32]) {
    let an = ap.len();
    let bn = bp.len();

    let n = (an + 2) / 3;
    let a0 = ap;
    let a1 = &ap[n..];
    let a2 = &ap[2 * n..];
    let b0 = bp;
    let b1 = &bp[n..];
    let b2 = &bp[2 * n..];

    let s = an - 2 * n;
    let t = bn - 2 * n;

    assert!(an >= bn);
    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    let mut vm1_neg = 0;
    {
        let (bs1, remainder) = pp.split_at_mut(n + 1);
        let (as2, bs2) = remainder.split_at_mut(n + 1);
        {
            // we need 4n+4 <= 4n+s+t
            let (gp, remainder) = scratch.split_at_mut(2 * n + 2);
            let (asm1, remainder) = remainder.split_at_mut(n + 1);
            let (bsm1, as1) = remainder.split_at_mut(n + 1);

            // Compute as1 and asm1.
            let mut cy = if limbs_add_to_out(gp, &a0[..n], &a2[..s]) {
                1
            } else {
                0
            };
            as1[n] = cy
                + if limbs_add_same_length_to_out(as1, &gp[..n], &a1[..n]) {
                    1
                } else {
                    0
                };
            if cy == 0 && limbs_cmp_same_length(&gp[..n], &a1[..n]) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, &a1[..n], &gp[..n]);
                asm1[n] = 0;
                vm1_neg = 1;
            } else {
                cy -= if limbs_sub_same_length_to_out(asm1, &gp[..n], &a1[..n]) {
                    1
                } else {
                    0
                };
                asm1[n] = cy;
            }

            // Compute as2.
            cy = if limbs_add_same_length_to_out(as2, &a2[..s], &as1[..s]) {
                1
            } else {
                0
            };
            if s != n {
                cy = if limbs_add_limb_to_out(&mut as2[s..], &as1[s..n], cy) {
                    1
                } else {
                    0
                };
            }
            cy.wrapping_add_assign(as1[n]);
            cy = 2 * cy + limbs_slice_shl_in_place(&mut as2[..n], 1);
            cy.wrapping_sub_assign(
                if limbs_sub_same_length_in_place_left(&mut as2[..n], &a0[..n]) {
                    1
                } else {
                    0
                },
            );
            as2[n] = cy;

            // Compute bs1 and bsm1.
            cy = if limbs_add_to_out(gp, &b0[..n], &b2[..t]) {
                1
            } else {
                0
            };
            bs1[n] = cy
                + if limbs_add_same_length_to_out(bs1, &gp[..n], &b1[..n]) {
                    1
                } else {
                    0
                };
            if cy == 0 && limbs_cmp_same_length(&gp[..n], &b1[..n]) == Ordering::Less {
                limbs_sub_same_length_to_out(bsm1, &b1[..n], &gp[..n]);
                bsm1[n] = 0;
                vm1_neg ^= 1;
            } else {
                cy.wrapping_sub_assign(if limbs_sub_same_length_to_out(bsm1, &gp[..n], &b1[..n]) {
                    1
                } else {
                    0
                });
                bsm1[n] = cy;
            }

            // Compute bs2.
            cy = if limbs_add_same_length_to_out(bs2, &bs1[..t], &b2[..t]) {
                1
            } else {
                0
            };
            if t != n {
                cy = if limbs_add_limb_to_out(&mut bs2[t..], &bs1[t..n], cy) {
                    1
                } else {
                    0
                };
            }
            cy.wrapping_add_assign(bs1[n]);
            cy = 2 * cy + limbs_slice_shl_in_place(&mut bs2[..n], 1);
            cy.wrapping_sub_assign(
                if limbs_sub_same_length_in_place_left(&mut bs2[..n], &b0[..n]) {
                    1
                } else {
                    0
                },
            );
            bs2[n] = cy;

            assert!(as1[n] <= 2);
            assert!(bs1[n] <= 2);
            assert!(asm1[n] <= 1);
            assert!(bsm1[n] <= 1);
            assert!(as2[n] <= 6);
            assert!(bs2[n] <= 6);
        }
        {
            let (vm1, remainder) = scratch.split_at_mut(2 * n + 2);
            let (asm1, remainder) = remainder.split_at_mut(n + 1);
            let (bsm1, scratch_out) = remainder.split_at_mut(2 * n + 2);
            if SMALLER_RECURSION {
                toom33_mul_n_rec(vm1, &asm1[..n], &bsm1[..n], scratch_out);
                let mut cy = 0;
                if asm1[n] != 0 {
                    cy = bsm1[n]
                        + if limbs_slice_add_same_length_in_place_left(
                            &mut vm1[n..2 * n],
                            &bsm1[..n],
                        ) {
                            1
                        } else {
                            0
                        };
                }
                if bsm1[n] != 0 {
                    cy += if limbs_slice_add_same_length_in_place_left(
                        &mut vm1[n..2 * n],
                        &asm1[..n],
                    ) {
                        1
                    } else {
                        0
                    };
                }
                vm1[2 * n] = cy;
            } else {
                toom33_mul_n_rec(vm1, &asm1[..n + 1], &bsm1[..n + 1], scratch_out);
            }
        }
        {
            let (_, remainder) = scratch.split_at_mut(2 * n + 1);
            let (v2, scratch_out) = remainder.split_at_mut(3 * n + 4);
            toom33_mul_n_rec(v2, &as2[..n + 1], &bs2[..n + 1], scratch_out); // v2, 2n+1 limbs
        }
    }
    let vinf0;
    {
        let vinf = &mut pp[4 * n..];
        // vinf, s+t limbs
        if s > t {
            mpn_mul(vinf, &a2[..s], &b2[..t]);
        } else {
            let (_, remainder) = scratch.split_at_mut(2 * n + 1);
            let (_, scratch_out) = remainder.split_at_mut(3 * n + 4);
            toom33_mul_n_rec(vinf, &a2[..s], &b2[..s], scratch_out);
        }
        vinf0 = vinf[0]; // v1 overlaps with this
    }

    if SMALLER_RECURSION {
        let (bs1, v1) = pp.split_at_mut(2 * n);
        let (_, remainder) = scratch[3 * n + 3..].split_at_mut(n + 1);
        let (as1, scratch_out) = remainder.split_at_mut(n + 1);
        let mut cy;
        // v1, 2n+1 limbs
        toom33_mul_n_rec(v1, &as1[..n], &bs1[..n], scratch_out);
        if as1[n] == 1 {
            cy = bs1[n]
                + if limbs_slice_add_same_length_in_place_left(&mut v1[n..2 * n], &bs1[..n]) {
                    1
                } else {
                    0
                };
        } else if as1[n] != 0 {
            cy = 2 * bs1[n] + mpn_addmul_1(&mut v1[n..], &bs1[n..], 2);
        } else {
            cy = 0;
        }
        if bs1[n] == 1 {
            cy += if limbs_slice_add_same_length_in_place_left(&mut v1[n..2 * n], &as1[..n]) {
                1
            } else {
                0
            };
        } else if bs1[n] != 0 {
            cy += mpn_addmul_1(&mut v1[n..], &as1[..n], 2);
        }
        v1[2 * n] = cy;
    } else {
        let cy;
        {
            let vinf = &mut pp[4 * n..];
            cy = vinf[1];
        }
        {
            let (bs1, v1) = pp.split_at_mut(2 * n);
            let (_, remainder) = scratch[3 * n + 3..].split_at_mut(n + 1);
            let (as1, scratch_out) = remainder.split_at_mut(n + 1);
            toom33_mul_n_rec(v1, &as1[..n + 1], &bs1[..n + 1], scratch_out);
        }
        let vinf = &mut pp[4 * n..];
        vinf[1] = cy;
    }

    {
        let (_, remainder) = scratch[3 * n + 3..].split_at_mut(n + 1);
        let (_, scratch_out) = remainder.split_at_mut(n + 1);
        toom33_mul_n_rec(pp, &ap[..n], &bp[..n], scratch_out); // v0, 2n limbs
    }

    let (vm1, v2) = scratch.split_at_mut(2 * n + 1);
    mpn_toom_interpolate_5pts(pp, v2, vm1, n, s + t, vm1_neg, vinf0);
}

//TODO test
// docs preserved
// Interpolate for toom3, 33, 42.
// mpn_toom_interpolate_5pts in mpn/generic/mpn_toom_interpolate_5pts.c
pub fn mpn_toom_interpolate_5pts(
    c: &mut [u32],
    v2: &mut [u32],
    vm1: &mut [u32],
    k: usize,
    twor: usize,
    sa: u32,
    mut vinf0: u32,
) {
    let twok = k + k;
    let kk1 = twok + 1;
    {
        let (_, remainder) = c.split_at_mut(k);
        let (v1, _) = remainder.split_at_mut(k);

        // (1) v2 <- v2-vm1 < v2+|vm1|,       (16 8 4 2 1) - (1 -1 1 -1  1) =
        // thus 0 <= v2 < 50*B^(2k) < 2^6*B^(2k)             (15 9 3  3  0)
        //
        if sa != 0 {
            assert!(!limbs_slice_add_same_length_in_place_left(
                &mut v2[..kk1],
                &vm1[..kk1]
            ));
        } else {
            assert!(!limbs_sub_same_length_in_place_left(
                &mut v2[..kk1],
                &vm1[..kk1]
            ));
        }

        // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
        //   v0       v1       hi(vinf)       |vm1|     v2-vm1      EMPTY
        limbs_div_exact_3_in_place(&mut v2[..kk1]); // v2 <- v2 / 3
                                                    // (5 3 1 1 0)

        // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
        //   v0       v1      hi(vinf)       |vm1|     (v2-vm1)/3    EMPTY
        //
        // (2) vm1 <- tm1 := (v1 - vm1) / 2  [(1 1 1 1 1) - (1 -1 1 -1 1)] / 2 =
        // tm1 >= 0                                         (0  1 0  1 0)
        // No carry comes out from {v1, kk1} +/- {vm1, kk1},
        // and the division by two is exact.
        // If (sa!=0) the sign of vm1 is negative
        if sa != 0 {
            assert!(!limbs_slice_add_same_length_in_place_left(
                &mut vm1[..kk1],
                &v1[..kk1]
            ));
            assert_eq!(limbs_slice_shr_in_place(&mut vm1[..kk1], 1), 0);
        } else {
            assert!(!limbs_sub_same_length_in_place_right(
                &v1[..kk1],
                &mut vm1[..kk1]
            ));
            assert_eq!(limbs_slice_shr_in_place(&mut vm1[..kk1], 1), 0);
        }

        // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
        //   v0       v1        hi(vinf)       tm1     (v2-vm1)/3    EMPTY
        //
        // (3) v1 <- t1 := v1 - v0    (1 1 1 1 1) - (0 0 0 0 1) = (1 1 1 1 0)
        // t1 >= 0
        //
    }
    let vinf_lowest = {
        let (c_lo, v1) = c.split_at_mut(twok);
        if limbs_sub_same_length_in_place_left(v1, c_lo) {
            1
        } else {
            0
        }
    };
    let (c1, remainder) = c.split_at_mut(k);
    let (v1, remainder) = remainder.split_at_mut(k);
    let (c3, remainder) = remainder.split_at_mut(k);
    let (vinf, _) = remainder.split_at_mut(k);
    vinf[0].wrapping_sub_assign(vinf_lowest);

    // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
    //   v0     v1-v0        hi(vinf)       tm1     (v2-vm1)/3    EMPTY
    //
    // (4) v2 <- t2 := ((v2-vm1)/3-t1)/2 = (v2-vm1-3*t1)/6
    // t2 >= 0                  [(5 3 1 1 0) - (1 1 1 1 0)]/2 = (2 1 0 0 0)
    //
    assert!(!limbs_sub_same_length_in_place_left(v2, &v1[..kk1]));
    assert_eq!(limbs_slice_shr_in_place(&mut v2[..kk1], 1), 0);

    // {c,2k} {c+2k,2k+1} {c+4k+1,2r-1} {t,2k+1} {t+2k+1,2k+1} {t+4k+2,2r}
    //   v0     v1-v0        hi(vinf)     tm1    (v2-vm1-3t1)/6    EMPTY
    //
    // (5) v1 <- t1-tm1           (1 1 1 1 0) - (0 1 0 1 0) = (1 0 1 0 0)
    // result is v1 >= 0
    //
    assert!(!limbs_sub_same_length_in_place_left(
        &mut v1[..kk1],
        &vm1[..kk1]
    ));

    // We do not need to read the value in vm1, so we add it in {c+k, ...}
    let mut cy = if limbs_slice_add_same_length_in_place_left(&mut c1[..kk1], &vm1[..kk1]) {
        1
    } else {
        0
    };
    // 2n-(3k+1) = 2r+k-1
    // Memory allocated for vm1 is now free, it can be recycled
    assert!(!limbs_slice_add_limb_in_place(&mut c3[1..twor + k], cy));

    // (6) v2 <- v2 - 2*vinf,     (2 1 0 0 0) - 2*(1 0 0 0 0) = (0 1 0 0 0)
    // result is v2 >= 0
    let saved = vinf[0]; // Remember v1's highest byte (will be overwritten).
    vinf[0] = vinf0; // Set the right value for vinf0
                     // Overwrite unused vm1
    cy = limbs_shl_to_out(vm1, &mut vinf[..twor], 1);
    cy += if limbs_sub_same_length_in_place_left(&mut v2[..twor], &vm1[..twor]) {
        1
    } else {
        0
    };
    assert!(!limbs_sub_limb_in_place(&mut v2[twor..kk1], cy));

    //  Current matrix is
    //  [1 0 0 0 0; vinf
    //   0 1 0 0 0; v2
    //   1 0 1 0 0; v1
    //   0 1 0 1 0; vm1
    //   0 0 0 0 1] v0
    //  Some values already are in-place (we added vm1 in the correct position)
    //  | vinf|  v1 |  v0 |
    //       | vm1 |
    //  One still is in a separated area
    // | +v2 |
    //  We have to compute v1-=vinf; vm1 -= v2,
    //    |-vinf|
    //       | -v2 |
    //  Carefully reordering operations we can avoid to compute twice the sum
    //  of the high half of v2 plus the low half of vinf.
    //
    // Add the high half of t2 in {vinf}
    if twor > k + 1 {
        // This is the expected flow
        cy = if limbs_slice_add_same_length_in_place_left(&mut vinf[..k + 1], &v2[k..2 * k + 1]) {
            1
        } else {
            0
        };
        assert!(!limbs_slice_add_limb_in_place(
            &mut c3[kk1..kk1 + twor - k - 1],
            cy
        )); // 2n-(5k+1) = 2r-k-1
    } else {
        // triggered only by very unbalanced cases like (k+k+(k-2))x(k+k+1), should be handled by
        // toom32
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut vinf[..twor],
            &v2[k..k + twor]
        ));
    }
    // (7) v1 <- v1 - vinf,       (1 0 1 0 0) - (1 0 0 0 0) = (0 0 1 0 0)
    // result is >= 0
    // Side effect: we also subtracted (high half) vm1 -= v2
    // vinf is at most twor long.
    cy = if limbs_sub_same_length_in_place_left(&mut v1[..twor], &vinf[..twor]) {
        1
    } else {
        0
    };
    vinf0 = vinf[0]; // Save again the right value for vinf0
    vinf[0] = saved;
    assert!(!limbs_sub_limb_in_place(&mut v1[twor..kk1], cy)); // Treat the last bytes.

    // (8) vm1 <- vm1-v2          (0 1 0 1 0) - (0 1 0 0 0) = (0 0 0 1 0)
    // Operate only on the low half.
    //
    cy = if limbs_sub_same_length_in_place_left(&mut c1[..k], &v2[..k]) {
        1
    } else {
        0
    };
    assert!(!limbs_sub_limb_in_place(&mut v1[..kk1], cy));

    // Beginning the final phase
    // Most of the recomposition was done
    // add t2 in {c+3k, ...}, but only the low half
    cy = if limbs_slice_add_same_length_in_place_left(&mut c3[..k], &v2[..k]) {
        1
    } else {
        0
    };
    vinf[0] += cy;
    assert!(vinf[0] >= cy); // No carry
                            // Add vinf0, propagate carry.
    assert!(!limbs_slice_add_limb_in_place(&mut vinf[..twor], vinf0));
}

//TODO test
// docs preserved
// Evaluate a degree 3 polynomial in +1 and -1
// mpn_toom_eval_dgr3_pm1 in mpn/generic/toom_eval_dgr3_pm1.c
pub fn mpn_toom_eval_dgr3_pm1(
    xp1: &mut [u32],
    xm1: &mut [u32],
    xp: &[u32],
    n: usize,
    x3n: usize,
    tp: &mut [u32],
) -> u32 {
    assert!(x3n > 0);
    assert!(x3n <= n);
    xp1[n] = if limbs_add_same_length_to_out(xp1, &xp[..n], &xp[n..3 * n]) {
        1
    } else {
        0
    };
    tp[n] = if limbs_add_to_out(tp, &xp[n..2 * n], &xp[3 * n..3 * n + x3n]) {
        1
    } else {
        0
    };
    let neg = if limbs_cmp_same_length(&xp1[..n + 1], &tp[..n + 1]) == Ordering::Less {
        u32::MAX
    } else {
        0
    };
    if neg != 0 {
        limbs_sub_same_length_to_out(xm1, &tp[..n + 1], &xp1[..n + 1]);
    } else {
        limbs_sub_same_length_to_out(xm1, &xp1[..n + 1], &tp[..n + 1]);
    }
    limbs_slice_add_same_length_in_place_left(&mut xp1[..n + 1], &tp[..n + 1]);
    assert!(xp1[n] <= 3);
    assert!(xm1[n] <= 1);
    neg
}

//TODO test
// docs preserved
// TOOM42_MUL_N_REC from mpn/generic/toom42_mul.c
pub fn toom42_mul_n_rec(p: &mut [u32], a: &[u32], b: &[u32]) {
    mpn_mul_n(p, a, b);
}

//TODO test
// docs preserved
// Multiply {ap,an} and {bp,bn} where an is nominally twice as large as bn. Or more accurately,
// (3 / 2)bn < an < 4bn.
// Evaluate in: -1, 0, +1, +2, +inf
//
// <-s-><--n--><--n--><--n-->
//  ___ ______ ______ ______
// |a3_|___a2_|___a1_|___a0_|
//          |_b1_|___b0_|
//          <-t--><--n-->
//
// v0  =  a0             * b0      #   A(0)*B(0)
// v1  = (a0+ a1+ a2+ a3)*(b0+ b1) #   A(1)*B(1)      ah  <= 3  bh <= 1
// vm1 = (a0- a1+ a2- a3)*(b0- b1) #  A(-1)*B(-1)    |ah| <= 1  bh  = 0
// v2  = (a0+2a1+4a2+8a3)*(b0+2b1) #   A(2)*B(2)      ah  <= 14 bh <= 2
// vinf=              a3 *     b1  # A(inf)*B(inf)
//
// mpn_toom42_mul from mpn/generic/toom42_mul.c
pub fn mpn_toom42_mul(pp: &mut [u32], ap: &[u32], bp: &[u32], scratch: &mut [u32]) {
    let an = ap.len();
    let bn = bp.len();
    let n = if an >= 2 * bn {
        (an + 3) >> 2
    } else {
        (bn + 1) >> 1
    };
    let a0 = ap;
    let a1 = &ap[n..];
    let a2 = &ap[2 * n..];
    let a3 = &ap[3 * n..];
    let b0 = bp;
    let b1 = &bp[n..];

    let s = an - 3 * n;
    let t = bn - n;

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    let mut tmp = vec![0; 6 * n + 5];
    let (as1, remainder) = tmp.split_at_mut(n + 1);
    let (asm1, remainder) = remainder.split_at_mut(n + 1);
    let (as2, remainder) = remainder.split_at_mut(n + 1);
    let (bs1, remainder) = remainder.split_at_mut(n + 1);
    let (bsm1, remainder) = remainder.split_at_mut(n);
    let (bs2, _) = remainder.split_at_mut(n + 1);

    // Compute as1 and asm1.
    let mut vm1_neg = mpn_toom_eval_dgr3_pm1(as1, asm1, ap, n, s, pp) & 1;

    // Compute as2.
    let mut cy = limbs_shl_to_out(as2, &a3[..s], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut as2[..s], &a2[..s]) {
        1
    } else {
        0
    };
    if s != n {
        cy = if limbs_add_limb_to_out(&mut as2[s..], &a2[s..n], cy) {
            1
        } else {
            0
        };
    }
    cy = 2 * cy + limbs_slice_shl_in_place(&mut as2[..n], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut as2[..n], &a1[..n]) {
        1
    } else {
        0
    };
    cy = 2 * cy + limbs_slice_shl_in_place(&mut as2[..n], 1);
    cy += if limbs_slice_add_same_length_in_place_left(&mut as2[..n], &a0[..n]) {
        1
    } else {
        0
    };
    as2[n] = cy;

    // Compute bs1 and bsm1.
    if t == n {
        bs1[n] = if limbs_add_same_length_to_out(bs1, &b0[..n], &b1[..n]) {
            1
        } else {
            0
        };
        if limbs_cmp_same_length(&b0[..n], &b1[..n]) == Ordering::Less {
            limbs_sub_same_length_to_out(bsm1, &b1[..n], &b0[..n]);
            vm1_neg ^= 1;
        } else {
            limbs_sub_same_length_to_out(bsm1, &b0[..n], &b1[..n]);
        }
    } else {
        bs1[n] = if limbs_add_to_out(bs1, &b0[..n], &b1[..t]) {
            1
        } else {
            0
        };

        if limbs_test_zero(&b0[t..n]) && limbs_cmp_same_length(&b0[..t], &b1[..t]) == Ordering::Less
        {
            limbs_sub_same_length_to_out(bsm1, &b1[..t], &b0[..t]);
            limbs_set_zero(&mut bsm1[t..n]);
            vm1_neg ^= 1;
        } else {
            limbs_sub_to_out(bsm1, &b0[..n], &b1[..t]);
        }
    }

    // Compute bs2, recycling bs1. bs2=bs1+b1
    limbs_add_to_out(bs2, &bs1[..n + 1], &b1[..t]);

    assert!(as1[n] <= 3);
    assert!(bs1[n] <= 1);
    assert!(asm1[n] <= 1);
    assert!(as2[n] <= 14);
    assert!(bs2[n] <= 2);

    let (vm1, v2) = scratch.split_at_mut(2 * n + 1);
    let vinf0;
    {
        let (v0, remainder) = pp.split_at_mut(2 * n);
        let (v1, vinf) = remainder.split_at_mut(2 * n);

        // vm1, 2n+1 limbs
        toom42_mul_n_rec(vm1, &asm1[..n], &bsm1[..n]);
        let mut cy = 0;
        if asm1[n] != 0 {
            cy = if limbs_slice_add_same_length_in_place_left(&mut vm1[n..], &bsm1[..n]) {
                1
            } else {
                0
            };
        }
        vm1[2 * n] = cy;

        toom42_mul_n_rec(v2, &as2[..n + 1], &bs2[..n + 1]); // v2, 2n+1 limbs

        // vinf, s+t limbs
        if s > t {
            mpn_mul(vinf, &a3[..s], &b1[..t]);
        } else {
            mpn_mul(vinf, &b1[..t], &a3[..s]);
        }

        vinf0 = vinf[0]; // v1 overlaps with this

        // v1, 2n+1 limbs
        toom42_mul_n_rec(v1, &as1[..n], &bs1[..n]);
        cy = if as1[n] == 1 {
            bs1[n]
                + if limbs_slice_add_same_length_in_place_left(&mut v1[n..2 * n], &bs1[..n]) {
                    1
                } else {
                    0
                }
        } else if as1[n] == 2 {
            2 * bs1[n] + mpn_addmul_1(&mut v1[n..], &bs1[..n], 2)
        } else if as1[n] == 3 {
            3 * bs1[n] + mpn_addmul_1(&mut v1[n..], &bs1[..n], 3)
        } else {
            0
        };
        if bs1[n] != 0 {
            cy += if limbs_slice_add_same_length_in_place_left(&mut v1[n..2 * n], &as1[..n]) {
                1
            } else {
                0
            };
        }
        v1[2 * n] = cy;

        toom42_mul_n_rec(v0, &ap[..n], &bp[..n]); // v0, 2n limbs
    }
    mpn_toom_interpolate_5pts(pp, v2, vm1, n, s + t, vm1_neg, vinf0);
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_32`.
///
/// This is mpn_toom32_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_32_scratch_size(an: usize, bn: usize) -> usize {
    let n = 1 + if 2 * an >= 3 * bn {
        (an - 1) / 3
    } else {
        (bn - 1) >> 1
    };
    2 * n + 1
}

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// This is TOOM32_MUL_N_REC from mpn/generic/toom32_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_32_recursive(p: &mut [u32], a: &[u32], b: &[u32]) {
    //TODO switch from basecase to mpn_mul_n
    _limbs_mul_to_out_basecase(p, a, b);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_32` are valid.
pub fn _limbs_mul_to_out_toom_32_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if xs_len < ys_len {
        return false;
    }
    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };
    if ys_len + 2 > xs_len || xs_len + 6 > 3 * ys_len {
        return false;
    }
    let s = xs_len - 2 * n;
    let t = ys_len - n;
    0 < s && s <= n && 0 < t && t <= n && s + t >= n
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_32_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_32_input_sizes_valid`. the gist is that `xs` must be less
///    than 3 times as long as `ys`.
///
/// This uses the Toom-32 aka Toom-2.5 algorithm.
///
///  Evaluate in: -1, 0, +1, +inf
///
/// <-s-><--n--><--n-->
///  ___________________
/// |xs2_|__xs1_|__xs0_|
///        |ys1_|__ys0_|
///        <-t--><--n-->
///
/// v0   =  xs0              * ys0         # A(0)   * B(0)
/// v1   = (xs0 + xs1 + xs2) * (ys0 + ys1) # A(1)   * B(1)    ah  <= 2  bh <= 1
/// vm1  = (xs0 - xs1 + xs2) * (ys0 - ys1) # A(-1)  * B(-1)  |ah| <= 1  bh = 0
/// vinf =               xs2 * ys1         # A(inf) * B(inf)
///
/// Time: TODO (should be something like O(n<sup>k</sup>), where k = 2log(2)/(log(5)-log(2))?)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom32_mul from mpn/generic/toom32_mul.c.
#[allow(unreachable_code)] //TODO remove
pub fn _limbs_mul_to_out_toom_32(
    out_limbs: &mut [u32],
    xs: &[u32],
    ys: &[u32],
    scratch: &mut [u32],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };

    // Required, to ensure that s + t >= n.
    assert!(ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len);

    let s = xs_len - 2 * n;
    let t = ys_len - n;

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    assert!(s + t >= n);

    let (xs0, remainder) = xs.split_at(n); // xs0: length n
    let (xs1, xs2) = remainder.split_at(n); // xs1: length n, xs2: length s
    let (ys0, ys1) = ys.split_at(n); // ys0: length n, ys1: length t

    let mut hi: i32;
    let mut v_neg_1_neg;
    {
        // Product area of size xs_len + ys_len = 3 * n + s + t >= 4 * n + 2.
        // out_limbs_lo: length 2 * n
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        let (am1, bm1) = out_limbs_hi.split_at_mut(n); // am1: length n
        {
            let (ap1, bp1) = out_limbs_lo.split_at_mut(n); // ap1: length n, bp1: length n

            // Compute ap1 = xs0 + xs1 + a3, am1 = xs0 - xs1 + a3
            let mut ap1_hi = 0;
            if limbs_add_to_out(ap1, xs0, xs2) {
                ap1_hi = 1;
            }
            if ap1_hi == 0 && limbs_cmp_same_length(ap1, xs1) == Ordering::Less {
                assert!(!limbs_sub_same_length_to_out(am1, xs1, ap1));
                hi = 0;
                v_neg_1_neg = true;
            } else {
                hi = ap1_hi;
                if limbs_sub_same_length_to_out(am1, ap1, xs1) {
                    hi -= 1;
                }
                v_neg_1_neg = false;
            }
            if limbs_slice_add_same_length_in_place_left(ap1, xs1) {
                ap1_hi += 1;
            }

            let bp1_hi;
            // Compute bp1 = ys0 + ys1 and bm1 = ys0 - ys1.
            if t == n {
                bp1_hi = limbs_add_same_length_to_out(bp1, ys0, ys1);
                if limbs_cmp_same_length(ys0, ys1) == Ordering::Less {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys1, ys0));
                    v_neg_1_neg.not_assign();
                } else {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys0, ys1));
                }
            } else {
                bp1_hi = limbs_add_to_out(bp1, ys0, ys1);
                if limbs_test_zero(&ys0[t..])
                    && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
                {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys1, &ys0[..t]));
                    limbs_set_zero(&mut bm1[t..n]);
                    v_neg_1_neg.not_assign();
                } else {
                    assert!(!limbs_sub_to_out(bm1, ys0, ys1));
                }
            }

            _limbs_mul_same_length_to_out_toom_32_recursive(scratch, ap1, bp1);
            let mut carry = 0;
            if ap1_hi == 1 {
                if limbs_slice_add_same_length_in_place_left(&mut scratch[n..2 * n], &bp1[..n]) {
                    carry = 1;
                }
                if bp1_hi {
                    carry += 1;
                }
            } else if ap1_hi == 2 {
                carry = mpn_addmul_1(&mut scratch[n..], bp1, 2);
                if bp1_hi {
                    carry += 2;
                }
            }
            if bp1_hi && limbs_slice_add_same_length_in_place_left(&mut scratch[n..2 * n], ap1) {
                carry += 1;
            }
            scratch[2 * n] = carry;
        }
        _limbs_mul_same_length_to_out_toom_32_recursive(out_limbs_lo, am1, &bm1[..n]);
        if hi != 0 {
            hi = 0;
            if limbs_slice_add_same_length_in_place_left(&mut out_limbs_lo[n..], &bm1[..n]) {
                hi = 1;
            }
        }
    }
    out_limbs[2 * n] = hi.to_unsigned_bitwise();

    // v1 <-- (v1 + vm1) / 2 = x0 + x2
    {
        let scratch = &mut scratch[..2 * n + 1];
        let out_limbs = &out_limbs[..2 * n + 1];
        if v_neg_1_neg {
            limbs_sub_same_length_in_place_left(scratch, out_limbs);
            assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
        } else {
            limbs_slice_add_same_length_in_place_left(scratch, &out_limbs);
            assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
        }
    }

    // We get x1 + x3 = (x0 + x2) - (x0 - x1 + x2 - x3), and hence
    //
    // y = x1 + x3 + (x0 + x2) * B
    //   = (x0 + x2) * B + (x0 + x2) - vm1.
    //
    // y is 3 * n + 1 limbs, y = y0 + y1 B + y2 B^2. We store them as follows: y0 at scratch, y1 at
    // out_limbs + 2 * n, and y2 at scratch + n (already in place, except for carry propagation).
    //
    // We thus add
    //
    //    B^3  B^2   B    1
    //     |    |    |    |
    //    +-----+----+
    //  + |  x0 + x2 |
    //    +----+-----+----+
    //  +      |  x0 + x2 |
    //         +----------+
    //  -      |  vm1     |
    //  --+----++----+----+-
    //    | y2  | y1 | y0 |
    //    +-----+----+----+
    //
    // Since we store y0 at the same location as the low half of x0 + x2, we need to do the middle
    // sum first.
    hi = out_limbs[2 * n].to_signed_bitwise();
    let mut scratch_high = scratch[2 * n];
    if limbs_add_same_length_to_out(&mut out_limbs[2 * n..], &scratch[..n], &scratch[n..2 * n]) {
        scratch_high += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut scratch[n..2 * n + 1],
        scratch_high
    ));

    if v_neg_1_neg {
        let carry = limbs_slice_add_same_length_in_place_left(&mut scratch[..n], &out_limbs[..n]);
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        if _limbs_add_same_length_with_carry_in_in_place_left(
            &mut out_limbs_hi[..n],
            &out_limbs_lo[n..],
            carry,
        ) {
            hi += 1;
        }
        assert!(!limbs_slice_add_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi.to_unsigned_bitwise()
        ));
    } else {
        let carry = limbs_sub_same_length_in_place_left(&mut scratch[..n], &out_limbs[..n]);
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        if _limbs_sub_same_length_with_borrow_in_in_place_left(
            &mut out_limbs_hi[..n],
            &out_limbs_lo[n..],
            carry,
        ) {
            hi += 1;
        }
        assert!(!limbs_sub_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi.to_unsigned_bitwise()
        ));
    }

    _limbs_mul_same_length_to_out_toom_32_recursive(out_limbs, xs0, ys0);
    // s + t limbs. Use mpn_mul for now, to handle unbalanced operands
    //TODO switch from basecase to to mpn_mul once ready
    if s > t {
        _limbs_mul_to_out_basecase(&mut out_limbs[3 * n..], xs2, ys1);
    } else {
        _limbs_mul_to_out_basecase(&mut out_limbs[3 * n..], ys1, xs2);
    }

    // Remaining interpolation.
    //
    //    y * B + x0 + x3 B^3 - x0 B^2 - x3 B
    //    = (x1 + x3) B + (x0 + x2) B^2 + x0 + x3 B^3 - x0 B^2 - x3 B
    //    = y0 B + y1 B^2 + y3 B^3 + Lx0 + H x0 B
    //      + L x3 B^3 + H x3 B^4 - Lx0 B^2 - H x0 B^3 - L x3 B - H x3 B^2
    //    = L x0 + (y0 + H x0 - L x3) B + (y1 - L x0 - H x3) B^2
    //      + (y2 - (H x0 - L x3)) B^3 + H x3 B^4
    //
    //     B^4       B^3       B^2        B         1
    //|         |         |         |         |         |
    //  +-------+                   +---------+---------+
    //  |  Hx3  |                   | Hx0-Lx3 |    Lx0  |
    //  +------+----------+---------+---------+---------+
    //     |    y2    |  y1     |   y0    |
    //     ++---------+---------+---------+
    //     -| Hx0-Lx3 | - Lx0   |
    //      +---------+---------+
    //             | - Hx3  |
    //             +--------+
    //
    // We must take into account the carry from Hx0 - Lx3.
    {
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        let (out_limbs_0, out_limbs_1) = out_limbs_lo.split_at_mut(n);
        // out_limbs_0: length n, out_limbs_1: length n
        let (out_limbs_2, out_limbs_3) = out_limbs_hi.split_at_mut(n); // out_limbs_2: length n
        let carry = limbs_sub_same_length_in_place_left(out_limbs_1, &out_limbs_3[..n]);
        hi = scratch[2 * n].to_signed_bitwise();
        if carry {
            hi.wrapping_add_assign(1);
        }

        let borrow =
            _limbs_sub_same_length_with_borrow_in_in_place_left(out_limbs_2, out_limbs_0, carry);
        if _limbs_sub_same_length_with_borrow_in_to_out(
            out_limbs_3,
            &scratch[n..2 * n],
            out_limbs_1,
            borrow,
        ) {
            hi -= 1;
        }
    }

    if limbs_slice_add_greater_in_place_left(&mut out_limbs[n..4 * n], &scratch[..n]) {
        hi += 1;
    }

    if s + t > n {
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(4 * n);
        // out_limbs_lo: length 4 * n
        let out_limbs_hi = &mut out_limbs_hi[..s + t - n];
        if limbs_sub_in_place_left(&mut out_limbs_lo[2 * n..], out_limbs_hi) {
            hi -= 1;
        }

        if hi < 0 {
            //TODO remove once this is seen
            panic!("hi < 0 second time: {:?} {:?}", xs, ys);
            assert!(!limbs_sub_limb_in_place(
                out_limbs_hi,
                u32::checked_from(-hi).unwrap()
            ));
        } else {
            assert!(!limbs_slice_add_limb_in_place(
                out_limbs_hi,
                u32::checked_from(hi).unwrap()
            ));
        }
    } else {
        assert_eq!(hi, 0);
    }
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_22`.
///
/// Scratch need is 2 * (xs.len() + k); k is the recursion depth. k is the smallest k such that
///   ceil(xs.len() / 2 ^ k) < MUL_TOOM22_THRESHOLD,
/// which implies that
///   k = bitsize of floor((xs.len() - 1) / (MUL_TOOM22_THRESHOLD - 1))
///     = 1 + floor(log_2(floor((xs.len() - 1) / (MUL_TOOM22_THRESHOLD - 1))))
///
/// The actual scratch size returned is a quicker-to-compute upper bound.
///
/// This is mpn_toom22_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_22_scratch_size(xs_len: usize) -> usize {
    2 * (xs_len + u32::WIDTH as usize)
}

// TODO make these compiler flags?
pub const TUNE_PROGRAM_BUILD: bool = true;
pub const WANT_FAT_BINARY: bool = false;

pub const MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD >= 2 * MUL_TOOM22_THRESHOLD;

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// This is TOOM22_MUL_N_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_same_length_to_out_toom_22_recursive(
    out_limbs: &mut [u32],
    xs: &[u32],
    ys: &[u32],
    scratch: &mut [u32],
) {
    assert_eq!(xs.len(), ys.len());
    if !MAYBE_MUL_TOOM22 || xs.len() < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else {
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    }
}

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// Normally, this calls `_limbs_mul_to_out_basecase` or `_limbs_mul_to_out_toom_22`. But when when
/// the fraction MUL_TOOM33_THRESHOLD / MUL_TOOM22_THRESHOLD is large, an initially small relative
/// unbalance will become a larger and larger relative unbalance with each recursion (the difference
/// s - t will be invariant over recursive calls). Therefore, we need to call
/// `_limbs_mul_to_out_toom_32`.
///
/// This is TOOM22_MUL_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_to_out_toom_22_recursive(
    out_limbs: &mut [u32],
    xs: &[u32],
    ys: &[u32],
    scratch: &mut [u32],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if !MAYBE_MUL_TOOM22 || ys_len < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else if 4 * xs_len < 5 * ys_len {
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    } else if _limbs_mul_to_out_toom_32_input_sizes_valid(xs_len, ys_len) {
        _limbs_mul_to_out_toom_32(out_limbs, xs, ys, scratch);
    } else {
        //TODO replace with mul eventually
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    }
}

/// Interpreting two slices of `u32`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_22_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. xs.len() > 2
/// 4. ys.len() > 0
/// 5a. If xs.len() is even, xs.len() < 2 * ys.len()
/// 5b. If xs.len() is odd, xs.len() + 1 < 2 * ys.len()
///
/// This uses the Toom-22, aka Toom-2, aka Karatsuba algorithm.
///
/// Evaluate in: -1, 0, +inf
///
///  <--s--><--n--->
///   ______________
///  |_xs1_|__xs0__|
///   |ys1_|__ys0__|
///   <-t--><--n--->
///
///  v0   = xs0         * ys0         # X(0)   * Y(0)
///  vm1  = (xs0 - xs1) * (ys0 - ys1) # X(-1)  * Y(-1)
///  vinf = xs1         * ys1         # X(inf) * Y(inf)
///
/// Time: TODO (should be something like O(n<sup>log<sub>2</sub>3</sup>))
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom22_mul from mpn/generic/toom22_mul.c.
pub fn _limbs_mul_to_out_toom_22(
    out_limbs: &mut [u32],
    xs: &[u32],
    ys: &[u32],
    scratch: &mut [u32],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let s = xs_len >> 1;
    let n = xs_len - s;
    assert!(ys_len >= n);
    let t = ys_len - n;

    assert!(s > 0 && (s == n || s == n - 1));
    assert!(0 < t && t <= s);

    let (xs0, xs1) = xs.split_at(n); // xs0: length n, xs1: length s
    let (ys0, ys1) = ys.split_at(n); // ys0: length n, ys1: length t

    let mut v_neg_1_neg = false;
    {
        let (asm1, bsm1) = out_limbs.split_at_mut(n); // asm1: length n

        // Compute asm1.
        if s == n {
            if limbs_cmp_same_length(xs0, xs1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs1, xs0);
                v_neg_1_neg = true;
            } else {
                limbs_sub_same_length_to_out(asm1, xs0, xs1);
            }
        } else {
            // n - s == 1
            if xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs1, &xs0[..s]);
                asm1[s] = 0;
                v_neg_1_neg = true;
            } else {
                asm1[s] = xs0[s];
                if limbs_sub_same_length_to_out(asm1, &xs0[..s], xs1) {
                    asm1[s].wrapping_sub_assign(1);
                }
            }
        }

        // Compute bsm1.
        if t == n {
            if limbs_cmp_same_length(ys0, ys1) == Ordering::Less {
                limbs_sub_same_length_to_out(bsm1, ys1, ys0);
                v_neg_1_neg.not_assign();
            } else {
                limbs_sub_same_length_to_out(bsm1, ys0, ys1);
            }
        } else {
            if limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
            {
                limbs_sub_same_length_to_out(bsm1, ys1, &ys0[..t]);
                limbs_set_zero(&mut bsm1[t..n]);
                v_neg_1_neg.not_assign();
            } else {
                limbs_sub_to_out(bsm1, ys0, ys1);
            }
        }

        let (v_neg_1, scratch_out) = scratch.split_at_mut(2 * n); // v_neg_1: length 2 * n
        _limbs_mul_same_length_to_out_toom_22_recursive(v_neg_1, asm1, &bsm1[..n], scratch_out);
    }
    let (v_neg_1, scratch_out) = scratch.split_at_mut(2 * n); // v_neg_1: length 2 * n
    let mut carry = 0;
    let mut carry2;
    {
        let (v_0, v_pos_inf) = out_limbs.split_at_mut(2 * n); // v_0: length 2 * n
        if s > t {
            _limbs_mul_to_out_toom_22_recursive(v_pos_inf, xs1, ys1, scratch_out);
        } else {
            _limbs_mul_same_length_to_out_toom_22_recursive(v_pos_inf, xs1, &ys1[..s], scratch_out);
        }

        // v_0, 2 * n limbs
        _limbs_mul_same_length_to_out_toom_22_recursive(v_0, xs0, ys0, scratch_out);

        // H(v_0) + L(v_pos_inf)
        if limbs_slice_add_same_length_in_place_left(&mut v_pos_inf[..n], &v_0[n..]) {
            carry += 1;
        }

        // L(v_0) + H(v_0)
        carry2 = carry;
        let (v_0_lo, v_0_hi) = v_0.split_at_mut(n); // v_0_lo: length n, vo_hi: length n
        if limbs_add_same_length_to_out(v_0_hi, &v_pos_inf[..n], v_0_lo) {
            carry2 += 1;
        }

        // L(v_pos_inf) + H(v_pos_inf)
        let (v_pos_inf_lo, v_pos_inf_hi) = v_pos_inf.split_at_mut(n); // v_pos_inf_lo: length n

        // s + t - n == either ys_len - (xs_len >> 1) or ys_len - (xs_len >> 1) - 2.
        // n == xs_len - (xs_len >> 1) and xs_len >= ys_len.
        // So n >= s + t - n.
        if limbs_slice_add_greater_in_place_left(v_pos_inf_lo, &v_pos_inf_hi[..s + t - n]) {
            carry += 1;
        }
    }

    if v_neg_1_neg {
        if limbs_slice_add_same_length_in_place_left(&mut out_limbs[n..3 * n], v_neg_1) {
            carry += 1;
        }
    } else {
        if limbs_sub_same_length_in_place_left(&mut out_limbs[n..3 * n], v_neg_1) {
            carry.wrapping_sub_assign(1);
        }
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut out_limbs[2 * n..2 * n + s + t],
        carry2
    ));
    if carry <= 2 {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[3 * n..2 * n + s + t],
            carry
        ));
    } else {
        assert!(!limbs_sub_limb_in_place(
            &mut out_limbs[3 * n..2 * n + s + t],
            1
        ));
    }
}

//TODO test
// multiply natural numbers.
// mpn_mul_n from mpn/generic/mul_n.c
pub fn mpn_mul_n(_p: &mut [u32], a: &[u32], b: &[u32]) {
    let n = a.len();
    assert_eq!(b.len(), n);
    unimplemented!();
    //TODO PASTE B
}

//TODO test
// Multiply two natural numbers.
// mpn_mul from mpn/generic/mul.c
pub fn mpn_mul(_prodp: &mut [u32], _up: &[u32], _vp: &[u32]) -> u32 {
    unimplemented!();
    //TODO PASTE C
}

//TODO update docs
// 1 < v.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < u.len()
//
// This is currently not measurably better than just basecase.
fn mpn_mul_basecase_mem_opt_helper(prod: &mut [u32], u: &[u32], v: &[u32]) {
    let u_len = u.len();
    let v_len = v.len();
    assert!(1 < v_len);
    assert!(v_len < MUL_TOOM22_THRESHOLD);
    assert!(MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN);
    assert!(MUL_BASECASE_MAX_UN < u_len);
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in u.chunks(MUL_BASECASE_MAX_UN) {
        if chunk.len() >= v_len {
            _limbs_mul_to_out_basecase(&mut prod[offset..], chunk, v);
        } else {
            _limbs_mul_to_out_basecase(&mut prod[offset..], v, chunk);
        }
        if offset != 0 {
            limbs_slice_add_greater_in_place_left(&mut prod[offset..], &triangle_buffer[..v_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < u_len {
            triangle_buffer[..v_len].copy_from_slice(&prod[offset..offset + v_len]);
        }
    }
}

//TODO update docs
fn mpn_mul_basecase_mem_opt(prod: &mut [u32], u: &[u32], v: &[u32]) {
    let u_len = u.len();
    let v_len = v.len();
    assert!(u_len >= v_len);
    if v_len > 1 && v_len < MUL_TOOM22_THRESHOLD && u.len() > MUL_BASECASE_MAX_UN {
        mpn_mul_basecase_mem_opt_helper(prod, u, v)
    } else {
        _limbs_mul_to_out_basecase(prod, u, v);
    }
}

pub fn mul_helper(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        mpn_mul(&mut product_limbs, xs, ys);
    } else {
        mpn_mul(&mut product_limbs, ys, xs);
    }
    product_limbs
}

pub fn mul_basecase_helper(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        _limbs_mul_to_out_basecase(&mut product_limbs, xs, ys);
    } else {
        _limbs_mul_to_out_basecase(&mut product_limbs, ys, xs);
    }
    product_limbs
}

fn mul_basecase_mem_opt_helper(xs: &[u32], ys: &[u32]) -> Vec<u32> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        mpn_mul_basecase_mem_opt(&mut product_limbs, xs, ys);
    } else {
        mpn_mul_basecase_mem_opt(&mut product_limbs, ys, xs);
    }
    product_limbs
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * Natural::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl Mul<Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<&'a Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: &'a Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by reference and the right
/// `Natural` by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * Natural::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<Natural> for &'a Natural {
    type Output = Natural;

    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a, 'b> Mul<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn mul(self, other: &'a Natural) -> Natural {
        if let Small(y) = *other {
            self * y
        } else if let Small(x) = *self {
            other * x
        } else {
            match (self, other) {
                (&Large(ref xs), &Large(ref ys)) => {
                    //TODO change to non-basecase once everything is implemented
                    let mut product = Large(mul_basecase_helper(xs, ys));
                    product.trim();
                    product
                }
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by value.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= Natural::from_str("1000").unwrap();
///     x *= Natural::from_str("2000").unwrap();
///     x *= Natural::from_str("3000").unwrap();
///     x *= Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "24000000000000");
/// }
/// ```
impl MulAssign<Natural> for Natural {
    fn mul_assign(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref mut ys)) => {
                    //TODO change to non-basecase once everything is implemented
                    *xs = mul_basecase_helper(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// Time: worst case O(mn)
///
/// Additional memory: worst case O(m+n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= &Natural::from_str("1000").unwrap();
///     x *= &Natural::from_str("2000").unwrap();
///     x *= &Natural::from_str("3000").unwrap();
///     x *= &Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "24000000000000");
/// }
/// ```
impl<'a> MulAssign<&'a Natural> for Natural {
    fn mul_assign(&mut self, other: &'a Natural) {
        if let Small(y) = *other {
            *self *= y;
        } else if let Small(x) = *self {
            *self = other * x;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    //TODO change to non-basecase once everything is implemented
                    *xs = mul_basecase_helper(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

impl Natural {
    pub fn _mul_assign_basecase_mem_opt(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref ys)) => {
                    *xs = mul_basecase_mem_opt_helper(xs, ys)
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}
