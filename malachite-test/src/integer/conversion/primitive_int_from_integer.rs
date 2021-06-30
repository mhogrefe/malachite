use malachite_base::named::Named;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{integers, integers_var_1, integers_var_2, rm_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_u8_checked_from_integer);
    register_demo!(registry, demo_u16_checked_from_integer);
    register_demo!(registry, demo_u32_checked_from_integer);
    register_demo!(registry, demo_u64_checked_from_integer);
    register_demo!(registry, demo_usize_checked_from_integer);
    register_demo!(registry, demo_u8_exact_from_integer);
    register_demo!(registry, demo_u16_exact_from_integer);
    register_demo!(registry, demo_u32_exact_from_integer);
    register_demo!(registry, demo_u64_exact_from_integer);
    register_demo!(registry, demo_usize_exact_from_integer);
    register_demo!(registry, demo_u8_wrapping_from_integer);
    register_demo!(registry, demo_u16_wrapping_from_integer);
    register_demo!(registry, demo_u32_wrapping_from_integer);
    register_demo!(registry, demo_u64_wrapping_from_integer);
    register_demo!(registry, demo_usize_wrapping_from_integer);
    register_demo!(registry, demo_u8_saturating_from_integer);
    register_demo!(registry, demo_u16_saturating_from_integer);
    register_demo!(registry, demo_u32_saturating_from_integer);
    register_demo!(registry, demo_u64_saturating_from_integer);
    register_demo!(registry, demo_usize_saturating_from_integer);
    register_demo!(registry, demo_u8_overflowing_from_integer);
    register_demo!(registry, demo_u16_overflowing_from_integer);
    register_demo!(registry, demo_u32_overflowing_from_integer);
    register_demo!(registry, demo_u64_overflowing_from_integer);
    register_demo!(registry, demo_usize_overflowing_from_integer);
    register_demo!(registry, demo_u8_convertible_from_integer);
    register_demo!(registry, demo_u16_convertible_from_integer);
    register_demo!(registry, demo_u32_convertible_from_integer);
    register_demo!(registry, demo_u64_convertible_from_integer);
    register_demo!(registry, demo_usize_convertible_from_integer);
    register_demo!(registry, demo_i8_checked_from_integer);
    register_demo!(registry, demo_i16_checked_from_integer);
    register_demo!(registry, demo_i32_checked_from_integer);
    register_demo!(registry, demo_i64_checked_from_integer);
    register_demo!(registry, demo_isize_checked_from_integer);
    register_demo!(registry, demo_i8_exact_from_integer);
    register_demo!(registry, demo_i16_exact_from_integer);
    register_demo!(registry, demo_i32_exact_from_integer);
    register_demo!(registry, demo_i64_exact_from_integer);
    register_demo!(registry, demo_isize_exact_from_integer);
    register_demo!(registry, demo_i8_wrapping_from_integer);
    register_demo!(registry, demo_i16_wrapping_from_integer);
    register_demo!(registry, demo_i32_wrapping_from_integer);
    register_demo!(registry, demo_i64_wrapping_from_integer);
    register_demo!(registry, demo_isize_wrapping_from_integer);
    register_demo!(registry, demo_i8_saturating_from_integer);
    register_demo!(registry, demo_i16_saturating_from_integer);
    register_demo!(registry, demo_i32_saturating_from_integer);
    register_demo!(registry, demo_i64_saturating_from_integer);
    register_demo!(registry, demo_isize_saturating_from_integer);
    register_demo!(registry, demo_i8_overflowing_from_integer);
    register_demo!(registry, demo_i16_overflowing_from_integer);
    register_demo!(registry, demo_i32_overflowing_from_integer);
    register_demo!(registry, demo_i64_overflowing_from_integer);
    register_demo!(registry, demo_isize_overflowing_from_integer);
    register_demo!(registry, demo_i8_convertible_from_integer);
    register_demo!(registry, demo_i16_convertible_from_integer);
    register_demo!(registry, demo_i32_convertible_from_integer);
    register_demo!(registry, demo_i64_convertible_from_integer);
    register_demo!(registry, demo_isize_convertible_from_integer);
    register_bench!(registry, Large, benchmark_u8_checked_from_integer);
    register_bench!(registry, Large, benchmark_u16_checked_from_integer);
    register_bench!(registry, Large, benchmark_u32_checked_from_integer);
    register_bench!(registry, Large, benchmark_u64_checked_from_integer);
    register_bench!(registry, Large, benchmark_usize_checked_from_integer);
    register_bench!(
        registry,
        Large,
        benchmark_u8_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u16_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_usize_checked_from_integer_algorithms
    );
    register_bench!(registry, Large, benchmark_u8_exact_from_integer);
    register_bench!(registry, Large, benchmark_u16_exact_from_integer);
    register_bench!(registry, Large, benchmark_u32_exact_from_integer);
    register_bench!(registry, Large, benchmark_u64_exact_from_integer);
    register_bench!(registry, Large, benchmark_usize_exact_from_integer);
    register_bench!(registry, Large, benchmark_u8_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_u16_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_u32_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_u64_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_usize_wrapping_from_integer);
    register_bench!(
        registry,
        Large,
        benchmark_u8_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u16_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_usize_wrapping_from_integer_algorithms
    );
    register_bench!(registry, Large, benchmark_u8_saturating_from_integer);
    register_bench!(registry, Large, benchmark_u16_saturating_from_integer);
    register_bench!(registry, Large, benchmark_u32_saturating_from_integer);
    register_bench!(registry, Large, benchmark_u64_saturating_from_integer);
    register_bench!(registry, Large, benchmark_usize_saturating_from_integer);
    register_bench!(registry, Large, benchmark_u8_overflowing_from_integer);
    register_bench!(registry, Large, benchmark_u16_overflowing_from_integer);
    register_bench!(registry, Large, benchmark_u32_overflowing_from_integer);
    register_bench!(registry, Large, benchmark_u64_overflowing_from_integer);
    register_bench!(registry, Large, benchmark_usize_overflowing_from_integer);
    register_bench!(
        registry,
        Large,
        benchmark_u8_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u16_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_usize_overflowing_from_integer_algorithms
    );
    register_bench!(registry, Large, benchmark_u8_convertible_from_integer);
    register_bench!(registry, Large, benchmark_u16_convertible_from_integer);
    register_bench!(registry, Large, benchmark_u32_convertible_from_integer);
    register_bench!(registry, Large, benchmark_u64_convertible_from_integer);
    register_bench!(registry, Large, benchmark_usize_convertible_from_integer);
    register_bench!(
        registry,
        Large,
        benchmark_u8_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u16_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_usize_convertible_from_integer_algorithms
    );
    register_bench!(registry, Large, benchmark_i8_checked_from_integer);
    register_bench!(registry, Large, benchmark_i16_checked_from_integer);
    register_bench!(registry, Large, benchmark_i32_checked_from_integer);
    register_bench!(registry, Large, benchmark_i64_checked_from_integer);
    register_bench!(registry, Large, benchmark_isize_checked_from_integer);
    register_bench!(
        registry,
        Large,
        benchmark_i8_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i16_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_isize_checked_from_integer_algorithms
    );
    register_bench!(registry, Large, benchmark_i8_exact_from_integer);
    register_bench!(registry, Large, benchmark_i16_exact_from_integer);
    register_bench!(registry, Large, benchmark_i32_exact_from_integer);
    register_bench!(registry, Large, benchmark_i64_exact_from_integer);
    register_bench!(registry, Large, benchmark_isize_exact_from_integer);
    register_bench!(registry, Large, benchmark_i8_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_i16_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_i32_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_i64_wrapping_from_integer);
    register_bench!(registry, Large, benchmark_isize_wrapping_from_integer);
    register_bench!(
        registry,
        Large,
        benchmark_i8_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i16_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_isize_wrapping_from_integer_algorithms
    );
    register_bench!(registry, Large, benchmark_i8_saturating_from_integer);
    register_bench!(registry, Large, benchmark_i16_saturating_from_integer);
    register_bench!(registry, Large, benchmark_i32_saturating_from_integer);
    register_bench!(registry, Large, benchmark_i64_saturating_from_integer);
    register_bench!(registry, Large, benchmark_isize_saturating_from_integer);
    register_bench!(registry, Large, benchmark_i8_overflowing_from_integer);
    register_bench!(registry, Large, benchmark_i16_overflowing_from_integer);
    register_bench!(registry, Large, benchmark_i32_overflowing_from_integer);
    register_bench!(registry, Large, benchmark_i64_overflowing_from_integer);
    register_bench!(registry, Large, benchmark_isize_overflowing_from_integer);
    register_bench!(
        registry,
        Large,
        benchmark_i8_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i16_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_isize_overflowing_from_integer_algorithms
    );
    register_bench!(registry, Large, benchmark_i8_convertible_from_integer);
    register_bench!(registry, Large, benchmark_i16_convertible_from_integer);
    register_bench!(registry, Large, benchmark_i32_convertible_from_integer);
    register_bench!(registry, Large, benchmark_i64_convertible_from_integer);
    register_bench!(registry, Large, benchmark_isize_convertible_from_integer);
    register_bench!(
        registry,
        Large,
        benchmark_i8_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i16_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_isize_convertible_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_checked_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_wrapping_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_checked_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u64_wrapping_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_checked_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_wrapping_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_checked_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_i64_wrapping_from_integer_library_comparison
    );
}

