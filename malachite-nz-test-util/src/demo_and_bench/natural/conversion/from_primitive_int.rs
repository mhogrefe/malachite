use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_base_test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, signed_gen_var_2, unsigned_gen};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_natural_from_unsigned);
    register_signed_demos!(runner, demo_natural_checked_from_signed);
    register_signed_demos!(runner, demo_natural_exact_from_signed);
    register_signed_demos!(runner, demo_natural_saturating_from_signed);
    register_signed_demos!(runner, demo_natural_convertible_from_signed);

    register_unsigned_benches!(runner, benchmark_natural_from_unsigned);
    register_signed_benches!(runner, benchmark_natural_checked_from_signed);
    register_signed_benches!(runner, benchmark_natural_exact_from_signed);
    register_signed_benches!(runner, benchmark_natural_saturating_from_signed);
    register_signed_benches!(runner, benchmark_natural_convertible_from_signed);
}

fn demo_natural_from_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    Natural: From<T>,
{
    for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("Natural::from({}) = {}", u, Natural::from(u));
    }
}

fn demo_natural_checked_from_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Natural: CheckedFrom<T>,
{
    for i in signed_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "Natural::checked_from({}) = {:?}",
            i,
            Natural::checked_from(i)
        );
    }
}

natural_signed_single_arg_demo_with_trait!(
    demo_natural_exact_from_signed,
    exact_from,
    signed_gen_var_2,
    ExactFrom
);
natural_signed_single_arg_demo_with_trait!(
    demo_natural_saturating_from_signed,
    saturating_from,
    signed_gen,
    SaturatingFrom
);

fn demo_natural_convertible_from_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    Natural: ConvertibleFrom<T>,
{
    for i in signed_gen::<T>().get(gm, &config).take(limit) {
        println!(
            "{} is {}convertible to a Limb",
            i,
            if Natural::convertible_from(i) {
                ""
            } else {
                "not "
            },
        );
    }
}

#[allow(unused_must_use)]
fn benchmark_natural_from_unsigned<T: PrimitiveUnsigned>(
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
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(Natural::from(u)))],
    );
}

natural_signed_single_arg_bench_with_trait!(
    benchmark_natural_checked_from_signed,
    checked_from,
    signed_gen,
    CheckedFrom
);
natural_signed_single_arg_bench_with_trait!(
    benchmark_natural_exact_from_signed,
    exact_from,
    signed_gen_var_2,
    ExactFrom
);
natural_signed_single_arg_bench_with_trait!(
    benchmark_natural_saturating_from_signed,
    saturating_from,
    signed_gen,
    SaturatingFrom
);
natural_signed_single_arg_bench_with_trait!(
    benchmark_natural_convertible_from_signed,
    convertible_from,
    signed_gen,
    ConvertibleFrom
);
