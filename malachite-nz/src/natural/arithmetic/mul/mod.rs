use malachite_base::misc::Max;
use malachite_base::num::{PrimitiveInteger, WrappingAddAssign};
use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::arithmetic::add_mul_limb::mpn_addmul_1;
use natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_32,
    _limbs_mul_greater_to_out_toom_33, _limbs_mul_greater_to_out_toom_33_scratch_size,
    _limbs_mul_greater_to_out_toom_42, MUL_TOOM22_THRESHOLD, MUL_TOOM33_THRESHOLD,
    MUL_TOOM33_THRESHOLD_LIMIT, MUL_TOOM44_THRESHOLD,
};
use natural::arithmetic::mul_limb::limbs_mul_limb_to_out;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::sub::limbs_sub_same_length_to_out;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural::{self, Large, Small};
use platform::Limb;
use std::cmp::Ordering;
use std::ops::{Mul, MulAssign};

//TODO use better algorithms

//TODO test
// docs preserved
// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, computation is mod
// B ^ rn - 1, and values are semi-normalised; zero is represented as either 0 or B ^ n - 1. Needs a
// scratch of 2rn limbs at tp.
// mpn_bc_mulmod_bnm1 from mpn/generic/mulmod_bnm1.c
pub fn mpn_bc_mulmod_bnm1(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], tp: &mut [Limb]) {
    let rn = ap.len();
    assert_ne!(rn, 0);
    limbs_mul_same_length_to_out(tp, ap, bp);
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
pub fn mpn_bc_mulmod_bnp1(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], tp: &mut [Limb]) {
    let rn = ap.len() - 1;
    assert_ne!(rn, 0);
    limbs_mul_same_length_to_out(tp, ap, bp);
    assert_eq!(tp[2 * rn + 1], 0);
    assert!(tp[2 * rn] < Limb::MAX);
    let cy = tp[2 * rn]
        + if limbs_sub_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
            1
        } else {
            0
        };
    rp[rn] = 0;
    limbs_slice_add_limb_in_place(&mut rp[..=rn], cy);
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
    FFTTableNK { n: 1_023, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1_215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1_279, k: 13 },
    FFTTableNK { n: 703, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1_663, k: 13 },
    FFTTableNK { n: 959, k: 15 },
    FFTTableNK { n: 255, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1_087, k: 12 },
    FFTTableNK { n: 2_175, k: 13 },
    FFTTableNK { n: 1_215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1_343, k: 12 },
    FFTTableNK { n: 2_687, k: 13 },
    FFTTableNK { n: 1_407, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1_535, k: 12 },
    FFTTableNK { n: 3_199, k: 13 },
    FFTTableNK { n: 1_663, k: 14 },
    FFTTableNK { n: 895, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1_023, k: 13 },
    FFTTableNK { n: 2_175, k: 14 },
    FFTTableNK { n: 1_151, k: 13 },
    FFTTableNK { n: 2_303, k: 12 },
    FFTTableNK { n: 4_607, k: 13 },
    FFTTableNK { n: 2_431, k: 12 },
    FFTTableNK { n: 4_863, k: 14 },
    FFTTableNK { n: 1_279, k: 13 },
    FFTTableNK { n: 2_687, k: 14 },
    FFTTableNK { n: 1_407, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1_535, k: 13 },
    FFTTableNK { n: 3_199, k: 14 },
    FFTTableNK { n: 1_663, k: 13 },
    FFTTableNK { n: 3_327, k: 12 },
    FFTTableNK { n: 6_655, k: 13 },
    FFTTableNK { n: 3_455, k: 12 },
    FFTTableNK { n: 6_911, k: 14 },
    FFTTableNK { n: 1_791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1_023, k: 14 },
    FFTTableNK { n: 2_175, k: 13 },
    FFTTableNK { n: 4_351, k: 12 },
    FFTTableNK { n: 8_703, k: 14 },
    FFTTableNK { n: 2_303, k: 13 },
    FFTTableNK { n: 4_607, k: 14 },
    FFTTableNK { n: 2_431, k: 13 },
    FFTTableNK { n: 4_863, k: 15 },
    FFTTableNK { n: 1_279, k: 14 },
    FFTTableNK { n: 2_815, k: 13 },
    FFTTableNK { n: 5_631, k: 14 },
    FFTTableNK { n: 2_943, k: 13 },
    FFTTableNK { n: 5_887, k: 12 },
    FFTTableNK { n: 11_775, k: 15 },
    FFTTableNK { n: 1_535, k: 14 },
    FFTTableNK { n: 3_199, k: 13 },
    FFTTableNK { n: 6_399, k: 14 },
    FFTTableNK { n: 3_327, k: 13 },
    FFTTableNK { n: 6_655, k: 14 },
    FFTTableNK { n: 3_455, k: 13 },
    FFTTableNK { n: 6_911, k: 15 },
    FFTTableNK { n: 1_791, k: 14 },
    FFTTableNK { n: 3_583, k: 13 },
    FFTTableNK { n: 7_167, k: 14 },
    FFTTableNK { n: 3_839, k: 13 },
    FFTTableNK { n: 7_679, k: 16 },
    FFTTableNK { n: 65_536, k: 17 },
    FFTTableNK { n: 131_072, k: 18 },
    FFTTableNK { n: 262_144, k: 19 },
    FFTTableNK { n: 524_288, k: 20 },
    FFTTableNK {
        n: 1_048_576,
        k: 21,
    },
    FFTTableNK {
        n: 2_097_152,
        k: 22,
    },
    FFTTableNK {
        n: 4_194_304,
        k: 23,
    },
    FFTTableNK {
        n: 8_388_608,
        k: 24,
    },
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
    FFTTableNK { n: 1_023, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1_215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1_279, k: 13 },
    FFTTableNK { n: 703, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1_663, k: 13 },
    FFTTableNK { n: 959, k: 15 },
    FFTTableNK { n: 255, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1_087, k: 12 },
    FFTTableNK { n: 2_175, k: 13 },
    FFTTableNK { n: 1_215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1_343, k: 12 },
    FFTTableNK { n: 2_687, k: 13 },
    FFTTableNK { n: 1_407, k: 12 },
    FFTTableNK { n: 2_815, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1_535, k: 12 },
    FFTTableNK { n: 3_199, k: 13 },
    FFTTableNK { n: 1_663, k: 14 },
    FFTTableNK { n: 895, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1_023, k: 13 },
    FFTTableNK { n: 2_175, k: 14 },
    FFTTableNK { n: 1_151, k: 13 },
    FFTTableNK { n: 2_303, k: 12 },
    FFTTableNK { n: 4_607, k: 13 },
    FFTTableNK { n: 2_431, k: 12 },
    FFTTableNK { n: 4_863, k: 14 },
    FFTTableNK { n: 1_279, k: 13 },
    FFTTableNK { n: 2_687, k: 14 },
    FFTTableNK { n: 1_407, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1_535, k: 13 },
    FFTTableNK { n: 3_199, k: 14 },
    FFTTableNK { n: 1_663, k: 13 },
    FFTTableNK { n: 3_327, k: 12 },
    FFTTableNK { n: 6_655, k: 13 },
    FFTTableNK { n: 3_455, k: 14 },
    FFTTableNK { n: 1_791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1_023, k: 14 },
    FFTTableNK { n: 2_175, k: 13 },
    FFTTableNK { n: 4_351, k: 12 },
    FFTTableNK { n: 8_703, k: 14 },
    FFTTableNK { n: 2_303, k: 13 },
    FFTTableNK { n: 4_607, k: 14 },
    FFTTableNK { n: 2_431, k: 13 },
    FFTTableNK { n: 4_863, k: 15 },
    FFTTableNK { n: 1_279, k: 14 },
    FFTTableNK { n: 2_815, k: 13 },
    FFTTableNK { n: 5_631, k: 14 },
    FFTTableNK { n: 2_943, k: 13 },
    FFTTableNK { n: 5_887, k: 12 },
    FFTTableNK { n: 11_775, k: 15 },
    FFTTableNK { n: 1_535, k: 14 },
    FFTTableNK { n: 3_199, k: 13 },
    FFTTableNK { n: 6_399, k: 14 },
    FFTTableNK { n: 3_327, k: 13 },
    FFTTableNK { n: 6_655, k: 14 },
    FFTTableNK { n: 3_455, k: 15 },
    FFTTableNK { n: 1_791, k: 14 },
    FFTTableNK { n: 3_583, k: 13 },
    FFTTableNK { n: 7_167, k: 14 },
    FFTTableNK { n: 3_839, k: 16 },
    FFTTableNK { n: 65_536, k: 17 },
    FFTTableNK { n: 131_072, k: 18 },
    FFTTableNK { n: 262_144, k: 19 },
    FFTTableNK { n: 524_288, k: 20 },
    FFTTableNK {
        n: 1_048_576,
        k: 21,
    },
    FFTTableNK {
        n: 2_097_152,
        k: 22,
    },
    FFTTableNK {
        n: 4_194_304,
        k: 23,
    },
    FFTTableNK {
        n: 8_388_608,
        k: 24,
    },
];

