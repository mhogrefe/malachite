use malachite_base::slices::slice_set_zero;
use malachite_base_test_util::bench::bucketers::vec_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_vec_gen;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_slice_set_zero);
    register_bench!(runner, benchmark_slice_set_zero);
}

fn demo_slice_set_zero(gm: GenMode, config: GenConfig, limit: usize) {
    for mut xs in unsigned_vec_gen::<u8>().get(gm, &config).take(limit) {
        let old_xs = xs.clone();
        slice_set_zero(&mut xs);
        println!("xs := {:?}; slice_set_zero(&mut xs); xs = {:?}", old_xs, xs);
    }
}

fn benchmark_slice_set_zero(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "slice_set_zero(&mut [T])",
        BenchmarkType::Single,
        unsigned_vec_gen::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |mut xs| slice_set_zero(&mut xs))],
    );
}
