use malachite_base::limbs::limbs_set_zero;
use malachite_base::num::{
    One, Parity, PrimitiveInteger, ShrRound, UnsignedAbs, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::round::RoundingMode;
use natural::arithmetic::add::limbs_add_same_length_to_out;
use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::add_limb::limbs_add_limb_to_out;
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul::limbs_mul_same_length_to_out;
use natural::arithmetic::mul::mul_mod::{
    _limbs_mul_mod_limb_width_to_n_minus_1, _limbs_mul_mod_limb_width_to_n_minus_1_next_size,
    _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size, MULMOD_BNM1_THRESHOLD,
    MUL_FFT_MODF_THRESHOLD,
};
use natural::arithmetic::shl_u::limbs_shl_to_out;
use natural::arithmetic::shl_u::limbs_shl_with_complement;
use natural::arithmetic::square::SQR_TOOM3_THRESHOLD;
use natural::arithmetic::sub::limbs_sub_same_length_in_place_right;
use natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::logic::not::limbs_not_to_out;
use platform::{Limb, SignedLimb};
use std::cmp::{max, Ordering};

//TODO tune
pub(crate) const MUL_FFT_THRESHOLD: usize = 4_736;
//TODO double check this
const SQR_FFT_MODF_THRESHOLD: usize = SQR_TOOM3_THRESHOLD * 3;

pub fn _limbs_mul_greater_to_out_fft_input_sizes_threshold(xs_len: usize, ys_len: usize) -> bool {
    if xs_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(xs_len + ys_len);
    n.even() && n >= MULMOD_BNM1_THRESHOLD
}

// Returns smallest possible number of limbs >= pl for a fft of size 2 ^ k, i.e. smallest multiple
// of 2 ^ k >= pl.
// This is mpn_fft_next_size from mpn/generic/mul-fft.c.
pub(crate) fn mpn_fft_next_size(pl: usize, k: usize) -> usize {
    pl.shr_round(k as u64, RoundingMode::Ceiling) << k
}

struct FFTTableNK {
    n: usize,
    k: usize,
}

const MUL_FFT_TABLE3_SIZE: usize = 208;

//TODO tune!!
// from mpn/*/*/gmp-mparam.h
const MUL_FFT_TABLE3: [FFTTableNK; MUL_FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 396, k: 5 },
    FFTTableNK { n: 19, k: 6 },
    FFTTableNK { n: 10, k: 5 },
    FFTTableNK { n: 21, k: 6 },
    FFTTableNK { n: 11, k: 5 },
    FFTTableNK { n: 23, k: 6 },
    FFTTableNK { n: 21, k: 7 },
    FFTTableNK { n: 11, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 13, k: 6 },
    FFTTableNK { n: 27, k: 7 },
    FFTTableNK { n: 21, k: 8 },
    FFTTableNK { n: 11, k: 7 },
    FFTTableNK { n: 25, k: 8 },
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
    FFTTableNK { n: 33, k: 9 },
    FFTTableNK { n: 19, k: 8 },
    FFTTableNK { n: 39, k: 9 },
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
    FFTTableNK { n: 135, k: 11 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 159, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 255, k: 9 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 143, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 11 },
    FFTTableNK { n: 159, k: 10 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 12 },
    FFTTableNK { n: 159, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 351, k: 10 },
    FFTTableNK { n: 703, k: 11 },
    FFTTableNK { n: 367, k: 10 },
    FFTTableNK { n: 735, k: 11 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 767, k: 11 },
    FFTTableNK { n: 415, k: 10 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 511, k: 10 },
    FFTTableNK { n: 1023, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 1151, k: 11 },
    FFTTableNK { n: 607, k: 12 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 671, k: 12 },
    FFTTableNK { n: 351, k: 11 },
    FFTTableNK { n: 735, k: 12 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 447, k: 11 },
    FFTTableNK { n: 895, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 1023, k: 12 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 1151, k: 12 },
    FFTTableNK { n: 607, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 735, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 255, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1087, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 703, k: 12 },
    FFTTableNK { n: 1407, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 1215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1343, k: 12 },
    FFTTableNK { n: 2687, k: 13 },
    FFTTableNK { n: 1407, k: 12 },
    FFTTableNK { n: 2815, k: 13 },
    FFTTableNK { n: 1471, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1663, k: 14 },
    FFTTableNK { n: 895, k: 13 },
    FFTTableNK { n: 1919, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 2175, k: 14 },
    FFTTableNK { n: 1151, k: 13 },
    FFTTableNK { n: 2431, k: 12 },
    FFTTableNK { n: 4863, k: 14 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 2687, k: 14 },
    FFTTableNK { n: 1407, k: 13 },
    FFTTableNK { n: 2815, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 3071, k: 14 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 3455, k: 14 },
    FFTTableNK { n: 1919, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1023, k: 14 },
    FFTTableNK { n: 2431, k: 13 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 1279, k: 14 },
    FFTTableNK { n: 2943, k: 13 },
    FFTTableNK { n: 5887, k: 15 },
    FFTTableNK { n: 1535, k: 14 },
    FFTTableNK { n: 3455, k: 15 },
    FFTTableNK { n: 1791, k: 14 },
    FFTTableNK { n: 3839, k: 13 },
    FFTTableNK { n: 7679, k: 16 },
    FFTTableNK { n: 1023, k: 15 },
    FFTTableNK { n: 2047, k: 14 },
    FFTTableNK { n: 4223, k: 15 },
    FFTTableNK { n: 2303, k: 14 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 2815, k: 14 },
    FFTTableNK { n: 5887, k: 16 },
    FFTTableNK { n: 1535, k: 15 },
    FFTTableNK { n: 3327, k: 14 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 3839, k: 14 },
    FFTTableNK { n: 7679, k: 17 },
    FFTTableNK { n: 1023, k: 16 },
    FFTTableNK { n: 2047, k: 15 },
    FFTTableNK { n: 4863, k: 16 },
    FFTTableNK { n: 2559, k: 15 },
    FFTTableNK { n: 5887, k: 14 },
    FFTTableNK { n: 11775, k: 16 },
    FFTTableNK { n: 65536, k: 17 },
    FFTTableNK { n: 131072, k: 18 },
    FFTTableNK { n: 262144, k: 19 },
    FFTTableNK { n: 524288, k: 20 },
    FFTTableNK { n: 1048576, k: 21 },
    FFTTableNK { n: 2097152, k: 22 },
    FFTTableNK { n: 4194304, k: 23 },
    FFTTableNK { n: 8388608, k: 24 },
];

