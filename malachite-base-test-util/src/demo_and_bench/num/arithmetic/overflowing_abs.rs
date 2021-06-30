use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base_test_util::bench::bucketers::signed_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::signed_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_signed_demos!(runner, demo_overflowing_abs_assign);
    register_signed_benches!(runner, benchmark_overflowing_abs_assign);
}

fn demo_overflowing_abs_assign<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for mut i in signed_gen::<T>().get(gm, &config).take(limit) {
        let old_i = i;
        let overflow = i.overflowing_abs_assign();
        println!(
            "i := {}; i.overflowing_abs_assign() = {}; i = {}",
            old_i, overflow, i
        );
    }
}

fn benchmark_overflowing_abs_assign<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.overflowing_abs_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [(
            "Malachite",
            &mut |mut i| no_out!(i.overflowing_abs_assign()),
        )],
    );
}
