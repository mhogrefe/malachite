use malachite_base::vecs::vec_pad_left;
use malachite_base_test_util::bench::bucketers::triple_1_vec_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_vec_unsigned_unsigned_triple_gen_var_1;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_vec_pad_left);
    register_bench!(runner, benchmark_vec_pad_left);
}

fn demo_vec_pad_left(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, pad_size, pad_value) in
        unsigned_vec_unsigned_unsigned_triple_gen_var_1::<u8, usize, u8>()
            .get(gm, &config)
            .take(limit)
    {
        let old_xs = xs.clone();
        vec_pad_left(&mut xs, pad_size, pad_value);
        println!(
            "xs := {:?}; vec_pad_left(&mut xs, {}, {}); xs = {:?}",
            old_xs, pad_size, pad_value, xs
        );
    }
}

fn benchmark_vec_pad_left(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "vec_pad_left(&mut [T], usize, T)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_unsigned_triple_gen_var_1::<u8, usize, u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, pad_size, pad_value)| {
            vec_pad_left(&mut xs, pad_size, pad_value)
        })],
    );
}