const SQR_FFT_TABLE3_SIZE: usize = 203;

// from mpn/*/*/gmp-mparam.h
const SQR_FFT_TABLE3: [FFTTableNK; SQR_FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 340, k: 5 },
    FFTTableNK { n: 11, k: 4 },
    FFTTableNK { n: 23, k: 5 },
    FFTTableNK { n: 21, k: 6 },
    FFTTableNK { n: 11, k: 5 },
    FFTTableNK { n: 23, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 13, k: 6 },
    FFTTableNK { n: 27, k: 7 },
    FFTTableNK { n: 25, k: 8 },
    FFTTableNK { n: 13, k: 7 },
    FFTTableNK { n: 28, k: 8 },
    FFTTableNK { n: 15, k: 7 },
    FFTTableNK { n: 31, k: 8 },
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
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 55, k: 11 },
    FFTTableNK { n: 31, k: 10 },
    FFTTableNK { n: 79, k: 11 },
    FFTTableNK { n: 47, k: 10 },
    FFTTableNK { n: 95, k: 12 },
    FFTTableNK { n: 31, k: 11 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 127, k: 9 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 135, k: 11 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 159, k: 9 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 255, k: 9 },
    FFTTableNK { n: 511, k: 10 },
    FFTTableNK { n: 271, k: 9 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 143, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 9 },
    FFTTableNK { n: 607, k: 11 },
    FFTTableNK { n: 159, k: 10 },
    FFTTableNK { n: 319, k: 9 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 10 },
    FFTTableNK { n: 607, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 335, k: 10 },
    FFTTableNK { n: 671, k: 11 },
    FFTTableNK { n: 351, k: 10 },
    FFTTableNK { n: 703, k: 11 },
    FFTTableNK { n: 367, k: 12 },
    FFTTableNK { n: 191, k: 11 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 767, k: 11 },
    FFTTableNK { n: 415, k: 10 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 607, k: 12 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 671, k: 12 },
    FFTTableNK { n: 351, k: 11 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 191, k: 12 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 447, k: 11 },
    FFTTableNK { n: 895, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 607, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 735, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1087, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1343, k: 13 },
    FFTTableNK { n: 703, k: 12 },
    FFTTableNK { n: 1407, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 1215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1343, k: 12 },
    FFTTableNK { n: 2687, k: 13 },
    FFTTableNK { n: 1407, k: 12 },
    FFTTableNK { n: 2815, k: 13 },
    FFTTableNK { n: 1471, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1599, k: 12 },
    FFTTableNK { n: 3199, k: 13 },
    FFTTableNK { n: 1663, k: 14 },
    FFTTableNK { n: 895, k: 13 },
    FFTTableNK { n: 1791, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 2175, k: 14 },
    FFTTableNK { n: 1151, k: 13 },
    FFTTableNK { n: 2431, k: 12 },
    FFTTableNK { n: 4863, k: 14 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 2687, k: 14 },
    FFTTableNK { n: 1407, k: 13 },
    FFTTableNK { n: 2815, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 3199, k: 14 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 3455, k: 12 },
    FFTTableNK { n: 6911, k: 14 },
    FFTTableNK { n: 1791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1023, k: 14 },
    FFTTableNK { n: 2431, k: 13 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 1279, k: 14 },
    FFTTableNK { n: 2943, k: 13 },
    FFTTableNK { n: 5887, k: 15 },
    FFTTableNK { n: 1535, k: 14 },
    FFTTableNK { n: 3455, k: 13 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 1791, k: 14 },
    FFTTableNK { n: 3839, k: 16 },
    FFTTableNK { n: 1023, k: 15 },
    FFTTableNK { n: 2047, k: 14 },
    FFTTableNK { n: 4223, k: 15 },
    FFTTableNK { n: 2303, k: 14 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 2815, k: 14 },
    FFTTableNK { n: 5887, k: 16 },
    FFTTableNK { n: 1535, k: 15 },
    FFTTableNK { n: 3327, k: 14 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 3839, k: 17 },
    FFTTableNK { n: 1023, k: 16 },
    FFTTableNK { n: 2047, k: 15 },
    FFTTableNK { n: 4863, k: 16 },
    FFTTableNK { n: 2559, k: 15 },
    FFTTableNK { n: 5887, k: 14 },
    FFTTableNK { n: 11775, k: 16 },
    FFTTableNK { n: 65536, k: 17 },
    FFTTableNK { n: 131072, k: 18 },
    FFTTableNK { n: 262144, k: 19 },
    FFTTableNK { n: 524288, k: 20 },
    FFTTableNK { n: 1048576, k: 21 },
    FFTTableNK { n: 2097152, k: 22 },
    FFTTableNK { n: 4194304, k: 23 },
    FFTTableNK { n: 8388608, k: 24 },
];

// Find the best k to use for a mod 2 ^ (m * Limb::WIDTH) + 1 FFT for m >= n.
// This is mpn_fft_best_k from mpn/generic/mul_fft.c, the mpn_fft_table3 variant.
pub(crate) fn _limbs_mul_fft_best_k(n: usize, square: bool) -> usize {
    let fft_table: &[FFTTableNK] = if square {
        &SQR_FFT_TABLE3
    } else {
        &MUL_FFT_TABLE3
    };
    let mut last_k = fft_table[0].k;
    for entry in &fft_table[1..] {
        let threshold = entry.n << last_k;
        if n <= threshold as usize {
            break;
        }
        last_k = entry.k;
    }
    last_k
}

