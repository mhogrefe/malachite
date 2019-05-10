use malachite_base::conversion::{CheckedFrom, RoundingFrom};
use malachite_base::named::Named;
use malachite_base::num::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    integers, pairs_of_integer_and_rounding_mode_var_1_f32,
    pairs_of_integer_and_rounding_mode_var_1_f64,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_f32_rounding_from_integer);
    register_demo!(registry, demo_f64_rounding_from_integer);
    register_demo!(registry, demo_f32_rounding_from_integer_ref);
    register_demo!(registry, demo_f64_rounding_from_integer_ref);
    register_demo!(registry, demo_f32_from_integer);
    register_demo!(registry, demo_f64_from_integer);
    register_demo!(registry, demo_f32_from_integer_ref);
    register_demo!(registry, demo_f64_from_integer_ref);
    register_demo!(registry, demo_f32_checked_from_integer);
    register_demo!(registry, demo_f64_checked_from_integer);
    register_demo!(registry, demo_f32_checked_from_integer_ref);
    register_demo!(registry, demo_f64_checked_from_integer_ref);
    register_bench!(registry, Small, benchmark_f32_rounding_from_integer);
    register_bench!(registry, Small, benchmark_f64_rounding_from_integer);
    register_bench!(registry, Small, benchmark_f32_from_integer);
    register_bench!(registry, Small, benchmark_f64_from_integer);
    register_bench!(registry, Small, benchmark_f32_checked_from_integer);
    register_bench!(registry, Small, benchmark_f64_checked_from_integer);
}

macro_rules! float_demos_and_benches {
    (
        $f: ident,
        $pairs_of_integer_and_rounding_mode_var_1: ident,
        $demo_float_rounding_from_integer: ident,
        $demo_float_rounding_from_integer_ref: ident,
        $demo_float_from_integer: ident,
        $demo_float_from_integer_ref: ident,
        $demo_float_checked_from_integer: ident,
        $demo_float_checked_from_integer_ref: ident,
        $benchmark_float_rounding_from_integer: ident,
        $benchmark_float_from_integer: ident,
        $benchmark_float_checked_from_integer: ident,
    ) => {
        fn $demo_float_rounding_from_integer(gm: GenerationMode, limit: usize) {
            for (n, rm) in $pairs_of_integer_and_rounding_mode_var_1(gm).take(limit) {
                println!(
                    "{}::rounding_from({}, {}) = {:?}",
                    $f::NAME,
                    n.clone(),
                    rm,
                    $f::rounding_from(n, rm)
                );
            }
        }

        fn $demo_float_rounding_from_integer_ref(gm: GenerationMode, limit: usize) {
            for (n, rm) in $pairs_of_integer_and_rounding_mode_var_1(gm).take(limit) {
                println!(
                    "{}::rounding_from(&{}, {}) = {:?}",
                    $f::NAME,
                    n,
                    rm,
                    $f::rounding_from(&n, rm)
                );
            }
        }

        fn $demo_float_from_integer(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!("{}::from({}) = {:?}", $f::NAME, n.clone(), $f::from(n));
            }
        }

        fn $demo_float_from_integer_ref(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!("{}::from({}) = {:?}", $f::NAME, n.clone(), $f::from(n));
            }
        }

        fn $demo_float_checked_from_integer(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!(
                    "{}::checked_from({}) = {:?}",
                    $f::NAME,
                    n.clone(),
                    $f::checked_from(n)
                );
            }
        }

        fn $demo_float_checked_from_integer_ref(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!(
                    "{}::checked_from({}) = {:?}",
                    $f::NAME,
                    n.clone(),
                    $f::checked_from(n)
                );
            }
        }

        fn $benchmark_float_rounding_from_integer(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("{}::rounding_from(Integer, RoundingMode)", stringify!($f)),
                BenchmarkType::EvaluationStrategy,
                $pairs_of_integer_and_rounding_mode_var_1(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
                "n.significant_bits()",
                &mut [
                    (
                        &format!("{}::rounding_from(Integer, RoundingMode)", stringify!($f)),
                        &mut (|(n, rm)| no_out!($f::rounding_from(n, rm))),
                    ),
                    (
                        &format!("{}::rounding_from(&Integer, RoundingMode)", stringify!($f)),
                        &mut (|(n, rm)| no_out!($f::rounding_from(&n, rm))),
                    ),
                ],
            );
        }

        fn $benchmark_float_from_integer(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("{}::from(Integer)", stringify!($f)),
                BenchmarkType::EvaluationStrategy,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
                "n.significant_bits()",
                &mut [
                    (
                        &format!("{}::from(Integer)", stringify!($f)),
                        &mut (|n| no_out!($f::from(n))),
                    ),
                    (
                        &format!("{}::from(&Integer)", stringify!($f)),
                        &mut (|n| no_out!($f::from(&n))),
                    ),
                ],
            );
        }

        fn $benchmark_float_checked_from_integer(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("{}::checked_from(Integer)", stringify!($f)),
                BenchmarkType::EvaluationStrategy,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
                "n.significant_bits()",
                &mut [
                    (
                        &format!("{}::checked_from(Integer)", stringify!($f)),
                        &mut (|n| no_out!($f::checked_from(n))),
                    ),
                    (
                        &format!("{}::checked_from(&Integer)", stringify!($f)),
                        &mut (|n| no_out!($f::checked_from(&n))),
                    ),
                ],
            );
        }
    };
}

float_demos_and_benches!(
    f32,
    pairs_of_integer_and_rounding_mode_var_1_f32,
    demo_f32_rounding_from_integer,
    demo_f32_rounding_from_integer_ref,
    demo_f32_from_integer,
    demo_f32_from_integer_ref,
    demo_f32_checked_from_integer,
    demo_f32_checked_from_integer_ref,
    benchmark_f32_rounding_from_integer,
    benchmark_f32_from_integer,
    benchmark_f32_checked_from_integer,
);

float_demos_and_benches!(
    f64,
    pairs_of_integer_and_rounding_mode_var_1_f64,
    demo_f64_rounding_from_integer,
    demo_f64_rounding_from_integer_ref,
    demo_f64_from_integer,
    demo_f64_from_integer_ref,
    demo_f64_checked_from_integer,
    demo_f64_checked_from_integer_ref,
    benchmark_f64_rounding_from_integer,
    benchmark_f64_from_integer,
    benchmark_f64_checked_from_integer,
);
