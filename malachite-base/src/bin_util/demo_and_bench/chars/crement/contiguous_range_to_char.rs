use malachite_base::chars::crement::contiguous_range_to_char;
use malachite_base::test_util::bench::bucketers::unsigned_direct_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_contiguous_range_to_char);
    register_bench!(runner, benchmark_contiguous_range_to_char);
}

fn demo_contiguous_range_to_char(gm: GenMode, config: &GenConfig, limit: usize) {
    for u in unsigned_gen().get(gm, config).take(limit) {
        println!(
            "contiguous_range_to_char({}) = {:?}",
            u,
            contiguous_range_to_char(u)
        );
    }
}

fn benchmark_contiguous_range_to_char(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "contiguous_range_to_char(u32)",
        BenchmarkType::Single,
        unsigned_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &unsigned_direct_bucketer(),
        &mut [("Malachite", &mut |u| no_out!(contiguous_range_to_char(u)))],
    );
}