// This is mpn_fft_mul from gmp-impl.h and mpn_nussbaumer_mul from mpn/generic/mpn_nussbaumer_mul.c.
#[inline]
pub fn _limbs_mul_greater_to_out_fft(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert_ne!(ys_len, 0);

    //TODO special case for squaring
    let n = _limbs_mul_mod_limb_width_to_n_minus_1_next_size(xs_len + ys_len);
    let mut scratch =
        vec![0; _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size(n, xs_len, ys_len)];
    _limbs_mul_mod_limb_width_to_n_minus_1(out, n, xs, ys, &mut scratch);
}

// Initialize l[i][j] with bitrev(j)
// This is mpn_fft_initl from mpn/generic/mul_fft.c.
fn _limbs_mul_fft_bit_reverse_table(mut scratch: &mut [usize], k: usize) -> Vec<&[usize]> {
    let mut table = Vec::with_capacity(k + 1);
    for i in 0..=k {
        // force scratch to move rather than be borrowed
        let (scratch_lo, scratch_hi) = { scratch }.split_at_mut(1 << i);
        table.push(scratch_lo);
        scratch = scratch_hi;
    }
    table[0][0] = 0;
    let mut big_k = 1;
    for i in 1..=k {
        for j in 0..big_k {
            table[i][j] = table[i - 1][j] << 1;
            table[i][big_k + j] = table[i][j] + 1;
        }
        big_k <<= 1;
    }
    table.into_iter().map(|row| &*row).collect()
}

// return the lcm of a and 2^k
// This is mpn_mul_fft_lcm from mpn/generic/mul_fft.c.
fn _limbs_mul_fft_lcm_of_a_and_two_pow_k(a: usize, k: usize) -> usize {
    a << k.saturating_sub(a.trailing_zeros() as usize)
}

// Given xs[0..n] with xs[n]<=1, reduce it modulo 2^(n*GMP_NUMB_BITS)+1,
// by subtracting that modulus if necessary.
//
// If xs[0..n] is exactly 2^(n*GMP_NUMB_BITS) then mpn_sub_1 produces a
// borrow and the limbs must be zeroed out again. This will occur very
// infrequently.
// This is mpn_fft_normalize from mpn/generic/mul_fft.c.
fn _limbs_mul_fft_normalize(xs: &mut [Limb], n: usize) {
    if xs[n] != 0 {
        assert!(!limbs_sub_limb_in_place(&mut xs[..n + 1], 1));
        if xs[n] == 0 {
            limbs_set_zero(&mut xs[..n]);
            xs[n] = 1;
        } else {
            xs[n] = 0;
        }
    }
}

// This is mpn_fft_add_modF from mpn/generic/mul_fft.c, where r == a.
fn _limbs_mul_fft_add_mod_f_in_place_left(xs: &mut [Limb], ys: &[Limb], n: usize) {
    let mut carry = xs[n].wrapping_add(ys[n]);
    if limbs_slice_add_same_length_in_place_left(&mut xs[..n], &ys[..n]) {
        carry.wrapping_add_assign(1);
    }
    // 0 <= carry <= 3
    if carry > 1 {
        xs[n] = 1; // r[n] - carry = 1
        assert!(!limbs_sub_limb_in_place(&mut xs[..n + 1], carry - 1));
    } else {
        xs[n] = carry;
    }
}

// out <- xs-ys mod 2^(n*GMP_NUMB_BITS)+1.
// Assumes xs and ys are semi-normalized.
// This is mpn_fft_sub_modF from mpn/generic/mul_fft.c.
fn _limbs_mul_fft_sub_mod_f_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb], n: usize) {
    let mut carry = xs[n].wrapping_sub(ys[n]);
    if limbs_sub_same_length_to_out(out, &xs[..n], &ys[..n]) {
        carry.wrapping_sub_assign(1);
    }
    // -2 <= carry <= 1
    if carry.get_highest_bit() {
        out[n] = 0;
        assert!(!limbs_slice_add_limb_in_place(
            &mut out[..n + 1],
            carry.wrapping_neg()
        ));
    } else {
        out[n] = carry;
    }
}

