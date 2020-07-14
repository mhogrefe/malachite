use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::num::floats::PrimitiveFloat;
use malachite_nz::natural::Natural;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::{
    f32s, f32s_var_1, f64s, f64s_var_1, pairs_of_finite_f32_and_rounding_mode_var_1,
    pairs_of_finite_f64_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    f32s_exactly_equal_to_natural, f64s_exactly_equal_to_natural,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_rounding_from_f32);
    register_demo!(registry, demo_natural_rounding_from_f64);
    register_demo!(registry, demo_natural_from_f32);
    register_demo!(registry, demo_natural_from_f64);
    register_demo!(registry, demo_natural_checked_from_f32);
    register_demo!(registry, demo_natural_checked_from_f64);
    register_demo!(registry, demo_natural_exact_from_f32);
    register_demo!(registry, demo_natural_exact_from_f64);
    register_demo!(registry, demo_natural_convertible_from_f32);
    register_demo!(registry, demo_natural_convertible_from_f64);
    register_bench!(registry, Small, benchmark_natural_rounding_from_f32);
    register_bench!(registry, Small, benchmark_natural_rounding_from_f64);
    register_bench!(registry, Small, benchmark_natural_from_f32);
    register_bench!(registry, Small, benchmark_natural_from_f64);
    register_bench!(registry, Small, benchmark_natural_checked_from_f32);
    register_bench!(registry, Small, benchmark_natural_checked_from_f64);
    register_bench!(registry, Small, benchmark_natural_exact_from_f32);
    register_bench!(registry, Small, benchmark_natural_exact_from_f64);
    register_bench!(
        registry,
        Small,
        benchmark_natural_convertible_from_f32_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_natural_convertible_from_f64_algorithms
    );
}

macro_rules! float_demos_and_benches {
    (
        $f: ident,
        $floats: ident,
        $floats_exactly_equal_to_natural: ident,
        $floats_var_1: ident,
        $pairs_of_float_and_rounding_mode_var_1: ident,
        $demo_natural_rounding_from_float: ident,
        $demo_natural_from_float: ident,
        $demo_natural_checked_from_float: ident,
        $demo_natural_exact_from_float: ident,
        $demo_natural_convertible_from_float: ident,
        $benchmark_natural_rounding_from_float: ident,
        $benchmark_natural_from_float: ident,
        $benchmark_natural_checked_from_float: ident,
        $benchmark_natural_exact_from_float: ident,
        $benchmark_natural_convertible_from_float_algorithms: ident,
    ) => {
        fn $demo_natural_rounding_from_float(gm: GenerationMode, limit: usize) {
            for (f, rm) in $pairs_of_float_and_rounding_mode_var_1(gm).take(limit) {
                println!(
                    "Natural::rounding_from({:?}, {}) = {:?}",
                    f,
                    rm,
                    Natural::rounding_from(f, rm)
                );
            }
        }

        fn $demo_natural_from_float(gm: GenerationMode, limit: usize) {
            for f in $floats_var_1(gm).take(limit) {
                println!("Natural::from({:?}) = {}", f, Natural::from(f));
            }
        }

        fn $demo_natural_checked_from_float(gm: GenerationMode, limit: usize) {
            for f in $floats(gm).take(limit) {
                println!(
                    "Natural::checked_from({:?}) = {:?}",
                    f,
                    Natural::checked_from(f)
                );
            }
        }

        fn $demo_natural_exact_from_float(gm: GenerationMode, limit: usize) {
            for f in $floats_exactly_equal_to_natural(gm).take(limit) {
                println!("Natural::exact_from({:?}) = {}", f, Natural::exact_from(f));
            }
        }

        fn $demo_natural_convertible_from_float(gm: GenerationMode, limit: usize) {
            for f in $floats(gm).take(limit) {
                println!(
                    "{} is {}convertible to a Natural",
                    f,
                    if Natural::convertible_from(f) {
                        ""
                    } else {
                        "not "
                    },
                );
            }
        }

        fn $benchmark_natural_rounding_from_float(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural::rounding_from({}, RoundingMode)", stringify!($f)),
                BenchmarkType::Single,
                $pairs_of_float_and_rounding_mode_var_1(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(f, _)| usize::exact_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [(
                    "malachite",
                    &mut (|(f, rm)| no_out!(Natural::rounding_from(f, rm))),
                )],
            );
        }

        fn $benchmark_natural_from_float(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("Natural::from({})", stringify!($f)),
                BenchmarkType::Single,
                $floats_var_1(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| usize::exact_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [("malachite", &mut (|f| no_out!(Natural::from(f))))],
            );
        }

        fn $benchmark_natural_checked_from_float(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural::checked_from({})", stringify!($f)),
                BenchmarkType::Single,
                $floats(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| usize::exact_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [("malachite", &mut (|f| no_out!(Natural::checked_from(f))))],
            );
        }

        fn $benchmark_natural_exact_from_float(gm: GenerationMode, limit: usize, file_name: &str) {
            m_run_benchmark(
                &format!("Natural::exact_from({})", stringify!($f)),
                BenchmarkType::Single,
                $floats_exactly_equal_to_natural(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| usize::exact_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [("malachite", &mut (|f| no_out!(Natural::exact_from(f))))],
            );
        }

        fn $benchmark_natural_convertible_from_float_algorithms(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Natural::convertible_from({})", stringify!($f)),
                BenchmarkType::Algorithms,
                $floats(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| usize::exact_from(f.adjusted_exponent())),
                "f.adjusted_exponent()",
                &mut [
                    ("standard", &mut (|f| no_out!(Natural::convertible_from(f)))),
                    (
                        "using checked_from",
                        &mut (|f| no_out!(Natural::checked_from(f).is_some())),
                    ),
                ],
            );
        }
    };
}

float_demos_and_benches!(
    f32,
    f32s,
    f32s_exactly_equal_to_natural,
    f32s_var_1,
    pairs_of_finite_f32_and_rounding_mode_var_1,
    demo_natural_rounding_from_f32,
    demo_natural_from_f32,
    demo_natural_checked_from_f32,
    demo_natural_exact_from_f32,
    demo_natural_convertible_from_f32,
    benchmark_natural_rounding_from_f32,
    benchmark_natural_from_f32,
    benchmark_natural_checked_from_f32,
    benchmark_natural_exact_from_f32,
    benchmark_natural_convertible_from_f32_algorithms,
);

float_demos_and_benches!(
    f64,
    f64s,
    f64s_exactly_equal_to_natural,
    f64s_var_1,
    pairs_of_finite_f64_and_rounding_mode_var_1,
    demo_natural_rounding_from_f64,
    demo_natural_from_f64,
    demo_natural_checked_from_f64,
    demo_natural_exact_from_f64,
    demo_natural_convertible_from_f64,
    benchmark_natural_rounding_from_f64,
    benchmark_natural_from_f64,
    benchmark_natural_checked_from_f64,
    benchmark_natural_exact_from_f64,
    benchmark_natural_convertible_from_f64_algorithms,
);
