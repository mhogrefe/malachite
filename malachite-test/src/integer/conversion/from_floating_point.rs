use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    f32s, f64s, finite_f32s, finite_f64s, pairs_of_finite_f32_and_rounding_mode_var_2,
    pairs_of_finite_f64_and_rounding_mode_var_2,
};
use malachite_base::conversion::{CheckedFrom, RoundingFrom};
use malachite_base::num::floats::PrimitiveFloat;
use malachite_nz::integer::Integer;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_rounding_from_f32);
    register_demo!(registry, demo_integer_rounding_from_f64);
    register_demo!(registry, demo_integer_from_f32);
    register_demo!(registry, demo_integer_from_f64);
    register_demo!(registry, demo_integer_checked_from_f32);
    register_demo!(registry, demo_integer_checked_from_f64);
    register_bench!(registry, Small, benchmark_integer_rounding_from_f32);
    register_bench!(registry, Small, benchmark_integer_rounding_from_f64);
    register_bench!(registry, Small, benchmark_integer_from_f32);
    register_bench!(registry, Small, benchmark_integer_from_f64);
    register_bench!(registry, Small, benchmark_integer_checked_from_f32);
    register_bench!(registry, Small, benchmark_integer_checked_from_f64);
}

macro_rules! float_demos_and_benches {
    (
        $f: ident,
        $floats: ident,
        $finite_floats: ident,
        $pairs_of_float_and_rounding_mode_var_2: ident,
        $demo_integer_rounding_from_float: ident,
        $demo_integer_from_float: ident,
        $demo_integer_checked_from_float: ident,
        $benchmark_integer_rounding_from_float: ident,
        $benchmark_integer_from_float: ident,
        $benchmark_integer_checked_from_float: ident,
    ) => {
        fn $demo_integer_rounding_from_float(gm: GenerationMode, limit: usize) {
            for (f, rm) in $pairs_of_float_and_rounding_mode_var_2(gm).take(limit) {
                println!(
                    "Integer::rounding_from({:?}, {}) = {:?}",
                    f,
                    rm,
                    Integer::rounding_from(f, rm)
                );
            }
        }

        fn $demo_integer_from_float(gm: GenerationMode, limit: usize) {
            for f in $finite_floats(gm).take(limit) {
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

        fn $benchmark_integer_rounding_from_float(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer::rounding_from({}, RoundingMode)", stringify!($f)),
                BenchmarkType::Single,
                $pairs_of_float_and_rounding_mode_var_2(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(f, _)| f.adjusted_exponent() as usize),
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
                $finite_floats(gm),
                gm.name(),
                limit,
                file_name,
                &(|&f| f.adjusted_exponent() as usize),
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
                &(|&f| f.adjusted_exponent() as usize),
                "f.adjusted_exponent()",
                &mut [("malachite", &mut (|f| no_out!(Integer::checked_from(f))))],
            );
        }
    };
}

float_demos_and_benches!(
    f32,
    f32s,
    finite_f32s,
    pairs_of_finite_f32_and_rounding_mode_var_2,
    demo_integer_rounding_from_f32,
    demo_integer_from_f32,
    demo_integer_checked_from_f32,
    benchmark_integer_rounding_from_f32,
    benchmark_integer_from_f32,
    benchmark_integer_checked_from_f32,
);

float_demos_and_benches!(
    f64,
    f64s,
    finite_f64s,
    pairs_of_finite_f64_and_rounding_mode_var_2,
    demo_integer_rounding_from_f64,
    demo_integer_from_f64,
    demo_integer_checked_from_f64,
    benchmark_integer_rounding_from_f64,
    benchmark_integer_from_f64,
    benchmark_integer_checked_from_f64,
);