// out <- xs*2^bits mod 2^(n*`Limb::WIDTH`)+1 with xs = {xs, n+1}
// Assumes xs is semi-normalized, i.e. xs[n] <= 1.
// out and xs must have n+1 limbs, and not overlap.
// This is mpn_fft_mul_2exp_modF from mpn/generic/mul_fft.c.
fn _limbs_mul_fft_shl_mod_f_to_out(out: &mut [Limb], xs: &[Limb], bits: usize, n: usize) {
    let small_bits = bits as u32 & Limb::WIDTH_MASK;
    let mut shift_limbs = bits / Limb::WIDTH as usize;
    // negate
    if shift_limbs >= n {
        // out[0..shift_limbs-1]  <-- lshift(xs[n-shift_limbs]..xs[n-1], small_bits)
        // out[shift_limbs..n-1]  <-- -lshift(xs[0]..xs[n-shift_limbs-1],  small_bits)
        shift_limbs -= n;
        let (xs_lo, xs_hi) = xs.split_at(n - shift_limbs);
        let mut carry2;
        let mut carry = if small_bits == 0 {
            out[..shift_limbs].copy_from_slice(&xs_hi[..shift_limbs]);
            carry2 = xs[n];
            limbs_not_to_out(&mut out[shift_limbs..], xs_lo);
            0
        } else {
            // no out shift below since xs[n] <= 1
            limbs_shl_to_out(out, &xs_hi[..shift_limbs + 1], small_bits);
            carry2 = out[shift_limbs];
            limbs_shl_with_complement(&mut out[shift_limbs..], xs_lo, small_bits)
        };

        // add carry to out[0], and add carry2 to out[shift_limbs]
        // now add 1 in out[shift_limbs], subtract 1 in out[n], i.e. add 1 in out[0]
        out[n] = 0;
        // carry < 2^small_bits <= 2^(Limb::WIDTH`-1) thus no overflow here
        carry += 1;
        limbs_slice_add_limb_in_place(out, carry);
        carry2.wrapping_add_assign(1);
        // carry2 might overflow when small_bits=Limb::WIDTH`-1
        if carry2 == 0 {
            limbs_slice_add_limb_in_place(&mut out[shift_limbs + 1..], 1);
        } else {
            limbs_slice_add_limb_in_place(&mut out[shift_limbs..], carry2);
        }
    } else {
        let carry2;
        // out[0..shift_limbs-1]  <-- -lshift(xs[n-shift_limbs]..xs[n-1], small_bits)
        // out[shift_limbs..n-1]  <-- lshift(xs[0]..xs[n-shift_limbs-1],  small_bits)
        let (xs_lo, xs_hi) = xs.split_at(n - shift_limbs);
        let mut carry = if small_bits != 0 {
            // no out bits below since xs[n] <= 1
            limbs_shl_with_complement(out, &xs_hi[..shift_limbs + 1], small_bits);
            carry2 = !out[shift_limbs];
            // {out, shift_limbs+1} = {xs+n-shift_limbs, shift_limbs+1} << small_bits
            // {out+shift_limbs, n-shift_limbs} = {xs, n-shift_limbs}<<small_bits
            limbs_shl_to_out(&mut out[shift_limbs..], xs_lo, small_bits)
        } else {
            // out[shift_limbs] is not used below, but we save xs test for shift_limbs=0
            limbs_not_to_out(out, &xs_hi[..shift_limbs + 1]);
            carry2 = xs[n];
            out[shift_limbs..n].copy_from_slice(xs_lo);
            0
        };

        // now complement {out, shift_limbs}, subtract carry from out[0], subtract carry2 from
        // out[shift_limbs]. If shift_limbs=0 we just have out[0]=xs[n] << small_bits
        if shift_limbs != 0 {
            // now add 1 in out[0], subtract 1 in out[shift_limbs]
            // then add 1 to out[0]
            if carry == 0 {
                if limbs_slice_add_limb_in_place(&mut out[..n], 1) {
                    carry = 1;
                }
            } else {
                carry -= 1;
            }
            // add 1 to carry instead of carry2 since carry2 might overflow
            carry = if limbs_sub_limb_in_place(&mut out[..shift_limbs], carry) {
                2
            } else {
                1
            };
        }

        // now subtract carry and carry2 from out[shift_limbs..n]
        let (out_last, out_init) = out[..n + 1].split_last_mut().unwrap();
        {
            let out_init_hi = &mut out_init[shift_limbs..];
            *out_last = if limbs_sub_limb_in_place(out_init_hi, carry) {
                Limb::ONE.wrapping_neg()
            } else {
                0
            };
            if limbs_sub_limb_in_place(out_init_hi, carry2) {
                out_last.wrapping_sub_assign(1);
            }
        }
        if out_last.get_highest_bit() {
            *out_last = if limbs_slice_add_limb_in_place(out_init, 1) {
                1
            } else {
                0
            };
        }
    }
}

// R <- A/2^bits mod 2^(n*GMP_NUMB_BITS)+1
// This is mpn_fft_div_2exp_modF from mpn/generic/mul_fft.c.
fn _limbs_mul_fft_shr_mod_f_to_out(out: &mut [Limb], xs: &[Limb], bits: usize, n: usize) {
    assert!(out.len() >= n + 1);
    let i = 2 * n * Limb::WIDTH as usize - bits;
    _limbs_mul_fft_shl_mod_f_to_out(out, xs, i, n);
    // 1/2^bits = 2^(2nL-bits) mod 2^(n*GMP_NUMB_BITS)+1
    // normalize so that R < 2^(n*GMP_NUMB_BITS)+1
    _limbs_mul_fft_normalize(out, n);
}

// store in A[0..width - 1] the first M bits from {xs, len},
// in A[width..] the following M bits, ...
// Assumes M is a multiple of GMP_NUMB_BITS (M = n * GMP_NUMB_BITS).
// T must have space for at least `width` limbs.
// We must have xs.len() <= 2 * k * n.
// This is mpn_mul_fft_decompose from mpn/generic/mul_fft.c. nl is omitted as it is just the length
// of n (here, xs).
fn _limbs_mul_fft_decompose<'a>(
    a_limbs: &'a mut [Limb],
    k: usize,
    width_minus_one: usize,
    xs: &[Limb],
    n: usize,
    m: usize,
    scratch: &mut [Limb],
) -> Vec<&'a mut [Limb]> {
    let mut len = xs.len();
    let width = width_minus_one + 1;
    let k_times_n = k * n;
    // normalize xs mod 2 ^ (k * n * Limb::WIDTH) + 1
    let mut cy: SignedLimb;
    let mut scratch2;
    let mut source: &[Limb] = if len > k_times_n {
        let difference = len - k_times_n;
        scratch2 = vec![0; k_times_n + 1];
        // difference > k_times_n -> len - k * n > k * n -> len > 2 * k * n, which violates the
        // precondition len <= 2 * k * n. So difference > k_times_n cannot happen.
        assert!(difference <= k_times_n);
        // difference <= k_times_n, i.e. len <= 2 * k_times_n
        let (xs_lo, xs_hi) = xs.split_at(k_times_n);
        cy = if limbs_sub_to_out(&mut scratch2, xs_lo, &xs_hi[..difference]) {
            1
        } else {
            0
        };
        cy = if limbs_slice_add_limb_in_place(&mut scratch2[..k_times_n], cy as Limb) {
            1
        } else {
            0
        };
        scratch2[k_times_n] = cy as Limb;
        len = k_times_n + 1;
        &scratch2
    } else {
        &xs
    };
    let mut a_table = Vec::with_capacity(k);
    let mut remainder: &mut [Limb] = a_limbs;
    for i in 0..k {
        // force remainder to move rather than be borrowed
        let (a_lo, a_hi) = { remainder }.split_at_mut(width);
        remainder = a_hi;
        // store the next M bits of xs into A[0..width_minus_one]
        // len is the number of remaining limbs
        if len > 0 {
            let j = if n <= len && i < k - 1 { n } else { len }; // store j next limbs
            len -= j;
            if i != 0 {
                source = &source[n..];
            }
            scratch[..j].copy_from_slice(&source[..j]);
            limbs_set_zero(&mut scratch[j..width]);
            _limbs_mul_fft_shl_mod_f_to_out(a_lo, scratch, i * m, width_minus_one);
        } else {
            limbs_set_zero(a_lo);
        }
        a_table.push(a_lo);
    }
    assert_eq!(len, 0);
    a_table
}

