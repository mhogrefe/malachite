extern crate malachite_test;

use malachite_test::base::char::char_to_contiguous_range::*;
use malachite_test::base::char::contiguous_range_to_char::*;
use malachite_test::base::char::decrement::*;
use malachite_test::base::char::increment::*;
use malachite_test::base::limbs::limbs_delete_left::*;
use malachite_test::base::limbs::limbs_pad_left::*;
use malachite_test::base::limbs::limbs_set_zero::*;
use malachite_test::base::limbs::limbs_test_zero::*;
use malachite_test::base::num::assign_bit::*;
use malachite_test::base::num::clear_bit::*;
use malachite_test::base::num::decrement::*;
use malachite_test::base::num::flip_bit::*;
use malachite_test::base::num::get_bit::*;
use malachite_test::base::num::increment::*;
use malachite_test::base::num::join_halves::*;
use malachite_test::base::num::log_two::*;
use malachite_test::base::num::lower_half::*;
use malachite_test::base::num::set_bit::*;
use malachite_test::base::num::significant_bits::*;
use malachite_test::base::num::split_in_half::*;
use malachite_test::base::num::upper_half::*;
use malachite_test::base::rounding_mode::clone::*;
use malachite_test::base::rounding_mode::eq::*;
use malachite_test::base::rounding_mode::hash::*;
use malachite_test::base::rounding_mode::neg::*;
use malachite_test::common::{GenerationMode, NoSpecialGenerationMode};
use malachite_test::integer::arithmetic::abs::*;
use malachite_test::integer::arithmetic::add::*;
use malachite_test::integer::arithmetic::add_i32::*;
use malachite_test::integer::arithmetic::add_u32::*;
use malachite_test::integer::arithmetic::add_mul::*;
use malachite_test::integer::arithmetic::add_mul_i32::*;
use malachite_test::integer::arithmetic::add_mul_u32::*;
use malachite_test::integer::arithmetic::divisible_by_power_of_two::*;
use malachite_test::integer::arithmetic::even_odd::*;
use malachite_test::integer::arithmetic::mod_power_of_two::*;
use malachite_test::integer::arithmetic::mul::*;
use malachite_test::integer::arithmetic::mul_i32::*;
use malachite_test::integer::arithmetic::mul_u32::*;
use malachite_test::integer::arithmetic::neg::*;
use malachite_test::integer::arithmetic::shl_i32::*;
use malachite_test::integer::arithmetic::shl_u32::*;
use malachite_test::integer::arithmetic::shr_i32::*;
use malachite_test::integer::arithmetic::shr_u32::*;
use malachite_test::integer::arithmetic::sub::*;
use malachite_test::integer::arithmetic::sub_i32::*;
use malachite_test::integer::arithmetic::sub_u32::*;
use malachite_test::integer::arithmetic::sub_mul::*;
use malachite_test::integer::arithmetic::sub_mul_i32::*;
use malachite_test::integer::arithmetic::sub_mul_u32::*;
use malachite_test::integer::basic::decrement::*;
use malachite_test::integer::basic::increment::*;
use malachite_test::integer::comparison::eq::*;
use malachite_test::integer::comparison::hash::*;
use malachite_test::integer::comparison::ord::*;
use malachite_test::integer::comparison::ord_abs::*;
use malachite_test::integer::comparison::partial_ord_abs_i32::*;
use malachite_test::integer::comparison::partial_ord_abs_natural::*;
use malachite_test::integer::comparison::partial_ord_abs_u32::*;
use malachite_test::integer::comparison::partial_ord_i32::*;
use malachite_test::integer::comparison::partial_ord_natural::*;
use malachite_test::integer::comparison::partial_ord_u32::*;
use malachite_test::integer::comparison::partial_eq_i32::*;
use malachite_test::integer::comparison::partial_eq_natural::*;
use malachite_test::integer::comparison::partial_eq_u32::*;
use malachite_test::integer::comparison::sign::*;
use malachite_test::integer::conversion::assign_i32::*;
use malachite_test::integer::conversion::assign_i64::*;
use malachite_test::integer::conversion::assign_natural::*;
use malachite_test::integer::conversion::assign_u32::*;
use malachite_test::integer::conversion::assign_u64::*;
use malachite_test::integer::conversion::clone_and_assign::*;
use malachite_test::integer::conversion::from_i32::*;
use malachite_test::integer::conversion::from_i64::*;
use malachite_test::integer::conversion::from_u32::*;
use malachite_test::integer::conversion::from_u64::*;
use malachite_test::integer::conversion::natural_assign_integer::*;
use malachite_test::integer::conversion::to_i32::*;
use malachite_test::integer::conversion::to_i64::*;
use malachite_test::integer::conversion::to_natural::*;
use malachite_test::integer::conversion::to_u32::*;
use malachite_test::integer::conversion::to_u64::*;
use malachite_test::integer::logic::assign_bit::*;
use malachite_test::integer::logic::clear_bit::*;
use malachite_test::integer::logic::flip_bit::*;
use malachite_test::integer::logic::from_sign_and_limbs::*;
use malachite_test::integer::logic::from_twos_complement_limbs::*;
use malachite_test::integer::logic::get_bit::*;
use malachite_test::integer::logic::not::*;
use malachite_test::integer::logic::set_bit::*;
use malachite_test::integer::logic::sign_and_limbs::*;
use malachite_test::integer::logic::significant_bits::*;
use malachite_test::integer::logic::trailing_zeros::*;
use malachite_test::integer::logic::twos_complement_limbs::*;
use malachite_test::natural::arithmetic::add::*;
use malachite_test::natural::arithmetic::add_u32::*;
use malachite_test::natural::arithmetic::add_mul::*;
use malachite_test::natural::arithmetic::add_mul_u32::*;
use malachite_test::natural::arithmetic::divisible_by_power_of_two::*;
use malachite_test::natural::arithmetic::even_odd::*;
use malachite_test::natural::arithmetic::is_power_of_two::*;
use malachite_test::natural::arithmetic::log_two::*;
use malachite_test::natural::arithmetic::mod_power_of_two::*;
use malachite_test::natural::arithmetic::mul::*;
use malachite_test::natural::arithmetic::mul_u32::*;
use malachite_test::natural::arithmetic::neg::*;
use malachite_test::natural::arithmetic::shl_i32::*;
use malachite_test::natural::arithmetic::shl_u32::*;
use malachite_test::natural::arithmetic::shr_i32::*;
use malachite_test::natural::arithmetic::shr_u32::*;
use malachite_test::natural::arithmetic::sub::*;
use malachite_test::natural::arithmetic::sub_u32::*;
use malachite_test::natural::arithmetic::sub_mul::*;
use malachite_test::natural::arithmetic::sub_mul_u32::*;
use malachite_test::natural::basic::decrement::*;
use malachite_test::natural::basic::increment::*;
use malachite_test::natural::comparison::eq::*;
use malachite_test::natural::comparison::hash::*;
use malachite_test::natural::comparison::ord::*;
use malachite_test::natural::comparison::partial_eq_u32::*;
use malachite_test::natural::comparison::partial_ord_u32::*;
use malachite_test::natural::conversion::assign_u32::*;
use malachite_test::natural::conversion::assign_u64::*;
use malachite_test::natural::conversion::clone_and_assign::*;
use malachite_test::natural::conversion::from_u32::*;
use malachite_test::natural::conversion::from_u64::*;
use malachite_test::natural::conversion::to_integer::*;
use malachite_test::natural::conversion::to_u32::*;
use malachite_test::natural::conversion::to_u64::*;
use malachite_test::natural::logic::assign_bit::*;
use malachite_test::natural::logic::clear_bit::*;
use malachite_test::natural::logic::flip_bit::*;
use malachite_test::natural::logic::from_limbs::*;
use malachite_test::natural::logic::get_bit::*;
use malachite_test::natural::logic::limb_count::*;
use malachite_test::natural::logic::limbs::*;
use malachite_test::natural::logic::not::*;
use malachite_test::natural::logic::set_bit::*;
use malachite_test::natural::logic::significant_bits::*;
use malachite_test::natural::logic::trailing_zeros::*;
use malachite_test::natural::random::random_natural_below::*;
use malachite_test::natural::random::random_natural_up_to_bits::*;
use malachite_test::natural::random::random_natural_with_bits::*;
use malachite_test::natural::random::special_random_natural_below::*;
use malachite_test::natural::random::special_random_natural_up_to_bits::*;
use malachite_test::natural::random::special_random_natural_with_bits::*;
use std::env;

