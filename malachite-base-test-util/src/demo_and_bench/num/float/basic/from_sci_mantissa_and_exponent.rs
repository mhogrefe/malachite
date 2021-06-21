use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::pair_1_primitive_float_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    primitive_float_signed_pair_gen_var_1, primitive_float_signed_pair_gen_var_2,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_from_sci_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_from_sci_mantissa_and_exponent_targeted);
    register_primitive_float_benches!(runner, benchmark_from_sci_mantissa_and_exponent);
    register_primitive_float_benches!(runner, benchmark_from_sci_mantissa_and_exponent_targeted);
}

fn demo_from_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in primitive_float_signed_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}::from_sci_mantissa_and_exponent({}, {}) = {:?}",
            T::NAME,
            NiceFloat(mantissa),
            exponent,
            T::from_sci_mantissa_and_exponent(mantissa, exponent).map(NiceFloat)
        );
    }
}

fn demo_from_sci_mantissa_and_exponent_targeted<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mantissa, exponent) in primitive_float_signed_pair_gen_var_2::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}::from_sci_mantissa_and_exponent({}, {}) = {}",
            T::NAME,
            NiceFloat(mantissa),
            exponent,
            NiceFloat(T::from_sci_mantissa_and_exponent(mantissa, exponent).unwrap())
        );
    }
}

fn benchmark_from_sci_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_sci_mantissa_and_exponent(u64, u64)", T::NAME,),
        BenchmarkType::Single,
        primitive_float_signed_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_sci_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}

fn benchmark_from_sci_mantissa_and_exponent_targeted<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from_sci_mantissa_and_exponent(u64, u64)", T::NAME,),
        BenchmarkType::Single,
        primitive_float_signed_pair_gen_var_2::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("mantissa"),
        &mut [("Malachite", &mut |(mantissa, exponent)| {
            no_out!(T::from_sci_mantissa_and_exponent(mantissa, exponent))
        })],
    );
}