// input: A[0] ... A[inc*(K-1)] are residues mod 2^N+1 where
//    N=n*GMP_NUMB_BITS, and 2^omega is a primitive root mod 2^N+1
// output: A[inc*l[k][i]] <- \sum (2^omega)^(ij) A[inc*j] mod 2^N+1
// This is mpn_fft_fft from mpn/generic/mul_fft.c.
pub fn mpn_fft_fft(
    ap: &mut [&mut [Limb]],
    k: usize,
    ll: &[&[usize]],
    ll_offset: usize,
    omega: usize,
    n: usize,
    inc: usize,
    tp: &mut [Limb],
) {
    if k == 2 {
        tp[..n + 1].copy_from_slice(&ap[0][..n + 1]);
        {
            let (ap_first, ap_tail) = ap.split_first_mut().unwrap();
            limbs_slice_add_same_length_in_place_left(
                &mut ap_first[..n + 1],
                &ap_tail[inc - 1][..n + 1],
            );
        }
        let cy = limbs_sub_same_length_in_place_right(&tp[..n + 1], &mut ap[inc][..n + 1]);
        // can be 2 or 3
        if ap[0][n] > 1 {
            let x = ap[0][n] - 1;
            ap[0][n] = 1 - if limbs_sub_limb_in_place(&mut ap[0][..n], x) {
                1
            } else {
                0
            };
        }
        // Ap[inc][n] can be -1 or -2
        if cy {
            let x = (!ap[inc][n]).wrapping_add(1);
            ap[inc][n] = if limbs_slice_add_limb_in_place(&mut ap[inc][..n], x) {
                1
            } else {
                0
            };
        }
    } else {
        let k2 = k >> 1;
        let mut lki = 0;

        mpn_fft_fft(ap, k2, ll, ll_offset - 1, 2 * omega, n, inc * 2, tp);
        mpn_fft_fft(
            &mut ap[inc..],
            k2,
            ll,
            ll_offset - 1,
            2 * omega,
            n,
            inc * 2,
            tp,
        );
        let mut ap_offset = 0;
        //  A[2*j*inc]   <- A[2*j*inc] + omega^l[k][2*j*inc] A[(2j+1)inc]
        // A[(2j+1)inc] <- A[2*j*inc] + omega^l[k][(2j+1)inc] A[(2j+1)inc]
        for _ in 0..k2 {
            /* Ap[inc] <- Ap[0] + Ap[inc] * 2^(lk[1] * omega)
            Ap[0]   <- Ap[0] + Ap[inc] * 2^(lk[0] * omega) */
            _limbs_mul_fft_shl_mod_f_to_out(
                tp,
                ap[ap_offset + inc],
                ll[ll_offset][lki].wrapping_mul(omega),
                n,
            );
            {
                let (ap_lo, ap_hi) = ap.split_at_mut(ap_offset + inc);
                _limbs_mul_fft_sub_mod_f_to_out(ap_hi[0], ap_lo[ap_offset], tp, n);
            }
            _limbs_mul_fft_add_mod_f_in_place_left(ap[ap_offset], tp, n);
            lki += 2;
            ap_offset += 2 * inc;
        }
    }
}

// input: A^[l[k][0]] A^[l[k][1]] ... A^[l[k][K-1]]
// output: K*A[0] K*A[K-1] ... K*A[1].
// Assumes the Ap[] are pseudo-normalized, i.e. 0 <= Ap[][n] <= 1.
// This condition is also fulfilled at exit.
// This is mpn_fft_fftinv from mpn/generic/mul_fft.c.
pub fn mpn_fft_fftinv(ap: &mut [&mut [Limb]], k: usize, omega: usize, n: usize, tp: &mut [Limb]) {
    if k == 2 {
        tp[..n + 1].copy_from_slice(&ap[0][..n + 1]);
        {
            let (ap_first, ap_tail) = ap.split_first_mut().unwrap();
            limbs_slice_add_same_length_in_place_left(&mut ap_first[..n + 1], &ap_tail[0][..n + 1]);
        }
        let cy = limbs_sub_same_length_in_place_right(&tp[..n + 1], &mut ap[1][..n + 1]);
        // can be 2 or 3
        if ap[0][n] > 1 {
            let x = ap[0][n] - 1;
            ap[0][n] = 1 - if limbs_sub_limb_in_place(&mut ap[0][..n], x) {
                1
            } else {
                0
            };
        }
        // Ap[1][n] can be -1 or -2
        if cy {
            let x = (!ap[1][n]).wrapping_add(1);
            ap[1][n] = if limbs_slice_add_limb_in_place(&mut ap[1][..n], x) {
                1
            } else {
                0
            };
        }
    } else {
        let k2 = k >> 1;
        mpn_fft_fftinv(ap, k2, 2 * omega, n, tp);
        mpn_fft_fftinv(&mut ap[k2..], k2, 2 * omega, n, tp);
        // A[j]     <- A[j] + omega^j A[j+K/2]
        // A[j+K/2] <- A[j] + omega^(j+K/2) A[j+K/2]
        let mut ap_offset = 0;
        for j in 0..k2 {
            // Ap[K2] <- Ap[0] + Ap[K2] * 2^((j + K2) * omega)
            // Ap[0]  <- Ap[0] + Ap[K2] * 2^(j * omega)
            _limbs_mul_fft_shl_mod_f_to_out(tp, ap[ap_offset + k2], j * omega, n);
            {
                let (ap_lo, ap_hi) = ap.split_at_mut(ap_offset + k2);
                _limbs_mul_fft_sub_mod_f_to_out(ap_hi[0], ap_lo[ap_offset], tp, n);
            }
            _limbs_mul_fft_add_mod_f_in_place_left(ap[ap_offset], tp, n);
            ap_offset += 1;
        }
    }
}