macro_rules! demos_and_benchmarks {
    (
        [$($demo_fn: ident,)*],
        [$($special_demo_fn: ident,)*],
        [$($no_scale_bench_fn: ident,)*],
        [$($special_no_scale_bench_fn: ident,)*],
        [$($small_scale_bench_fn: ident,)*],
        [$($special_small_scale_bench_fn: ident,)*],
        [$($large_scale_bench_fn: ident,)*],
        [$($special_large_scale_bench_fn: ident,)*]
    ) => {
        fn main() {
            let args: Vec<String> = env::args().collect();
            if args.len() != 3 && args.len() != 4 {
                panic!("Usage: [exhaustive|random|special_random] [limit] [demo/bench name]");
            }
            let generation_mode = &args[1];
            assert!(
                generation_mode == "exhaustive" || generation_mode == "random" || generation_mode == "special_random",
                "Bad generation mode"
            );
            let gm_demo = match generation_mode.as_ref() {
                "exhaustive" => NoSpecialGenerationMode::Exhaustive,
                "random" | "special_random" => NoSpecialGenerationMode::Random(32),
                _ => unreachable!(),
            };
            let gm_small = match generation_mode.as_ref() {
                "exhaustive" => NoSpecialGenerationMode::Exhaustive,
                "random" | "special_random" => NoSpecialGenerationMode::Random(128),
                _ => unreachable!(),
            };
            let gm_large = match generation_mode.as_ref() {
                "exhaustive" => NoSpecialGenerationMode::Exhaustive,
                "random" | "special_random" => NoSpecialGenerationMode::Random(2048),
                _ => unreachable!(),
            };
            let sgm_demo = match generation_mode.as_ref() {
                "exhaustive" => GenerationMode::Exhaustive,
                "random" => GenerationMode::Random(32),
                "special_random" => GenerationMode::SpecialRandom(32),
                _ => unreachable!(),
            };
            let sgm_small = match generation_mode.as_ref() {
                "exhaustive" => GenerationMode::Exhaustive,
                "random" => GenerationMode::Random(128),
                "special_random" => GenerationMode::SpecialRandom(128),
                _ => unreachable!(),
            };
            let sgm_large = match generation_mode.as_ref() {
                "exhaustive" => GenerationMode::Exhaustive,
                "random" => GenerationMode::Random(2048),
                "special_random" => GenerationMode::SpecialRandom(2048),
                _ => unreachable!(),
            };
            let limit = if args.len() == 4 {
                args[2].parse().unwrap()
            } else {
                usize::max_value()
            };
            let item_name = &*args.last().unwrap();
            match item_name.as_ref() {
                $(stringify!($demo_fn) => $demo_fn(gm_demo, limit)),*,
                $(stringify!($special_demo_fn) => $special_demo_fn(sgm_demo, limit)),*,
                $(
                    stringify!($no_scale_bench_fn) => {
                        $no_scale_bench_fn(gm_small, limit, "temp.gp")
                    }
                ),*,
                $(
                    stringify!($special_no_scale_bench_fn) => {
                        $special_no_scale_bench_fn(sgm_small, limit, "temp.gp")
                    }
                ),*,
                $(
                    stringify!($small_scale_bench_fn) => {
                        $small_scale_bench_fn(gm_small, limit, "temp.gp")
                    }
                ),*
                $(
                    stringify!($special_small_scale_bench_fn) => {
                        $special_small_scale_bench_fn(sgm_small, limit, "temp.gp")
                    }
                ),*
                $(
                    stringify!($large_scale_bench_fn) => {
                        $large_scale_bench_fn(gm_large, limit, "temp.gp")
                    }
                ),*,
                $(
                    stringify!($special_large_scale_bench_fn) => {
                        $special_large_scale_bench_fn(sgm_large, limit, "temp.gp")
                    }
                ),*
                "all" => {
                    $(
                        $no_scale_bench_fn(
                            NoSpecialGenerationMode::Exhaustive,
                            limit,
                            &format!("exhaustive_{}.gp", stringify!($no_scale_bench_fn))
                        );
                        $no_scale_bench_fn(
                            NoSpecialGenerationMode::Random(32),
                            limit,
                            &format!("random_{}.gp", stringify!($no_scale_bench_fn))
                        );
                    )*
                    $(
                        $special_no_scale_bench_fn(
                            GenerationMode::Exhaustive,
                            limit,
                            &format!("exhaustive_{}.gp", stringify!($special_no_scale_bench_fn))
                        );
                        $special_no_scale_bench_fn(
                            GenerationMode::Random(32),
                            limit,
                            &format!("random_{}.gp", stringify!($special_no_scale_bench_fn))
                        );
                        $special_no_scale_bench_fn(
                            GenerationMode::SpecialRandom(32),
                            limit,
                            &format!("special_random_{}.gp", stringify!($special_no_scale_bench_fn))
                        );
                    )*
                    $(
                        $small_scale_bench_fn(
                            NoSpecialGenerationMode::Exhaustive,
                            limit,
                            &format!("exhaustive_{}.gp", stringify!($small_scale_bench_fn))
                        );
                        $small_scale_bench_fn(
                            NoSpecialGenerationMode::Random(128),
                            limit,
                            &format!("random_{}.gp", stringify!($small_scale_bench_fn))
                        );
                    )*
                    $(
                        $special_small_scale_bench_fn(
                            GenerationMode::Exhaustive,
                            limit,
                            &format!("exhaustive_{}.gp", stringify!($special_small_scale_bench_fn))
                        );
                        $special_small_scale_bench_fn(
                            GenerationMode::Random(128),
                            limit,
                            &format!("random_{}.gp", stringify!($special_small_scale_bench_fn))
                        );
                        $special_small_scale_bench_fn(
                            GenerationMode::SpecialRandom(128),
                            limit,
                            &format!("special_random_{}.gp", stringify!($special_small_scale_bench_fn))
                        );
                    )*
                    $(
                        $large_scale_bench_fn(
                            NoSpecialGenerationMode::Exhaustive,
                            limit,
                            &format!("exhaustive_{}.gp", stringify!($large_scale_bench_fn))
                        );
                        $large_scale_bench_fn(
                            NoSpecialGenerationMode::Random(2048),
                            limit,
                            &format!("random_{}.gp", stringify!($large_scale_bench_fn))
                        );
                    )*
                    $(
                        $special_large_scale_bench_fn(
                            GenerationMode::Exhaustive,
                            limit,
                            &format!("exhaustive_{}.gp", stringify!($special_large_scale_bench_fn))
                        );
                        $special_large_scale_bench_fn(
                            GenerationMode::Random(2048),
                            limit,
                            &format!("random_{}.gp", stringify!($special_large_scale_bench_fn))
                        );
                        $special_large_scale_bench_fn(
                            GenerationMode::SpecialRandom(2048),
                            limit,
                            &format!("special_random_{}.gp", stringify!($special_large_scale_bench_fn))
                        );
                    )*
                }
                _ => panic!("Invalid demo/bench name: {}", item_name),
            }
        }
    }
}

