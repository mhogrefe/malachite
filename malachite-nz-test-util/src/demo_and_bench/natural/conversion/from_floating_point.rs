use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, primitive_float_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_1, primitive_float_gen_var_2,
    primitive_float_rounding_mode_pair_gen_var_1,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_natural_rounding_from_float);
    register_primitive_float_demos!(runner, demo_natural_from_float);
    register_primitive_float_demos!(runner, demo_natural_checked_from_float);
    register_primitive_float_demos!(runner, demo_natural_exact_from_float);
    register_primitive_float_demos!(runner, demo_natural_convertible_from_float);

    register_primitive_float_benches!(runner, benchmark_natural_rounding_from_float);
    register_primitive_float_benches!(runner, benchmark_natural_from_float);
    register_primitive_float_benches!(runner, benchmark_natural_checked_from_float);
    register_primitive_float_benches!(runner, benchmark_natural_exact_from_float);
    register_primitive_float_benches!(runner, benchmark_natural_convertible_from_float_algorithms);
}

fn demo_natural_rounding_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Natural: RoundingFrom<T>,
{
    for (f, rm) in primitive_float_rounding_mode_pair_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Natural::rounding_from({}, {}) = {}",
            NiceFloat(f),
            rm,
            Natural::rounding_from(f, rm)
        );
    }
}

fn demo_natural_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Natural: From<T>,
{
    for f in primitive_float_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("Natural::from({}) = {}", NiceFloat(f), Natural::from(f));
    }
}

fn demo_natural_checked_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Natural: CheckedFrom<T>,
{
    for f in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "Natural::checked_from({}) = {:?}",
            NiceFloat(f),
            Natural::checked_from(f)
        );
    }
}

fn demo_natural_exact_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Natural: ExactFrom<T>,
{
    for f in primitive_float_gen_var_2::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Natural::exact_from({}) = {}",
            NiceFloat(f),
            Natural::exact_from(f)
        );
    }
}

fn demo_natural_convertible_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Natural: ConvertibleFrom<T>,
{
    for f in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        if Natural::convertible_from(f) {
            println!("{} is convertible to a Natural", NiceFloat(f));
        } else {
            println!("{} is not convertible to a Natural", NiceFloat(f));
        }
    }
}

fn benchmark_natural_rounding_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: RoundingFrom<T>,
{
    run_benchmark(
        &format!("Natural::rounding_from({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        primitive_float_rounding_mode_pair_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |(f, rm)| {
            no_out!(Natural::rounding_from(f, rm))
        })],
    );
}

fn benchmark_natural_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T>,
{
    run_benchmark(
        &format!("Natural::from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Natural::from(f)))],
    );
}

fn benchmark_natural_checked_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: CheckedFrom<T>,
{
    run_benchmark(
        &format!("Natural::checked_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Natural::checked_from(f)))],
    );
}

fn benchmark_natural_exact_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: ExactFrom<T>,
{
    run_benchmark(
        &format!("Natural::exact_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_2::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Natural::exact_from(f)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_natural_convertible_from_float_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: CheckedFrom<T> + ConvertibleFrom<T>,
{
    run_benchmark(
        &format!("Natural::convertible_from({})", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("standard", &mut |f| no_out!(Natural::convertible_from(f))),
            ("using checked_from", &mut |f| {
                no_out!(Natural::checked_from(f).is_some())
            }),
        ],
    );
}
