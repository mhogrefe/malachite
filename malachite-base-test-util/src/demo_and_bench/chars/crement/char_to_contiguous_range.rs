use malachite_base::chars::crement::char_to_contiguous_range;

use malachite_base_test_util::bench::bucketers::char_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::char_gen;
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_char_to_contiguous_range);
    register_bench!(runner, benchmark_char_to_contiguous_range);
}

fn demo_char_to_contiguous_range(gm: GenMode, config: GenConfig, limit: usize) {
    for c in char_gen().get(gm, &config).take(limit) {
        println!(
            "char_to_contiguous_range({:?}) = {}",
            c,
            char_to_contiguous_range(c)
        );
    }
}

fn benchmark_char_to_contiguous_range(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "char_to_contiguous_range(char)",
        BenchmarkType::Single,
        char_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &char_bucketer(),
        &mut [("Malachite", &mut (|c| no_out!(char_to_contiguous_range(c))))],
    );
}
