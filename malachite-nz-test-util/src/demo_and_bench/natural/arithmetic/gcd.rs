use crate::bench::bucketers::{
    pair_natural_max_bit_bucketer, triple_3_pair_natural_max_bit_bucketer,
};
use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::arithmetic::gcd::{_gcd_binary, _gcd_euclidean};
use malachite_nz_test_util::generators::{natural_pair_gen, natural_pair_gen_nrm};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_gcd);
    register_demo!(runner, demo_natural_gcd_val_ref);
    register_demo!(runner, demo_natural_gcd_ref_val);
    register_demo!(runner, demo_natural_gcd_ref_ref);
    register_demo!(runner, demo_natural_gcd_assign);
    register_demo!(runner, demo_natural_gcd_assign_ref);

    register_bench!(runner, benchmark_natural_gcd_algorithms);
    register_bench!(runner, benchmark_natural_gcd_library_comparison);
    register_bench!(runner, benchmark_natural_gcd_evaluation_strategy);
    register_bench!(runner, benchmark_natural_gcd_assign_evaluation_strategy);
}

fn demo_natural_gcd(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.gcd({}) = {}", x_old, y_old, x.gcd(y));
    }
}

fn demo_natural_gcd_val_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        println!("{}.gcd(&{}) = {}", x_old, y, x.gcd(&y));
    }
}

fn demo_natural_gcd_ref_val(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).gcd({}) = {}", x, y_old, (&x).gcd(y));
    }
}

fn demo_natural_gcd_ref_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        println!("(&{}).gcd(&{}) = {}", x, y, (&x).gcd(&y));
    }
}

fn demo_natural_gcd_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        x.gcd_assign(y.clone());
        println!("x := {}; x.gcd_assign({}); x = {}", x_old, y, x);
    }
}

fn demo_natural_gcd_assign_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        x.gcd_assign(&y);
        println!("x := {}; x.gcd_assign(&{}); x = {}", x_old, y, x);
    }
}

fn benchmark_natural_gcd_algorithms(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Euclidean", &mut |(x, y)| no_out!(_gcd_euclidean(x, y))),
            ("binary", &mut |(x, y)| no_out!(_gcd_binary(x, y))),
        ],
    );
}

fn benchmark_natural_gcd_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.gcd(y))),
            ("num", &mut |(_, _, (x, y))| no_out!(x.gcd(y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.gcd(&y))),
        ],
    );
}

fn benchmark_natural_gcd_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.gcd(Natural)", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Natural.gcd(&Natural)", &mut |(x, y)| no_out!(x.gcd(&y))),
            ("&Natural.gcd(Natural)", &mut |(x, y)| no_out!((&x).gcd(y))),
            (
                "&Natural.gcd(&Natural)",
                &mut |(x, y)| no_out!((&x).gcd(&y)),
            ),
        ],
    );
}

fn benchmark_natural_gcd_assign_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.gcd_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.gcd(Natural)", &mut |(x, y)| no_out!(x.gcd(y))),
            ("Natural.gcd(&Natural)", &mut |(x, y)| no_out!(x.gcd(&y))),
        ],
    );
}
