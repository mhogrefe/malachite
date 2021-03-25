use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::pair_2_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_unsigned_pair_gen_var_3, unsigned_pair_gen_var_3,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_set_bit_unsigned);
    register_signed_demos!(runner, demo_set_bit_signed);
    register_unsigned_benches!(runner, benchmark_set_bit_unsigned);
    register_signed_benches!(runner, benchmark_set_bit_signed);
}

fn demo_set_bit_unsigned<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, index) in unsigned_pair_gen_var_3::<T>().get(gm, &config).take(limit) {
        let n_old = n;
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_set_bit_signed<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut n, index) in signed_unsigned_pair_gen_var_3::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let n_old = n;
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_set_bit_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.set_bit(u64)", T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen_var_3::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index)| n.set_bit(index))],
    );
}

fn benchmark_set_bit_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.set_bit(u64)", T::NAME),
        BenchmarkType::Single,
        signed_unsigned_pair_gen_var_3::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_bucketer("index"),
        &mut [("Malachite", &mut |(mut n, index)| n.set_bit(index))],
    );
}