macro_rules! demo_and_bench {
    (
        $t:ident,
        $exact_from_generator:ident,
        $checked_from_demo_name:ident,
        $exact_from_demo_name:ident,
        $wrapping_from_demo_name:ident,
        $saturating_from_demo_name:ident,
        $overflowing_from_demo_name:ident,
        $convertible_from_demo_name:ident,
        $checked_from_es_bench_name:ident,
        $checked_from_a_bench_name:ident,
        $exact_from_es_bench_name:ident,
        $wrapping_from_es_bench_name:ident,
        $wrapping_from_a_bench_name:ident,
        $saturating_from_es_bench_name:ident,
        $overflowing_from_es_bench_name:ident,
        $overflowing_from_a_bench_name:ident,
        $convertible_from_es_bench_name:ident,
        $convertible_from_a_bench_name:ident
    ) => {
        fn $checked_from_demo_name(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!(
                    "{}::checked_from(&{}) = {:?}",
                    $t::NAME,
                    n,
                    $t::checked_from(&n)
                );
            }
        }

        fn $exact_from_demo_name(gm: GenerationMode, limit: usize) {
            for n in $exact_from_generator::<$t>(gm).take(limit) {
                println!("{}::exact_from(&{}) = {}", $t::NAME, n, $t::exact_from(&n));
            }
        }

        fn $wrapping_from_demo_name(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!(
                    "{}::wrapping_from(&{}) = {}",
                    $t::NAME,
                    n,
                    $t::wrapping_from(&n)
                );
            }
        }

        fn $saturating_from_demo_name(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!(
                    "{}::saturating_from(&{}) = {}",
                    $t::NAME,
                    n,
                    $t::saturating_from(&n)
                );
            }
        }

        fn $overflowing_from_demo_name(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!(
                    "{}::overflowing_from(&{}) = {:?}",
                    $t::NAME,
                    n,
                    $t::overflowing_from(&n)
                );
            }
        }

        fn $convertible_from_demo_name(gm: GenerationMode, limit: usize) {
            for n in integers(gm).take(limit) {
                println!(
                    "{} is {}convertible to a {}",
                    n,
                    if $t::convertible_from(&n) { "" } else { "not " },
                    $t::NAME,
                );
            }
        }

        fn $checked_from_es_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::checked_from(Integer)", $t::NAME),
                BenchmarkType::Single,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [("Malachite", &mut (|n| no_out!($t::checked_from(&n))))],
            );
        }

        fn $checked_from_a_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::checked_from(Integer)", $t::NAME),
                BenchmarkType::Algorithms,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [
                    ("standard", &mut (|n| no_out!($t::checked_from(&n)))),
                    (
                        "using overflowing_from",
                        &mut (|n| {
                            let (value, overflow) = $t::overflowing_from(&n);
                            if overflow {
                                None
                            } else {
                                Some(value)
                            };
                        }),
                    ),
                ],
            );
        }

        fn $exact_from_es_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::exact_from(Integer)", $t::NAME),
                BenchmarkType::Single,
                $exact_from_generator::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [("Malachite", &mut (|n| no_out!($t::exact_from(&n))))],
            );
        }

        fn $wrapping_from_es_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::wrapping_from(Integer)", $t::NAME),
                BenchmarkType::Single,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [("Malachite", &mut (|n| no_out!($t::wrapping_from(&n))))],
            );
        }

        fn $wrapping_from_a_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::wrapping_from(Integer)", $t::NAME),
                BenchmarkType::Algorithms,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [
                    ("standard", &mut (|n| no_out!($t::wrapping_from(&n)))),
                    (
                        "using overflowing_from",
                        &mut (|n| {
                            $t::overflowing_from(&n).0;
                        }),
                    ),
                ],
            );
        }

        fn $saturating_from_es_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::saturating_from(Integer)", $t::NAME),
                BenchmarkType::Single,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [("Malachite", &mut (|n| no_out!($t::saturating_from(&n))))],
            );
        }

        fn $overflowing_from_es_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::overflowing_from(Integer)", $t::NAME),
                BenchmarkType::Single,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [("Malachite", &mut (|n| no_out!($t::overflowing_from(&n))))],
            );
        }

        fn $overflowing_from_a_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::overflowing_from(Integer)", $t::NAME),
                BenchmarkType::Algorithms,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|ref n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [
                    ("standard", &mut (|n| no_out!($t::overflowing_from(&n)))),
                    (
                        "using wrapping_from and convertible_from",
                        &mut (|n| no_out!(($t::wrapping_from(&n), !$t::convertible_from(&n)))),
                    ),
                ],
            );
        }

        fn $convertible_from_es_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::convertible_from(Integer)", $t::NAME),
                BenchmarkType::Single,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [("Malachite", &mut (|n| no_out!($t::convertible_from(&n))))],
            );
        }

        fn $convertible_from_a_bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            run_benchmark_old(
                &format!("{}::convertible_from(Integer)", $t::NAME),
                BenchmarkType::Algorithms,
                integers(gm),
                gm.name(),
                limit,
                file_name,
                &(|n| usize::exact_from(n.significant_bits())),
                "n.significant_bits()",
                &mut [
                    ("standard", &mut (|n| no_out!($t::convertible_from(&n)))),
                    (
                        "using checked_from",
                        &mut (|n| no_out!($t::checked_from(&n).is_some())),
                    ),
                ],
            );
        }
    };
}

