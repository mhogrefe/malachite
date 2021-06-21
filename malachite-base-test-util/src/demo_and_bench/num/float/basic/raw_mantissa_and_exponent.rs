use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::primitive_float_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::primitive_float_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_raw_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_raw_mantissa);
    register_primitive_float_demos!(runner, demo_raw_exponent);
    register_primitive_float_benches!(runner, benchmark_raw_mantissa_and_exponent_algorithms);
    register_primitive_float_benches!(runner, benchmark_raw_mantissa_algorithms);
    register_primitive_float_benches!(runner, benchmark_raw_exponent_algorithms);
}

fn demo_raw_mantissa_and_exponent<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "raw_mantissa_and_exponent({}) = {:?}",
            NiceFloat(x),
            x.raw_mantissa_and_exponent()
        );
    }
}

fn demo_raw_mantissa<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        println!("raw_mantissa({}) = {}", NiceFloat(x), x.raw_mantissa());
    }
}

fn demo_raw_exponent<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        println!("raw_exponent({}) = {}", NiceFloat(x), x.raw_exponent());
    }
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_raw_mantissa_and_exponent_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.raw_mantissa_and_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.raw_mantissa_and_exponent())),
            ("alt", &mut |x| {
                no_out!((x.raw_mantissa(), x.raw_exponent()))
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_raw_mantissa_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.raw_mantissa()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.raw_mantissa())),
            ("alt", &mut |x| no_out!(x.raw_mantissa_and_exponent().0)),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_raw_exponent_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.raw_exponent()", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("default", &mut |x| no_out!(x.raw_exponent())),
            ("alt", &mut |x| no_out!(x.raw_mantissa_and_exponent().1)),
        ],
    );
}
