extern crate malachite_test;

use malachite_test::common::{get_gm, get_no_special_gm, DemoBenchRegistry, GenerationMode,
                             ScaleType};
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
use malachite_test::natural::conversion::from_bits::*;
use malachite_test::natural::conversion::from_limbs::*;
use malachite_test::natural::conversion::from_u32::*;
use malachite_test::natural::conversion::from_u64::*;
use malachite_test::natural::conversion::serde::*;
use malachite_test::natural::conversion::to_bits::*;
use malachite_test::natural::conversion::to_integer::*;
use malachite_test::natural::conversion::to_limbs::*;
use malachite_test::natural::conversion::to_u32::*;
use malachite_test::natural::conversion::to_u64::*;
use malachite_test::natural::logic::assign_bit::*;
use malachite_test::natural::logic::clear_bit::*;
use malachite_test::natural::logic::flip_bit::*;
use malachite_test::natural::logic::get_bit::*;
use malachite_test::natural::logic::limb_count::*;
use malachite_test::natural::logic::not::*;
use malachite_test::natural::logic::set_bit::*;
use malachite_test::natural::logic::significant_bits::*;
use malachite_test::natural::logic::trailing_zeros::*;
use std::env;

pub fn main_2() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: [exhaustive|random|special_random] [limit] [demo/bench name]");
    }
    let generation_mode = &args[1];
    assert!(
        generation_mode == "exhaustive" || generation_mode == "random"
            || generation_mode == "special_random",
        "Bad generation mode"
    );
    let limit = if args.len() == 4 {
        args[2].parse().unwrap()
    } else {
        usize::max_value()
    };
    let item_name = args.last().unwrap();

    let mut registry = DemoBenchRegistry::default();
    malachite_test::register(&mut registry);

    if let Some(f) = registry.lookup_demo(item_name) {
        f(get_gm(generation_mode, ScaleType::None), limit);
        return;
    }
    if let Some(&(scale_type, f)) = registry.lookup_bench(item_name) {
        f(get_gm(generation_mode, scale_type), limit, "temp.gp");
        return;
    }
    if let Some(f) = registry.lookup_no_special_demo(item_name) {
        f(get_no_special_gm(generation_mode, ScaleType::None), limit);
        return;
    }
    if let Some(&(scale_type, f)) = registry.lookup_no_special_bench(item_name) {
        f(
            get_no_special_gm(generation_mode, scale_type),
            limit,
            "temp.gp",
        );
        return;
    }
}

