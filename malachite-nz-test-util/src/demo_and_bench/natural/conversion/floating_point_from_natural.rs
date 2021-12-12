use crate::bench::bucketers::{natural_bit_bucketer, pair_1_natural_bit_bucketer};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::generators::{
    natural_gen, natural_gen_var_3, natural_rounding_mode_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_float_rounding_from_natural);
    register_primitive_float_demos!(runner, demo_float_from_natural);
    register_primitive_float_demos!(runner, demo_float_checked_from_natural);
    register_primitive_float_demos!(runner, demo_float_exact_from_natural);
    register_primitive_float_demos!(runner, demo_float_convertible_from_natural);

    register_primitive_float_benches!(runner, benchmark_float_rounding_from_natural);
    register_primitive_float_benches!(runner, benchmark_float_from_natural);
    register_primitive_float_benches!(runner, benchmark_float_checked_from_natural);
    register_primitive_float_benches!(runner, benchmark_float_exact_from_natural);
    register_primitive_float_benches!(runner, benchmark_float_convertible_from_natural);
}

fn demo_float_rounding_from_natural<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat + for<'a> RoundingFrom<&'a Natural>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, rm) in natural_rounding_mode_pair_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}::rounding_from(&{}, {}) = {}",
            T::NAME,
            n,
            rm,
            NiceFloat(T::rounding_from(&n, rm))
        );
    }
}

fn demo_float_from_natural<T: for<'a> From<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, &config).take(limit) {
        println!(
            "{}::from(&{}) = {}",
            T::NAME,
            n.clone(),
            NiceFloat(T::from(&n))
        );
    }
}

fn demo_float_checked_from_natural<T: for<'a> CheckedFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from(&{}) = {:?}",
            T::NAME,
            n.clone(),
            T::checked_from(&n).map(NiceFloat)
        );
    }
}

fn demo_float_exact_from_natural<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Natural: From<T>,
{
    for n in natural_gen_var_3::<T>().get(gm, &config).take(limit) {
        println!(
            "{}::exact_from(&{}) = {}",
            T::NAME,
            n.clone(),
            NiceFloat(T::exact_from(&n))
        );
    }
}

fn demo_float_convertible_from_natural<T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, &config).take(limit) {
        if T::convertible_from(&n) {
            println!("{} is convertible to an {}", n, T::NAME);
        } else {
            println!("{} is not convertible to an {}", n, T::NAME);
        }
    }
}

fn benchmark_float_rounding_from_natural<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat + for<'a> RoundingFrom<&'a Natural>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::rounding_from(Natural, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        natural_rounding_mode_pair_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |(n, rm)| {
            no_out!(T::rounding_from(&n, rm))
        })],
    );
}

#[allow(unused_must_use)]
fn benchmark_float_from_natural<T: for<'a> From<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from(Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::from(&n)))],
    );
}

fn benchmark_float_checked_from_natural<T: for<'a> CheckedFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_from(Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::checked_from(&n)))],
    );
}

fn benchmark_float_exact_from_natural<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T>,
{
    run_benchmark(
        &format!("{}::exact_from(Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen_var_3::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::exact_from(&n)))],
    );
}

fn benchmark_float_convertible_from_natural<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(Natural)", T::NAME),
        BenchmarkType::Single,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [("Malachite", &mut |n| no_out!(T::convertible_from(&n)))],
    );
}