demos_and_benchmarks!(
    [
        demo_char_to_contiguous_range,
        demo_char_decrement,
        demo_char_increment,
        demo_rounding_mode_clone,
        demo_rounding_mode_clone_from,
        demo_rounding_mode_eq,
        demo_rounding_mode_hash,
        demo_rounding_mode_neg,
        demo_natural_random_natural_up_to_bits,
        demo_natural_random_natural_with_bits,
        demo_natural_special_random_natural_up_to_bits,
        demo_natural_special_random_natural_with_bits,
    ],
    [
        demo_contiguous_range_to_char,
        demo_limbs_ceiling_log_two,
        demo_limbs_delete_left,
        demo_limbs_floor_log_two,
        demo_limbs_pad_left,
        demo_limbs_set_zero,
        demo_limbs_significant_bits,
        demo_limbs_test_zero,
        demo_u8_assign_bit,
        demo_u16_assign_bit,
        demo_u32_assign_bit,
        demo_u64_assign_bit,
        demo_i8_assign_bit,
        demo_i16_assign_bit,
        demo_i32_assign_bit,
        demo_i64_assign_bit,
        demo_u8_ceiling_log_two,
        demo_u16_ceiling_log_two,
        demo_u32_ceiling_log_two,
        demo_u64_ceiling_log_two,
        demo_u8_clear_bit,
        demo_u16_clear_bit,
        demo_u32_clear_bit,
        demo_u64_clear_bit,
        demo_i8_clear_bit,
        demo_i16_clear_bit,
        demo_i32_clear_bit,
        demo_i64_clear_bit,
        demo_u8_decrement,
        demo_u16_decrement,
        demo_u32_decrement,
        demo_u64_decrement,
        demo_i8_decrement,
        demo_i16_decrement,
        demo_i32_decrement,
        demo_i64_decrement,
        demo_u8_flip_bit,
        demo_u16_flip_bit,
        demo_u32_flip_bit,
        demo_u64_flip_bit,
        demo_i8_flip_bit,
        demo_i16_flip_bit,
        demo_i32_flip_bit,
        demo_i64_flip_bit,
        demo_u8_floor_log_two,
        demo_u16_floor_log_two,
        demo_u32_floor_log_two,
        demo_u64_floor_log_two,
        demo_u8_get_bit,
        demo_u16_get_bit,
        demo_u32_get_bit,
        demo_u64_get_bit,
        demo_i8_get_bit,
        demo_i16_get_bit,
        demo_i32_get_bit,
        demo_i64_get_bit,
        demo_u8_increment,
        demo_u16_increment,
        demo_u32_increment,
        demo_u64_increment,
        demo_i8_increment,
        demo_i16_increment,
        demo_i32_increment,
        demo_i64_increment,
        demo_u16_join_halves,
        demo_u32_join_halves,
        demo_u64_join_halves,
        demo_u16_lower_half,
        demo_u32_lower_half,
        demo_u64_lower_half,
        demo_u8_set_bit,
        demo_u16_set_bit,
        demo_u32_set_bit,
        demo_u64_set_bit,
        demo_i8_set_bit,
        demo_i16_set_bit,
        demo_i32_set_bit,
        demo_i64_set_bit,
        demo_u8_significant_bits,
        demo_u16_significant_bits,
        demo_u32_significant_bits,
        demo_u64_significant_bits,
        demo_i8_significant_bits,
        demo_i16_significant_bits,
        demo_i32_significant_bits,
        demo_i64_significant_bits,
        demo_u16_split_in_half,
        demo_u32_split_in_half,
        demo_u64_split_in_half,
        demo_u16_upper_half,
        demo_u32_upper_half,
        demo_u64_upper_half,
        demo_integer_abs_assign,
        demo_integer_abs,
        demo_integer_abs_ref,
        demo_integer_natural_abs,
        demo_integer_natural_abs_ref,
        demo_integer_add_assign,
        demo_integer_add_assign_ref,
        demo_integer_add,
        demo_integer_add_val_ref,
        demo_integer_add_ref_val,
        demo_integer_add_ref_ref,
        demo_integer_add_assign_i32,
        demo_integer_add_i32,
        demo_integer_add_i32_ref,
        demo_i32_add_integer,
        demo_i32_add_integer_ref,
        demo_integer_add_assign_u32,
        demo_integer_add_u32,
        demo_integer_add_u32_ref,
        demo_u32_add_integer,
        demo_u32_add_integer_ref,
        demo_integer_add_mul_assign,
        demo_integer_add_mul_assign_val_ref,
        demo_integer_add_mul_assign_ref_val,
        demo_integer_add_mul_assign_ref_ref,
        demo_integer_add_mul,
        demo_integer_add_mul_val_val_ref,
        demo_integer_add_mul_val_ref_val,
        demo_integer_add_mul_val_ref_ref,
        demo_integer_add_mul_ref_ref_ref,
        demo_integer_add_mul_assign_i32,
        demo_integer_add_mul_assign_i32_ref,
        demo_integer_add_mul_i32,
        demo_integer_add_mul_i32_val_ref,
        demo_integer_add_mul_i32_ref_val,
        demo_integer_add_mul_i32_ref_ref,
        demo_integer_add_mul_assign_u32,
        demo_integer_add_mul_assign_u32_ref,
        demo_integer_add_mul_u32,
        demo_integer_add_mul_u32_val_ref,
        demo_integer_add_mul_u32_ref_val,
        demo_integer_add_mul_u32_ref_ref,
        demo_integer_assign,
        demo_integer_assign_ref,
        demo_integer_assign_i32,
        demo_integer_assign_i64,
        demo_integer_assign_natural,
        demo_integer_assign_natural_ref,
        demo_integer_assign_u32,
        demo_integer_assign_u64,
        demo_integer_assign_bit,
        demo_integer_clear_bit,
        demo_integer_clone,
        demo_integer_clone_from,
        demo_integer_cmp,
        demo_integer_cmp_abs,
        demo_integer_decrement,
        demo_integer_divisible_by_power_of_two,
        demo_integer_eq,
        demo_integer_flip_bit,
        demo_integer_from_i32,
        demo_integer_from_i64,
        demo_integer_from_u32,
        demo_integer_from_u64,
        demo_integer_from_sign_and_limbs_asc,
        demo_integer_from_sign_and_limbs_desc,
        demo_integer_from_twos_complement_limbs_asc,
        demo_integer_from_twos_complement_limbs_desc,
        demo_integer_get_bit,
        demo_integer_hash,
        demo_integer_increment,
        demo_integer_is_even,
        demo_integer_is_odd,
        demo_integer_mod_power_of_two_assign,
        demo_integer_mod_power_of_two,
        demo_integer_mod_power_of_two_ref,
        demo_integer_rem_power_of_two_assign,
        demo_integer_rem_power_of_two,
        demo_integer_rem_power_of_two_ref,
        demo_integer_ceiling_mod_power_of_two_assign,
        demo_integer_ceiling_mod_power_of_two,
        demo_integer_ceiling_mod_power_of_two_ref,
        demo_integer_mul_assign,
        demo_integer_mul_assign_ref,
        demo_integer_mul,
        demo_integer_mul_val_ref,
        demo_integer_mul_ref_val,
        demo_integer_mul_ref_ref,
        demo_integer_mul_assign_i32,
        demo_integer_mul_i32,
        demo_integer_mul_i32_ref,
        demo_i32_mul_integer,
        demo_i32_mul_integer_ref,
        demo_integer_mul_assign_u32,
        demo_integer_mul_u32,
        demo_integer_mul_u32_ref,
        demo_u32_mul_integer,
        demo_u32_mul_integer_ref,
        demo_integer_neg_assign,
        demo_integer_neg,
        demo_integer_neg_ref,
        demo_integer_not_assign,
        demo_integer_not,
        demo_integer_not_ref,
        demo_integer_partial_cmp_abs_i32,
        demo_i32_partial_cmp_abs_integer,
        demo_integer_partial_cmp_abs_u32,
        demo_u32_partial_cmp_abs_integer,
        demo_integer_partial_cmp_abs_natural,
        demo_integer_partial_cmp_i32,
        demo_i32_partial_cmp_integer,
        demo_integer_partial_cmp_u32,
        demo_u32_partial_cmp_integer,
        demo_integer_partial_cmp_natural,
        demo_integer_partial_eq_i32,
        demo_i32_partial_eq_integer,
        demo_integer_partial_eq_u32,
        demo_u32_partial_eq_integer,
        demo_integer_partial_eq_natural,
        demo_integer_set_bit,
        demo_integer_shl_assign_i32,
        demo_integer_shl_i32,
        demo_integer_shl_i32_ref,
        demo_integer_shl_round_assign_i32,
        demo_integer_shl_round_i32,
        demo_integer_shl_round_i32_ref,
        demo_integer_shl_assign_u32,
        demo_integer_shl_u32,
        demo_integer_shl_u32_ref,
        demo_integer_shr_assign_i32,
        demo_integer_shr_i32,
        demo_integer_shr_i32_ref,
        demo_integer_shr_round_assign_i32,
        demo_integer_shr_round_i32,
        demo_integer_shr_round_i32_ref,
        demo_integer_shr_assign_u32,
        demo_integer_shr_u32,
        demo_integer_shr_u32_ref,
        demo_integer_shr_round_assign_u32,
        demo_integer_shr_round_u32,
        demo_integer_shr_round_u32_ref,
        demo_integer_sign,
        demo_integer_sign_and_limbs_asc,
        demo_integer_sign_and_limbs_desc,
        demo_integer_significant_bits,
        demo_integer_sub_assign,
        demo_integer_sub_assign_ref,
        demo_integer_sub,
        demo_integer_sub_val_ref,
        demo_integer_sub_ref_val,
        demo_integer_sub_ref_ref,
        demo_integer_sub_assign_i32,
        demo_integer_sub_i32,
        demo_integer_sub_i32_ref,
        demo_i32_sub_integer,
        demo_i32_sub_integer_ref,
        demo_integer_sub_assign_u32,
        demo_integer_sub_u32,
        demo_integer_sub_u32_ref,
        demo_u32_sub_integer,
        demo_u32_sub_integer_ref,
        demo_integer_sub_mul_assign,
        demo_integer_sub_mul_assign_val_ref,
        demo_integer_sub_mul_assign_ref_val,
        demo_integer_sub_mul_assign_ref_ref,
        demo_integer_sub_mul,
        demo_integer_sub_mul_val_val_ref,
        demo_integer_sub_mul_val_ref_val,
        demo_integer_sub_mul_val_ref_ref,
        demo_integer_sub_mul_ref_ref_ref,
        demo_integer_sub_mul_assign_i32,
        demo_integer_sub_mul_assign_i32_ref,
        demo_integer_sub_mul_i32,
        demo_integer_sub_mul_i32_val_ref,
        demo_integer_sub_mul_i32_ref_val,
        demo_integer_sub_mul_i32_ref_ref,
        demo_integer_sub_mul_assign_u32,
        demo_integer_sub_mul_assign_u32_ref,
        demo_integer_sub_mul_u32,
        demo_integer_sub_mul_u32_val_ref,
        demo_integer_sub_mul_u32_ref_val,
        demo_integer_sub_mul_u32_ref_ref,
        demo_integer_to_i32,
        demo_integer_to_i32_wrapping,
        demo_integer_to_i64,
        demo_integer_to_i64_wrapping,
        demo_integer_into_natural,
        demo_integer_to_natural,
        demo_integer_to_u32,
        demo_integer_to_u32_wrapping,
        demo_integer_to_u64,
        demo_integer_to_u64_wrapping,
        demo_integer_trailing_zeros,
        demo_integer_twos_complement_limbs_asc,
        demo_integer_twos_complement_limbs_desc,
        demo_natural_add_assign,
        demo_natural_add_assign_ref,
        demo_natural_add,
        demo_natural_add_val_ref,
        demo_natural_add_ref_val,
        demo_natural_add_ref_ref,
        demo_natural_add_assign_u32,
        demo_natural_add_u32,
        demo_natural_add_u32_ref,
        demo_u32_add_natural,
        demo_u32_add_natural_ref,
        demo_natural_add_mul_assign,
        demo_natural_add_mul_assign_val_ref,
        demo_natural_add_mul_assign_ref_val,
        demo_natural_add_mul_assign_ref_ref,
        demo_natural_add_mul,
        demo_natural_add_mul_val_val_ref,
        demo_natural_add_mul_val_ref_val,
        demo_natural_add_mul_val_ref_ref,
        demo_natural_add_mul_ref_ref_ref,
        demo_natural_add_mul_assign_u32,
        demo_natural_add_mul_assign_u32_ref,
        demo_natural_add_mul_u32,
        demo_natural_add_mul_u32_val_ref,
        demo_natural_add_mul_u32_ref_val,
        demo_natural_add_mul_u32_ref_ref,
        demo_natural_assign,
        demo_natural_assign_ref,
        demo_natural_assign_integer,
        demo_natural_assign_integer_ref,
        demo_natural_assign_u32,
        demo_natural_assign_u64,
        demo_natural_assign_bit,
        demo_natural_ceiling_log_two,
        demo_natural_clear_bit,
        demo_natural_clone,
        demo_natural_clone_from,
        demo_natural_cmp,
        demo_natural_decrement,
        demo_natural_divisible_by_power_of_two,
        demo_natural_eq,
        demo_natural_flip_bit,
        demo_natural_floor_log_two,
        demo_natural_from_limbs_asc,
        demo_natural_from_limbs_desc,
        demo_natural_from_u32,
        demo_natural_from_u64,
        demo_natural_get_bit,
        demo_natural_hash,
        demo_natural_increment,
        demo_natural_is_even,
        demo_natural_is_odd,
        demo_natural_is_power_of_two,
        demo_natural_limb_count,
        demo_natural_to_limbs_asc,
        demo_natural_to_limbs_desc,
        demo_natural_mod_power_of_two_assign,
        demo_natural_mod_power_of_two,
        demo_natural_mod_power_of_two_ref,
        demo_natural_neg_mod_power_of_two_assign,
        demo_natural_neg_mod_power_of_two,
        demo_natural_neg_mod_power_of_two_ref,
        demo_natural_mul_assign,
        demo_natural_mul_assign_ref,
        demo_natural_mul,
        demo_natural_mul_val_ref,
        demo_natural_mul_ref_val,
        demo_natural_mul_ref_ref,
        demo_natural_mul_assign_u32,
        demo_natural_mul_u32,
        demo_natural_mul_u32_ref,
        demo_u32_mul_natural,
        demo_u32_mul_natural_ref,
        demo_natural_neg,
        demo_natural_neg_ref,
        demo_natural_not,
        demo_natural_not_ref,
        demo_natural_partial_eq_u32,
        demo_u32_partial_eq_natural,
        demo_natural_partial_eq_integer,
        demo_natural_partial_cmp_abs_integer,
        demo_natural_partial_cmp_integer,
        demo_natural_partial_cmp_u32,
        demo_u32_partial_cmp_natural,
        demo_natural_random_natural_below,
        demo_natural_set_bit,
        demo_natural_shl_assign_i32,
        demo_natural_shl_i32,
        demo_natural_shl_i32_ref,
        demo_natural_shl_round_assign_i32,
        demo_natural_shl_round_i32,
        demo_natural_shl_round_i32_ref,
        demo_natural_shl_assign_u32,
        demo_natural_shl_u32,
        demo_natural_shl_u32_ref,
        demo_natural_shr_assign_i32,
        demo_natural_shr_i32,
        demo_natural_shr_i32_ref,
        demo_natural_shr_round_assign_i32,
        demo_natural_shr_round_i32,
        demo_natural_shr_round_i32_ref,
        demo_natural_shr_assign_u32,
        demo_natural_shr_u32,
        demo_natural_shr_u32_ref,
        demo_natural_shr_round_assign_u32,
        demo_natural_shr_round_u32,
        demo_natural_shr_round_u32_ref,
        demo_natural_significant_bits,
        demo_natural_special_random_natural_below,
        demo_natural_sub_assign,
        demo_natural_sub,
        demo_natural_sub_ref_ref,
        demo_natural_sub_assign_u32,
        demo_natural_sub_u32,
        demo_natural_sub_u32_ref,
        demo_u32_sub_natural,
        demo_natural_sub_mul_assign,
        demo_natural_sub_mul,
        demo_natural_sub_mul_ref,
        demo_natural_sub_mul_assign_u32,
        demo_natural_sub_mul_u32,
        demo_natural_sub_mul_u32_ref,
        demo_natural_into_integer,
        demo_natural_to_integer,
        demo_natural_to_u32,
        demo_natural_to_u32_wrapping,
        demo_natural_to_u64,
        demo_natural_to_u64_wrapping,
        demo_natural_trailing_zeros,
    ],
    [benchmark_char_to_contiguous_range,],
    [
        benchmark_contiguous_range_to_char,
        benchmark_u8_assign_bit,
        benchmark_u16_assign_bit,
        benchmark_u32_assign_bit,
        benchmark_u64_assign_bit,
        benchmark_i8_assign_bit,
        benchmark_i16_assign_bit,
        benchmark_i32_assign_bit,
        benchmark_i64_assign_bit,
        benchmark_u8_ceiling_log_two,
        benchmark_u16_ceiling_log_two,
        benchmark_u32_ceiling_log_two,
        benchmark_u64_ceiling_log_two,
        benchmark_u8_decrement,
        benchmark_u16_decrement,
        benchmark_u32_decrement,
        benchmark_u64_decrement,
        benchmark_i8_decrement,
        benchmark_i16_decrement,
        benchmark_i32_decrement,
        benchmark_i64_decrement,
        benchmark_u8_flip_bit,
        benchmark_u16_flip_bit,
        benchmark_u32_flip_bit,
        benchmark_u64_flip_bit,
        benchmark_i8_flip_bit,
        benchmark_i16_flip_bit,
        benchmark_i32_flip_bit,
        benchmark_i64_flip_bit,
        benchmark_u8_floor_log_two,
        benchmark_u16_floor_log_two,
        benchmark_u32_floor_log_two,
        benchmark_u64_floor_log_two,
        benchmark_u8_get_bit,
        benchmark_u16_get_bit,
        benchmark_u32_get_bit,
        benchmark_u64_get_bit,
        benchmark_i8_get_bit,
        benchmark_i16_get_bit,
        benchmark_i32_get_bit,
        benchmark_i64_get_bit,
        benchmark_u8_increment,
        benchmark_u16_increment,
        benchmark_u32_increment,
        benchmark_u64_increment,
        benchmark_i8_increment,
        benchmark_i16_increment,
        benchmark_i32_increment,
        benchmark_i64_increment,
        benchmark_u16_join_halves,
        benchmark_u32_join_halves,
        benchmark_u64_join_halves,
        benchmark_u8_set_bit,
        benchmark_u16_set_bit,
        benchmark_u32_set_bit,
        benchmark_u64_set_bit,
        benchmark_i8_set_bit,
        benchmark_i16_set_bit,
        benchmark_i32_set_bit,
        benchmark_i64_set_bit,
        benchmark_u8_clear_bit,
        benchmark_u16_clear_bit,
        benchmark_u32_clear_bit,
        benchmark_u64_clear_bit,
        benchmark_i8_clear_bit,
        benchmark_i16_clear_bit,
        benchmark_i32_clear_bit,
        benchmark_i64_clear_bit,
        benchmark_u16_lower_half,
        benchmark_u32_lower_half,
        benchmark_u64_lower_half,
        benchmark_u8_significant_bits,
        benchmark_u16_significant_bits,
        benchmark_u32_significant_bits,
        benchmark_u64_significant_bits,
        benchmark_i8_significant_bits,
        benchmark_i16_significant_bits,
        benchmark_i32_significant_bits,
        benchmark_i64_significant_bits,
        benchmark_u16_split_in_half,
        benchmark_u32_split_in_half,
        benchmark_u64_split_in_half,
        benchmark_u16_upper_half,
        benchmark_u32_upper_half,
        benchmark_u64_upper_half,
        benchmark_integer_from_i32,
        benchmark_integer_from_i64,
        benchmark_integer_from_u32,
        benchmark_integer_from_u64,
        benchmark_integer_to_i32,
        benchmark_integer_to_i32_wrapping,
        benchmark_integer_to_i64,
        benchmark_integer_to_i64_wrapping,
        benchmark_integer_to_u32,
        benchmark_integer_to_u32_wrapping,
        benchmark_integer_to_u64,
        benchmark_integer_to_u64_wrapping,
        benchmark_natural_from_u32,
        benchmark_natural_from_u64,
        benchmark_natural_to_u32,
        benchmark_natural_to_u32_wrapping,
        benchmark_natural_to_u64,
        benchmark_natural_to_u64_wrapping,
    ],
    [],
    [
        benchmark_limbs_ceiling_log_two,
        benchmark_limbs_delete_left,
        benchmark_limbs_floor_log_two,
        benchmark_limbs_pad_left,
        benchmark_limbs_set_zero,
        benchmark_limbs_significant_bits,
        benchmark_limbs_test_zero,
        benchmark_integer_from_sign_and_limbs_asc,
        benchmark_integer_from_sign_and_limbs_desc,
        benchmark_integer_from_twos_complement_limbs_asc,
        benchmark_integer_from_twos_complement_limbs_desc,
        benchmark_natural_from_limbs_asc,
        benchmark_natural_from_limbs_desc,
    ],
    [
        benchmark_natural_random_natural_up_to_bits,
        benchmark_natural_random_natural_with_bits,
        benchmark_natural_special_random_natural_up_to_bits,
        benchmark_natural_special_random_natural_with_bits,
    ],
    [
        benchmark_integer_abs_assign,
        benchmark_integer_abs_evaluation_strategy,
        benchmark_integer_natural_abs,
        benchmark_integer_natural_abs_evaluation_strategy,
        benchmark_integer_add_assign,
        benchmark_integer_add_assign_evaluation_strategy,
        benchmark_integer_add_evaluation_strategy,
        benchmark_integer_add_assign_i32,
        benchmark_integer_add_i32,
        benchmark_integer_add_i32_evaluation_strategy,
        benchmark_i32_add_integer,
        benchmark_i32_add_integer_evaluation_strategy,
        benchmark_integer_add_assign_u32,
        benchmark_integer_add_u32,
        benchmark_integer_add_u32_evaluation_strategy,
        benchmark_u32_add_integer,
        benchmark_u32_add_integer_evaluation_strategy,
        benchmark_integer_add_mul_assign,
        benchmark_integer_add_mul_assign_evaluation_strategy,
        benchmark_integer_add_mul_assign_algorithms,
        benchmark_integer_add_mul_assign_val_ref_algorithms,
        benchmark_integer_add_mul_assign_ref_val_algorithms,
        benchmark_integer_add_mul_assign_ref_ref_algorithms,
        benchmark_integer_add_mul,
        benchmark_integer_add_mul_evaluation_strategy,
        benchmark_integer_add_mul_algorithms,
        benchmark_integer_add_mul_val_val_ref_algorithms,
        benchmark_integer_add_mul_val_ref_val_algorithms,
        benchmark_integer_add_mul_val_ref_ref_algorithms,
        benchmark_integer_add_mul_ref_ref_ref_algorithms,
        benchmark_integer_add_mul_assign_i32,
        benchmark_integer_add_mul_assign_i32_evaluation_strategy,
        benchmark_integer_add_mul_assign_i32_algorithms,
        benchmark_integer_add_mul_assign_i32_ref_algorithms,
        benchmark_integer_add_mul_i32,
        benchmark_integer_add_mul_i32_evaluation_strategy,
        benchmark_integer_add_mul_i32_algorithms,
        benchmark_integer_add_mul_i32_val_ref_algorithms,
        benchmark_integer_add_mul_i32_ref_val_algorithms,
        benchmark_integer_add_mul_i32_ref_ref_algorithms,
        benchmark_integer_add_mul_assign_u32,
        benchmark_integer_add_mul_assign_u32_evaluation_strategy,
        benchmark_integer_add_mul_assign_u32_algorithms,
        benchmark_integer_add_mul_assign_u32_ref_algorithms,
        benchmark_integer_add_mul_u32,
        benchmark_integer_add_mul_u32_evaluation_strategy,
        benchmark_integer_add_mul_u32_algorithms,
        benchmark_integer_add_mul_u32_val_ref_algorithms,
        benchmark_integer_add_mul_u32_ref_val_algorithms,
        benchmark_integer_add_mul_u32_ref_ref_algorithms,
        benchmark_integer_assign_evaluation_strategy,
        benchmark_integer_assign_i32,
        benchmark_integer_assign_i64,
        benchmark_integer_assign_natural,
        benchmark_integer_assign_natural_evaluation_strategy,
        benchmark_integer_assign_u32,
        benchmark_integer_assign_u64,
        benchmark_integer_assign_bit,
        benchmark_integer_clear_bit,
        benchmark_integer_clone_from,
        benchmark_integer_cmp_abs,
        benchmark_integer_decrement,
        benchmark_integer_flip_bit,
        benchmark_integer_get_bit,
        benchmark_integer_increment,
        benchmark_integer_is_even,
        benchmark_integer_mod_power_of_two_assign,
        benchmark_integer_mod_power_of_two,
        benchmark_integer_mod_power_of_two_ref,
        benchmark_integer_rem_power_of_two_assign,
        benchmark_integer_rem_power_of_two,
        benchmark_integer_rem_power_of_two_ref,
        benchmark_integer_ceiling_mod_power_of_two_assign,
        benchmark_integer_ceiling_mod_power_of_two,
        benchmark_integer_ceiling_mod_power_of_two_ref,
        benchmark_integer_mul_assign,
        benchmark_integer_mul_assign_evaluation_strategy,
        benchmark_integer_mul_evaluation_strategy,
        benchmark_integer_mul_assign_i32,
        benchmark_integer_mul_i32,
        benchmark_integer_mul_i32_evaluation_strategy,
        benchmark_i32_mul_integer,
        benchmark_i32_mul_integer_evaluation_strategy,
        benchmark_integer_mul_assign_u32,
        benchmark_integer_mul_u32,
        benchmark_integer_mul_u32_evaluation_strategy,
        benchmark_u32_mul_integer,
        benchmark_u32_mul_integer_evaluation_strategy,
        benchmark_integer_neg_assign,
        benchmark_integer_neg_evaluation_strategy,
        benchmark_integer_not_assign,
        benchmark_integer_not_evaluation_strategy,
        benchmark_integer_partial_cmp_abs_i32,
        benchmark_i32_partial_cmp_abs_integer,
        benchmark_integer_partial_cmp_abs_u32,
        benchmark_u32_partial_cmp_abs_integer,
        benchmark_integer_partial_cmp_abs_natural,
        benchmark_integer_partial_cmp_i32,
        benchmark_i32_partial_cmp_integer,
        benchmark_integer_partial_cmp_u32,
        benchmark_u32_partial_cmp_integer,
        benchmark_integer_partial_cmp_natural,
        benchmark_integer_partial_eq_i32,
        benchmark_i32_partial_eq_integer,
        benchmark_integer_partial_eq_u32,
        benchmark_u32_partial_eq_integer,
        benchmark_integer_partial_eq_natural,
        benchmark_integer_set_bit,
        benchmark_integer_shl_assign_i32,
        benchmark_integer_shl_i32,
        benchmark_integer_shl_i32_ref,
        benchmark_integer_shl_round_assign_i32,
        benchmark_integer_shl_round_i32,
        benchmark_integer_shl_round_i32_ref,
        benchmark_integer_shl_assign_u32,
        benchmark_integer_shl_u32,
        benchmark_integer_shl_u32_ref,
        benchmark_integer_shr_assign_i32,
        benchmark_integer_shr_i32,
        benchmark_integer_shr_i32_ref,
        benchmark_integer_shr_round_assign_i32,
        benchmark_integer_shr_round_i32,
        benchmark_integer_shr_round_i32_ref,
        benchmark_integer_shr_assign_u32,
        benchmark_integer_shr_u32,
        benchmark_integer_shr_u32_ref,
        benchmark_integer_shr_round_assign_u32,
        benchmark_integer_shr_round_u32,
        benchmark_integer_shr_round_u32_ref,
        benchmark_integer_sign_and_limbs_asc,
        benchmark_integer_sign_and_limbs_desc,
        benchmark_integer_significant_bits,
        benchmark_integer_sub_assign,
        benchmark_integer_sub_assign_evaluation_strategy,
        benchmark_integer_sub_evaluation_strategy,
        benchmark_integer_sub_assign_i32,
        benchmark_integer_sub_i32,
        benchmark_integer_sub_i32_evaluation_strategy,
        benchmark_i32_sub_integer,
        benchmark_i32_sub_integer_evaluation_strategy,
        benchmark_integer_sub_assign_u32,
        benchmark_integer_sub_u32,
        benchmark_integer_sub_u32_evaluation_strategy,
        benchmark_u32_sub_integer,
        benchmark_u32_sub_integer_evaluation_strategy,
        benchmark_integer_sub_mul_assign,
        benchmark_integer_sub_mul_assign_evaluation_strategy,
        benchmark_integer_sub_mul_assign_algorithms,
        benchmark_integer_sub_mul_assign_val_ref_algorithms,
        benchmark_integer_sub_mul_assign_ref_val_algorithms,
        benchmark_integer_sub_mul_assign_ref_ref_algorithms,
        benchmark_integer_sub_mul,
        benchmark_integer_sub_mul_evaluation_strategy,
        benchmark_integer_sub_mul_algorithms,
        benchmark_integer_sub_mul_val_val_ref_algorithms,
        benchmark_integer_sub_mul_val_ref_val_algorithms,
        benchmark_integer_sub_mul_val_ref_ref_algorithms,
        benchmark_integer_sub_mul_ref_ref_ref_algorithms,
        benchmark_integer_sub_mul_assign_i32,
        benchmark_integer_sub_mul_assign_i32_evaluation_strategy,
        benchmark_integer_sub_mul_assign_i32_algorithms,
        benchmark_integer_sub_mul_assign_i32_ref_algorithms,
        benchmark_integer_sub_mul_i32,
        benchmark_integer_sub_mul_i32_evaluation_strategy,
        benchmark_integer_sub_mul_i32_algorithms,
        benchmark_integer_sub_mul_i32_val_ref_algorithms,
        benchmark_integer_sub_mul_i32_ref_val_algorithms,
        benchmark_integer_sub_mul_i32_ref_ref_algorithms,
        benchmark_integer_sub_mul_assign_u32,
        benchmark_integer_sub_mul_assign_u32_evaluation_strategy,
        benchmark_integer_sub_mul_assign_u32_algorithms,
        benchmark_integer_sub_mul_assign_u32_ref_algorithms,
        benchmark_integer_sub_mul_u32,
        benchmark_integer_sub_mul_u32_evaluation_strategy,
        benchmark_integer_sub_mul_u32_algorithms,
        benchmark_integer_sub_mul_u32_val_ref_algorithms,
        benchmark_integer_sub_mul_u32_ref_val_algorithms,
        benchmark_integer_sub_mul_u32_ref_ref_algorithms,
        benchmark_integer_to_natural,
        benchmark_integer_to_natural_evaluation_strategy,
        benchmark_integer_trailing_zeros,
        benchmark_integer_twos_complement_limbs_asc,
        benchmark_integer_twos_complement_limbs_desc,
        benchmark_natural_add_assign,
        benchmark_natural_add_assign_evaluation_strategy,
        benchmark_natural_add_evaluation_strategy,
        benchmark_natural_add_assign_u32,
        benchmark_natural_add_u32,
        benchmark_natural_add_u32_evaluation_strategy,
        benchmark_u32_add_natural,
        benchmark_u32_add_natural_evaluation_strategy,
        benchmark_natural_add_mul_assign,
        benchmark_natural_add_mul_assign_evaluation_strategy,
        benchmark_natural_add_mul_assign_algorithms,
        benchmark_natural_add_mul_assign_val_ref_algorithms,
        benchmark_natural_add_mul_assign_ref_val_algorithms,
        benchmark_natural_add_mul_assign_ref_ref_algorithms,
        benchmark_natural_add_mul,
        benchmark_natural_add_mul_evaluation_strategy,
        benchmark_natural_add_mul_algorithms,
        benchmark_natural_add_mul_val_val_ref_algorithms,
        benchmark_natural_add_mul_val_ref_val_algorithms,
        benchmark_natural_add_mul_val_ref_ref_algorithms,
        benchmark_natural_add_mul_ref_ref_ref_algorithms,
        benchmark_natural_add_mul_assign_u32,
        benchmark_natural_add_mul_assign_u32_evaluation_strategy,
        benchmark_natural_add_mul_assign_u32_algorithms,
        benchmark_natural_add_mul_assign_u32_ref_algorithms,
        benchmark_natural_add_mul_u32,
        benchmark_natural_add_mul_u32_evaluation_strategy,
        benchmark_natural_add_mul_u32_algorithms,
        benchmark_natural_add_mul_u32_val_ref_algorithms,
        benchmark_natural_add_mul_u32_ref_val_algorithms,
        benchmark_natural_add_mul_u32_ref_ref_algorithms,
        benchmark_natural_assign_evaluation_strategy,
        benchmark_natural_assign_integer,
        benchmark_natural_assign_integer_evaluation_strategy,
        benchmark_natural_assign_u32,
        benchmark_natural_assign_u64,
        benchmark_natural_assign_bit,
        benchmark_natural_ceiling_log_two,
        benchmark_natural_clear_bit,
        benchmark_natural_clone_from,
        benchmark_natural_decrement,
        benchmark_natural_divisible_by_power_of_two,
        benchmark_natural_divisible_by_power_of_two_algorithms,
        benchmark_natural_flip_bit,
        benchmark_natural_floor_log_two,
        benchmark_natural_get_bit,
        benchmark_natural_increment,
        benchmark_natural_is_even,
        benchmark_natural_is_power_of_two,
        benchmark_natural_limb_count,
        benchmark_natural_to_limbs_asc,
        benchmark_natural_to_limbs_desc,
        benchmark_natural_mod_power_of_two_assign,
        benchmark_natural_mod_power_of_two,
        benchmark_natural_mod_power_of_two_ref,
        benchmark_natural_neg_mod_power_of_two_assign,
        benchmark_natural_neg_mod_power_of_two,
        benchmark_natural_neg_mod_power_of_two_ref,
        benchmark_natural_mul_assign,
        benchmark_natural_mul_assign_evaluation_strategy,
        benchmark_natural_mul_assign_algorithms,
        benchmark_natural_mul_evaluation_strategy,
        benchmark_natural_mul_assign_u32,
        benchmark_natural_mul_u32,
        benchmark_natural_mul_u32_evaluation_strategy,
        benchmark_u32_mul_natural,
        benchmark_u32_mul_natural_evaluation_strategy,
        benchmark_natural_neg_evaluation_strategy,
        benchmark_natural_not_evaluation_strategy,
        benchmark_natural_partial_cmp_abs_integer,
        benchmark_natural_partial_cmp_integer,
        benchmark_natural_partial_cmp_u32,
        benchmark_u32_partial_cmp_natural,
        benchmark_natural_partial_eq_u32,
        benchmark_u32_partial_eq_natural,
        benchmark_natural_partial_eq_integer,
        benchmark_natural_random_natural_below,
        benchmark_natural_set_bit,
        benchmark_natural_shl_assign_i32,
        benchmark_natural_shl_i32,
        benchmark_natural_shl_i32_ref,
        benchmark_natural_shl_round_assign_i32,
        benchmark_natural_shl_round_i32,
        benchmark_natural_shl_round_i32_ref,
        benchmark_natural_shl_assign_u32,
        benchmark_natural_shl_u32,
        benchmark_natural_shl_u32_ref,
        benchmark_natural_shr_assign_i32,
        benchmark_natural_shr_i32,
        benchmark_natural_shr_i32_ref,
        benchmark_natural_shr_round_assign_i32,
        benchmark_natural_shr_round_i32,
        benchmark_natural_shr_round_i32_ref,
        benchmark_natural_shr_assign_u32,
        benchmark_natural_shr_u32,
        benchmark_natural_shr_u32_ref,
        benchmark_natural_shr_round_assign_u32,
        benchmark_natural_shr_round_u32,
        benchmark_natural_shr_round_u32_ref,
        benchmark_natural_significant_bits,
        benchmark_natural_special_random_natural_below,
        benchmark_natural_sub_assign,
        benchmark_natural_sub_evaluation_strategy,
        benchmark_natural_sub_assign_u32,
        benchmark_natural_sub_u32,
        benchmark_natural_sub_u32_ref,
        benchmark_u32_sub_natural,
        benchmark_natural_sub_mul_assign,
        benchmark_natural_sub_mul_assign_algorithms,
        benchmark_natural_sub_mul,
        benchmark_natural_sub_mul_evaluation_strategy,
        benchmark_natural_sub_mul_algorithms,
        benchmark_natural_sub_mul_ref_algorithms,
        benchmark_natural_sub_mul_assign_u32,
        benchmark_natural_sub_mul_assign_u32_algorithms,
        benchmark_natural_sub_mul_u32,
        benchmark_natural_sub_mul_u32_evaluation_strategy,
        benchmark_natural_sub_mul_u32_algorithms,
        benchmark_natural_sub_mul_u32_ref_algorithms,
        benchmark_natural_to_integer,
        benchmark_natural_to_integer_evaluation_strategy,
        benchmark_natural_trailing_zeros,
    ]
);
