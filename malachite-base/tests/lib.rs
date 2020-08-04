#![warn(
    clippy::cast_lossless,
    clippy::decimal_literal_representation,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map,
    clippy::filter_map_next,
    clippy::find_map,
    clippy::large_digit_groups,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::missing_const_for_fn,
    clippy::mut_mut,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::print_stdout,
    clippy::redundant_closure_for_method_calls,
    clippy::single_match_else,
    clippy::type_repetition_in_bounds,
    clippy::unused_self
)]
#![allow(
    clippy::cognitive_complexity,
    clippy::too_many_arguments,
    unstable_name_collisions
)]

extern crate core;
extern crate itertools;
#[macro_use]
extern crate malachite_base;
extern crate malachite_base_test_util;
extern crate rand;
extern crate rand_chacha;

pub mod bools {
    pub mod constants;
    pub mod exhaustive;
    pub mod not_assign;
    pub mod random;
}
pub mod comparison {
    pub mod macros;
}
pub mod chars {
    pub mod char_to_contiguous_range;
    pub mod char_type;
    pub mod constants;
    pub mod contiguous_range_to_char;
    pub mod exhaustive {
        pub mod ascii_chars_increasing;
        pub mod chars_increasing;
        pub mod exhaustive_ascii_chars;
        pub mod exhaustive_chars;
    }
}
pub mod iterators {
    pub mod comparison {
        pub mod is_strictly_ascending;
        pub mod is_strictly_descending;
        pub mod is_weakly_ascending;
        pub mod is_weakly_descending;
    }
    pub mod nonzero_values;
}
pub mod named;
pub mod num {
    pub mod arithmetic {
        pub mod abs;
        pub mod add_mul;
        pub mod arithmetic_checked_shl;
        pub mod arithmetic_checked_shr;
        pub mod checked_abs;
        pub mod checked_add_mul;
        pub mod checked_neg;
        pub mod checked_square;
        pub mod checked_sub_mul;
        pub mod div_exact;
        pub mod div_mod;
        pub mod div_round;
        pub mod divisible_by;
        pub mod divisible_by_power_of_two;
        pub mod eq_mod;
        pub mod eq_mod_power_of_two;
        pub mod log_two;
        pub mod mod_add;
        pub mod mod_is_reduced;
        pub mod mod_mul;
        pub mod mod_neg;
        pub mod mod_op;
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
        pub mod constants;
        pub mod iverson;
    }
    pub mod comparison {
        pub mod ord_abs;
    }
    pub mod conversion {
        pub mod from;
        pub mod half;
        pub mod slice;
    }
    pub mod exhaustive {
        pub mod exhaustive_natural_signeds;
        pub mod exhaustive_negative_signeds;
        pub mod exhaustive_nonzero_signeds;
        pub mod exhaustive_positive_primitives;
        pub mod exhaustive_signed_range;
        pub mod exhaustive_signed_range_to_max;
        pub mod exhaustive_signeds;
        pub mod exhaustive_unsigneds;
        pub mod primitive_integer_increasing_range;
        pub mod primitive_integer_increasing_range_to_max;
    }
    pub mod float {
        pub mod nice_float;
    }
    pub mod logic {
        pub mod bit_access;
        pub mod bit_block_access;
        pub mod bit_convertible;
        pub mod bit_iterable;
        pub mod bit_scan;
        pub mod get_highest_bit;
        pub mod hamming_distance;
        pub mod low_mask;
        pub mod not;
        pub mod power_of_two_digit_iterable;
        pub mod power_of_two_digits;
        pub mod rotate;
        pub mod significant_bits;
    }
    pub mod random {
        pub mod geometric {
            pub mod geometric_random_natural_signeds;
            pub mod geometric_random_negative_signeds;
            pub mod geometric_random_nonzero_signeds;
            pub mod geometric_random_positive_signeds;
            pub mod geometric_random_positive_unsigneds;
            pub mod geometric_random_signeds;
            pub mod geometric_random_unsigneds;
            pub mod mean;
        }
        pub mod random_highest_bit_set_unsigneds;
        pub mod random_natural_signeds;
        pub mod random_negative_signeds;
        pub mod random_nonzero_signeds;
        pub mod random_positive_signeds;
        pub mod random_positive_unsigneds;
        pub mod random_primitive_integers;
        pub mod random_signed_bit_chunks;
        pub mod random_signed_range;
        pub mod random_signed_range_to_max;
        pub mod random_unsigned_bit_chunks;
        pub mod random_unsigned_range;
        pub mod random_unsigned_range_to_max;
        pub mod random_unsigneds_less_than;
        pub mod striped {
            pub mod striped_bit_source;
            pub mod striped_random_natural_signeds;
            pub mod striped_random_negative_signeds;
            pub mod striped_random_nonzero_signeds;
            pub mod striped_random_positive_signeds;
            pub mod striped_random_positive_unsigneds;
            pub mod striped_random_signeds;
            pub mod striped_random_unsigneds;
        }
    }
}
pub mod orderings {
    pub mod exhaustive;
    pub mod random;
}
pub mod random {
    pub mod random_values_from_slice;
    pub mod random_values_from_vec;
    pub mod seed {
        pub mod fork;
        pub mod from_bytes;
    }
}
pub mod rounding_modes {
    pub mod clone;
    pub mod display;
    pub mod eq;
    pub mod exhaustive;
    pub mod from_str;
    pub mod neg;
    pub mod random;
    pub mod size;
}
pub mod slices {
    pub mod slice_leading_zeros;
    pub mod slice_move_left;
    pub mod slice_set_zero;
    pub mod slice_test_zero;
    pub mod slice_trailing_zeros;
    pub mod split_into_chunks;
}
pub mod strings {
    pub mod string_is_subset;
    pub mod string_nub;
    pub mod string_sort;
    pub mod to_debug_string;
}
pub mod vecs;
