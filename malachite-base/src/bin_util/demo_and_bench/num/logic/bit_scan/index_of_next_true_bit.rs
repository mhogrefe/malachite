use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::bench::bucketers::pair_2_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, unsigned_pair_gen_var_2,
};
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_index_of_next_true_bit_unsigned);
    register_signed_demos!(runner, demo_index_of_next_true_bit_signed);
    register_unsigned_benches!(runner, benchmark_index_of_next_true_bit_unsigned);
    register_signed_benches!(runner, benchmark_index_of_next_true_bit_signed);
}

fn demo_index_of_next_true_bit_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, start) in unsigned_pair_gen_var_2::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.index_of_next_true_bit({}) = {:?}",
            n,
            start,
            n.index_of_next_true_bit(start)
        );
    }
}

fn demo_index_of_next_true_bit_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
) {
    for (n, start) in signed_unsigned_pair_gen_var_1::<T, u64>()
        .get(gm, config)
        .take(limit)
    {
        println!(
            "{}.index_of_next_true_bit({}) = {:?}",
            n,
            start,
            n.index_of_next_true_bit(start)
        );
    }
}

fn benchmark_index_of_next_true_bit_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.index_of_next_true_bit(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_2::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("start"),
        &mut [("Malachite", &mut |(n, start)| {
            no_out!(n.index_of_next_true_bit(start))
        })],
    );
}

fn benchmark_index_of_next_true_bit_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.index_of_next_true_bit(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_1::<T, u64>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(n, start)| {
            no_out!(n.index_of_next_true_bit(start))
        })],
    );
}
