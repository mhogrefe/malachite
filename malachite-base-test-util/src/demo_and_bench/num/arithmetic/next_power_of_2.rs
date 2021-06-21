use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::unsigned_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_gen_var_14;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_next_power_of_2_assign);
    register_unsigned_benches!(runner, benchmark_next_power_of_2_assign);
}

fn demo_next_power_of_2_assign<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for mut n in unsigned_gen_var_14::<T>().get(gm, &config).take(limit) {
        let old_n = n;
        n.next_power_of_2_assign();
        println!("n := {}; n.next_power_of_2_assign(); n = {}", old_n, n);
    }
}

fn benchmark_next_power_of_2_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.next_power_of_2_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen_var_14::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut n| n.next_power_of_2_assign())],
    );
}
