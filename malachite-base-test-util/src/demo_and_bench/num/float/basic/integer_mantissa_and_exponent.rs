use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::primitive_float_gen_var_12;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_integer_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_integer_mantissa);
    register_primitive_float_demos!(runner, demo_integer_exponent);
    register_primitive_float_benches!(runner, benchmark_integer_mantissa_and_exponent_algorithms);
    register_primitive_float_benches!(runner, benchmark_integer_mantissa_algorithms);
    register_primitive_float_benches!(runner, benchmark_integer_exponent_algorithms);
}

fn demo_integer_mantissa_and_exponent<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "integer_mantissa_and_exponent({}) = {:?}",
            NiceFloat(x),
            x.integer_mantissa_and_exponent()
        );
    }
}

fn demo_integer_mantissa<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "integer_mantissa({}) = {}",
            NiceFloat(x),
            x.integer_mantissa()
        );
    }
}

fn demo_integer_exponent<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen_var_12::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "integer_exponent({}) = {}",
            NiceFloat(x),
            x.integer_exponent()
        );
    }
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_mantissa_and_exponent_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_mantissa_and_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| {
                no_out!(x.integer_mantissa_and_exponent())
            }),
            ("alt", &mut |x| {
                no_out!((x.integer_mantissa(), x.integer_exponent()))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_mantissa_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_mantissa()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.integer_mantissa())),
            ("alt", &mut |x| no_out!(x.integer_mantissa_and_exponent().0)),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_integer_exponent_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.integer_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen_var_12::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.integer_exponent())),
            ("alt", &mut |x| no_out!(x.integer_mantissa_and_exponent().1)),
        ],
    );
}
