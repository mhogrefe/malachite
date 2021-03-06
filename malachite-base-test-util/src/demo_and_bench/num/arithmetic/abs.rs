use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base_test_util::bench::bucketers::signed_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, signed_gen_var_1};
use malachite_base_test_util::runner::Runner;
use std::fmt::Display;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_abs_assign);
    register_signed_demos!(runner, demo_unsigned_abs);
    register_signed_benches!(runner, benchmark_abs_assign);
    register_signed_benches!(runner, benchmark_unsigned_abs);
}

fn demo_abs_assign<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for mut i in signed_gen_var_1::<T>().get(gm, &config).take(limit) {
        let old_i = i;
        i.abs_assign();
        println!("i := {}; i.abs_assign(); i = {}", old_i, i);
    }
}

fn demo_unsigned_abs<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize)
where
    <T as UnsignedAbs>::Output: Display,
{
    for i in signed_gen::<T>().get(gm, &config).take(limit) {
        println!("{}.unsigned_abs() = {}", i, i.unsigned_abs());
    }
}

fn benchmark_abs_assign<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.abs_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen_var_1::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut i| i.abs_assign())],
    );
}

fn benchmark_unsigned_abs<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.unsigned_abs()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |i| no_out!(i.unsigned_abs()))],
    );
}