// {rp,n} <- {ap,an} mod 2^(n*GMP_NUMB_BITS)+1, n <= an <= 3*n.
// Returns carry out, i.e. 1 iff {ap,an} = -1 mod 2^(n*GMP_NUMB_BITS)+1,
// then {rp,n}=0.
// This is mpn_fft_norm_modF from mpn/generic/mul_fft.c.
pub fn mpn_fft_norm_mod_f(rp: &mut [Limb], n: usize, ap: &[Limb], an: usize) -> Limb {
    assert!(n <= an && an <= 3 * n);
    let m = an as isize - 2 * n as isize;
    let l;
    let mut rpn: SignedLimb;
    if m > 0 {
        let m = m as usize;
        l = n;
        // add {ap, m} and {ap+2n, m} in {rp, m}
        let cc = if limbs_add_same_length_to_out(rp, &ap[..m], &ap[2 * n..2 * n + m]) {
            1
        } else {
            0
        };
        // copy {ap+m, n-m} to {rp+m, n-m}
        rpn = if limbs_add_limb_to_out(&mut rp[m..n], &ap[m..n], cc) {
            1
        } else {
            0
        };
    } else {
        l = an - n; // l <= n
        rp[..n].copy_from_slice(&ap[..n]);
        rpn = 0;
    }
    // remains to subtract {ap+n, l} from {rp, n+1}
    let cc = if limbs_sub_same_length_in_place_left(&mut rp[..l], &ap[n..n + l]) {
        1
    } else {
        0
    };
    rpn -= if limbs_sub_limb_in_place(&mut rp[l..n], cc) {
        1
    } else {
        0
    };
    // necessarily rpn = -1
    if rpn < 0 {
        if limbs_slice_add_limb_in_place(&mut rp[..n], 1) {
            1
        } else {
            0
        }
    } else {
        rpn.unsigned_abs()
    }
}

// This is mpn_fft_mul_modF_K from mpn/generic/mul_fft.c, where ap != bp.
fn mpn_fft_mul_mod_f_k(ap: &mut [&mut [Limb]], bp: &mut [&mut [Limb]], n: usize, big_k: usize) {
    if n >= MUL_FFT_MODF_THRESHOLD {
        let k = _limbs_mul_fft_best_k(n, false);
        let k2 = 1 << k;
        assert_eq!(n & (k2 - 1), 0);
        let max_lk = max(k2, Limb::WIDTH as usize);
        let m2 = n * Limb::WIDTH as usize >> k;
        let l = n >> k;
        let mut big_nprime2 = (2 * m2 + k + 2 + max_lk) / max_lk * max_lk;
        // Nprime2 = ceil((2*M2+k+3)/maxLK)*maxLK
        let mut nprime2 = big_nprime2 / Limb::WIDTH as usize;

        // we should ensure that nprime2 is a multiple of the next K
        if nprime2 >= MUL_FFT_MODF_THRESHOLD {
            loop {
                let k3 = 1 << _limbs_mul_fft_best_k(nprime2, false);
                if nprime2 & (k3 - 1) == 0 {
                    break;
                }
                nprime2 = (nprime2 + k3 - 1) & k3.wrapping_neg();
                big_nprime2 = nprime2 * Limb::WIDTH as usize;
                // warning: since nprime2 changed, K3 may change too!
            }
        }
        assert!(nprime2 < n); // otherwise we'll loop

        let mp2 = big_nprime2 >> k;

        let mut a = vec![0; 2 * (nprime2 + 1) << k];
        let (a, b) = a.split_at_mut((nprime2 + 1) << k);
        let mut t = vec![0; 2 * (nprime2 + 1)];
        let mut tmp = vec![0; 2 << k];
        let bit_reverse_table = _limbs_mul_fft_bit_reverse_table(&mut tmp, k);
        let mut api = 0;
        let mut bpi = 0;
        for _ in 0..big_k {
            _limbs_mul_fft_normalize(ap[api], n);
            _limbs_mul_fft_normalize(bp[bpi], n);
            let mut big_ap =
                _limbs_mul_fft_decompose(a, k2, nprime2, &ap[api][..(l << k) + 1], l, mp2, &mut t);
            _limbs_mul_fft_decompose(b, k2, nprime2, &bp[bpi][..(l << k) + 1], l, mp2, &mut t);
            let cy = mpn_mul_fft_internal(
                ap[api],
                n,
                k,
                big_ap,
                b,
                nprime2,
                l,
                mp2,
                &bit_reverse_table,
                &mut t,
                false,
            );
            ap[api][n] = cy;
            api += 1;
            bpi += 1;
        }
    } else {
        let n2 = 2 * n;
        let mut tp = vec![0; n2];
        let mut api = 0;
        let mut bpi = 0;
        for _ in 0..big_k {
            let a = &mut ap[api];
            let b = &mut bp[bpi];
            api += 1;
            bpi += 1;
            limbs_mul_same_length_to_out(&mut tp, &b[..n], &a[..n]);
            let mut cc = if a[n] != 0 {
                if limbs_slice_add_same_length_in_place_left(&mut tp[n..2 * n], &b[..n]) {
                    1
                } else {
                    0
                }
            } else {
                0
            };
            if b[n] != 0 {
                cc += if limbs_slice_add_same_length_in_place_left(&mut tp[n..2 * n], &a[..n]) {
                    1
                } else {
                    0
                } + a[n];
            }
            if cc != 0 {
                assert!(!limbs_slice_add_limb_in_place(&mut tp[..n2], cc));
            }
            a[n] = if limbs_sub_same_length_to_out(a, &tp[..n], &tp[n..2 * n])
                && limbs_slice_add_limb_in_place(&mut a[..n], 1)
            {
                1
            } else {
                0
            };
        }
    }
}

