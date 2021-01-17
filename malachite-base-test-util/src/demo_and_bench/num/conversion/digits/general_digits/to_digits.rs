use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{Digits, SaturatingFrom};
use malachite_base_test_util::bench::bucketers::pair_1_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_pair_gen_var_6;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_unsigned_demos!(runner, demo_to_digits_asc);
    register_unsigned_unsigned_demos!(runner, demo_to_digits_desc);
    register_unsigned_unsigned_benches!(runner, benchmark_to_digits_asc);
    register_unsigned_unsigned_benches!(runner, benchmark_to_digits_desc);
}

fn demo_to_digits_asc<T: Digits<U, u64> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    u64: SaturatingFrom<T> + SaturatingFrom<U>,
{
    for (x, base) in unsigned_pair_gen_var_6::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.to_digits_asc({}) = {:?}",
            x,
            base,
            Digits::<U, u64>::to_digits_asc(&x, base)
        );
    }
}

fn demo_to_digits_desc<T: Digits<U, u64> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) where
    u64: SaturatingFrom<T> + SaturatingFrom<U>,
{
    for (x, base) in unsigned_pair_gen_var_6::<T, U>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.to_digits_desc({}) = {:?}",
            x,
            base,
            Digits::<U, u64>::to_digits_desc(&x, base)
        );
    }
}

fn benchmark_to_digits_asc<T: Digits<U, u64> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    u64: SaturatingFrom<T> + SaturatingFrom<U>,
{
    run_benchmark(
        &format!("Digits::<{}>::to_digits_asc({}, u64)", U::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_6::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base)| {
            no_out!(Digits::<U, u64>::to_digits_asc(&x, base))
        })],
    );
}

fn benchmark_to_digits_desc<T: Digits<U, u64> + PrimitiveUnsigned, U: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    u64: SaturatingFrom<T> + SaturatingFrom<U>,
{
    run_benchmark(
        &format!("Digits::<{}>::to_digits_asc({}, u64)", U::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_6::<T, U>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base)| {
            no_out!(Digits::<U, u64>::to_digits_desc(&x, base))
        })],
    );
}
