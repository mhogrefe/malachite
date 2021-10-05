use crate::bench::bucketers::triple_3_integer_bit_bucketer;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::{integer_gen, integer_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_significant_bits);
    register_bench!(
        runner,
        benchmark_integer_significant_bits_library_comparison
    );
}

fn demo_integer_significant_bits(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("significant_bits({}) = {}", n, n.significant_bits());
    }
}

fn benchmark_integer_significant_bits_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.significant_bits()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("n"),
        &mut [
            (
                "Malachite",
                &mut (|(_, _, n)| no_out!(n.significant_bits())),
            ),
            ("num", &mut (|(n, _, _)| no_out!(n.bits()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.significant_bits()))),
        ],
    );
}
