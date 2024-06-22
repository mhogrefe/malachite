// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#![warn(
    clippy::cast_lossless,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filter_map_next,
    clippy::large_digit_groups,
    clippy::manual_filter_map,
    clippy::manual_find_map,
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
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::uninlined_format_args,
    clippy::unused_self,
    clippy::if_not_else,
    clippy::manual_assert,
    clippy::range_plus_one,
    clippy::redundant_else,
    clippy::semicolon_if_nothing_returned,
    clippy::cloned_instead_of_copied,
    clippy::flat_map_option,
    clippy::unnecessary_wraps,
    clippy::unnested_or_patterns,
    clippy::trivially_copy_pass_by_ref
)]
#![allow(
    clippy::assertions_on_constants, // Compile-time asserts still useful
    clippy::bool_assert_comparison, // Often clearer than using !
    clippy::cognitive_complexity,
    clippy::excessive_precision,
    clippy::float_cmp,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    unstable_name_collisions
)]

extern crate core;
extern crate itertools;
#[macro_use]
extern crate malachite_base;
#[macro_use]
extern crate maplit;
extern crate rand;
extern crate rand_chacha;
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;

fn get_sample_output_types(len: usize) -> Vec<Vec<BitDistributorOutputType>> {
    if len == 2 {
        vec![
            vec![BitDistributorOutputType::normal(1); 2],
            vec![BitDistributorOutputType::normal(2); 2],
            vec![BitDistributorOutputType::normal(1), BitDistributorOutputType::normal(2)],
            vec![BitDistributorOutputType::normal(1), BitDistributorOutputType::tiny()],
        ]
    } else if len == 3 {
        vec![
            vec![BitDistributorOutputType::normal(1); 3],
            vec![BitDistributorOutputType::normal(2); 3],
            vec![
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(2),
                BitDistributorOutputType::normal(3),
            ],
            vec![
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ],
            vec![
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ],
            vec![
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ],
            vec![
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::tiny(),
            ],
            vec![
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ],
            vec![
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ],
        ]
    } else {
        panic!()
    }
}

