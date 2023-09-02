use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::bench::bucketers::pair_sum_vec_len_bucketer;
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::generators::unsigned_vec_pair_gen;
use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_from_vecs);
    register_demo!(runner, demo_rational_sequence_from_slices);

    register_bench!(
        runner,
        benchmark_rational_sequence_from_vecs_evaluation_strategy
    );
}

fn demo_rational_sequence_from_vecs(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen::<u8>().get(gm, config).take(limit) {
        println!(
            "from_vecs({:?}, {:?}) = {}",
            xs.clone(),
            ys.clone(),
            RationalSequence::from_vecs(xs, ys)
        );
    }
}

fn demo_rational_sequence_from_slices(gm: GenMode, config: &GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen::<u8>().get(gm, config).take(limit) {
        println!(
            "from_slices(&{:?}, &{:?}) = {}",
            xs,
            ys,
            RationalSequence::from_slices(&xs, &ys)
        );
    }
}

fn benchmark_rational_sequence_from_vecs_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence::from_vecs(Vec<T>, Vec<T>)",
        BenchmarkType::EvaluationStrategy,
        unsigned_vec_pair_gen::<u8>().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_sum_vec_len_bucketer("xs", "ys"),
        &mut [
            ("from_vecs", &mut |(xs, ys)| {
                no_out!(RationalSequence::from_vecs(xs, ys))
            }),
            ("from_slices", &mut |(xs, ys)| {
                no_out!(RationalSequence::from_slices(&xs, &ys))
            }),
        ],
    );
}
