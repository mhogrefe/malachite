use malachite_base::vecs::vec_delete_left;
use malachite_base_test_util::bench::bucketers::pair_1_vec_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_vec_unsigned_pair_gen_var_1;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_vec_delete_left);
    register_bench!(runner, benchmark_vec_delete_left);
}

fn demo_vec_delete_left(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, amount) in unsigned_vec_unsigned_pair_gen_var_1::<u8>()
        .get(gm, &config)
        .take(limit)
    {
        let old_xs = xs.clone();
        vec_delete_left(&mut xs, amount);
        println!(
            "xs := {:?}; vec_delete_left(&mut xs, {}); xs = {:?}",
            old_xs, amount, xs
        );
    }
}

fn benchmark_vec_delete_left(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "vec_delete_left(&mut [T], usize)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_1::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, amount)| {
            vec_delete_left(&mut xs, amount)
        })],
    );
}