pub mod bools {
    pub mod constants;
    pub mod exhaustive;
    pub mod not_assign;
    pub mod random {
        pub mod get_weighted_random_bool;
        pub mod random_bools;
        pub mod weighted_random_bools;
    }
}
pub mod comparison {
    pub mod macros;
}
pub mod chars {
    pub mod char_type;
    pub mod constants;
    pub mod crement {
        pub mod char_to_contiguous_range;
        pub mod contiguous_range_to_char;
        #[allow(clippy::module_inception)]
        pub mod crement;
    }
    pub mod exhaustive {
        pub mod ascii_chars_increasing;
        pub mod chars_increasing;
        pub mod exhaustive_ascii_chars;
        pub mod exhaustive_chars;
    }
    pub mod is_graphic;
    pub mod random {
        pub mod graphic_weighted_random_ascii_chars;
        pub mod graphic_weighted_random_char_inclusive_range;
        pub mod graphic_weighted_random_char_range;
        pub mod graphic_weighted_random_chars;
        pub mod random_ascii_chars;
        pub mod random_char_inclusive_range;
        pub mod random_char_range;
        pub mod random_chars;
    }
}
pub mod extra_variadic;
pub mod iterators {
    pub mod bit_distributor {
        pub mod bit_map_as_slice;
        pub mod get_output;
        pub mod increment_counter;
        pub mod new;
        pub mod set_max_bits;
    }
    pub mod comparison {
        pub mod delta_directions;
        pub mod is_strictly_ascending;
        pub mod is_strictly_descending;
        pub mod is_strictly_zigzagging;
        pub mod is_weakly_ascending;
        pub mod is_weakly_descending;
        pub mod is_weakly_zigzagging;
    }
    pub mod count_is_at_least;
    pub mod count_is_at_most;
    pub mod first_and_last;
    pub mod is_constant;
    pub mod is_unique;
    pub mod iter_windows;
    pub mod iterator_cache;
    pub mod matching_intervals_in_iterator;
    pub mod nonzero_values;
    pub mod prefix_to_string;
    pub mod with_special_value;
    pub mod with_special_values;
}
pub mod named;
pub mod nevers {
    #[allow(clippy::module_inception)]
    pub mod nevers;
}
pub mod num {
    pub mod arithmetic {
        pub mod abs;
        pub mod abs_diff;
        pub mod add_mul;
        pub mod arithmetic_checked_shl;
        pub mod arithmetic_checked_shr;
        pub mod binomial_coefficient;
        pub mod ceiling;
        pub mod checked_abs;
        pub mod checked_add_mul;
        pub mod checked_neg;
        pub mod checked_pow;
        pub mod checked_square;
        pub mod checked_sub_mul;
        pub mod coprime_with;
        pub mod div_exact;
        pub mod div_mod;
        pub mod div_round;
        pub mod divisible_by;
        pub mod divisible_by_power_of_2;
        pub mod eq_mod;
        pub mod eq_mod_power_of_2;
        pub mod extended_gcd;
        pub mod factorial;
        pub mod floor;
        pub mod gcd;
        pub mod is_power_of_2;
        pub mod kronecker_symbol;
        pub mod lcm;
        pub mod log_base;
        pub mod log_base_2;
        pub mod log_base_power_of_2;
        pub mod mod_add;
        pub mod mod_inverse;
        pub mod mod_is_reduced;
        pub mod mod_mul;
        pub mod mod_neg;
        pub mod mod_op;
        pub mod mod_pow;
        pub mod mod_power_of_2;
        pub mod mod_power_of_2_add;
        pub mod mod_power_of_2_inverse;
        pub mod mod_power_of_2_is_reduced;
        pub mod mod_power_of_2_mul;
        pub mod mod_power_of_2_neg;
        pub mod mod_power_of_2_pow;
        pub mod mod_power_of_2_shl;
        pub mod mod_power_of_2_shr;
        pub mod mod_power_of_2_square;
        pub mod mod_power_of_2_sub;
        pub mod mod_shl;
        pub mod mod_shr;
        pub mod mod_square;
        pub mod mod_sub;
        pub mod neg;
        pub mod next_power_of_2;
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
        pub mod power_of_2;
        pub mod primorial;
        pub mod root;
        pub mod rotate;
        pub mod round_to_multiple;
        pub mod round_to_multiple_of_power_of_2;
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
        pub mod sqrt;
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
        pub mod x_mul_y_to_zz;
        pub mod xx_add_yy_to_zz;
        pub mod xx_div_mod_y_to_qr;
        pub mod xx_sub_yy_to_zz;
        pub mod xxx_add_yyy_to_zzz;
        pub mod xxx_sub_yyy_to_zzz;
        pub mod xxxx_add_yyyy_to_zzzz;
    }
    pub mod basic {
        pub mod constants;
    }
    pub mod comparison {
        pub mod cmp_abs_partial_cmp_abs_and_comparators;
        pub mod eq_abs_partial_eq_abs_and_comparators;
    }
    pub mod conversion {
        pub mod digits {
            pub mod general_digits {
                pub mod from_digits;
                pub mod to_digits;
            }
            pub mod power_of_2_digits {
                pub mod from_power_of_2_digits;
                pub mod power_of_2_digit_iterable;
                pub mod to_power_of_2_digits;
            }
        }
        pub mod froms {
            pub mod convertible_from;
            pub mod from;
            pub mod overflowing_from;
            pub mod rounding_from;
            pub mod saturating_from;
            pub mod try_from_and_exact_from;
            pub mod wrapping_from;
        }
        pub mod half {
            pub mod join_halves;
            pub mod lower_half;
            pub mod split_in_half;
            pub mod upper_half;
        }
        pub mod is_integer;
        pub mod mantissa_and_exponent {
            pub mod integer_mantissa_and_exponent;
            pub mod raw_mantissa_and_exponent;
            pub mod sci_mantissa_and_exponent;
        }
        pub mod slice {
            pub mod from_other_type_slice;
            pub mod vec_from_other_type;
            pub mod vec_from_other_type_slice;
        }
        pub mod string {
            pub mod from_sci_string;
            pub mod from_string;
            pub mod options {
                pub mod from_sci_string_options;
                pub mod to_sci_options;
            }
            pub mod to_sci;
            pub mod to_string;
        }
    }
    pub mod exhaustive {
        pub mod exhaustive_finite_primitive_floats;
        pub mod exhaustive_natural_signeds;
        pub mod exhaustive_negative_finite_primitive_floats;
        pub mod exhaustive_negative_primitive_floats;
        pub mod exhaustive_negative_signeds;
        pub mod exhaustive_nonzero_finite_primitive_floats;
        pub mod exhaustive_nonzero_finite_primitive_floats_in_range;
        pub mod exhaustive_nonzero_primitive_floats;
        pub mod exhaustive_nonzero_signeds;
        pub mod exhaustive_positive_finite_primitive_floats;
        pub mod exhaustive_positive_finite_primitive_floats_in_range;
        pub mod exhaustive_positive_primitive_floats;
        pub mod exhaustive_positive_primitive_ints;
        pub mod exhaustive_primitive_float_inclusive_range;
        pub mod exhaustive_primitive_float_range;
        pub mod exhaustive_primitive_floats;
        pub mod exhaustive_primitive_floats_with_sci_exponent;
        pub mod exhaustive_primitive_floats_with_sci_exponent_and_precision;
        pub mod exhaustive_primitive_floats_with_sci_exponent_and_precision_in_range;
        pub mod exhaustive_primitive_floats_with_sci_exponent_in_range;
        pub mod exhaustive_signed_inclusive_range;
        pub mod exhaustive_signed_range;
        pub mod exhaustive_signeds;
        pub mod exhaustive_unsigneds;
        pub mod finite_primitive_floats_increasing;
        pub mod negative_finite_primitive_floats_increasing;
        pub mod negative_primitive_floats_increasing;
        pub mod nonzero_finite_primitive_floats_increasing;
        pub mod nonzero_primitive_floats_increasing;
        pub mod positive_finite_primitive_floats_increasing;
        pub mod positive_primitive_floats_increasing;
        pub mod primitive_float_increasing_inclusive_range;
        pub mod primitive_float_increasing_range;
        pub mod primitive_floats_increasing;
        pub mod primitive_int_increasing_inclusive_range;
        pub mod primitive_int_increasing_range;
    }
    pub mod factorization {
        pub mod prime_sieve;
        pub mod primes;
    }
    pub mod float {
        pub mod basic {
            pub mod abs_negative_zero;
            pub mod from_ordered_representation;
            pub mod is_negative_zero;
            pub mod max_precision_for_sci_exponent;
            pub mod next_higher;
            pub mod next_lower;
            pub mod precision;
            pub mod to_ordered_representation;
        }
        pub mod nice_float {
            pub mod cmp;
            pub mod eq;
            pub mod from_str;
            pub mod hash;
            pub mod to_string;
        }
    }
    pub mod iterators {
        pub mod bit_distributor_sequence;
        pub mod iterator_to_bit_chunks;
        pub mod ruler_sequence;
    }
    pub mod logic {
        pub mod bit_access {
            pub mod assign_bit;
            pub mod clear_bit;
            pub mod flip_bit;
            pub mod get_bit;
            pub mod set_bit;
        }
        pub mod bit_block_access {
            pub mod assign_bits;
            pub mod get_bits;
        }
        pub mod bit_convertible {
            pub mod from_bits;
            pub mod to_bits;
        }
        pub mod bit_iterable;
        pub mod bit_scan {
            pub mod index_of_next_false_bit;
            pub mod index_of_next_true_bit;
        }
        pub mod get_highest_bit;
        pub mod hamming_distance;
        pub mod low_mask;
        pub mod not_assign;
        pub mod significant_bits;
    }
    pub mod random {
        pub mod geometric {
            pub mod geometric_random_natural_signeds;
            pub mod geometric_random_negative_signeds;
            pub mod geometric_random_nonzero_signeds;
            pub mod geometric_random_positive_signeds;
            pub mod geometric_random_positive_unsigneds;
            pub mod geometric_random_signed_inclusive_range;
            pub mod geometric_random_signed_range;
            pub mod geometric_random_signeds;
            pub mod geometric_random_unsigned_inclusive_range;
            pub mod geometric_random_unsigned_range;
            pub mod geometric_random_unsigneds;
            pub mod get_geometric_random_signed_from_inclusive_range;
            pub mod mean;
        }
        pub mod random_finite_primitive_floats;
        pub mod random_highest_bit_set_unsigneds;
        pub mod random_natural_signeds;
        pub mod random_negative_finite_primitive_floats;
        pub mod random_negative_primitive_floats;
        pub mod random_negative_signeds;
        pub mod random_nonzero_finite_primitive_floats;
        pub mod random_nonzero_primitive_floats;
        pub mod random_nonzero_signeds;
        pub mod random_positive_finite_primitive_floats;
        pub mod random_positive_primitive_floats;
        pub mod random_positive_signeds;
        pub mod random_positive_unsigneds;
        pub mod random_primitive_float_inclusive_range;
        pub mod random_primitive_float_range;
        pub mod random_primitive_floats;
        pub mod random_primitive_ints;
        pub mod random_signed_bit_chunks;
        pub mod random_signed_inclusive_range;
        pub mod random_signed_range;
        pub mod random_unsigned_bit_chunks;
        pub mod random_unsigned_inclusive_range;
        pub mod random_unsigned_range;
        pub mod random_unsigneds_less_than;
        pub mod special_random_finite_primitive_floats;
        pub mod special_random_negative_finite_primitive_floats;
        pub mod special_random_negative_primitive_floats;
        pub mod special_random_nonzero_finite_primitive_floats;
        pub mod special_random_nonzero_primitive_floats;
        pub mod special_random_positive_finite_primitive_floats;
        pub mod special_random_positive_primitive_floats;
        pub mod special_random_primitive_float_inclusive_range;
        pub mod special_random_primitive_float_range;
        pub mod special_random_primitive_floats;
        pub mod striped {
            pub mod get_striped_bool_vec;
            pub mod get_striped_unsigned_vec;
            pub mod striped_bit_source;
            pub mod striped_random_bool_vecs;
            pub mod striped_random_bool_vecs_from_length_iterator;
            pub mod striped_random_bool_vecs_length_inclusive_range;
            pub mod striped_random_bool_vecs_length_range;
            pub mod striped_random_bool_vecs_min_length;
            pub mod striped_random_fixed_length_bool_vecs;
            pub mod striped_random_fixed_length_unsigned_vecs;
            pub mod striped_random_natural_signeds;
            pub mod striped_random_negative_signeds;
            pub mod striped_random_nonzero_signeds;
            pub mod striped_random_positive_signeds;
            pub mod striped_random_positive_unsigneds;
            pub mod striped_random_signed_inclusive_range;
            pub mod striped_random_signed_range;
            pub mod striped_random_signeds;
            pub mod striped_random_unsigned_bit_chunks;
            pub mod striped_random_unsigned_inclusive_range;
            pub mod striped_random_unsigned_range;
            pub mod striped_random_unsigned_vecs;
            pub mod striped_random_unsigned_vecs_from_length_iterator;
            pub mod striped_random_unsigned_vecs_length_inclusive_range;
            pub mod striped_random_unsigned_vecs_length_range;
            pub mod striped_random_unsigned_vecs_min_length;
            pub mod striped_random_unsigneds;
        }
        pub mod variable_range_generator {
            pub mod next_bit_chunk;
            pub mod next_bool;
            pub mod next_in_inclusive_range;
            pub mod next_in_range;
            pub mod next_less_than;
        }
    }
}
pub mod options {
    pub mod exhaustive {
        pub mod exhaustive_options;
        pub mod exhaustive_somes;
    }
    pub mod option_from_str;
    pub mod random {
        pub mod random_options;
        pub mod random_somes;
    }
}
pub mod orderings {
    pub mod exhaustive;
    pub mod ordering_from_str;
    pub mod random;
}
pub mod random {
    pub mod fork;
    pub mod from_bytes;
    pub mod get_rng;
    pub mod next;
}
pub mod rational_sequences {
    pub mod access {
        pub mod get;
        pub mod mutate;
    }
    pub mod basic {
        pub mod component_len;
        pub mod is_empty;
        pub mod is_finite;
        pub mod iter;
        pub mod len;
    }
    pub mod comparison {
        pub mod cmp;
        pub mod eq;
        pub mod hash;
    }
    pub mod conversion {
        pub mod clone;
        pub mod from_vec;
        pub mod from_vecs;
        pub mod to_vecs;
    }
    pub mod exhaustive;
    pub mod random;
    pub mod to_string;
}
pub mod rounding_modes {
    pub mod clone;
    pub mod cmp;
    pub mod eq;
    pub mod exhaustive;
    pub mod from_str;
    pub mod hash;
    pub mod neg;
    pub mod random;
    pub mod size;
    pub mod to_string;
}
pub mod sets {
    pub mod exhaustive {
        pub mod exhaustive_b_tree_sets;
        pub mod exhaustive_b_tree_sets_fixed_length;
        pub mod exhaustive_b_tree_sets_length_inclusive_range;
        pub mod exhaustive_b_tree_sets_length_range;
        pub mod exhaustive_b_tree_sets_min_length;
        pub mod exhaustive_hash_sets;
        pub mod exhaustive_hash_sets_fixed_length;
        pub mod exhaustive_hash_sets_length_inclusive_range;
        pub mod exhaustive_hash_sets_length_range;
        pub mod exhaustive_hash_sets_min_length;
        pub mod lex_b_tree_sets;
        pub mod lex_b_tree_sets_fixed_length;
        pub mod lex_b_tree_sets_length_inclusive_range;
        pub mod lex_b_tree_sets_length_range;
        pub mod lex_b_tree_sets_min_length;
        pub mod lex_hash_sets;
        pub mod lex_hash_sets_fixed_length;
        pub mod lex_hash_sets_length_inclusive_range;
        pub mod lex_hash_sets_length_range;
        pub mod lex_hash_sets_min_length;
        pub mod shortlex_b_tree_sets;
        pub mod shortlex_b_tree_sets_length_inclusive_range;
        pub mod shortlex_b_tree_sets_length_range;
        pub mod shortlex_b_tree_sets_min_length;
        pub mod shortlex_hash_sets;
        pub mod shortlex_hash_sets_length_inclusive_range;
        pub mod shortlex_hash_sets_length_range;
        pub mod shortlex_hash_sets_min_length;
    }
    pub mod random {
        pub mod random_b_tree_sets;
        pub mod random_b_tree_sets_fixed_length;
        pub mod random_b_tree_sets_from_length_iterator;
        pub mod random_b_tree_sets_length_inclusive_range;
        pub mod random_b_tree_sets_length_range;
        pub mod random_b_tree_sets_min_length;
        pub mod random_hash_sets;
        pub mod random_hash_sets_fixed_length;
        pub mod random_hash_sets_from_length_iterator;
        pub mod random_hash_sets_length_inclusive_range;
        pub mod random_hash_sets_length_range;
        pub mod random_hash_sets_min_length;
    }
}
pub mod slices {
    pub mod exhaustive_slice_permutations;
    pub mod min_repeating_len;
    pub mod random_slice_permutations;
    pub mod slice_leading_zeros;
    pub mod slice_move_left;
    pub mod slice_set_zero;
    pub mod slice_test_zero;
    pub mod slice_trailing_zeros;
    pub mod split_into_chunks;
}
pub mod strings {
    pub mod exhaustive {
        pub mod exhaustive_fixed_length_strings;
        pub mod exhaustive_fixed_length_strings_using_chars;
        pub mod exhaustive_strings;
        pub mod exhaustive_strings_using_chars;
        pub mod lex_fixed_length_strings;
        pub mod lex_fixed_length_strings_using_chars;
        pub mod shortlex_strings;
        pub mod shortlex_strings_using_chars;
    }
    pub mod random {
        pub mod random_fixed_length_strings;
        pub mod random_fixed_length_strings_using_chars;
        pub mod random_strings;
        pub mod random_strings_using_chars;
    }
    pub mod string_is_subset;
    pub mod string_sort;
    pub mod string_unique;
    pub mod strings_from_char_vecs;
    pub mod to_binary_string;
    pub mod to_debug_string;
    pub mod to_lower_hex_string;
    pub mod to_octal_string;
    pub mod to_upper_hex_string;
}
pub mod tuples {
    pub mod exhaustive {
        pub mod exhaustive_custom_tuples;
        pub mod exhaustive_dependent_pairs;
        pub mod exhaustive_ordered_unique_tuples;
        pub mod exhaustive_tuples_1_input;
        pub mod exhaustive_tuples_custom_output;
        pub mod exhaustive_tuples_from_single;
        pub mod exhaustive_unique_tuples;
        pub mod exhaustive_units;
        pub mod lex_custom_tuples;
        pub mod lex_dependent_pairs;
        pub mod lex_ordered_unique_tuples;
        pub mod lex_tuples;
        pub mod lex_tuples_from_single;
        pub mod lex_unique_tuples;
    }
    pub mod random {
        pub mod random_custom_tuples;
        pub mod random_ordered_unique_tuples;
        pub mod random_tuples;
        pub mod random_tuples_from_single;
        pub mod random_unique_tuples;
        pub mod random_units;
    }
    pub mod singletons;
}
pub mod unions {
    pub mod clone;
    pub mod debug;
    pub mod display;
    pub mod eq;
    pub mod exhaustive {
        pub mod exhaustive_unions;
        pub mod lex_unions;
    }
    pub mod from_str;
    pub mod ord;
    pub mod random {
        pub mod random_unions;
    }
    pub mod unwrap;
}
pub mod vecs {
    pub mod exhaustive {
        pub mod exhaustive_combined_k_compositions;
        pub mod exhaustive_ordered_unique_vecs;
        pub mod exhaustive_ordered_unique_vecs_fixed_length;
        pub mod exhaustive_ordered_unique_vecs_length_inclusive_range;
        pub mod exhaustive_ordered_unique_vecs_length_range;
        pub mod exhaustive_ordered_unique_vecs_min_length;
        pub mod exhaustive_unique_vecs;
        pub mod exhaustive_unique_vecs_fixed_length;
        pub mod exhaustive_unique_vecs_length_inclusive_range;
        pub mod exhaustive_unique_vecs_length_range;
        pub mod exhaustive_unique_vecs_min_length;
        pub mod exhaustive_vecs;
        pub mod exhaustive_vecs_fixed_length_from_single;
        pub mod exhaustive_vecs_fixed_length_m_inputs;
        pub mod exhaustive_vecs_from_length_iterator;
        pub mod exhaustive_vecs_length_inclusive_range;
        pub mod exhaustive_vecs_length_n;
        pub mod exhaustive_vecs_length_range;
        pub mod exhaustive_vecs_min_length;
        pub mod lex_k_compositions;
        pub mod lex_ordered_unique_vecs;
        pub mod lex_ordered_unique_vecs_fixed_length;
        pub mod lex_ordered_unique_vecs_length_inclusive_range;
        pub mod lex_ordered_unique_vecs_length_range;
        pub mod lex_ordered_unique_vecs_min_length;
        pub mod lex_unique_vecs;
        pub mod lex_unique_vecs_fixed_length;
        pub mod lex_unique_vecs_length_inclusive_range;
        pub mod lex_unique_vecs_length_range;
        pub mod lex_unique_vecs_min_length;
        pub mod lex_vecs_fixed_length_from_single;
        pub mod lex_vecs_fixed_length_m_inputs;
        pub mod lex_vecs_length_n;
        pub mod next_bit_pattern;
        pub mod shortlex_ordered_unique_vecs;
        pub mod shortlex_ordered_unique_vecs_length_inclusive_range;
        pub mod shortlex_ordered_unique_vecs_length_range;
        pub mod shortlex_ordered_unique_vecs_min_length;
        pub mod shortlex_unique_vecs;
        pub mod shortlex_unique_vecs_length_inclusive_range;
        pub mod shortlex_unique_vecs_length_range;
        pub mod shortlex_unique_vecs_min_length;
        pub mod shortlex_vecs;
        pub mod shortlex_vecs_from_length_iterator;
        pub mod shortlex_vecs_length_inclusive_range;
        pub mod shortlex_vecs_length_range;
        pub mod shortlex_vecs_min_length;
    }
    pub mod exhaustive_vec_permutations;
    pub mod random {
        pub mod random_ordered_unique_vecs;
        pub mod random_ordered_unique_vecs_fixed_length;
        pub mod random_ordered_unique_vecs_from_length_iterator;
        pub mod random_ordered_unique_vecs_length_inclusive_range;
        pub mod random_ordered_unique_vecs_length_range;
        pub mod random_ordered_unique_vecs_min_length;
        pub mod random_unique_vecs;
        pub mod random_unique_vecs_fixed_length;
        pub mod random_unique_vecs_from_length_iterator;
        pub mod random_unique_vecs_length_inclusive_range;
        pub mod random_unique_vecs_length_range;
        pub mod random_unique_vecs_min_length;
        pub mod random_vecs;
        pub mod random_vecs_fixed_length;
        pub mod random_vecs_fixed_length_from_single;
        pub mod random_vecs_fixed_length_m_inputs;
        pub mod random_vecs_from_length_iterator;
        pub mod random_vecs_length_inclusive_range;
        pub mod random_vecs_length_range;
        pub mod random_vecs_min_length;
    }
    pub mod random_values_from_vec;
    pub mod random_vec_permutations;
    pub mod vec_delete_left;
    pub mod vec_from_str;
    pub mod vec_pad_left;
}