const MPN_FFT_TABLE_3: [[FFTTableNK; FFT_TABLE3_SIZE]; 2] = [MUL_FFT_TABLE3, SQR_FFT_TABLE3];

//TODO test
// checked
// docs preserved
// Find the best k to use for a mod 2 ^ (m * Limb::WIDTH) + 1 FFT for m >= n. We have sqr = 0 if for
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

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
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
pub fn _limbs_mul_greater_to_out_basecase(out_limbs: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
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
}

/// Given a `Natural` whose highest limb is `carry` and remaining limbs are `xs`, multiplies the
/// `Natural` by 4 and adds the `Natural` whose limbs are `ys`. The highest limb of the result is
/// written back to `carry` and the remaining limbs are written to `out_limbs`.
///
/// /// Time: worst case O(n)
/////
///// Additional memory: worst case O(1)
/////
///// where n = max(`xs.len()`, `ys.len()`)
///
/// This is DO_addlsh2 from mpn/generic/toom_eval_pm2.c, with d == `out_limbs`, a == `xs`, and b ==
/// `ys`.
fn shl_2_and_add_with_carry_to_out(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    carry: &mut Limb,
) {
    *carry <<= 2;
    *carry += limbs_shl_to_out(out_limbs, xs, 2);
    if limbs_slice_add_same_length_in_place_left(out_limbs, ys) {
        *carry += 1;
    }
}