demo_and_bench!(
    u8,
    integers_var_1,
    demo_u8_checked_from_integer,
    demo_u8_exact_from_integer,
    demo_u8_wrapping_from_integer,
    demo_u8_saturating_from_integer,
    demo_u8_overflowing_from_integer,
    demo_u8_convertible_from_integer,
    benchmark_u8_checked_from_integer,
    benchmark_u8_checked_from_integer_algorithms,
    benchmark_u8_exact_from_integer,
    benchmark_u8_wrapping_from_integer,
    benchmark_u8_wrapping_from_integer_algorithms,
    benchmark_u8_saturating_from_integer,
    benchmark_u8_overflowing_from_integer,
    benchmark_u8_overflowing_from_integer_algorithms,
    benchmark_u8_convertible_from_integer,
    benchmark_u8_convertible_from_integer_algorithms
);
demo_and_bench!(
    u16,
    integers_var_1,
    demo_u16_checked_from_integer,
    demo_u16_exact_from_integer,
    demo_u16_wrapping_from_integer,
    demo_u16_saturating_from_integer,
    demo_u16_overflowing_from_integer,
    demo_u16_convertible_from_integer,
    benchmark_u16_checked_from_integer,
    benchmark_u16_checked_from_integer_algorithms,
    benchmark_u16_exact_from_integer,
    benchmark_u16_wrapping_from_integer,
    benchmark_u16_wrapping_from_integer_algorithms,
    benchmark_u16_saturating_from_integer,
    benchmark_u16_overflowing_from_integer,
    benchmark_u16_overflowing_from_integer_algorithms,
    benchmark_u16_convertible_from_integer,
    benchmark_u16_convertible_from_integer_algorithms
);
demo_and_bench!(
    u32,
    integers_var_1,
    demo_u32_checked_from_integer,
    demo_u32_exact_from_integer,
    demo_u32_wrapping_from_integer,
    demo_u32_saturating_from_integer,
    demo_u32_overflowing_from_integer,
    demo_u32_convertible_from_integer,
    benchmark_u32_checked_from_integer,
    benchmark_u32_checked_from_integer_algorithms,
    benchmark_u32_exact_from_integer,
    benchmark_u32_wrapping_from_integer,
    benchmark_u32_wrapping_from_integer_algorithms,
    benchmark_u32_saturating_from_integer,
    benchmark_u32_overflowing_from_integer,
    benchmark_u32_overflowing_from_integer_algorithms,
    benchmark_u32_convertible_from_integer,
    benchmark_u32_convertible_from_integer_algorithms
);
demo_and_bench!(
    u64,
    integers_var_1,
    demo_u64_checked_from_integer,
    demo_u64_exact_from_integer,
    demo_u64_wrapping_from_integer,
    demo_u64_saturating_from_integer,
    demo_u64_overflowing_from_integer,
    demo_u64_convertible_from_integer,
    benchmark_u64_checked_from_integer,
    benchmark_u64_checked_from_integer_algorithms,
    benchmark_u64_exact_from_integer,
    benchmark_u64_wrapping_from_integer,
    benchmark_u64_wrapping_from_integer_algorithms,
    benchmark_u64_saturating_from_integer,
    benchmark_u64_overflowing_from_integer,
    benchmark_u64_overflowing_from_integer_algorithms,
    benchmark_u64_convertible_from_integer,
    benchmark_u64_convertible_from_integer_algorithms
);
demo_and_bench!(
    usize,
    integers_var_1,
    demo_usize_checked_from_integer,
    demo_usize_exact_from_integer,
    demo_usize_wrapping_from_integer,
    demo_usize_saturating_from_integer,
    demo_usize_overflowing_from_integer,
    demo_usize_convertible_from_integer,
    benchmark_usize_checked_from_integer,
    benchmark_usize_checked_from_integer_algorithms,
    benchmark_usize_exact_from_integer,
    benchmark_usize_wrapping_from_integer,
    benchmark_usize_wrapping_from_integer_algorithms,
    benchmark_usize_saturating_from_integer,
    benchmark_usize_overflowing_from_integer,
    benchmark_usize_overflowing_from_integer_algorithms,
    benchmark_usize_convertible_from_integer,
    benchmark_usize_convertible_from_integer_algorithms
);
demo_and_bench!(
    i8,
    integers_var_2,
    demo_i8_checked_from_integer,
    demo_i8_exact_from_integer,
    demo_i8_wrapping_from_integer,
    demo_i8_saturating_from_integer,
    demo_i8_overflowing_from_integer,
    demo_i8_convertible_from_integer,
    benchmark_i8_checked_from_integer,
    benchmark_i8_checked_from_integer_algorithms,
    benchmark_i8_exact_from_integer,
    benchmark_i8_wrapping_from_integer,
    benchmark_i8_wrapping_from_integer_algorithms,
    benchmark_i8_saturating_from_integer,
    benchmark_i8_overflowing_from_integer,
    benchmark_i8_overflowing_from_integer_algorithms,
    benchmark_i8_convertible_from_integer,
    benchmark_i8_convertible_from_integer_algorithms
);
demo_and_bench!(
    i16,
    integers_var_2,
    demo_i16_checked_from_integer,
    demo_i16_exact_from_integer,
    demo_i16_wrapping_from_integer,
    demo_i16_saturating_from_integer,
    demo_i16_overflowing_from_integer,
    demo_i16_convertible_from_integer,
    benchmark_i16_checked_from_integer,
    benchmark_i16_checked_from_integer_algorithms,
    benchmark_i16_exact_from_integer,
    benchmark_i16_wrapping_from_integer,
    benchmark_i16_wrapping_from_integer_algorithms,
    benchmark_i16_saturating_from_integer,
    benchmark_i16_overflowing_from_integer,
    benchmark_i16_overflowing_from_integer_algorithms,
    benchmark_i16_convertible_from_integer,
    benchmark_i16_convertible_from_integer_algorithms
);
demo_and_bench!(
    i32,
    integers_var_2,
    demo_i32_checked_from_integer,
    demo_i32_exact_from_integer,
    demo_i32_wrapping_from_integer,
    demo_i32_saturating_from_integer,
    demo_i32_overflowing_from_integer,
    demo_i32_convertible_from_integer,
    benchmark_i32_checked_from_integer,
    benchmark_i32_checked_from_integer_algorithms,
    benchmark_i32_exact_from_integer,
    benchmark_i32_wrapping_from_integer,
    benchmark_i32_wrapping_from_integer_algorithms,
    benchmark_i32_saturating_from_integer,
    benchmark_i32_overflowing_from_integer,
    benchmark_i32_overflowing_from_integer_algorithms,
    benchmark_i32_convertible_from_integer,
    benchmark_i32_convertible_from_integer_algorithms
);
demo_and_bench!(
    i64,
    integers_var_2,
    demo_i64_checked_from_integer,
    demo_i64_exact_from_integer,
    demo_i64_wrapping_from_integer,
    demo_i64_saturating_from_integer,
    demo_i64_overflowing_from_integer,
    demo_i64_convertible_from_integer,
    benchmark_i64_checked_from_integer,
    benchmark_i64_checked_from_integer_algorithms,
    benchmark_i64_exact_from_integer,
    benchmark_i64_wrapping_from_integer,
    benchmark_i64_wrapping_from_integer_algorithms,
    benchmark_i64_saturating_from_integer,
    benchmark_i64_overflowing_from_integer,
    benchmark_i64_overflowing_from_integer_algorithms,
    benchmark_i64_convertible_from_integer,
    benchmark_i64_convertible_from_integer_algorithms
);
demo_and_bench!(
    isize,
    integers_var_2,
    demo_isize_checked_from_integer,
    demo_isize_exact_from_integer,
    demo_isize_wrapping_from_integer,
    demo_isize_saturating_from_integer,
    demo_isize_overflowing_from_integer,
    demo_isize_convertible_from_integer,
    benchmark_isize_checked_from_integer,
    benchmark_isize_checked_from_integer_algorithms,
    benchmark_isize_exact_from_integer,
    benchmark_isize_wrapping_from_integer,
    benchmark_isize_wrapping_from_integer_algorithms,
    benchmark_isize_saturating_from_integer,
    benchmark_isize_overflowing_from_integer,
    benchmark_isize_overflowing_from_integer_algorithms,
    benchmark_isize_convertible_from_integer,
    benchmark_isize_convertible_from_integer_algorithms
);

fn benchmark_u32_checked_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "u32::checked_from(Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(u32::checked_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32()))),
        ],
    );
}

fn benchmark_u32_wrapping_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "u32::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(u32::wrapping_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32_wrapping()))),
        ],
    );
}

fn benchmark_u64_checked_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "u64::checked_from(Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(u64::checked_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u64()))),
        ],
    );
}

fn benchmark_u64_wrapping_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "u64::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(u64::wrapping_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u64_wrapping()))),
        ],
    );
}

fn benchmark_i32_checked_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "i32::checked_from(Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(i32::checked_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_i32()))),
        ],
    );
}

fn benchmark_i32_wrapping_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "i32::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(i32::wrapping_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_i32_wrapping()))),
        ],
    );
}

fn benchmark_i64_checked_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "i64::checked_from(Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(i64::checked_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_i64()))),
        ],
    );
}

fn benchmark_i64_wrapping_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "i64::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(i64::wrapping_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_i64_wrapping()))),
        ],
    );
}