// This is mpn_fft_mul_modF_K from mpn/generic/mul_fft.c.
fn mpn_fft_mul_mod_f_k_sqr(ap: &mut [&mut [Limb]], n: usize, big_k: usize) {
    if n >= SQR_FFT_MODF_THRESHOLD {
        let k = _limbs_mul_fft_best_k(n, false);
        let k2 = 1 << k;
        assert_eq!(n & (k2 - 1), 0);
        let max_lk = max(k2, Limb::WIDTH as usize);
        let m2 = n * Limb::WIDTH as usize >> k;
        let l = n >> k;
        let mut big_nprime2 = (2 * m2 + k + 2 + max_lk) / max_lk * max_lk;
        // Nprime2 = ceil((2*M2+k+3)/maxLK)*maxLK
        let mut nprime2 = big_nprime2 / Limb::WIDTH as usize;

        // we should ensure that nprime2 is a multiple of the next K
        if nprime2 >= SQR_FFT_MODF_THRESHOLD {
            //mp_size_t K3;
            loop {
                let k3 = 1 << _limbs_mul_fft_best_k(nprime2, true);
                if nprime2 & (k3 - 1) == 0 {
                    break;
                }
                nprime2 = (nprime2 + k3 - 1) & k3.wrapping_neg();
                big_nprime2 = nprime2 * Limb::WIDTH as usize;
                // warning: since nprime2 changed, K3 may change too!
            }
        }
        assert!(nprime2 < n); // otherwise we'll loop

        let mp2 = big_nprime2 >> k;
        let mut a = vec![0; 2 * (nprime2 + 1) << k];
        let (a_lo, a_hi) = a.split_at_mut((nprime2 + 1) << k);
        let mut t = vec![0; 2 * (nprime2 + 1)];
        let mut tmp = vec![0; 2 << k];
        let bit_reverse_table = _limbs_mul_fft_bit_reverse_table(&mut tmp, k);
        let mut api = 0;
        for _ in 0..big_k {
            _limbs_mul_fft_normalize(ap[api], n);
            let mut big_ap = _limbs_mul_fft_decompose(
                a_lo,
                k2,
                nprime2,
                &ap[api][..(l << k) + 1],
                l,
                mp2,
                &mut t,
            );
            let cy = mpn_mul_fft_internal(
                ap[api],
                n,
                k,
                big_ap,
                a_hi,
                nprime2,
                l,
                mp2,
                &bit_reverse_table,
                &mut t,
                true,
            );
            ap[api][n] = cy;
            api += 1;
        }
    } else {
        let n2 = 2 * n;
        let mut tp = vec![0; n2];
        let mut api = 0;
        for _ in 0..big_k {
            let a = &mut ap[api];
            api += 1;
            //TODO use square
            limbs_mul_same_length_to_out(&mut tp, &a[..n], &a[..n]);
            let mut cc = if a[n] != 0 {
                if limbs_slice_add_same_length_in_place_left(&mut tp[n..2 * n], &a[..n]) {
                    1
                } else {
                    0
                }
            } else {
                0
            };
            if a[n] != 0 {
                cc += if limbs_slice_add_same_length_in_place_left(&mut tp[n..2 * n], &a[..n]) {
                    1
                } else {
                    0
                } + a[n];
            }
            if cc != 0 {
                assert!(!limbs_slice_add_limb_in_place(&mut tp[..n2], cc));
            }
            a[n] = if limbs_sub_same_length_to_out(a, &tp[..n], &tp[n..2 * n])
                && limbs_slice_add_limb_in_place(&mut a[..n], 1)
            {
                1
            } else {
                0
            };
        }
    }
}

