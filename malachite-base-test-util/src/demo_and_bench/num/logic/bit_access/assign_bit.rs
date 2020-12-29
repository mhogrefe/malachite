use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

use malachite_base_test_util::bench::bucketers::triple_2_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_unsigned_bool_triple_gen_var_1, unsigned_unsigned_bool_triple_gen_var_1,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_assign_bit);
    register_signed_demos!(runner, demo_signed_assign_bit);
    register_unsigned_benches!(runner, benchmark_unsigned_assign_bit);
    register_signed_benches!(runner, benchmark_signed_assign_bit);
}

fn demo_unsigned_assign_bit<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, index, bit) in unsigned_unsigned_bool_triple_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n;
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn demo_signed_assign_bit<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, index, bit) in signed_unsigned_bool_triple_gen_var_1::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n;
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn benchmark_unsigned_assign_bit<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.assign_bit(u64, bool)", T::NAME),
        BenchmarkType::Single,
        unsigned_unsigned_bool_triple_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index, bit)| {
            n.assign_bit(index, bit)
        })],
    );
}

fn benchmark_signed_assign_bit<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.assign_bit(u64, bool)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_bool_triple_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index, bit)| {
            n.assign_bit(index, bit)
        })],
    );
}
