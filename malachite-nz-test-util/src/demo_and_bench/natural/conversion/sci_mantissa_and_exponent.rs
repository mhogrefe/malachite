use crate::bench::bucketers::natural_bit_bucketer;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::natural_gen_var_2;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent);

    register_primitive_float_benches!(runner, benchmark_sci_mantissa_and_exponent);
}

fn demo_sci_mantissa_and_exponent<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for n in natural_gen_var_2().get(gm, &config).take(limit) {
        let (mantissa, exponent) = n.sci_mantissa_and_exponent::<T>();
        println!(
            "sci_mantissa_and_exponent::<{}>({}) = {:?}",
            T::NAME,
            n,
            (NiceFloat(mantissa), exponent)
        );
    }
}

fn benchmark_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("Natural.sci_mantissa_and_exponent::<{}>()", T::NAME),
        BenchmarkType::Single,
        natural_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| {
            no_out!(n.sci_mantissa_and_exponent::<T>())
        })],
    );
}
