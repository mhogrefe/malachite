use malachite_base_test_util::bench::bucketers::quadruple_1_rational_sequence_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::large_type_gen_var_22;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_rational_sequence_mutate);
    register_bench!(runner, benchmark_rational_sequence_mutate);
}

fn demo_rational_sequence_mutate(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, index, y, z) in large_type_gen_var_22::<u8>().get(gm, &config).take(limit) {
        let xs_old = xs.clone();
        let out = xs.mutate(index, |x| {
            *x = y;
            z
        });
        println!(
            "xs := {}; xs.mutate({}, |x| {{ *x = {}; {} }}) = {}; xs = {}",
            xs_old, index, y, z, out, xs
        );
    }
}

fn benchmark_rational_sequence_mutate(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "RationalSequence.mutate(usize, FnOnce(&mut T) -> U)",
        BenchmarkType::Single,
        large_type_gen_var_22::<u8>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &quadruple_1_rational_sequence_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, index, y, z)| {
            no_out!(xs.mutate(index, |x| {
                *x = y;
                z
            }))
        })],
    );
}
