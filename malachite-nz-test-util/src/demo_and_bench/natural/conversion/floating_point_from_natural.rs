use crate::bench::bucketers::{natural_bit_bucketer, pair_1_natural_bit_bucketer};
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::generators::{
    natural_gen, natural_gen_var_3, natural_rounding_mode_pair_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_primitive_float_demos!(runner, demo_float_rounding_from_natural);
    register_primitive_float_demos!(runner, demo_float_rounding_from_natural_ref);
    register_primitive_float_demos!(runner, demo_float_from_natural);
    register_primitive_float_demos!(runner, demo_float_from_natural_ref);
    register_primitive_float_demos!(runner, demo_float_checked_from_natural);
    register_primitive_float_demos!(runner, demo_float_checked_from_natural_ref);
    register_primitive_float_demos!(runner, demo_float_exact_from_natural);
    register_primitive_float_demos!(runner, demo_float_exact_from_natural_ref);
    register_primitive_float_demos!(runner, demo_float_convertible_from_natural);
    register_primitive_float_demos!(runner, demo_float_convertible_from_natural_ref);

    register_primitive_float_benches!(
        runner,
        benchmark_float_rounding_from_natural_evaluation_strategy
    );
    register_primitive_float_benches!(runner, benchmark_float_from_natural_evaluation_strategy);
    register_primitive_float_benches!(
        runner,
        benchmark_float_checked_from_natural_evaluation_strategy
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_exact_from_natural_evaluation_strategy
    );
    register_primitive_float_benches!(
        runner,
        benchmark_float_convertible_from_natural_evaluation_strategy
    );
}

fn demo_float_rounding_from_natural<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat + RoundingFrom<Natural>,
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
            "{}::rounding_from({}, {}) = {}",
            T::NAME,
            n.clone(),
            rm,
            NiceFloat(T::rounding_from(n, rm))
        );
    }
}

fn demo_float_rounding_from_natural_ref<
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

fn demo_float_from_natural<T: From<Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, &config).take(limit) {
        println!(
            "{}::from({}) = {}",
            T::NAME,
            n.clone(),
            NiceFloat(T::from(n))
        );
    }
}

fn demo_float_from_natural_ref<T: for<'a> From<&'a Natural> + PrimitiveFloat>(
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

fn demo_float_checked_from_natural<T: CheckedFrom<Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, &config).take(limit) {
        println!(
            "{}::checked_from({}) = {:?}",
            T::NAME,
            n.clone(),
            T::checked_from(n).map(NiceFloat)
        );
    }
}

fn demo_float_checked_from_natural_ref<T: for<'a> CheckedFrom<&'a Natural> + PrimitiveFloat>(
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

fn demo_float_exact_from_natural<T: ExactFrom<Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    for n in natural_gen_var_3::<T>().get(gm, &config).take(limit) {
        println!(
            "{}::exact_from({}) = {}",
            T::NAME,
            n.clone(),
            NiceFloat(T::exact_from(n))
        );
    }
}

fn demo_float_exact_from_natural_ref<T: for<'a> ExactFrom<&'a Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
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

fn demo_float_convertible_from_natural<T: ConvertibleFrom<Natural> + PrimitiveFloat>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for n in natural_gen().get(gm, &config).take(limit) {
        if T::convertible_from(n.clone()) {
            println!("{} is convertible to an {}", n, T::NAME);
        } else {
            println!("{} is not convertible to an {}", n, T::NAME);
        }
    }
}

fn demo_float_convertible_from_natural_ref<
    T: for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>(
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

fn benchmark_float_rounding_from_natural_evaluation_strategy<
    T: for<'a> ConvertibleFrom<&'a Natural>
        + PrimitiveFloat
        + RoundingFrom<Natural>
        + for<'a> RoundingFrom<&'a Natural>,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::rounding_from(Natural, RoundingMode)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_rounding_mode_pair_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_natural_bit_bucketer("n"),
        &mut [
            (
                &format!("{}::rounding_from(Natural, RoundingMode)", T::NAME),
                &mut |(n, rm)| no_out!(T::rounding_from(n, rm)),
            ),
            (
                &format!("{}::rounding_from(&Natural, RoundingMode)", T::NAME),
                &mut |(n, rm)| no_out!(T::rounding_from(&n, rm)),
            ),
        ],
    );
}

fn benchmark_float_from_natural_evaluation_strategy<
    T: From<Natural> + for<'a> From<&'a Natural> + PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::from(Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            (&format!("{}::from(Natural)", T::NAME), &mut |n| {
                no_out!(T::from(n))
            }),
            (&format!("{}::from(&Natural)", T::NAME), &mut |n| {
                no_out!(T::from(&n))
            }),
        ],
    );
}

fn benchmark_float_checked_from_natural_evaluation_strategy<
    T: CheckedFrom<Natural> + for<'a> CheckedFrom<&'a Natural> + PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::checked_from(Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            (&format!("{}::checked_from(Natural)", T::NAME), &mut |n| {
                no_out!(T::checked_from(n))
            }),
            (&format!("{}::checked_from(&Natural)", T::NAME), &mut |n| {
                no_out!(T::checked_from(&n))
            }),
        ],
    );
}

fn benchmark_float_exact_from_natural_evaluation_strategy<
    T: ExactFrom<Natural> + for<'a> ExactFrom<&'a Natural> + PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    Natural: From<T> + From<T::UnsignedOfEqualWidth>,
{
    run_benchmark(
        &format!("{}::exact_from(Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_gen_var_3::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            (&format!("{}::exact_from(Natural)", T::NAME), &mut |n| {
                no_out!(T::exact_from(n))
            }),
            (&format!("{}::exact_from(&Natural)", T::NAME), &mut |n| {
                no_out!(T::exact_from(&n))
            }),
        ],
    );
}

fn benchmark_float_convertible_from_natural_evaluation_strategy<
    T: ConvertibleFrom<Natural> + for<'a> ConvertibleFrom<&'a Natural> + PrimitiveFloat,
>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}::convertible_from(Natural)", T::NAME),
        BenchmarkType::EvaluationStrategy,
        natural_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &natural_bit_bucketer("n"),
        &mut [
            (
                &format!("{}::convertible_from(Natural)", T::NAME),
                &mut |n| no_out!(T::convertible_from(n)),
            ),
            (
                &format!("{}::convertible_from(&Natural)", T::NAME),
                &mut |n| no_out!(T::convertible_from(&n)),
            ),
        ],
    );
}
