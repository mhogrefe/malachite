use malachite_base::conversion::{CheckedFrom, RoundingFrom};
use malachite_base::num::floats::PrimitiveFloat;
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    f32s, f32s_var_1, f64s, f64s_var_1, pairs_of_finite_f32_and_rounding_mode_var_1,
    pairs_of_finite_f64_and_rounding_mode_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_rounding_from_f32);
    register_demo!(registry, demo_natural_rounding_from_f64);
    register_demo!(registry, demo_natural_from_f32);
    register_demo!(registry, demo_natural_from_f64);
    register_demo!(registry, demo_natural_checked_from_f32);
    register_demo!(registry, demo_natural_checked_from_f64);
    register_bench!(registry, Small, benchmark_natural_rounding_from_f32);
    register_bench!(registry, Small, benchmark_natural_rounding_from_f64);
    register_bench!(registry, Small, benchmark_natural_from_f32);
    register_bench!(registry, Small, benchmark_natural_from_f64);
    register_bench!(registry, Small, benchmark_natural_checked_from_f32);
    register_bench!(registry, Small, benchmark_natural_checked_from_f64);
}

macro_rules! float_demos_and_benches {
    (
        $f: ident,
        $floats: ident,
        $floats_var_1: ident,
        $pairs_of_float_and_rounding_mode_var_1: ident,
        $demo_natural_rounding_from_float: ident,
        $demo_natural_from_float: ident,
        $demo_natural_checked_from_float: ident,
        $benchmark_natural_rounding_from_float: ident,
        $benchmark_natural_from_float: ident,
        $benchmark_natural_checked_from_float: ident,
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
                &(|&(f, _)| f.adjusted_exponent() as usize),
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
                &(|&f| f.adjusted_exponent() as usize),
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
                &(|&f| f.adjusted_exponent() as usize),
                "f.adjusted_exponent()",
                &mut [("malachite", &mut (|f| no_out!(Natural::checked_from(f))))],
            );
        }
    };
}

float_demos_and_benches!(
    f32,
    f32s,
    f32s_var_1,
    pairs_of_finite_f32_and_rounding_mode_var_1,
    demo_natural_rounding_from_f32,
    demo_natural_from_f32,
    demo_natural_checked_from_f32,
    benchmark_natural_rounding_from_f32,
    benchmark_natural_from_f32,
    benchmark_natural_checked_from_f32,
);

float_demos_and_benches!(
    f64,
    f64s,
    f64s_var_1,
    pairs_of_finite_f64_and_rounding_mode_var_1,
    demo_natural_rounding_from_f64,
    demo_natural_from_f64,
    demo_natural_checked_from_f64,
    benchmark_natural_rounding_from_f64,
    benchmark_natural_from_f64,
    benchmark_natural_checked_from_f64,
);
