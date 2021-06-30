use malachite_base::num::float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, primitive_float_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    primitive_float_gen_var_12, primitive_float_signed_pair_gen_var_1,
    primitive_float_signed_pair_gen_var_2,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_sci_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_sci_mantissa);
    register_primitive_float_demos!(runner, demo_sci_exponent);
    register_primitive_float_demos!(runner, demo_from_sci_mantissa_and_exponent);
    register_primitive_float_demos!(runner, demo_from_sci_mantissa_and_exponent_targeted);
    register_primitive_float_benches!(runner, benchmark_sci_mantissa_and_exponent_algorithms);
    register_primitive_float_benches!(runner, benchmark_sci_mantissa_algorithms);
    register_primitive_float_benches!(runner, benchmark_sci_exponent_algorithms);
    register_primitive_float_benches!(runner, benchmark_from_sci_mantissa_and_exponent);
    register_primitive_float_benches!(runner, benchmark_from_sci_mantissa_and_exponent_targeted);
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
