#[doc(hidden)]
#[macro_use]
pub mod macros;

pub mod arithmetic {
    pub mod abs;
    pub mod add_mul;
    pub mod arithmetic_checked_shl;
    pub mod arithmetic_checked_shr;
    pub mod checked_abs;
    pub mod checked_add;
    pub mod checked_add_mul;
    pub mod checked_div;
    pub mod checked_mul;
    pub mod checked_neg;
    pub mod checked_next_power_of_two;
    pub mod checked_pow;
    pub mod checked_square;
    pub mod checked_sub;
    pub mod checked_sub_mul;
    pub mod div_exact;
    pub mod div_mod;
    pub mod div_round;
    pub mod divisible_by;
    pub mod divisible_by_power_of_two;
    pub mod eq_mod;
    pub mod eq_mod_power_of_two;
    pub mod is_power_of_two;
    pub mod mod_add;
    pub mod mod_is_reduced;
    pub mod mod_mul;
    pub mod mod_neg;
    pub mod mod_op;
    pub mod mod_pow;
    pub mod mod_power_of_two;
    pub mod mod_power_of_two_add;
    pub mod mod_power_of_two_is_reduced;
    pub mod mod_power_of_two_mul;
    pub mod mod_power_of_two_neg;
    pub mod mod_power_of_two_shl;
    pub mod mod_power_of_two_shr;
    pub mod mod_power_of_two_sub;
    pub mod mod_sub;
    pub mod neg;
    pub mod next_power_of_two;
    pub mod overflowing_abs;
    pub mod overflowing_add;
    pub mod overflowing_add_mul;
    pub mod overflowing_div;
    pub mod overflowing_mul;
    pub mod overflowing_neg;
    pub mod overflowing_pow;
    pub mod overflowing_square;
    pub mod overflowing_sub;
    pub mod overflowing_sub_mul;
    pub mod parity;
    pub mod pow;
    pub mod power_of_two;
    pub mod round_to_multiple;
    pub mod round_to_multiple_of_power_of_two;
    pub mod saturating_abs;
    pub mod saturating_add;
    pub mod saturating_add_mul;
    pub mod saturating_mul;
    pub mod saturating_neg;
    pub mod saturating_pow;
    pub mod saturating_square;
    pub mod saturating_sub;
    pub mod saturating_sub_mul;
    pub mod shl_round;
    pub mod shr_round;
    pub mod sign;
    pub mod square;
    pub mod sub_mul;
    pub mod traits;
    pub mod unsigneds;
    pub mod wrapping_abs;
    pub mod wrapping_add;
    pub mod wrapping_add_mul;
    pub mod wrapping_div;
    pub mod wrapping_mul;
    pub mod wrapping_neg;
    pub mod wrapping_pow;
    pub mod wrapping_square;
    pub mod wrapping_sub;
    pub mod wrapping_sub_mul;
    pub mod x_mul_y_is_zz;
    pub mod xx_add_yy_is_zz;
    pub mod xx_div_mod_y_is_qr;
    pub mod xx_sub_yy_is_zz;
    pub mod xxx_add_yyy_is_zzz;
    pub mod xxx_sub_yyy_is_zzz;
    pub mod xxxx_add_yyyy_is_zzzz;
}
pub mod basic {
    pub mod integers;
    pub mod signeds;
    pub mod traits;
    pub mod unsigneds;
}
pub mod comparison {
    pub mod ord_abs;
    pub mod traits;
}
pub mod conversion {
    pub mod from;
    pub mod half;
    pub mod slice;
    pub mod traits;
}
pub mod exhaustive;
pub mod floats;
pub mod iterator;
pub mod logic {
    pub mod bit_access;
    pub mod bit_block_access;
    pub mod bit_convertible;
    pub mod bit_iterable;
    pub mod bit_scan;
    pub mod count_ones;
    pub mod count_zeros;
    pub mod hamming_distance;
    pub mod leading_zeros;
    pub mod low_mask;
    pub mod not;
    pub mod power_of_two_digit_iterable;
    pub mod power_of_two_digits;
    pub mod rotate;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod traits;
}
/// This module contains iterators that generate primitive integers randomly.
pub mod random;