/// Given a `Natural` whose highest limb is `carry` and remaining limbs are `limbs`, multiplies the
/// `Natural` by 4 and adds the `Natural` whose limbs are `out_limbs`. The highest limb of the
/// result is written back to `carry` and the remaining limbs are written to `out_limbs`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is DO_addlsh2 from mpn/generic/toom_eval_pm2.c, with d == b == `out_limbs` and a ==
/// `limbs`.
fn shl_2_and_add_with_carry_in_place_left(
    out_limbs: &mut [Limb],
    limbs: &[Limb],
    carry: &mut Limb,
) {
    *carry <<= 2;
    *carry += limbs_slice_shl_in_place(out_limbs, 2);
    if limbs_slice_add_same_length_in_place_left(out_limbs, limbs) {
        *carry += 1;
    }
}

// Evaluates a polynomial of degree 2 < `degree` < GMP_NUMB_BITS, in the points +2 and -2, where
// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
//
// This is mpn_toom_eval_pm2 from mpn/generic/toom_eval_pm2.c.
// TODO continue cleaning
pub fn mpn_toom_eval_pm2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    mut degree: u32,
    poly: &[Limb],
    n: usize,
    n_high: usize,
    scratch: &mut [Limb],
) -> Limb {
    assert!(degree > 2);
    assert!(degree < Limb::WIDTH);
    assert_ne!(n_high, 0);
    assert!(n_high <= n);

    // The degree `degree` is also the number of full-size coefficients, so that last coefficient,
    // of size `n_high`, starts at `poly[degree * n..]`.
    let degree_u = degree as usize;
    let mut cy = 0;
    shl_2_and_add_with_carry_to_out(
        v_2,
        &poly[degree_u * n..degree_u * n + n_high],
        &poly[(degree_u - 2) * n..(degree_u - 2) * n + n_high],
        &mut cy,
    );
    if n_high != n {
        cy = if limbs_add_limb_to_out(
            &mut v_2[n_high..],
            &poly[(degree_u - 2) * n + n_high..(degree_u - 1) * n],
            cy,
        ) {
            1
        } else {
            0
        };
    }
    let mut i = degree_u - 4;
    loop {
        shl_2_and_add_with_carry_in_place_left(&mut v_2[..n], &poly[i * n..(i + 1) * n], &mut cy);
        if i <= 2 {
            break;
        }
        i -= 2;
    }
    v_2[n] = cy;

    degree.wrapping_add_assign(1);

    cy = 0;
    shl_2_and_add_with_carry_to_out(
        scratch,
        &poly[degree_u * n..(degree_u + 1) * n],
        &poly[(degree_u - 2) * n..(degree_u - 1) * n],
        &mut cy,
    );
    let mut i = degree_u - 4;
    loop {
        shl_2_and_add_with_carry_in_place_left(
            &mut scratch[..n],
            &poly[i * n..(i + 1) * n],
            &mut cy,
        );
        if i <= 2 {
            break;
        }
        i -= 2;
    }
    scratch[n] = cy;

    let limit = n + 1;
    if (degree & 1) != 0 {
        assert_eq!(limbs_slice_shl_in_place(&mut scratch[..limit], 1), 0);
    } else {
        assert_eq!(limbs_slice_shl_in_place(&mut v_2[..limit], 1), 0);
    }

    let mut neg = if limbs_cmp_same_length(&v_2[..limit], &scratch[..limit]) == Ordering::Less {
        Limb::MAX
    } else {
        0
    };

    if neg != 0 {
        limbs_sub_same_length_to_out(v_neg_2, &scratch[..limit], &v_2[..limit]);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, &v_2[..limit], &scratch[..limit]);
    }

    limbs_slice_add_same_length_in_place_left(&mut v_2[..limit], &scratch[..limit]);

    assert!(v_2[n] < (1 << (degree + 2)) - 1);
    assert!(v_neg_2[n] < Limb::from(((1 << (degree + 3)) - 1 - (1 ^ degree & 1)) / 3));

    neg ^= (Limb::from(degree) & 1).wrapping_sub(1);
    neg
}

