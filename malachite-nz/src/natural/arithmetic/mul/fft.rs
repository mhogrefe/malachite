use malachite_base::limbs::limbs_set_zero;
use malachite_base::num::{
    One, Parity, PrimitiveInteger, ShrRound, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::round::RoundingMode;
use natural::arithmetic::add::limbs_add_to_out;
use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
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
    limbs_sub_in_place_left, limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
    limbs_sub_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::logic::not::limbs_not_to_out;
use platform::Limb;
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
    let mut shift_limbs = bits >> Limb::LOG_WIDTH as usize;
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
    _limbs_mul_fft_shl_mod_f_to_out(out, xs, (n << (Limb::LOG_WIDTH + 1)) - bits, n);
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
    let mut scratch2;
    let mut source: &[Limb] = if len > k_times_n {
        let difference = len - k_times_n;
        scratch2 = vec![0; k_times_n + 1];
        // difference > k_times_n -> len - k * n > k * n -> len > 2 * k * n, which violates the
        // precondition len <= 2 * k * n. So difference > k_times_n cannot happen.
        assert!(difference <= k_times_n);
        // difference <= k_times_n, i.e. len <= 2 * k_times_n
        let (xs_lo, xs_hi) = xs.split_at(k_times_n);
        scratch2[k_times_n] = if limbs_sub_to_out(&mut scratch2, xs_lo, &xs_hi[..difference])
            && limbs_slice_add_limb_in_place(&mut scratch2[..k_times_n], 1)
        {
            1
        } else {
            0
        };
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
        if len == 0 {
            limbs_set_zero(a_lo);
        } else {
            let j = if n <= len && i < k - 1 { n } else { len }; // store j next limbs
            len -= j;
            if i != 0 {
                source = &source[n..];
            }
            scratch[..j].copy_from_slice(&source[..j]);
            limbs_set_zero(&mut scratch[j..width]);
            _limbs_mul_fft_shl_mod_f_to_out(a_lo, scratch, i * m, width_minus_one);
        }
        a_table.push(a_lo);
    }
    assert_eq!(len, 0);
    a_table
}

// input: A[0] ... A[increment*(K-1)] are residues mod 2^N+1 where
//    N=n*GMP_NUMB_BITS, and 2^omega is a primitive root mod 2^N+1
// output: A[increment*l[k][i]] <- \sum (2^omega)^(ij) A[increment*j] mod 2^N+1
// This is mpn_fft_fft from mpn/generic/mul_fft.c.
pub fn _limbs_mul_fft(
    xss: &mut [&mut [Limb]],
    k: usize,
    bit_reverse_table: &[&[usize]],
    bit_reverse_table_offset: usize,
    omega: usize,
    n: usize,
    increment: usize,
    scratch: &mut [Limb],
) {
    assert_ne!(increment, 0);
    if k == 2 {
        {
            let (xss_first, xss_tail) = xss.split_first_mut().unwrap();
            scratch[..n + 1].copy_from_slice(xss_first);
            limbs_slice_add_same_length_in_place_left(xss_first, &xss_tail[increment - 1]);
            let (xss_0_last, xss_0_init) = xss_first.split_last_mut().unwrap();
            // can be 2 or 3
            if *xss_0_last > 1 {
                *xss_0_last = if limbs_sub_limb_in_place(xss_0_init, *xss_0_last - 1) {
                    0
                } else {
                    1
                };
            }
        }
        // Ap[increment][n] can be -1 or -2
        if limbs_sub_same_length_in_place_right(&scratch[..n + 1], &mut xss[increment]) {
            let (xss_increment_last, xss_increment_init) = xss[increment].split_last_mut().unwrap();
            *xss_increment_last = if limbs_slice_add_limb_in_place(
                xss_increment_init,
                xss_increment_last.wrapping_neg(),
            ) {
                1
            } else {
                0
            };
        }
    } else {
        let half_k = k >> 1;
        let twice_omega = omega << 1;
        let twice_increment = increment << 1;
        let offset_minus_one = bit_reverse_table_offset - 1;
        _limbs_mul_fft(
            xss,
            half_k,
            bit_reverse_table,
            offset_minus_one,
            twice_omega,
            n,
            twice_increment,
            scratch,
        );
        _limbs_mul_fft(
            &mut xss[increment..],
            half_k,
            bit_reverse_table,
            offset_minus_one,
            twice_omega,
            n,
            twice_increment,
            scratch,
        );
        //  A[2*j*increment]   <- A[2*j*increment] + omega^l[k][2*j*increment] A[(2j+1)increment]
        // A[(2j+1)increment] <- A[2*j*increment] + omega^l[k][(2j+1)increment] A[(2j+1)increment]
        let bit_reverse_row = bit_reverse_table[bit_reverse_table_offset];
        let mut xss_offset = 0;
        for i in 0..half_k {
            // Ap[increment] <- Ap[0] + Ap[increment] * 2^(lk[1] * omega)
            // Ap[0]   <- Ap[0] + Ap[increment] * 2^(lk[0] * omega)
            let (xss_lo, xss_hi) = xss.split_at_mut(xss_offset + increment);
            _limbs_mul_fft_shl_mod_f_to_out(
                scratch,
                xss_hi[0],
                bit_reverse_row[i << 1].wrapping_mul(omega),
                n,
            );
            _limbs_mul_fft_sub_mod_f_to_out(xss_hi[0], xss_lo[xss_offset], scratch, n);
            _limbs_mul_fft_add_mod_f_in_place_left(xss_lo[xss_offset], scratch, n);
            xss_offset += increment << 1;
        }
    }
}

