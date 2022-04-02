use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::triple_3_pair_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::{natural_pair_gen, natural_pair_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_eq);
    register_bench!(runner, benchmark_natural_eq_library_comparison);
}

fn demo_natural_eq(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_eq_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural == Natural",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x == y)),
            ("num", &mut |((x, y), _, _)| no_out!(x == y)),
            ("rug", &mut |(_, (x, y), _)| no_out!(x == y)),
        ],
    );
}
