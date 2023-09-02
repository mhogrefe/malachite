use malachite_base::num::logic::traits::LowMask;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_base::test_util::runner::Runner;
use malachite_nz::integer::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_low_mask);
    register_bench!(runner, benchmark_integer_low_mask);
}

fn demo_integer_low_mask(gm: GenMode, config: &GenConfig, limit: usize) {
    for bits in unsigned_gen_var_5().get(gm, config).take(limit) {
        println!("Integer::low_mask({}) = {}", bits, Integer::low_mask(bits));
    }
}

fn benchmark_integer_low_mask(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.low_mask(u64)",
        BenchmarkType::Single,
        unsigned_gen_var_5().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |bits| no_out!(Integer::low_mask(bits)))],
    );
}
