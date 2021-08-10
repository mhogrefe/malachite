use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::signed_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::signed_gen_var_11;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_max_precision_for_sci_exponent);
    register_primitive_float_benches!(runner, benchmark_max_precision_for_sci_exponent);
}

fn demo_max_precision_for_sci_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for exp in signed_gen_var_11::<T>().get(gm, &config).take(limit) {
        println!(
            "{}.max_precision_for_sci_exponent() = {}",
            exp,
            T::max_precision_for_sci_exponent(exp)
        );
    }
}

fn benchmark_max_precision_for_sci_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::max_precision_for_sci_exponent(i64)", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_11::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |exp| {
            no_out!(T::max_precision_for_sci_exponent(exp))
        })],
    );
}
