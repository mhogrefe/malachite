use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::primitive_float_gen_var_12;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_sci_mantissa);
    register_primitive_float_demos!(runner, demo_sci_exponent);
    register_primitive_float_benches!(runner, benchmark_sci_mantissa_and_exponent_algorithms);
    register_primitive_float_benches!(runner, benchmark_sci_mantissa_algorithms);
    register_primitive_float_benches!(runner, benchmark_sci_exponent_algorithms);
}

fn demo_sci_mantissa_and_exponent<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let (m, e) = x.sci_mantissa_and_exponent();
        println!(
            "sci_mantissa_and_exponent({}) = {:?}",
            NiceFloat(x),
            (NiceFloat(m), e)
        );
    }
}

fn demo_sci_mantissa<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "sci_mantissa({}) = {}",
            NiceFloat(x),
            NiceFloat(x.sci_mantissa())
        );
    }
}

fn demo_sci_exponent<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("sci_exponent({}) = {}", NiceFloat(x), x.sci_exponent());
    }
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_mantissa_and_exponent_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_mantissa_and_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.sci_mantissa_and_exponent())),
            ("alt", &mut |x| {
                no_out!((x.sci_mantissa(), x.sci_exponent()))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_mantissa_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_mantissa()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.sci_mantissa())),
            ("alt", &mut |x| no_out!(x.sci_mantissa_and_exponent().0)),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_sci_exponent_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.sci_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.sci_exponent())),
            ("alt", &mut |x| no_out!(x.sci_mantissa_and_exponent().1)),
        ],
    );
}
