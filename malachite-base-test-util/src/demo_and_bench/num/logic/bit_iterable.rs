use itertools::Itertools;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::bucketers::{pair_1_bit_bucketer, unsigned_bit_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_gen, signed_unsigned_pair_gen_var_1, unsigned_gen, unsigned_pair_gen_var_2,
};
use malachite_base_test_util::runner::Runner;
use std::ops::Index;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_bits);
    register_signed_demos!(runner, demo_signed_bits);
    register_unsigned_demos!(runner, demo_unsigned_bits_rev);
    register_signed_demos!(runner, demo_signed_bits_rev);
    register_unsigned_demos!(runner, demo_unsigned_bits_size_hint);
    register_signed_demos!(runner, demo_signed_bits_index);

    register_unsigned_benches!(runner, benchmark_unsigned_bits_size_hint);
    register_unsigned_benches!(runner, benchmark_unsigned_bits_get_algorithms);
    register_signed_benches!(runner, benchmark_signed_bits_get_algorithms);
}

fn demo_unsigned_bits<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("bits({}) = {:?}", u, u.bits().collect_vec());
    }
}

fn demo_signed_bits<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, &config).take(limit) {
        println!("bits({}) = {:?}", i, i.bits().collect_vec());
    }
}

fn demo_unsigned_bits_rev<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("bits({}).rev() = {:?}", u, u.bits().rev().collect_vec());
    }
}

fn demo_signed_bits_rev<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for i in signed_gen::<T>().get(gm, &config).take(limit) {
        println!("bits({}).rev() = {:?}", i, i.bits().rev().collect_vec());
    }
}

fn demo_unsigned_bits_size_hint<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("bits({}).size_hint() = {:?}", u, u.bits().size_hint());
    }
}

fn demo_signed_bits_index<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    T::BitIterator: Index<u64, Output = bool>,
{
    for (n, i) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, &config)
        .take(limit)
    {
        println!("bits({})[{}] = {:?}", n, i, n.bits()[i]);
    }
}

fn benchmark_unsigned_bits_size_hint<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.bits().size_hint()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [(&format!("{}.bits().size_hint()", T::NAME), &mut |n| {
            no_out!(n.bits().size_hint())
        })],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_unsigned_bits_get_algorithms<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    T::BitIterator: Index<u64, Output = bool>,
{
    run_benchmark(
        &format!("{}.bits()[u64]", T::NAME),
        BenchmarkType::Algorithms,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            (&format!("{}.bits()[u]", T::NAME), &mut |(n, u)| {
                no_out!(n.bits()[u])
            }),
            (&format!("{}.to_bits_asc()[u]", T::NAME), &mut |(n, u)| {
                let bits = n.to_bits_asc();
                let u = usize::exact_from(u);
                if u >= bits.len() {
                    n < T::ZERO
                } else {
                    bits[u]
                };
            }),
        ],
    );
}

#[allow(clippy::unnecessary_operation)]
fn benchmark_signed_bits_get_algorithms<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) where
    T::BitIterator: Index<u64, Output = bool>,
{
    run_benchmark(
        &format!("{}.bits()[u64]", T::NAME),
        BenchmarkType::Algorithms,
        signed_unsigned_pair_gen_var_1::<T, u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_bit_bucketer("n"),
        &mut [
            (&format!("{}.bits()[u]", T::NAME), &mut |(n, u)| {
                no_out!(n.bits()[u])
            }),
            (&format!("{}.to_bits_asc()[u]", T::NAME), &mut |(n, u)| {
                let bits = n.to_bits_asc();
                let u = usize::exact_from(u);
                if u >= bits.len() {
                    n < T::ZERO
                } else {
                    bits[u]
                };
            }),
        ],
    );
}