macro_rules! demos_and_benchmarks {
    (
        [$($special_demo_fn: ident,)*],
        [$($special_no_scale_bench_fn: ident,)*],
        [$($special_small_scale_bench_fn: ident,)*],
        [$($special_large_scale_bench_fn: ident,)*]
    ) => {
        fn main() {
            main_2();
            let args: Vec<String> = env::args().collect();
            if args.len() != 3 && args.len() != 4 {
                panic!("Usage: [exhaustive|random|special_random] [limit] [demo/bench name]");
            }
            let generation_mode = &args[1];
            assert!(
                generation_mode == "exhaustive" || generation_mode == "random" || generation_mode == "special_random",
                "Bad generation mode"
            );
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
                $(stringify!($special_demo_fn) => $special_demo_fn(sgm_demo, limit)),*,
                $(
                    stringify!($special_no_scale_bench_fn) => {
                        $special_no_scale_bench_fn(sgm_small, limit, "temp.gp")
                    }
                ),*,
                $(
                    stringify!($special_small_scale_bench_fn) => {
                        $special_small_scale_bench_fn(sgm_small, limit, "temp.gp")
                    }
                ),*
                $(
                    stringify!($special_large_scale_bench_fn) => {
                        $special_large_scale_bench_fn(sgm_large, limit, "temp.gp")
                    }
                ),*
                "all" => {
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
    // special_demo_fn
    [
        demo_limbs_asc_from_bits_asc,
        demo_limbs_asc_from_bits_desc,
        demo_limbs_ceiling_log_two,
        demo_limbs_floor_log_two,
        demo_limbs_significant_bits,
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
        demo_natural_assign_u32,
        demo_natural_assign_u64,
        demo_natural_assign_bit,
        demo_natural_bits,
        demo_natural_bits_rev,
        demo_natural_bits_size_hint,
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
        demo_natural_from_bits_asc,
        demo_natural_from_bits_desc,
        demo_natural_from_limbs_asc,
        demo_natural_from_limbs_desc,
        demo_natural_from_owned_limbs_asc,
        demo_natural_from_owned_limbs_desc,
        demo_natural_from_u32,
        demo_natural_from_u64,
        demo_natural_get_bit,
        demo_natural_hash,
        demo_natural_increment,
        demo_natural_into_limbs_asc,
        demo_natural_into_limbs_desc,
        demo_natural_is_even,
        demo_natural_is_odd,
        demo_natural_is_power_of_two,
        demo_natural_limb_count,
        demo_natural_limbs,
        demo_natural_limbs_index,
        demo_natural_limbs_rev,
        demo_natural_limbs_size_hint,
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
        demo_natural_partial_cmp_u32,
        demo_u32_partial_cmp_natural,
        demo_natural_serialize,
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
        demo_natural_to_bits_asc,
        demo_natural_to_bits_desc,
        demo_natural_to_integer,
        demo_natural_to_limbs_asc,
        demo_natural_to_limbs_desc,
        demo_natural_to_u32,
        demo_natural_to_u32_wrapping,
        demo_natural_to_u64,
        demo_natural_to_u64_wrapping,
        demo_natural_trailing_zeros,
    ],
    // special_no_scale_bench_fn
    [
        benchmark_natural_from_u32_library_comparison,
        benchmark_natural_from_u64_library_comparison,
        benchmark_natural_to_u32,
        benchmark_natural_to_u32_wrapping,
        benchmark_natural_to_u64,
        benchmark_natural_to_u64_wrapping,
    ],
    // special_small_scale_bench_fn
    [
        benchmark_limbs_asc_from_bits_asc,
        benchmark_limbs_asc_from_bits_desc,
        benchmark_limbs_ceiling_log_two,
        benchmark_limbs_floor_log_two,
        benchmark_limbs_significant_bits,
        benchmark_natural_from_limbs_asc_evaluation_strategy,
        benchmark_natural_from_limbs_desc_evaluation_strategy,
    ],
    // special_large_scale_bench_fn
    [
        benchmark_natural_add_library_comparison,
        benchmark_natural_add_assign_library_comparison,
        benchmark_natural_add_assign_evaluation_strategy,
        benchmark_natural_add_evaluation_strategy,
        benchmark_natural_add_assign_u32_library_comparison,
        benchmark_natural_add_u32_library_comparison,
        benchmark_natural_add_u32_evaluation_strategy,
        benchmark_u32_add_natural_library_comparison,
        benchmark_u32_add_natural_evaluation_strategy,
        benchmark_natural_add_mul_assign_evaluation_strategy,
        benchmark_natural_add_mul_assign_algorithms,
        benchmark_natural_add_mul_assign_val_ref_algorithms,
        benchmark_natural_add_mul_assign_ref_val_algorithms,
        benchmark_natural_add_mul_assign_ref_ref_algorithms,
        benchmark_natural_add_mul_evaluation_strategy,
        benchmark_natural_add_mul_algorithms,
        benchmark_natural_add_mul_val_val_ref_algorithms,
        benchmark_natural_add_mul_val_ref_val_algorithms,
        benchmark_natural_add_mul_val_ref_ref_algorithms,
        benchmark_natural_add_mul_ref_ref_ref_algorithms,
        benchmark_natural_add_mul_assign_u32_evaluation_strategy,
        benchmark_natural_add_mul_assign_u32_algorithms,
        benchmark_natural_add_mul_assign_u32_ref_algorithms,
        benchmark_natural_add_mul_u32_evaluation_strategy,
        benchmark_natural_add_mul_u32_algorithms,
        benchmark_natural_add_mul_u32_val_ref_algorithms,
        benchmark_natural_add_mul_u32_ref_val_algorithms,
        benchmark_natural_add_mul_u32_ref_ref_algorithms,
        benchmark_natural_assign_library_comparison,
        benchmark_natural_assign_evaluation_strategy,
        benchmark_natural_assign_u32_library_comparison,
        benchmark_natural_assign_u64_library_comparison,
        benchmark_natural_assign_bit_library_comparison,
        benchmark_natural_bits_evaluation_strategy,
        benchmark_natural_bits_rev_evaluation_strategy,
        benchmark_natural_bits_size_hint,
        benchmark_natural_ceiling_log_two,
        benchmark_natural_clear_bit,
        benchmark_natural_clone_library_comparison,
        benchmark_natural_clone_from_library_comparison,
        benchmark_natural_cmp,
        benchmark_natural_decrement,
        benchmark_natural_divisible_by_power_of_two_algorithms,
        benchmark_natural_eq,
        benchmark_natural_flip_bit_library_comparison,
        benchmark_natural_floor_log_two,
        benchmark_natural_from_bits_asc,
        benchmark_natural_from_bits_desc,
        benchmark_natural_get_bit_library_comparison,
        benchmark_natural_hash,
        benchmark_natural_increment,
        benchmark_natural_is_even,
        benchmark_natural_is_power_of_two,
        benchmark_natural_limb_count,
        benchmark_natural_limbs_evaluation_strategy,
        benchmark_natural_limbs_index_algorithms,
        benchmark_natural_limbs_rev_evaluation_strategy,
        benchmark_natural_limbs_size_hint,
        benchmark_natural_mod_power_of_two_assign,
        benchmark_natural_mod_power_of_two_evaluation_strategy,
        benchmark_natural_neg_mod_power_of_two_assign,
        benchmark_natural_neg_mod_power_of_two_evaluation_strategy,
        benchmark_natural_mul_assign_library_comparison,
        benchmark_natural_mul_assign_evaluation_strategy,
        benchmark_natural_mul_assign_algorithms,
        benchmark_natural_mul_evaluation_strategy,
        benchmark_natural_mul_library_comparison,
        benchmark_natural_mul_assign_u32_library_comparison,
        benchmark_natural_mul_u32_library_comparison,
        benchmark_natural_mul_u32_evaluation_strategy,
        benchmark_u32_mul_natural_library_comparison,
        benchmark_u32_mul_natural_evaluation_strategy,
        benchmark_natural_neg_library_comparison,
        benchmark_natural_neg_evaluation_strategy,
        benchmark_natural_not_library_comparison,
        benchmark_natural_not_evaluation_strategy,
        benchmark_natural_partial_cmp_u32_library_comparison,
        benchmark_u32_partial_cmp_natural_library_comparison,
        benchmark_natural_partial_eq_u32_library_comparison,
        benchmark_u32_partial_eq_natural_library_comparison,
        benchmark_natural_serialize,
        benchmark_natural_set_bit_library_comparison,
        benchmark_natural_shl_assign_i32_library_comparison,
        benchmark_natural_shl_i32_evaluation_strategy,
        benchmark_natural_shl_round_assign_i32,
        benchmark_natural_shl_round_i32_evaluation_strategy,
        benchmark_natural_shl_assign_u32_library_comparison,
        benchmark_natural_shl_u32_evaluation_strategy,
        benchmark_natural_shr_assign_i32_library_comparison,
        benchmark_natural_shr_i32_evaluation_strategy,
        benchmark_natural_shr_round_assign_i32,
        benchmark_natural_shr_round_i32_evaluation_strategy,
        benchmark_natural_shr_assign_u32_library_comparison,
        benchmark_natural_shr_u32_evaluation_strategy,
        benchmark_natural_shr_round_assign_u32,
        benchmark_natural_shr_round_u32_evaluation_strategy,
        benchmark_natural_significant_bits,
        benchmark_natural_sub_assign_library_comparison,
        benchmark_natural_sub_evaluation_strategy,
        benchmark_natural_sub_library_comparison,
        benchmark_natural_sub_assign_u32_library_comparison,
        benchmark_natural_sub_u32_library_comparison,
        benchmark_natural_sub_u32_evaluation_strategy,
        benchmark_u32_sub_natural_library_comparison,
        benchmark_natural_sub_mul_assign,
        benchmark_natural_sub_mul_assign_algorithms,
        benchmark_natural_sub_mul_evaluation_strategy,
        benchmark_natural_sub_mul_algorithms,
        benchmark_natural_sub_mul_ref_algorithms,
        benchmark_natural_sub_mul_assign_u32,
        benchmark_natural_sub_mul_assign_u32_algorithms,
        benchmark_natural_sub_mul_u32_evaluation_strategy,
        benchmark_natural_sub_mul_u32_algorithms,
        benchmark_natural_sub_mul_u32_ref_algorithms,
        benchmark_natural_to_integer_evaluation_strategy,
        benchmark_natural_trailing_zeros,
    ]
);