// input: A^[l[k][0]] A^[l[k][1]] ... A^[l[k][K-1]]
// output: K*A[0] K*A[K-1] ... K*A[1].
// Assumes the Ap[] are pseudo-normalized, i.e. 0 <= Ap[][n] <= 1.
// This condition is also fulfilled at exit.
// This is mpn_fft_fftinv from mpn/generic/mul_fft.c.
pub fn _limbs_mul_fft_inverse(
    xss: &mut [&mut [Limb]],
    k: usize,
    omega: usize,
    n: usize,
    scratch: &mut [Limb],
) {
    if k == 2 {
        let (xss_first, xss_tail) = xss.split_first_mut().unwrap();
        scratch[..n + 1].copy_from_slice(xss_first);
        limbs_slice_add_same_length_in_place_left(xss_first, &xss_tail[0]);
        // can be 2 or 3
        let (xss_0_last, xss_0_init) = xss_first.split_last_mut().unwrap();
        if *xss_0_last > 1 {
            *xss_0_last = if limbs_sub_limb_in_place(xss_0_init, *xss_0_last - 1) {
                0
            } else {
                1
            };
        }
        // Ap[1][n] can be -1 or -2
        if limbs_sub_same_length_in_place_right(&scratch[..n + 1], &mut xss_tail[0]) {
            let (xss_1_last, xss_1_init) = xss_tail[0].split_last_mut().unwrap();
            *xss_1_last = if limbs_slice_add_limb_in_place(xss_1_init, xss_1_last.wrapping_neg()) {
                1
            } else {
                0
            };
        }
    } else {
        let half_k = k >> 1;
        let twice_omega = omega << 1;
        _limbs_mul_fft_inverse(xss, half_k, twice_omega, n, scratch);
        _limbs_mul_fft_inverse(&mut xss[half_k..], half_k, twice_omega, n, scratch);
        // A[i]     <- A[i] + omega^i A[i+K/2]
        // A[i+K/2] <- A[i] + omega^(i+K/2) A[i+K/2]
        for i in 0..half_k {
            // Ap[K2] <- Ap[0] + Ap[K2] * 2^((i + K2) * omega)
            // Ap[0]  <- Ap[0] + Ap[K2] * 2^(i * omega)
            let (xss_lo, xss_hi) = xss.split_at_mut(half_k + i);
            _limbs_mul_fft_shl_mod_f_to_out(scratch, xss_hi[0], i * omega, n);
            _limbs_mul_fft_sub_mod_f_to_out(xss_hi[0], xss_lo[i], scratch, n);
            _limbs_mul_fft_add_mod_f_in_place_left(xss_lo[i], scratch, n);
        }
    }
}

