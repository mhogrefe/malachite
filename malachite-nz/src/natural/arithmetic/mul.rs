use malachite_base::limbs::{limbs_set_zero, limbs_test_zero};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{PrimitiveInteger, WrappingSubAssign};
use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left, mpn_add_nc_in_place,
};
use natural::arithmetic::add_mul_u32::mpn_addmul_1;
use natural::arithmetic::add_u32::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::arithmetic::div_exact_u32::limbs_div_exact_3_in_place;
use natural::arithmetic::mul_u32::limbs_mul_limb_to_out;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::{
    limbs_sub_in_place_left, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_to_out, limbs_sub_to_out,
    mpn_sub_nc, mpn_sub_nc_in_place,
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

//TODO test
// docs preserved
// Internal routine to multiply two natural numbers of length m and n.
// Multiply up by vpand write the result to {prodp, up.len() + vp.len()}. Must have usize >= vsize.
//
// Note that prodp gets up.len() + vp.len() limbs stored, even if the actual result only needs
// up.len() + vp.len() -1.
//
// There's no good reason to call here with v.len() >= MUL_TOOM22_THRESHOLD. Currently this is
// allowed, but it might not be in the future.
//
// This is the most critical code for multiplication. All multiplies rely on this, both small and
// huge. Small ones arrive here immediately, huge ones arrive here as this is the base case for
// Karatsuba's recursive algorithm.
// mul_basecase from mpn/generic/mul_basecase.c
pub fn mpn_mul_basecase(rp: &mut [u32], up: &[u32], vp: &[u32]) -> u32 {
    let un = up.len();
    let mut vn = vp.len();
    assert!(un >= vn);
    assert!(vn >= 1);

    // We first multiply by the low order limb (or depending on optional function availability,
    // limbs). This result can be stored, not added, to rp. We also avoid a loop for zeroing this
    // way.
    rp[un] = limbs_mul_limb_to_out(rp, up, vp[0]);
    let mut rp_offset = 1;
    let mut vp_offset = 1;
    vn -= 1;

    // Now accumulate the product of up[] and the next higher limb (or depending on optional
    // function availability, limbs) from vp[].

    while vn >= 1 {
        rp[rp_offset + un] = mpn_addmul_1(&mut rp[rp_offset..], up, vp[vp_offset]);
        rp_offset += 1;
        vp_offset += 1;
        vn -= 1;
    }
    rp[un + vp.len() - 1] // remove once mpn_mul is ready
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
    assert!(!limbs_slice_add_limb_in_place(&mut c3[1..twor + k], cy)); // 2n-(3k+1) = 2r+k-1
    // Memory allocated for vm1 is now free, it can be recycled

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

//TODO test
// docs preserved
//  Evaluate in: -1, 0, +1, +inf
//
// <-s-><--n--><--n-->
//  ___ ______ ______
// |a2_|___a1_|___a0_|
//   |_b1_|___b0_|
//   <-t--><--n-->
//
// v0  =  a0         * b0      #   A(0)*B(0)
// v1  = (a0+ a1+ a2)*(b0+ b1) #   A(1)*B(1)      ah  <= 2  bh <= 1
// vm1 = (a0- a1+ a2)*(b0- b1) #  A(-1)*B(-1)    |ah| <= 1  bh = 0
// vinf=          a2 *     b1  # A(inf)*B(inf)
// TOOM32_MUL_N_REC from mpn/generic/toom32_mul.c
pub fn toom32_mul_n_rec(p: &mut [u32], a: &[u32], b: &[u32]) {
    mpn_mul_n(p, a, b)
}

//TODO test
// docs preserved
// Multiply {ap,an} and {bp,bn} where an is nominally 1.5 times as large as bn. Or more accurately,
// bn < an < 3bn.
// mpn_toom32_mul from mpn/generic/toom32_mul.c
pub fn mpn_toom32_mul(pp: &mut [u32], ap: &[u32], bp: &[u32], scratch: &mut [u32]) {
    let an = ap.len();
    let bn = bp.len();

    let n = 1 + if 2 * an >= 3 * bn {
        (an - 1) / 3
    } else {
        (bn - 1) >> 1
    };

    let a0 = ap;
    let a1 = &ap[n..];
    let a2 = &ap[2 * n..];
    let b0 = bp;
    let b1 = &bp[n..];

    // Required, to ensure that s + t >= n.
    assert!(bn + 2 <= an && an + 6 <= 3 * bn);

    let s = an - 2 * n;
    let t = bn - n;

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    assert!(s + t >= n);
    let mut hi;
    let mut vm1_neg;
    {
        // Product area of size an + bn = 3*n + s + t >= 4*n + 2.
        let (pp_lo, pp_hi) = pp.split_at_mut(2 * n);
        let (am1, bm1) = pp_hi.split_at_mut(n);
        {
            let (ap1, bp1) = pp_lo.split_at_mut(n);

            // Scratch need: 2*n + 1 + scratch for the recursive multiplications.

            // Compute ap1 = a0 + a1 + a3, am1 = a0 - a1 + a3
            let mut ap1_hi = if limbs_add_to_out(ap1, &a0[..n], &a2[..s]) {
                1
            } else {
                0
            };
            if ap1_hi == 0 && limbs_cmp_same_length(&ap1[..n], &a1[..n]) == Ordering::Less {
                assert!(!limbs_sub_same_length_to_out(am1, &a1[..n], &ap1[..n]));
                hi = 0;
                vm1_neg = 1;
            } else {
                hi = ap1_hi
                    - if limbs_sub_same_length_to_out(am1, &ap1[..n], &a1[..n]) {
                        1
                    } else {
                        0
                    };
                vm1_neg = 0;
            }
            ap1_hi += if limbs_slice_add_same_length_in_place_left(&mut ap1[..n], &a1[..n]) {
                1
            } else {
                0
            };

            let bp1_hi;
            // Compute bp1 = b0 + b1 and bm1 = b0 - b1.
            if t == n {
                bp1_hi = if limbs_add_same_length_to_out(bp1, &b0[..n], &b1[..n]) {
                    1
                } else {
                    0
                };

                if limbs_cmp_same_length(&b0[..n], &b1[..n]) == Ordering::Less {
                    assert!(!limbs_sub_same_length_to_out(bm1, &b1[..n], &b0[..n]));
                    vm1_neg ^= 1;
                } else {
                    assert!(!limbs_sub_same_length_to_out(bm1, &b0[..n], &b1[..n]));
                }
            } else {
                bp1_hi = if limbs_add_to_out(bp1, &b0[..n], &b1[..t]) {
                    1
                } else {
                    0
                };
                if limbs_test_zero(&b0[t..n])
                    && limbs_cmp_same_length(&b0[..t], &b1[..t]) == Ordering::Less
                {
                    assert!(!limbs_sub_same_length_to_out(bm1, &b1[..t], &b0[..t]));
                    limbs_set_zero(&mut bm1[t..n]);
                    vm1_neg ^= 1;
                } else {
                    assert!(!limbs_sub_to_out(bm1, &b0[..n], &b1[..t]));
                }
            }

            toom32_mul_n_rec(scratch, ap1, bp1);
            let mut cy = if ap1_hi == 1 {
                bp1_hi
                    + if limbs_slice_add_same_length_in_place_left(
                        &mut scratch[n..2 * n],
                        &bp1[..n],
                    ) {
                        1
                    } else {
                        0
                    }
            } else if ap1_hi == 2 {
                2 * bp1_hi + mpn_addmul_1(&mut scratch[n..], bp1, 2)
            } else {
                0
            };
            if bp1_hi != 0 {
                cy +=
                    if limbs_slice_add_same_length_in_place_left(&mut scratch[n..2 * n], &ap1[..n])
                    {
                        1
                    } else {
                        0
                    };
            }
            scratch[2 * n] = cy;
        }
        // This isn't quite right; would like to write to pp_lo + 1 limb
        toom32_mul_n_rec(pp_lo, am1, bm1);
        let (_, bp1) = pp_lo.split_at_mut(n);
        if hi != 0 {
            hi = if limbs_slice_add_same_length_in_place_left(bp1, &bm1[..n]) {
                1
            } else {
                0
            };
        }
    }
    pp[2 * n] = hi;

    // v1 <-- (v1 + vm1) / 2 = x0 + x2
    if vm1_neg != 0 {
        limbs_sub_same_length_in_place_left(&mut scratch[..2 * n + 1], &pp[..2 * n + 1]);
        assert_eq!(limbs_slice_shr_in_place(&mut scratch[..2 * n + 1], 1), 0);
    } else {
        limbs_slice_add_same_length_in_place_left(&mut scratch[..2 * n + 1], &pp[..2 * n + 1]);
        assert_eq!(limbs_slice_shr_in_place(&mut scratch[..2 * n + 1], 1), 0);
    }

    // We get x1 + x3 = (x0 + x2) - (x0 - x1 + x2 - x3), and hence
    //
    // y = x1 + x3 + (x0 + x2) * B
    //   = (x0 + x2) * B + (x0 + x2) - vm1.
    //
    // y is 3*n + 1 limbs, y = y0 + y1 B + y2 B^2. We store them as
    // follows: y0 at scratch, y1 at pp + 2*n, and y2 at scratch + n
    // (already in place, except for carry propagation).
    //
    // We thus add
    //
    //    B^3  B^2   B    1
    //     |    |    |    |
    //    +-----+----+
    //  + |  x0 + x2 |
    //    +----+-----+----+
    //  +      |  x0 + x2 |
    //     +----------+
    //  -      |  vm1     |
    //  --+----++----+----+-
    //    | y2  | y1 | y0 |
    //    +-----+----+----+

    // Since we store y0 at the same location as the low half of x0 + x2, we need to do the middle
    // sum first.
    hi = pp[2 * n];
    let mut cy =
        if limbs_add_same_length_to_out(&mut pp[2 * n..], &scratch[..n], &scratch[n..2 * n]) {
            1
        } else {
            0
        };
    let v1_high = scratch[2 * n];
    assert!(!limbs_slice_add_limb_in_place(
        &mut scratch[n..2 * n + 1],
        cy + v1_high
    ));

    if vm1_neg != 0 {
        cy = if limbs_slice_add_same_length_in_place_left(&mut scratch[..n], &pp[..n]) {
            1
        } else {
            0
        };
        let (pp_lo, pp_hi) = pp.split_at_mut(2 * n);
        hi += mpn_add_nc_in_place(&mut pp_hi[..n], pp_lo, cy);
        assert!(!limbs_slice_add_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi
        ));
    } else {
        cy = if limbs_sub_same_length_in_place_left(&mut scratch[..n], &pp[..n]) {
            1
        } else {
            0
        };
        let (pp_lo, pp_hi) = pp.split_at_mut(2 * n);
        hi += mpn_sub_nc_in_place(&mut pp_hi[..n], pp_lo, cy);
        assert!(!limbs_sub_limb_in_place(&mut scratch[n..2 * n + 1], hi));
    }

    toom32_mul_n_rec(pp, a0, b0);
    // vinf, s+t limbs. Use mpn_mul for now, to handle unbalanced operands
    if s > t {
        mpn_mul(&mut pp[3 * n..], &a2[..s], &b1[..t]);
    } else {
        mpn_mul(&mut pp[3 * n..], &b1[..t], &a2[..s]);
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
    //
    {
        let (pp_lo, pp_hi) = pp.split_at_mut(2 * n);
        let (pp0, pp1) = pp_lo.split_at_mut(n);
        let (pp2, pp3) = pp_hi.split_at_mut(n);
        cy = if limbs_sub_same_length_in_place_left(pp1, pp3) {
            1
        } else {
            0
        };
        hi = scratch[2 * n] + cy;

        cy = mpn_sub_nc_in_place(pp2, pp0, cy);
        hi -= mpn_sub_nc(pp3, &scratch[n..2 * n], pp1, cy);
    }

    hi += if limbs_slice_add_greater_in_place_left(&mut pp[n..4 * n], &scratch[..n]) {
        1
    } else {
        0
    };

    if s + t > n {
        let (pp_lo, pp_hi) = pp.split_at_mut(4 * n);
        let mut hi = hi as i32;
        assert!(hi >= 0);
        hi -= if limbs_sub_in_place_left(&mut pp_lo[2 * n..], &pp_hi[..s + t - n]) {
            1
        } else {
            0
        };

        if hi < 0 {
            assert!(!limbs_sub_limb_in_place(
                &mut pp_hi[..s + t - n],
                u32::checked_from(-hi).unwrap()
            ));
        } else {
            assert!(!limbs_slice_add_limb_in_place(
                &mut pp_hi[..s + t - n],
                u32::checked_from(hi).unwrap()
            ));
        }
    } else {
        assert_eq!(hi, 0);
    }
}

//TODO test
// docs preserved
// Scratch need is 2*(an + k), k is the recursion depth. k is the smallest k such that
//   ceil(an / 2 ^ k) < MUL_TOOM22_THRESHOLD,
// which implies that
//   k = bitsize of floor ((an - 1) / (MUL_TOOM22_THRESHOLD - 1))
//     = 1 + floor (log_2(floor((an - 1) / (MUL_TOOM22_THRESHOLD - 1))))
// mpn_toom22_mul_itch from gmp-impl.h
pub fn mpn_toom22_mul_itch(an: usize) -> usize {
    2 * (an + u32::WIDTH as usize)
}

// TODO make these compiler flags?
pub const TUNE_PROGRAM_BUILD: bool = true;
pub const WANT_FAT_BINARY: bool = false;

pub const MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD >= 2 * MUL_TOOM22_THRESHOLD;

//TODO test
// docs preserved
// TOOM22_MUL_N_REC in mpn/generic/toom22_mul.c
pub fn toom22_mul_n_rec(p: &mut [u32], a: &[u32], b: &[u32], ws: &mut [u32]) {
    assert_eq!(a.len(), b.len());
    if !MAYBE_MUL_TOOM22 || a.len() < MUL_TOOM22_THRESHOLD {
        mpn_mul_basecase(p, a, b);
    } else {
        mpn_toom22_mul(p, a, b, ws);
    }
}

//TODO test
// docs preserved
// Normally, this calls mul_basecase or toom22_mul. But when when the fraction
// MUL_TOOM33_THRESHOLD / MUL_TOOM22_THRESHOLD is large, an initially small relative unbalance will
// become a larger and larger relative unbalance with each recursion (the difference s - t will be
// invariant over recursive calls). Therefore, we need to call toom32_mul.
// MPN_TOOM22_MUL from mpn/generic/toom22_mul.c
pub fn toom22_mul_rec(p: &mut [u32], a: &[u32], b: &[u32], ws: &mut [u32]) {
    let an = a.len();
    let bn = b.len();
    if !MAYBE_MUL_TOOM22 || bn < MUL_TOOM22_THRESHOLD {
        mpn_mul_basecase(p, a, b);
    } else if 4 * an < 5 * bn {
        mpn_toom22_mul(p, a, b, ws);
    } else {
        mpn_toom32_mul(p, a, b, ws);
    }
}

//TODO test
// docs preserved
// Multiply {ap,an} and {bp,bn} where an >= bn. Or more accurately, bn <= an < 2bn.
//
// Evaluate in: -1, 0, +inf
//
//  <-s--><--n-->
//   ____ ______
//  |_a1_|___a0_|
//   |b1_|___b0_|
//   <-t-><--n-->
//
//  v0  =  a0     * b0       #   A(0)*B(0)
//  vm1 = (a0- a1)*(b0- b1)  #  A(-1)*B(-1)
//  vinf =      a1 *     b1   # A(inf)*B(inf)
//
//
// mpn_toom22_mul from mpn/generic/toom22_mul.c
pub fn mpn_toom22_mul(pp: &mut [u32], ap: &[u32], bp: &[u32], scratch: &mut [u32]) {
    let an = ap.len();
    let bn = bp.len();

    let s = an >> 1;
    let n = an - s;
    let t = bn - n;

    let a0 = ap;
    let a1 = &ap[n..];
    let b0 = bp;
    let b1 = &bp[n..];

    assert!(an >= bn);

    assert!(0 < s && s <= n && s >= n - 1);
    assert!(0 < t && t <= s);
    let mut vm1_neg = 0;
    {
        let (asm1, bsm1) = pp.split_at_mut(n);
        // Compute asm1.
        if s == n {
            if limbs_cmp_same_length(&a0[..n], &a1[..n]) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, &a1[..n], &a0[..n]);
                vm1_neg = 1;
            } else {
                limbs_sub_same_length_to_out(asm1, &a0[..n], &a1[..n]);
            }
        } else {
            // n - s == 1
            if a0[s] == 0 && limbs_cmp_same_length(&a0[..s], &a1[..s]) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, &a1[..s], &a0[..s]);
                asm1[s] = 0;
                vm1_neg = 1;
            } else {
                asm1[s] = a0[s]
                    - if limbs_sub_same_length_to_out(asm1, &a0[..s], &a1[..s]) {
                        1
                    } else {
                        0
                    };
            }
        }

        // Compute bsm1.
        if t == n {
            if limbs_cmp_same_length(&b0[..n], &b1[..n]) == Ordering::Less {
                limbs_sub_same_length_to_out(bsm1, &b1[..n], &b0[..n]);
                vm1_neg ^= 1;
            } else {
                limbs_sub_same_length_to_out(bsm1, &b0[..n], &b1[..n]);
            }
        } else {
            if limbs_test_zero(&b0[t..n])
                && limbs_cmp_same_length(&b0[..t], &b1[..t]) == Ordering::Less
            {
                limbs_sub_same_length_to_out(bsm1, &b1[..t], &b0[..t]);
                limbs_set_zero(&mut bsm1[t..n]);
                vm1_neg ^= 1;
            } else {
                limbs_sub_to_out(bsm1, &b0[..n], &b1[..t]);
            }
        }

        let (vm1, scratch_out) = scratch.split_at_mut(2 * n); // 2n

        // vm1, 2n limbs
        toom22_mul_n_rec(vm1, &asm1[..n], &bsm1[..n], scratch_out);
    }
    let (vm1, scratch_out) = scratch.split_at_mut(2 * n);
    let mut cy;
    let cy2;
    {
        let (v0, vinf) = pp.split_at_mut(2 * n); // 2n, s + t
        if s > t {
            toom22_mul_rec(vinf, &a1[..s], &b1[..t], scratch_out);
        } else {
            toom22_mul_n_rec(vinf, &a1[..s], &b1[..s], scratch_out);
        }

        // v0, 2n limbs
        toom22_mul_n_rec(v0, &ap[..n], &bp[..n], scratch_out);

        // H(v0) + L(vinf)
        cy = if limbs_slice_add_same_length_in_place_left(&mut vinf[n..2 * n], &v0[n..2 * n]) {
            1
        } else {
            0
        };

        // L(v0) + H(v0)
        cy2 = {
            let (v0_lo, v0_hi) = v0.split_at_mut(n);
            cy + if limbs_add_same_length_to_out(&mut v0_hi[..n], &vinf[..n], v0_lo) {
                1
            } else {
                0
            }
        };

        // L(vinf) + H(vinf)
        cy += {
            let (vinf_lo, vinf_hi) = vinf.split_at_mut(n);
            if limbs_slice_add_greater_in_place_left(vinf_lo, &vinf_hi[..s + t - n]) {
                1
            } else {
                0
            }
        };
    }

    if vm1_neg != 0 {
        cy += if limbs_slice_add_same_length_in_place_left(&mut pp[n..3 * n], &vm1[..2 * n]) {
            1
        } else {
            0
        };
    } else {
        cy -= if limbs_sub_same_length_in_place_left(&mut pp[n..3 * n], &vm1[..2 * n]) {
            1
        } else {
            0
        };
    }

    assert!(cy + 1 <= 3);
    assert!(cy2 <= 2);

    assert!(!limbs_slice_add_limb_in_place(
        &mut pp[2 * n..2 * n + s + t],
        cy2
    ));
    if cy <= 2 {
        // if s + t == n, cy is zero, but we should not access pp[3 * n] at all.
        assert!(!limbs_slice_add_limb_in_place(
            &mut pp[3 * n..2 * n + s + t],
            cy
        ));
    } else {
        assert!(!limbs_sub_limb_in_place(&mut pp[3 * n..2 * n + s + t], 1));
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
            mpn_mul_basecase(&mut prod[offset..], chunk, v);
        } else {
            mpn_mul_basecase(&mut prod[offset..], v, chunk);
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
        mpn_mul_basecase(prod, u, v);
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
        mpn_mul_basecase(&mut product_limbs, xs, ys);
    } else {
        mpn_mul_basecase(&mut product_limbs, ys, xs);
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