// This is mpn_mul_fft_internal from mpn/generic/mul_fft.c. A is excluded as it is unused.
pub fn mpn_mul_fft_internal(
    op: &mut [Limb],
    pl: usize,
    k: usize,
    mut ap: Vec<&mut [Limb]>,
    b: &mut [Limb],
    nprime: usize,
    l: usize,
    mp: usize,
    fft_l: &[&[usize]],
    t: &mut [Limb],
    sqr: bool,
) -> Limb {
    let big_k = 1usize << k;
    {
        let mut bp = Vec::with_capacity(big_k);
        let mut remainder: &mut [Limb] = b;
        for _ in 0..big_k {
            // force remainder to move rather than be borrowed
            let (b_lo, b_hi) = { remainder }.split_at_mut(nprime + 1);
            bp.push(b_lo);
            remainder = b_hi;
        }
        // direct fft's
        mpn_fft_fft(&mut ap, big_k, fft_l, k, 2 * mp, nprime, 1, t);
        if !sqr {
            mpn_fft_fft(&mut bp, big_k, fft_l, k, 2 * mp, nprime, 1, t);
        }

        // term to term multiplications
        if sqr {
            mpn_fft_mul_mod_f_k_sqr(&mut ap, nprime, big_k);
        } else {
            mpn_fft_mul_mod_f_k(&mut ap, &mut bp, nprime, big_k);
        }
    }

    // inverse fft's
    mpn_fft_fftinv(&mut ap, big_k, 2 * mp, nprime, t);

    // division of terms after inverse fft
    let mut bp = Vec::with_capacity(big_k);
    let (t_lo, t_hi) = t.split_at_mut(nprime + 1);
    bp.push(t_hi);
    _limbs_mul_fft_shr_mod_f_to_out(&mut bp[0], &mut ap[0], k, nprime);

    for i in 1..big_k {
        let (ap_lo, ap_hi) = ap.split_at_mut(i);
        _limbs_mul_fft_shr_mod_f_to_out(
            &mut ap_lo[i - 1],
            &mut ap_hi[0],
            k + (big_k - i) * mp,
            nprime,
        );
    }
    bp.extend(ap.drain(..big_k - 1));

    // addition of terms in result p
    limbs_set_zero(t_lo);
    let pla = l * (big_k - 1) + nprime + 1; // number of required limbs for p

    // B has K*(n' + 1) limbs, which is >= pla, i.e. enough
    limbs_set_zero(&mut b[..pla]);
    let mut cc: SignedLimb = 0; // will accumulate the (signed) carry at p[pla]
    let mut i = big_k - 1;
    let mut lo = l * i + nprime;
    let mut sh = l * i;
    loop {
        let j;
        {
            let n = &mut b[sh..];
            j = (big_k - i) & (big_k - 1);
            if limbs_slice_add_same_length_in_place_left(&mut n[..nprime + 1], &bp[j][..nprime + 1])
            {
                cc += if limbs_slice_add_limb_in_place(&mut n[nprime + 1..pla - sh], 1) {
                    1
                } else {
                    0
                };
            }
            if 2 * l < t_lo.len() {
                t_lo[2 * l] = i as Limb + 1; // T = (i + 1)*2^(2*M)
            } else {
                bp[0][2 * l - t_lo.len()] = i as Limb + 1; // T = (i + 1)*2^(2*M)
            }
        }
        if limbs_cmp_same_length(&bp[j][..nprime + 1], t_lo) == Ordering::Greater {
            // subtract 2^N'+1
            {
                let n = &mut b[sh..];
                cc -= if limbs_sub_limb_in_place(&mut n[..pla - sh], 1) {
                    1
                } else {
                    0
                };
            }
            cc -= if limbs_sub_limb_in_place(&mut b[lo..pla], 1) {
                1
            } else {
                0
            };
        }

        if i == 0 {
            break;
        }
        i -= 1;
        lo -= l;
        sh -= l;
    }
    if cc == -1 {
        cc = if limbs_slice_add_limb_in_place(&mut b[pla - pl..pla], 1) {
            1
        } else {
            0
        };
        if cc != 0 {
            // p[pla-pl]...p[pla-1] are all zero
            limbs_sub_limb_in_place(&mut b[pla - pl - 1..pla], 1);
            limbs_sub_limb_in_place(&mut b[pla - 1..pla], 1);
        }
    } else if cc == 1 {
        // This branch is untested!
        let mut cc = 1 as Limb;
        if pla >= 2 * pl {
            loop {
                cc = if limbs_slice_add_limb_in_place(&mut b[pla - 2 * pl..pla], cc) {
                    1
                } else {
                    0
                };
                if cc == 0 {
                    break;
                }
            }
        } else {
            cc = if limbs_sub_limb_in_place(&mut b[pla - pl..pla], cc) {
                1
            } else {
                0
            };
            assert_eq!(cc, 0);
        }
    } else {
        assert_eq!(cc, 0);
    }
    // here p < 2^(2M) [K 2^(M(K-1)) + (K-1) 2^(M(K-2)) + ... ]
    // < K 2^(2M) [2^(M(K-1)) + 2^(M(K-2)) + ... ]
    // < K 2^(2M) 2^(M(K-1))*2 = 2^(M*K+M+k+1)
    mpn_fft_norm_mod_f(op, pl, b, pla)
}

// This is mpn_mul_fft from mpn/generic/mul_fft.c.
pub(crate) fn mpn_mul_fft(op: &mut [Limb], pl: usize, n: &[Limb], m: &[Limb], k: usize) -> Limb {
    let nl = n.len();
    let ml = m.len();
    let sqr = n as *const [Limb] == m as *const [Limb];
    assert_eq!(mpn_fft_next_size(pl, k), pl);

    let big_n = pl * Limb::WIDTH as usize;
    let big_k = 1 << k;
    let big_m = big_n >> k; // N = 2^k M
    let l = 1 + (big_m - 1) / Limb::WIDTH as usize;
    // lcm(GMP_NUMB_BITS, 2^k)
    let max_lk = _limbs_mul_fft_lcm_of_a_and_two_pow_k(Limb::WIDTH as usize, k);

    let mut big_nprime = (1 + (2 * big_m + k + 2) / max_lk) * max_lk;
    // Nprime = ceil((2*M+k+3)/maxLK)*maxLK;
    let mut nprime = big_nprime / Limb::WIDTH as usize;
    // we should ensure that recursively, nprime is a multiple of the next big_k
    if nprime
        >= if sqr {
            SQR_FFT_MODF_THRESHOLD
        } else {
            MUL_FFT_MODF_THRESHOLD
        }
    {
        loop {
            let k2 = 1 << _limbs_mul_fft_best_k(nprime, sqr);
            if (nprime & (k2 - 1)) == 0 {
                break;
            }
            nprime = (nprime + k2 - 1) & k2.wrapping_neg();
            big_nprime = nprime * Limb::WIDTH as usize;
            // warning: since nprime changed, K2 may change too!
        }
    }
    assert!(nprime < pl); // otherwise we'll loop
    let mut t = vec![0; 2 * (nprime + 1)];
    let mp = big_nprime >> k;

    let mut a = vec![0; big_k * (nprime + 1)];
    let ap = _limbs_mul_fft_decompose(&mut a, big_k, nprime, &n[..nl], l, mp, &mut t);
    let mut tmp = vec![0; 2 << k];
    let bit_reverse_table = _limbs_mul_fft_bit_reverse_table(&mut tmp, k);
    if sqr {
        let pla = l * (big_k - 1) + nprime + 1; // number of required limbs for p
        let mut b = vec![0; pla];
        mpn_mul_fft_internal(
            op,
            pl,
            k,
            ap,
            &mut b,
            nprime,
            l,
            mp,
            &bit_reverse_table,
            &mut t,
            sqr,
        )
    } else {
        let mut b = vec![0; big_k * (nprime + 1)];
        _limbs_mul_fft_decompose(&mut b, big_k, nprime, &m[..ml], l, mp, &mut t);
        mpn_mul_fft_internal(
            op,
            pl,
            k,
            ap,
            &mut b,
            nprime,
            l,
            mp,
            &bit_reverse_table,
            &mut t,
            sqr,
        )
    }
}