// {out,n} <- {xs,an} mod 2^(n*GMP_NUMB_BITS)+1, n <= an <= 3*n.
// Returns carry out, i.e. 1 iff {xs,an} = -1 mod 2^(n*GMP_NUMB_BITS)+1,
// then {out,n}=0.
// This is mpn_fft_norm_modF from mpn/generic/mul_fft.c.
pub fn _limbs_mul_fft_normalize_mod_f(out: &mut [Limb], n: usize, xs: &[Limb]) -> bool {
    let xs_len = xs.len();
    assert!(n <= xs_len && xs_len <= 3 * n);
    let out = &mut out[..n];
    if xs_len >= 2 * n {
        // add {xs, m} and {xs+2n, m} in {out, m}
        // copy {xs+m, n-m} to {out+m, n-m}
        split_into_chunks!(xs, n, _unused, [xs_0, xs_1], xs_2);
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

// This is mpn_fft_mul_modF_K from mpn/generic/mul_fft.c, where ap != bp. K is omitted because it is
// unused; it is just the length of `xss` and `yss`.
fn _limbs_mul_fft_mul_mod_f_k(xss: &mut [&mut [Limb]], yss: &mut [&mut [Limb]], n: usize) {
    if n >= MUL_FFT_MODF_THRESHOLD {
        let k = _limbs_mul_fft_best_k(n, false);
        let two_pow_k = 1 << k;
        assert_eq!(n & (two_pow_k - 1), 0);
        let max_k_pow_2_width = max(two_pow_k, Limb::WIDTH as usize);
        let m = n << Limb::LOG_WIDTH >> k;
        let p = n >> k;
        let mut q = (2 * m + k + 2 + max_k_pow_2_width) / max_k_pow_2_width * max_k_pow_2_width;
        // r = ceil((2 * m + k + 3) / max_k_pow_2_width) * max_k_pow_2_width
        let mut r = q >> Limb::LOG_WIDTH;

        // we should ensure that r is a multiple of the next K
        if r >= MUL_FFT_MODF_THRESHOLD {
            loop {
                let two_pow_best_k = 1 << _limbs_mul_fft_best_k(r, false);
                if r & (two_pow_best_k - 1) == 0 {
                    break;
                }
                r = (r + two_pow_best_k - 1) & two_pow_best_k.wrapping_neg();
                q = r << Limb::LOG_WIDTH;
                // warning: since r changed, K3 may change too!
            }
        }
        assert!(r < n); // otherwise we'll loop
        let q_shifted = q >> k;
        let s = r + 1;
        let mut a = vec![0; s << (k + 1)];
        let (a, b) = a.split_at_mut(s << k);
        let mut scratch = vec![0; s << 1];
        let mut scratch2 = vec![0; 2 << k];
        let bit_reverse_table = _limbs_mul_fft_bit_reverse_table(&mut scratch2, k);
        for (xs, ys) in xss.iter_mut().zip(yss.iter_mut()) {
            _limbs_mul_fft_normalize(xs, n);
            _limbs_mul_fft_normalize(ys, n);
            let mut residues =
                _limbs_mul_fft_decompose(a, two_pow_k, r, xs, p, q_shifted, &mut scratch);
            _limbs_mul_fft_decompose(b, two_pow_k, r, ys, p, q_shifted, &mut scratch);
            xs[n] = if _limbs_mul_fft_internal(
                xs,
                n,
                k,
                residues,
                b,
                r,
                p,
                q_shifted,
                &bit_reverse_table,
                &mut scratch,
                false,
            ) {
                1
            } else {
                0
            };
        }
    } else {
        let mut scratch = vec![0; n << 1];
        for (xs, ys) in xss.iter_mut().zip(yss.iter_mut()) {
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            let (ys_last, ys_init) = ys.split_last_mut().unwrap();
            limbs_mul_same_length_to_out(&mut scratch, ys_init, xs_init);
            let mut carry = 0;
            {
                let scratch_hi = &mut scratch[n..];
                if *xs_last != 0 && limbs_slice_add_same_length_in_place_left(scratch_hi, ys_init) {
                    carry = 1;
                }
                if *ys_last != 0 {
                    if limbs_slice_add_same_length_in_place_left(scratch_hi, xs_init) {
                        carry += 1;
                    }
                    carry.wrapping_add_assign(*xs_last);
                }
            }
            if carry != 0 {
                assert!(!limbs_slice_add_limb_in_place(&mut scratch, carry));
            }
            let (scratch_lo, scratch_hi) = scratch.split_at(n);
            *xs_last = if limbs_sub_same_length_to_out(xs_init, scratch_lo, scratch_hi)
                && limbs_slice_add_limb_in_place(xs_init, 1)
            {
                1
            } else {
                0
            };
        }
    }
}

// This is mpn_fft_mul_modF_K from mpn/generic/mul_fft.c, where ap == bp. K is omitted because it is
// unused; it is just the length of `xss`.
fn _limbs_mul_fft_mul_mod_f_k_square(xss: &mut [&mut [Limb]], n: usize) {
    if n >= SQR_FFT_MODF_THRESHOLD {
        let k = _limbs_mul_fft_best_k(n, false);
        let two_pow_k = 1 << k;
        assert_eq!(n & (two_pow_k - 1), 0);
        let max_k_pow_2_width = max(two_pow_k, Limb::WIDTH as usize);
        let m = n << Limb::LOG_WIDTH >> k;
        let p = n >> k;
        let mut q = (2 * m + k + 2 + max_k_pow_2_width) / max_k_pow_2_width * max_k_pow_2_width;
        // r = ceil((2 * m + k + 3) / max_k_pow_2_width) * max_k_pow_2_width
        let mut r = q >> Limb::LOG_WIDTH;

        // we should ensure that r is a multiple of the next K
        if r >= SQR_FFT_MODF_THRESHOLD {
            loop {
                let two_pow_best_k = 1 << _limbs_mul_fft_best_k(r, true);
                if r & (two_pow_best_k - 1) == 0 {
                    break;
                }
                r = (r + two_pow_best_k - 1) & two_pow_best_k.wrapping_neg();
                q = r << Limb::LOG_WIDTH;
                // warning: since r changed, K3 may change too!
            }
        }
        assert!(r < n); // otherwise we'll loop
        let q_shifted = q >> k;
        let s = r + 1;
        let mut a = vec![0; s << (k + 1)];
        let (a_lo, a_hi) = a.split_at_mut(s << k);
        let mut scratch = vec![0; s << 1];
        let mut scratch2 = vec![0; 2 << k];
        let bit_reverse_table = _limbs_mul_fft_bit_reverse_table(&mut scratch2, k);
        for xs in xss.iter_mut() {
            _limbs_mul_fft_normalize(xs, n);
            let mut residues =
                _limbs_mul_fft_decompose(a_lo, two_pow_k, r, xs, p, q_shifted, &mut scratch);
            xs[n] = if _limbs_mul_fft_internal(
                xs,
                n,
                k,
                residues,
                a_hi,
                r,
                p,
                q_shifted,
                &bit_reverse_table,
                &mut scratch,
                true,
            ) {
                1
            } else {
                0
            };
        }
    } else {
        let mut scratch = vec![0; n << 1];
        for xs in xss.iter_mut() {
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            //TODO use square
            limbs_mul_same_length_to_out(&mut scratch, xs_init, xs_init);
            let mut carry = 0;
            {
                let scratch_hi = &mut scratch[n..];
                if *xs_last != 0 {
                    //TODO use addmul
                    if limbs_slice_add_same_length_in_place_left(scratch_hi, xs_init) {
                        carry = 1;
                    }
                    if limbs_slice_add_same_length_in_place_left(scratch_hi, xs_init) {
                        carry += 1;
                    }
                    carry.wrapping_add_assign(*xs_last);
                }
            }
            if carry != 0 {
                assert!(!limbs_slice_add_limb_in_place(&mut scratch, carry));
            }
            let (scratch_lo, scratch_hi) = scratch.split_at(n);
            *xs_last = if limbs_sub_same_length_to_out(xs_init, scratch_lo, scratch_hi)
                && limbs_slice_add_limb_in_place(xs_init, 1)
            {
                1
            } else {
                0
            };
        }
    }
}

// This is mpn_mul_fft_internal from mpn/generic/mul_fft.c. A is excluded as it is unused.
pub fn _limbs_mul_fft_internal(
    out: &mut [Limb],
    p: usize,
    k: usize,
    mut xss: Vec<&mut [Limb]>,
    ys: &mut [Limb],
    width_minus_one: usize,
    a: usize,
    omega: usize,
    bit_reverse_table: &[&[usize]],
    scratch: &mut [Limb],
    square: bool,
) -> bool {
    let two_pow_k = 1usize << k;
    let twice_omega = omega << 1;
    let width = width_minus_one + 1;
    // direct fft's
    _limbs_mul_fft(
        &mut xss,
        two_pow_k,
        bit_reverse_table,
        k,
        twice_omega,
        width_minus_one,
        1,
        scratch,
    );
    if square {
        // term to term multiplications
        _limbs_mul_fft_mul_mod_f_k_square(&mut xss, width_minus_one);
    } else {
        let mut yss = Vec::with_capacity(two_pow_k);
        let mut remainder: &mut [Limb] = ys;
        for _ in 0..two_pow_k {
            // force remainder to move rather than be borrowed
            let (ys_lo, ys_hi) = { remainder }.split_at_mut(width);
            yss.push(ys_lo);
            remainder = ys_hi;
        }
        _limbs_mul_fft(
            &mut yss,
            two_pow_k,
            bit_reverse_table,
            k,
            twice_omega,
            width_minus_one,
            1,
            scratch,
        );
        // term to term multiplications
        _limbs_mul_fft_mul_mod_f_k(&mut xss, &mut yss, width_minus_one);
    }
    // inverse fft's
    _limbs_mul_fft_inverse(&mut xss, two_pow_k, twice_omega, width_minus_one, scratch);
    // division of terms after inverse fft
    let mut yss = Vec::with_capacity(two_pow_k);
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(width);
    yss.push(scratch_hi);
    _limbs_mul_fft_shr_mod_f_to_out(&mut yss[0], &mut xss[0], k, width_minus_one);
    for i in 1..two_pow_k {
        let (xss_lo, yss_hi) = xss.split_at_mut(i);
        _limbs_mul_fft_shr_mod_f_to_out(
            &mut xss_lo[i - 1],
            &mut yss_hi[0],
            k + (two_pow_k - i) * omega,
            width_minus_one,
        );
    }
    yss.extend(xss.drain(..two_pow_k - 1));

    // addition of terms in result p
    limbs_set_zero(scratch_lo);
    let q = a * (two_pow_k - 1) + width; // number of required limbs for p
    let ys = &mut ys[..q];
    // B has K*width limbs, which is >= q, i.e. enough
    limbs_set_zero(ys);
    let mut carry = 0i32; // will accumulate the (signed) carry at p[q]
    let mut sh = a * two_pow_k;
    let mut lo = sh + width_minus_one;
    for i in (0..two_pow_k).rev() {
        lo -= a;
        sh -= a;
        let j = (two_pow_k - i) & (two_pow_k - 1);
        if limbs_slice_add_same_length_in_place_left(&mut ys[sh..sh + width], &yss[j])
            && limbs_slice_add_limb_in_place(&mut ys[sh + width..], 1)
        {
            carry += 1;
        }
        if 2 * a < width {
            scratch_lo[2 * a] = i as Limb + 1; // T = (i + 1)*2^(2*M)
        } else {
            yss[0][2 * a - width] = i as Limb + 1; // T = (i + 1)*2^(2*M)
        }
        if limbs_cmp_same_length(&yss[j], scratch_lo) == Ordering::Greater {
            // subtract 2^N'+1
            if limbs_sub_limb_in_place(&mut ys[sh..], 1) {
                carry -= 1;
            }
            if limbs_sub_limb_in_place(&mut ys[lo..], 1) {
                carry -= 1;
            }
        }
    }
    if carry == -1 {
        let ys = &mut ys[q - p - 1..];
        if limbs_slice_add_limb_in_place(&mut ys[1..], 1) {
            // p[q-p]...p[q-1] are all zero
            limbs_sub_limb_in_place(ys, 1);
            limbs_sub_limb_in_place(&mut ys[p..], 1);
        }
    } else if carry == 1 {
        // This branch is untested!
        if q >= 2 * p {
            let ys = &mut ys[q - 2 * p..];
            while limbs_slice_add_limb_in_place(ys, 1) {}
        } else {
            assert!(!limbs_sub_limb_in_place(&mut ys[q - p..], 1));
        }
    } else {
        assert_eq!(carry, 0);
    }
    // here p < 2^(2M) [K 2^(M(K-1)) + (K-1) 2^(M(K-2)) + ... ]
    // < K 2^(2M) [2^(M(K-1)) + 2^(M(K-2)) + ... ]
    // < K 2^(2M) 2^(M(K-1))*2 = 2^(M*K+M+k+1)
    _limbs_mul_fft_normalize_mod_f(out, p, ys)
}

// This is mpn_mul_fft from mpn/generic/mul_fft.c.
pub(crate) fn mpn_mul_fft(op: &mut [Limb], pl: usize, n: &[Limb], m: &[Limb], k: usize) -> bool {
    let nl = n.len();
    let ml = m.len();
    let sqr = n as *const [Limb] == m as *const [Limb];
    assert_eq!(mpn_fft_next_size(pl, k), pl);

    let big_n = pl << Limb::LOG_WIDTH;
    let big_k = 1 << k;
    let big_m = big_n >> k; // N = 2^k M
    let l = 1 + (big_m - 1) >> Limb::LOG_WIDTH;
    // lcm(GMP_NUMB_BITS, 2^k)
    let max_lk = _limbs_mul_fft_lcm_of_a_and_two_pow_k(Limb::WIDTH as usize, k);

    let mut big_nprime = (1 + (2 * big_m + k + 2) / max_lk) * max_lk;
    // Nprime = ceil((2*M+k+3)/maxLK)*maxLK;
    let mut nprime = big_nprime >> Limb::LOG_WIDTH;
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
            big_nprime = nprime << Limb::LOG_WIDTH as usize;
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
        _limbs_mul_fft_internal(
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
        _limbs_mul_fft_internal(
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
