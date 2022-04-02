use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_is_power_of_2);
    register_primitive_float_benches!(runner, benchmark_is_power_of_2);
}

fn demo_is_power_of_2<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for f in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        if f.is_power_of_2() {
            println!("{} is a power of 2", NiceFloat(f));
        } else {
            println!("{} is not a power of 2", NiceFloat(f));
        }
    }
}

fn benchmark_is_power_of_2<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.is_power_of_2()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(f.is_power_of_2()))],
    );
}
