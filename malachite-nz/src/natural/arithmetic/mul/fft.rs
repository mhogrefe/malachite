use malachite_base::limbs::limbs_set_zero;
use malachite_base::num::{NotAssign, PrimitiveInteger, WrappingSubAssign};
use natural::arithmetic::add::{
    limbs_slice_add_greater_in_place_left, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul::mul_mod::{
    mpn_mulmod_bnm1, mpn_mulmod_bnm1_itch, mpn_mulmod_bnm1_next_size,
};
use natural::arithmetic::shl_u::limbs_shl_to_out;
use natural::arithmetic::shl_u::mpn_lshiftc;
use natural::arithmetic::sub::{
    limbs_sub_in_place_left, limbs_sub_same_length_in_place_left, limbs_sub_same_length_to_out,
    limbs_sub_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::logic::not::limbs_not_to_out;
use platform::{Limb, SignedLimb};

//TODO tune
pub const MUL_FFT_THRESHOLD: usize = 4_736;

//TODO test
// checked
// docs preserved
// Returns smallest possible number of limbs >= pl for a fft of size 2 ^ k, i.e. smallest multiple
// of 2 ^ k >= pl.
// This is mpn_fft_next_size from mpn/generic/mul-fft.c.
pub fn mpn_fft_next_size(mut pl: usize, k: u32) -> usize {
    pl = 1 + ((pl - 1) >> k); // ceil(pl / 2 ^ k)
    pl << k
}

struct FFTTableNK {
    n: u32,
    k: u32,
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

//TODO test
// checked
// docs preserved
// Find the best k to use for a mod 2 ^ (m * Limb::WIDTH) + 1 FFT for m >= n. We have sqr = 0 if for
// a multiply, sqr = 1 for a square.
// mpn_fft_best_k from mpn/generic/mul-fft.c, mpn_fft_table3 variant
pub fn mpn_fft_best_k(n: usize, sqr: bool) -> u32 {
    let fft_tab: &[FFTTableNK] = if sqr {
        &SQR_FFT_TABLE3
    } else {
        &MUL_FFT_TABLE3
    };
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

// This is mpn_fft_mul from gmp-impl.h.
#[inline]
pub fn mpn_fft_mul(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    mpn_nussbaumer_mul(out, xs, ys);
}

// This is mpn_nussbaumer_mul from mpn/generic/mpn_nussbaumer_mul.c.
fn mpn_nussbaumer_mul(pp: &mut [Limb], ap: &[Limb], bp: &[Limb]) {
    let an = ap.len();
    let bn = bp.len();
    assert!(an >= bn);
    assert_ne!(bn, 0);

    //TODO special case for squaring
    let rn = mpn_mulmod_bnm1_next_size(an + bn);
    let mut tp = vec![0; mpn_mulmod_bnm1_itch(rn, an, bn)];
    mpn_mulmod_bnm1(pp, rn, ap, bp, &mut tp);
}

// Initialize l[i][j] with bitrev(j)
//TODO make not pub
// This is mpn_fft_initl from mpn/generic/mul_fft.c.
pub fn mpn_fft_initl(l: &mut [&mut [u32]], k: usize) {
    l[0][0] = 0;
    let mut i = 1;
    let mut big_k = 1;
    while i <= k {
        for j in 0..big_k {
            l[i][j] = 2 * l[i - 1][j];
            l[i][big_k + j] = 1 + l[i][j];
        }
        i += 1;
        big_k <<= 1;
    }
}

// return the lcm of a and 2^k
//TODO make not pub
// This is mpn_mul_fft_lcm from mpn/generic/mul_fft.c.
pub fn mpn_mul_fft_lcm(mut a: u32, mut k: u32) -> u32 {
    let l = k;
    while a % 2 == 0 && k > 0 {
        a >>= 1;
        k -= 1;
    }
    a << l
}

// r <- a*2^d mod 2^(n*`Limb::WIDTH`)+1 with a = {a, n+1}
// Assumes a is semi-normalized, i.e. a[n] <= 1.
// r and a must have n+1 limbs, and not overlap.
// TODO make not pub
// This is mpn_fft_mul_2exp_modF from mpn/generic/mul_fft.c.
pub fn mpn_fft_mul_2exp_mod_f(r: &mut [Limb], a: &[Limb], d: u32, n: usize) {
    let sh = d % Limb::WIDTH;
    let mut m = (d / Limb::WIDTH) as usize;

    // negate
    if m >= n {
        // r[0..m-1]  <-- lshift(a[n-m]..a[n-1], sh)
        // r[m..n-1]  <-- -lshift(a[0]..a[n-m-1],  sh)

        m -= n;
        let mut cc;
        let mut rd;
        if sh != 0 {
            // no out shift below since a[n] <= 1
            limbs_shl_to_out(r, &a[n - m..n + 1], sh);
            rd = r[m];
            cc = mpn_lshiftc(&mut r[m..], &a[..n - m], sh);
        } else {
            r[..m].copy_from_slice(&a[n - m..n]);
            rd = a[n];
            limbs_not_to_out(&mut r[m..], &a[..n - m]);
            cc = 0;
        }

        // add cc to r[0], and add rd to r[m]
        // now add 1 in r[m], subtract 1 in r[n], i.e. add 1 in r[0]
        r[n] = 0;
        // cc < 2^sh <= 2^(Limb::WIDTH`-1) thus no overflow here
        cc += 1;
        limbs_slice_add_limb_in_place(r, cc);
        rd += 1;
        // rd might overflow when sh=Limb::WIDTH`-1
        cc = if rd == 0 { 1 } else { rd };
        limbs_slice_add_limb_in_place(&mut r[m + if rd == 0 { 1 } else { 0 }..], cc);
    } else {
        let mut cc;
        let rd;
        // r[0..m-1]  <-- -lshift(a[n-m]..a[n-1], sh)
        // r[m..n-1]  <-- lshift(a[0]..a[n-m-1],  sh)
        if sh != 0 {
            // no out bits below since a[n] <= 1
            mpn_lshiftc(r, &a[n - m..n + 1], sh);
            rd = !r[m];
            // {r, m+1} = {a+n-m, m+1} << sh
            cc = limbs_shl_to_out(&mut r[m..], &a[..n - m], sh); // {r+m, n-m} = {a, n-m}<<sh
        } else {
            // r[m] is not used below, but we save a test for m=0
            limbs_not_to_out(r, &a[n - m..n + 1]);
            rd = a[n];
            r[m..n].copy_from_slice(&a[..n - m]);
            cc = 0;
        }

        // now complement {r, m}, subtract cc from r[0], subtract rd from r[m]
        // if m=0 we just have r[0]=a[n] << sh
        if m != 0 {
            // now add 1 in r[0], subtract 1 in r[m]
            // then add 1 to r[0]
            if cc == 0 {
                cc = if limbs_slice_add_limb_in_place(&mut r[..n], 1) {
                    1
                } else {
                    0
                };
            } else {
                cc -= 1;
            }
            cc = if limbs_sub_limb_in_place(&mut r[..m], cc) {
                1
            } else {
                0
            } + 1;
            // add 1 to cc instead of rd since rd might overflow
        }

        // now subtract cc and rd from r[m..n]
        let (r_last, r_init) = r[..n + 1].split_last_mut().unwrap();
        *r_last = (if limbs_sub_limb_in_place(&mut r_init[m..], cc) {
            1 as Limb
        } else {
            0
        })
        .wrapping_neg();
        r_last.wrapping_sub_assign(if limbs_sub_limb_in_place(&mut r_init[m..], rd) {
            1
        } else {
            0
        });
        if r_last.get_highest_bit() {
            *r_last = if limbs_slice_add_limb_in_place(r_init, 1) {
                1
            } else {
                0
            };
        }
    }
}

// store in A[0..nprime] the first M bits from {n, nl},
// in A[nprime+1..] the following M bits, ...
// Assumes M is a multiple of GMP_NUMB_BITS (M = l * GMP_NUMB_BITS).
// T must have space for at least (nprime + 1) limbs.
// We must have nl <= 2*K*l.
// TODO make not pub
// This is mpn_mul_fft_decompose from mpn/generic/mul_fft.c.
pub fn mpn_mul_fft_decompose(
    a: &mut [Limb],
    ap: &mut [&mut [Limb]],
    k: usize,
    nprime: usize,
    n: &[Limb],
    mut nl: usize,
    l: usize,
    mp: u32,
    t: &mut [Limb],
) {
    let kl = k * l;

    // normalize {n, nl} mod 2^(Kl*GMP_NUMB_BITS)+1
    let mut n_is_tmp = false;
    let mut n_offset = 0;
    let mut cy: SignedLimb;
    let mut tmp;
    if nl > kl {
        let mut dif = nl - kl;
        tmp = vec![0; kl + 1];

        if dif > kl {
            let mut subp = false;
            cy = if limbs_sub_same_length_to_out(&mut tmp, &n[..kl], &n[kl..2 * kl]) {
                1
            } else {
                0
            };
            n_offset = 2 * kl;
            dif -= kl;

            // now dif > 0
            while dif > kl {
                if subp {
                    cy += if limbs_sub_same_length_in_place_left(
                        &mut tmp[..kl],
                        &n[n_offset..n_offset + kl],
                    ) {
                        1
                    } else {
                        0
                    };
                } else {
                    cy -= if limbs_slice_add_same_length_in_place_left(
                        &mut tmp[..kl],
                        &n[n_offset..n_offset + kl],
                    ) {
                        1
                    } else {
                        0
                    };
                }
                subp.not_assign();
                n_offset += kl;
                dif -= kl;
            }
            // now dif <= Kl
            if subp {
                cy += if limbs_sub_in_place_left(&mut tmp[..kl], &n[n_offset..n_offset + dif]) {
                    1
                } else {
                    0
                };
            } else {
                cy -= if limbs_slice_add_greater_in_place_left(
                    &mut tmp[..kl],
                    &n[n_offset..n_offset + dif],
                ) {
                    1
                } else {
                    0
                };
            }
            if cy >= 0 {
                cy = if limbs_slice_add_limb_in_place(&mut tmp[..kl], cy as Limb) {
                    1
                } else {
                    0
                };
            } else {
                cy = if limbs_sub_limb_in_place(&mut tmp[..kl], -cy as Limb) {
                    1
                } else {
                    0
                };
            }
        } else {
            // dif <= Kl, i.e. nl <= 2 * Kl
            cy = if limbs_sub_to_out(&mut tmp, &n[..kl], &n[kl..kl + dif]) {
                1
            } else {
                0
            };
            cy = if limbs_slice_add_limb_in_place(&mut tmp[..kl], cy as Limb) {
                1
            } else {
                0
            };
        }
        tmp[kl] = cy as Limb;
        nl = kl + 1;
        n_is_tmp = true;
    } else {
        tmp = Vec::with_capacity(0);
    }
    let mut tmp_offset = 0;
    let mut a_offset = 0;
    for i in 0..k {
        //TODO make sure this really needs to be a copy
        ap[i].copy_from_slice(&a[a_offset..]);
        // store the next M bits of n into A[0..nprime]
        // nl is the number of remaining limbs
        if nl > 0 {
            let j = if l <= nl && i < k - 1 { l } else { nl }; // store j next limbs
            nl -= j;
            if n_is_tmp {
                t[..j].copy_from_slice(&tmp[tmp_offset..tmp_offset + j]);
            } else {
                t[..j].copy_from_slice(&n[n_offset..n_offset + j]);
            }
            limbs_set_zero(&mut t[j..nprime + 1]);
            if n_is_tmp {
                tmp_offset += l;
            } else {
                n_offset += l;
            }
            mpn_fft_mul_2exp_mod_f(&mut a[a_offset..], t, i as u32 * mp, nprime);
        } else {
            limbs_set_zero(&mut a[a_offset..a_offset + nprime + 1]);
        }
        a_offset += nprime + 1;
    }
    assert_eq!(nl, 0);
}

//TODO mpn_fft_add_modF and mpn_fft_sub_modF next

//TODO make not pub
// This is mpn_mul_fft from mpn/generic/mul_fft.c.
pub fn mpn_mul_fft(xp: &mut [Limb], _n: usize, ap1: &[Limb], bp1: &[Limb], _k: u32) -> Limb {
    let anp = ap1.len();
    let bnp = bp1.len();
    //TODO implement
    xp[anp + bnp - 1]
}
