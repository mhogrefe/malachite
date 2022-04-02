use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen_var_9;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_next_higher);
    register_primitive_float_benches!(runner, benchmark_next_higher);
}

fn demo_next_higher<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen_var_9::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "next_higher({}) = {}",
            NiceFloat(x),
            NiceFloat(x.next_higher())
        );
    }
}

fn benchmark_next_higher<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.next_higher()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_9::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |x| no_out!(x.next_higher()))],
    );
}
