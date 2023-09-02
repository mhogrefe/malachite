use malachite_base::slices::slice_test_zero;
use malachite_base::test_util::bench::bucketers::vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_slice_test_zero);
    register_bench!(runner, benchmark_slice_test_zero);
}

fn demo_slice_test_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for xs in unsigned_vec_gen::<u8>().get(gm, config).take(limit) {
        println!("slice_test_zero({:?}) = {:?}", xs, slice_test_zero(&xs));
    }
}

fn benchmark_slice_test_zero(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "slice_test_zero(&[T])",
        BenchmarkType::Single,
        unsigned_vec_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [("Malachite", &mut |xs| no_out!(slice_test_zero(&xs)))],
    );
}
