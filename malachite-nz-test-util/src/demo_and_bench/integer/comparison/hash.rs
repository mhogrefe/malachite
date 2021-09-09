use crate::bench::bucketers::triple_3_integer_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::hash::hash;
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::{integer_gen, integer_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_hash);
    register_bench!(runner, benchmark_integer_hash_library_comparison);
}

fn demo_integer_hash(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("hash({}) = {}", n, hash(&n));
    }
}

fn benchmark_integer_hash_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer hash",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(hash(&n))),
            ("num", &mut |(_, n, _)| no_out!(hash(&n))),
            ("rug", &mut |(n, _, _)| no_out!(hash(&n))),
        ],
    );
}
