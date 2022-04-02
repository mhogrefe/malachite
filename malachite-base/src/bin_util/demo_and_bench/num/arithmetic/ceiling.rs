use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_ceiling_assign);
    register_primitive_float_benches!(runner, benchmark_ceiling_assign);
}

fn demo_ceiling_assign<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for mut f in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        let old_f = f;
        f.ceiling_assign();
        println!(
            "i := {}; i.ceiling_assign(); i = {}",
            NiceFloat(old_f),
            NiceFloat(f)
        );
    }
}

fn benchmark_ceiling_assign<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.ceiling_assign()", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |mut f| f.ceiling_assign())],
    );
}
