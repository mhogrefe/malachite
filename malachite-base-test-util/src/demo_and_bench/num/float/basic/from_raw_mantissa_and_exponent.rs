use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_pair_gen_var_26;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_from_raw_mantissa_and_exponent);
    register_primitive_float_benches!(runner, benchmark_from_raw_mantissa_and_exponent);
}

fn demo_from_raw_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in unsigned_pair_gen_var_26::<T>().get(gm, &config).take(limit) {
        println!(
            "{}::from_raw_mantissa_and_exponent({}, {}) = {}",
            T::NAME,
            mantissa,
            exponent,
            NiceFloat(T::from_raw_mantissa_and_exponent(mantissa, exponent))
        );
    }
}

fn benchmark_from_raw_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_raw_mantissa_and_exponent(u64, u64)", T::NAME,),
        BenchmarkType::Single,
        unsigned_pair_gen_var_26::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_raw_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}
