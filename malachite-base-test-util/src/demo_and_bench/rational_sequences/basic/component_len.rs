use malachite_base_test_util::bench::bucketers::rational_sequence_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_rational_sequence_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_component_len);
    register_bench!(runner, benchmark_rational_sequence_component_len);
}

fn demo_rational_sequence_component_len(gm: GenMode, config: GenConfig, limit: usize) {
    for xs in unsigned_rational_sequence_gen::<u8>()
        .get(gm, &config)
        .take(limit)
    {
        println!("component_len({}) = {}", xs, xs.component_len());
    }
}

fn benchmark_rational_sequence_component_len(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.component_len()",
        BenchmarkType::Single,
        unsigned_rational_sequence_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |xs| no_out!(xs.component_len()))],
    );
}
