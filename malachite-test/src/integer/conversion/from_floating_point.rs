use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom, WrappingFrom,
};
use malachite_base::num::floats::PrimitiveFloat;
use malachite_nz::integer::Integer;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    f32s, f64s, finite_f32s, finite_f64s, pairs_of_finite_f32_and_rounding_mode_var_2,
    pairs_of_finite_f64_and_rounding_mode_var_2,
};
use inputs::integer::{f32s_exactly_equal_to_integer, f64s_exactly_equal_to_integer};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_rounding_from_f32);
    register_demo!(registry, demo_integer_rounding_from_f64);
    register_demo!(registry, demo_integer_from_f32);
    register_demo!(registry, demo_integer_from_f64);
    register_demo!(registry, demo_integer_checked_from_f32);
    register_demo!(registry, demo_integer_checked_from_f64);
    register_demo!(registry, demo_integer_exact_from_f32);
    register_demo!(registry, demo_integer_exact_from_f64);
    register_demo!(registry, demo_integer_convertible_from_f32);
    register_demo!(registry, demo_integer_convertible_from_f64);
    register_bench!(registry, Small, benchmark_integer_rounding_from_f32);
    register_bench!(registry, Small, benchmark_integer_rounding_from_f64);
    register_bench!(registry, Small, benchmark_integer_from_f32);
    register_bench!(registry, Small, benchmark_integer_from_f64);
    register_bench!(registry, Small, benchmark_integer_checked_from_f32);
    register_bench!(registry, Small, benchmark_integer_checked_from_f64);
    register_bench!(registry, Small, benchmark_integer_exact_from_f32);
    register_bench!(registry, Small, benchmark_integer_exact_from_f64);
    register_bench!(
        registry,
        Small,
        benchmark_integer_convertible_from_f32_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_integer_convertible_from_f64_algorithms
    );
}

macro_rules! float_demos_and_benches {
    (
        $f: ident,
        $floats: ident,
        $floats_exactly_equal_to_integer: ident,
        $floats_var_1: ident,
        $pairs_of_float_and_rounding_mode_var_1: ident,
        $demo_integer_rounding_from_float: ident,
        $demo_integer_from_float: ident,
        $demo_integer_checked_from_float: ident,
        $demo_integer_exact_from_float: ident,
        $demo_integer_convertible_from_float: ident,
        $benchmark_integer_rounding_from_float: ident,
        $benchmark_integer_from_float: ident,
        $benchmark_integer_checked_from_float: ident,
        $benchmark_integer_exact_from_float: ident,
        $benchmark_integer_convertible_from_float_algorithms: ident,
    ) => {
        fn $demo_integer_rounding_from_float(gm: GenerationMode, limit: usize) {
            for (f, rm) in $pairs_of_float_and_rounding_mode_var_1(gm).take(limit) {
                println!(
                    "Integer::rounding_from({:?}, {}) = {:?}",
                    f,
                    rm,
                    Integer::rounding_from(f, rm)
                );
            }
        }

        fn $demo_integer_from_float(gm: GenerationMode, limit: usize) {
            for f in $floats_var_1(gm).take(limit) {
                println!("Integer::from({:?}) = {}", f, Integer::from(f));
            }
        }

        fn $demo_integer_checked_from_float(gm: GenerationMode, limit: usize) {
            for f in $floats(gm).take(limit) {
                println!(
                    "Integer::checked_from({:?}) = {:?}",
                    f,
                    Integer::checked_from(f)
                );
            }
        }

        fn $demo_integer_exact_from_float(gm: GenerationMode, limit: usize) {
            for f in $floats_exactly_equal_to_integer(gm).take(limit) {
                println!("Integer::exact_from({:?}) = {}", f, Integer::exact_from(f));
            }
        }

        fn $demo_integer_convertible_from_float(gm: GenerationMode, limit: usize) {
            for f in $floats(gm).take(limit) {
                println!(
                    "{} is {}convertible to a Integer",
                    f,
                    if Integer::convertible_from(f) {
                        ""
                    } else {
                        "not "
                    },
                );
            }
        }

        fn $benchmark_integer_rounding_from_float(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer::rounding_from({}, RoundingMode)", stringify!($f)),
                BenchmarkType::Single,
                $pairs_of_float_and_rounding_mode_var_1(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(f, _)| usize::wrapping_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [(
                    "malachite",
                    &mut (|(f, rm)| no_out!(Integer::rounding_from(f, rm))),
                )],
            );
        }

        fn $benchmark_integer_from_float(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("Integer::from({})", stringify!($f)),
                BenchmarkType::Single,
                $floats_var_1(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| usize::wrapping_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [("malachite", &mut (|f| no_out!(Integer::from(f))))],
            );
        }

        fn $benchmark_integer_checked_from_float(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer::checked_from({})", stringify!($f)),
                BenchmarkType::Single,
                $floats(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| usize::wrapping_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [("malachite", &mut (|f| no_out!(Integer::checked_from(f))))],
            );
        }

        fn $benchmark_integer_exact_from_float(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("Integer::exact_from({})", stringify!($f)),
                BenchmarkType::Single,
                $floats_exactly_equal_to_integer(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| usize::wrapping_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [("malachite", &mut (|f| no_out!(Integer::exact_from(f))))],
            );
        }

        fn $benchmark_integer_convertible_from_float_algorithms(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer::convertible_from({})", stringify!($f)),
                BenchmarkType::Algorithms,
                $floats(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| usize::wrapping_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [
                    ("standard", &mut (|f| no_out!(Integer::convertible_from(f)))),
                    (
                        "using checked_from",
                        &mut (|f| no_out!(Integer::checked_from(f).is_some())),
                    ),
                ],
            );
        }
    };
}

float_demos_and_benches!(
    f32,
    f32s,
    f32s_exactly_equal_to_integer,
    finite_f32s,
    pairs_of_finite_f32_and_rounding_mode_var_2,
    demo_integer_rounding_from_f32,
    demo_integer_from_f32,
    demo_integer_checked_from_f32,
    demo_integer_exact_from_f32,
    demo_integer_convertible_from_f32,
    benchmark_integer_rounding_from_f32,
    benchmark_integer_from_f32,
    benchmark_integer_checked_from_f32,
    benchmark_integer_exact_from_f32,
    benchmark_integer_convertible_from_f32_algorithms,
);

float_demos_and_benches!(
    f64,
    f64s,
    f64s_exactly_equal_to_integer,
    finite_f64s,
    pairs_of_finite_f64_and_rounding_mode_var_2,
    demo_integer_rounding_from_f64,
    demo_integer_from_f64,
    demo_integer_checked_from_f64,
    demo_integer_exact_from_f64,
    demo_integer_convertible_from_f64,
    benchmark_integer_rounding_from_f64,
    benchmark_integer_from_f64,
    benchmark_integer_checked_from_f64,
    benchmark_integer_exact_from_f64,
    benchmark_integer_convertible_from_f64_algorithms,
);
