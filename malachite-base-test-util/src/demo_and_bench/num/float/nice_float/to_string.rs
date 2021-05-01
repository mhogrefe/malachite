use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::primitive_float_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_nice_float_to_string);
    register_primitive_float_benches!(runner, benchmark_nice_float_to_string_algorithms);
}

fn demo_nice_float_to_string<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for f in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        println!("{}", NiceFloat(f));
    }
}

fn benchmark_nice_float_to_string_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("NiceFloat::<{}>.to_string()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("Malachite", &mut |f| no_out!(NiceFloat(f).to_string())),
            ("Rust default", &mut |f| no_out!(f.to_string())),
        ],
    );
}
