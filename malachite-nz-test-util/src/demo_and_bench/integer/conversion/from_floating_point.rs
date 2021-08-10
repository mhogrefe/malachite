use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::num::float::NiceFloat;
use malachite_base_test_util::bench::bucketers::{
    pair_1_primitive_float_bucketer, primitive_float_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    primitive_float_gen, primitive_float_gen_var_5, primitive_float_gen_var_8,
    primitive_float_rounding_mode_pair_gen_var_2,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_integer_rounding_from_float);
    register_primitive_float_demos!(runner, demo_integer_from_float);
    register_primitive_float_demos!(runner, demo_integer_checked_from_float);
    register_primitive_float_demos!(runner, demo_integer_exact_from_float);
    register_primitive_float_demos!(runner, demo_integer_convertible_from_float);

    register_primitive_float_benches!(runner, benchmark_integer_rounding_from_float);
    register_primitive_float_benches!(runner, benchmark_integer_from_float);
    register_primitive_float_benches!(runner, benchmark_integer_checked_from_float);
    register_primitive_float_benches!(runner, benchmark_integer_exact_from_float);
    register_primitive_float_benches!(runner, benchmark_integer_convertible_from_float_algorithms);
}

fn demo_integer_rounding_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Integer: RoundingFrom<T>,
{
    for (f, rm) in primitive_float_rounding_mode_pair_gen_var_2::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Integer::rounding_from({}, {}) = {}",
            NiceFloat(f),
            rm,
            Integer::rounding_from(f, rm)
        );
    }
}

fn demo_integer_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Integer: From<T>,
{
    for f in primitive_float_gen_var_8::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("Integer::from({}) = {}", NiceFloat(f), Integer::from(f));
    }
}

fn demo_integer_checked_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Integer: CheckedFrom<T>,
{
    for f in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "Integer::checked_from({}) = {:?}",
            NiceFloat(f),
            Integer::checked_from(f)
        );
    }
}

fn demo_integer_exact_from_float<T: PrimitiveFloat>(gm: GenMode, config: GenConfig, limit: usize)
where
    Integer: ExactFrom<T>,
{
    for f in primitive_float_gen_var_5::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "Integer::exact_from({}) = {}",
            NiceFloat(f),
            Integer::exact_from(f)
        );
    }
}

fn demo_integer_convertible_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Integer: ConvertibleFrom<T>,
{
    for f in primitive_float_gen::<T>().get(gm, &config).take(limit) {
        if Integer::convertible_from(f) {
            println!("{} is convertible to a Integer", NiceFloat(f));
        } else {
            println!("{} is not convertible to a Integer", NiceFloat(f));
        }
    }
}

fn benchmark_integer_rounding_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: RoundingFrom<T>,
{
    run_benchmark(
        &format!("Integer::rounding_from({}, RoundingMode)", T::NAME),
        BenchmarkType::Single,
        primitive_float_rounding_mode_pair_gen_var_2::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |(f, rm)| {
            no_out!(Integer::rounding_from(f, rm))
        })],
    );
}

fn benchmark_integer_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: From<T>,
{
    run_benchmark(
        &format!("Integer::from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_8::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Integer::from(f)))],
    );
}

fn benchmark_integer_checked_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: CheckedFrom<T>,
{
    run_benchmark(
        &format!("Integer::checked_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Integer::checked_from(f)))],
    );
}

fn benchmark_integer_exact_from_float<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: ExactFrom<T>,
{
    run_benchmark(
        &format!("Integer::exact_from({})", T::NAME),
        BenchmarkType::Single,
        primitive_float_gen_var_5::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [("Malachite", &mut |f| no_out!(Integer::exact_from(f)))],
    );
}

#[allow(unused_must_use)]
fn benchmark_integer_convertible_from_float_algorithms<T: PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Integer: CheckedFrom<T> + ConvertibleFrom<T>,
{
    run_benchmark(
        &format!("Integer::convertible_from({})", T::NAME),
        BenchmarkType::Algorithms,
        primitive_float_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &primitive_float_bucketer("f"),
        &mut [
            ("standard", &mut |f| no_out!(Integer::convertible_from(f))),
            ("using checked_from", &mut |f| {
                no_out!(Integer::checked_from(f).is_some())
            }),
        ],
    );
}