//TODO test
// multiply natural numbers.
// mpn_mul_n from mpn/generic/mul_n.c
pub fn limbs_mul_same_length_to_out(out_limbs: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    assert!(len >= 1);

    if len < MUL_TOOM22_THRESHOLD {
        _limbs_mul_greater_to_out_basecase(out_limbs, xs, ys);
    } else if len < MUL_TOOM33_THRESHOLD {
        // TODO once const fn is stable, make this
        // _limbs_mul_greater_to_out_toom_22_scratch_size(MUL_TOOM33_THRESHOLD_LIMIT - 1)

        // Allocate workspace of fixed size on stack: fast!
        let scratch = &mut [0; 2 * (MUL_TOOM33_THRESHOLD_LIMIT - 1 + Limb::WIDTH as usize)];
        assert!(MUL_TOOM33_THRESHOLD <= MUL_TOOM33_THRESHOLD_LIMIT);
        _limbs_mul_greater_to_out_toom_22(out_limbs, xs, ys, scratch);
    } else if len < MUL_TOOM44_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(len)];
        _limbs_mul_greater_to_out_toom_33(out_limbs, xs, ys, &mut scratch);
    } else {
        //TODO remove
        _limbs_mul_greater_to_out_basecase(out_limbs, xs, ys);
    }
    /*
    else if (BELOW_THRESHOLD (len, MUL_TOOM6H_THRESHOLD))
      {
        mp_ptr ws;
        TMP_SDECL;
        TMP_SMARK;
        ws = TMP_SALLOC_LIMBS (mpn_toom44_mul_itch (len, len));
        mpn_toom44_mul (out_limbs, xs, len, ys, len, ws);
        TMP_SFREE;
      }
    else if (BELOW_THRESHOLD (len, MUL_TOOM8H_THRESHOLD))
      {
        mp_ptr ws;
        TMP_SDECL;
        TMP_SMARK;
        ws = TMP_SALLOC_LIMBS (mpn_toom6_mul_n_itch (len));
        mpn_toom6h_mul (out_limbs, xs, len, ys, len, ws);
        TMP_SFREE;
      }
    else if (BELOW_THRESHOLD (len, MUL_FFT_THRESHOLD))
      {
        mp_ptr ws;
        TMP_DECL;
        TMP_MARK;
        ws = TMP_ALLOC_LIMBS (mpn_toom8_mul_n_itch (len));
        mpn_toom8h_mul (out_limbs, xs, len, ys, len, ws);
        TMP_FREE;
      }
    else
      {
        /* The current FFT code allocates its own space.  That should probably
       change.  */
    mpn_fft_mul (out_limbs, xs, len, ys, len);
    }*/
}

