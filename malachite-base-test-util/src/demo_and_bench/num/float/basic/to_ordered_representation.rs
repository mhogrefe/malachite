use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::primitive_float_gen_var_11;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_to_ordered_representation);
    register_primitive_float_benches!(runner, benchmark_to_ordered_representation);
}

fn demo_to_ordered_representation<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen_var_11::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "to_ordered_representation({}) = {}",
            NiceFloat(x),
            x.to_ordered_representation()
        );
    }
}

fn benchmark_to_ordered_representation<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.to_ordered_representation()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_11::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |x| no_out!(x.to_ordered_representation()))],
    );
}