//TODO test
// Multiply two natural numbers.
// mpn_mul from mpn/generic/mul.c
pub fn limbs_mul_greater_to_out(out_limbs: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> Limb {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert!(ys_len >= 1);

    if xs_len == ys_len {
        //TODO if xs as *const [Limb] == ys as *const [Limb] {
        //TODO     mpn_sqr(out_limbs, xs, xs_len);
        //TODO     mpn_mul_n(out_limbs, xs, ys);
        //TODO } else {
        //TODO     mpn_mul_n(out_limbs, xs, ys);
        //TODO }
        limbs_mul_same_length_to_out(out_limbs, xs, ys);
    } else if ys_len < MUL_TOOM22_THRESHOLD {
        // plain schoolbook multiplication. Unless xs_len is very large, or else if have an
        // applicable mpn_mul_N, perform basecase multiply directly.
        _limbs_mul_greater_to_out_basecase(out_limbs, xs, ys);
    } else if ys_len < MUL_TOOM33_THRESHOLD {
        let toom_x2_scratch_size = 9 * ys_len / 2 + Limb::WIDTH as usize * 2;
        // toom_22_scratch_size((5 * ys_len - 1) / 4) <= toom_x2_scratch_size
        // toom_32_scratch_size((7 * ys_len - 1) / 4, ys_len) <= toom_x2_scratch_size
        // toom_42_scratch_size(3 * ys_len - 1, ys_len) <= toom_x2_scratch_size
        let mut scratch = vec![0; toom_x2_scratch_size];
        if xs_len >= 3 * ys_len {
            _limbs_mul_greater_to_out_toom_42(out_limbs, &xs[..ys_len << 1], ys, &mut scratch);
            let two_ys_len = ys_len + ys_len;
            let three_ys_len = two_ys_len + ys_len;
            // The maximum scratch2 usage is for the mpn_mul result.
            let mut scratch2 = vec![0; two_ys_len << 1];
            let mut xs_len = xs_len - two_ys_len;
            let mut xs_offset = two_ys_len;
            let mut out_limbs_offset = two_ys_len;
            while xs_len >= three_ys_len {
                _limbs_mul_greater_to_out_toom_42(
                    &mut scratch2,
                    &xs[xs_offset..xs_offset + two_ys_len],
                    ys,
                    &mut scratch,
                );
                xs_len -= two_ys_len;
                xs_offset += two_ys_len;
                let carry = limbs_slice_add_same_length_in_place_left(
                    &mut out_limbs[out_limbs_offset..out_limbs_offset + ys_len],
                    &scratch2[..ys_len],
                );
                out_limbs[out_limbs_offset + ys_len..out_limbs_offset + three_ys_len]
                    .copy_from_slice(&scratch2[ys_len..three_ys_len]);
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(
                        &mut out_limbs[out_limbs_offset + ys_len..],
                        1
                    ));
                }
                out_limbs_offset += two_ys_len;
            }

            // ys_len <= xs_len < 3 * ys_len
            if 4 * xs_len < 5 * ys_len {
                _limbs_mul_greater_to_out_toom_22(
                    &mut scratch2,
                    &xs[xs_offset..],
                    ys,
                    &mut scratch,
                );
            } else if 4 * xs_len < 7 * ys_len {
                _limbs_mul_greater_to_out_toom_32(
                    &mut scratch2,
                    &xs[xs_offset..],
                    ys,
                    &mut scratch,
                );
            } else {
                _limbs_mul_greater_to_out_toom_42(
                    &mut scratch2,
                    &xs[xs_offset..],
                    ys,
                    &mut scratch,
                );
            }

            let carry = limbs_slice_add_same_length_in_place_left(
                &mut out_limbs[out_limbs_offset..out_limbs_offset + ys_len],
                &scratch2[..ys_len],
            );
            out_limbs[out_limbs_offset + ys_len..out_limbs_offset + ys_len + xs_len]
                .copy_from_slice(&scratch2[ys_len..ys_len + xs_len]);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut out_limbs[out_limbs_offset + ys_len..],
                    1
                ));
            }
        } else if 4 * xs_len < 5 * ys_len {
            _limbs_mul_greater_to_out_toom_22(out_limbs, xs, ys, &mut scratch);
        } else if 4 * xs_len < 7 * ys_len {
            _limbs_mul_greater_to_out_toom_32(out_limbs, xs, ys, &mut scratch);
        } else {
            _limbs_mul_greater_to_out_toom_42(out_limbs, xs, ys, &mut scratch);
        }
    //TODO PASTE C
    } else {
        //TODO remove
        _limbs_mul_greater_to_out_basecase(out_limbs, xs, ys);
    }
    out_limbs[xs_len + ys_len - 1]
}

pub fn limbs_mul_to_out(out_limbs: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> Limb {
    if xs.len() >= ys.len() {
        limbs_mul_greater_to_out(out_limbs, xs, ys)
    } else {
        limbs_mul_greater_to_out(out_limbs, ys, xs)
    }
}

//TODO update docs
// 1 < v.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < u.len()
//
// This is currently not measurably better than just basecase.
fn limbs_mul_greater_to_out_basecase_mem_opt(prod: &mut [Limb], u: &[Limb], v: &[Limb]) {
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
            _limbs_mul_greater_to_out_basecase(&mut prod[offset..], chunk, v);
        } else {
            _limbs_mul_greater_to_out_basecase(&mut prod[offset..], v, chunk);
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
fn limbs_mul_greater_to_out_basecase_or_mem_opt(prod: &mut [Limb], u: &[Limb], v: &[Limb]) {
    let u_len = u.len();
    let v_len = v.len();
    assert!(u_len >= v_len);
    if v_len > 1 && v_len < MUL_TOOM22_THRESHOLD && u.len() > MUL_BASECASE_MAX_UN {
        limbs_mul_greater_to_out_basecase_mem_opt(prod, u, v)
    } else {
        _limbs_mul_greater_to_out_basecase(prod, u, v);
    }
}

pub fn limbs_mul_greater(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    limbs_mul_greater_to_out(&mut product_limbs, xs, ys);
    product_limbs
}

pub fn limbs_mul(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    if xs.len() >= ys.len() {
        limbs_mul_greater(xs, ys)
    } else {
        limbs_mul_greater(ys, xs)
    }
}

fn limbs_mul_basecase_mem_opt(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        limbs_mul_greater_to_out_basecase_or_mem_opt(&mut product_limbs, xs, ys);
    } else {
        limbs_mul_greater_to_out_basecase_or_mem_opt(&mut product_limbs, ys, xs);
    }
    product_limbs
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by value.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
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
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
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
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
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
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
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
                    let mut product = Large(limbs_mul(xs, ys));
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
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
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
                    *xs = limbs_mul(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
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
                    *xs = limbs_mul(xs, ys);
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
                (&mut Large(ref mut xs), Large(ref ys)) => *xs = limbs_mul_basecase_mem_opt(xs, ys),
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

pub mod poly_eval;
pub mod poly_interpolate;
pub mod toom;
