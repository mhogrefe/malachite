use crate::bench::bucketers::{
    pair_natural_max_bit_bucketer, triple_3_pair_natural_max_bit_bucketer,
};
use malachite_base::num::arithmetic::traits::{Lcm, LcmAssign};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::{natural_pair_gen, natural_pair_gen_nrm};
use num::Integer;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_lcm);
    register_demo!(runner, demo_natural_lcm_val_ref);
    register_demo!(runner, demo_natural_lcm_ref_val);
    register_demo!(runner, demo_natural_lcm_ref_ref);
    register_demo!(runner, demo_natural_lcm_assign);
    register_demo!(runner, demo_natural_lcm_assign_ref);

    register_bench!(runner, benchmark_natural_lcm_library_comparison);
    register_bench!(runner, benchmark_natural_lcm_evaluation_strategy);
    register_bench!(runner, benchmark_natural_lcm_assign_evaluation_strategy);
}

fn demo_natural_lcm(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.lcm({}) = {}", x_old, y_old, x.lcm(y));
    }
}

fn demo_natural_lcm_val_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        println!("{}.lcm(&{}) = {}", x_old, y, x.lcm(&y));
    }
}

fn demo_natural_lcm_ref_val(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let y_old = y.clone();
        println!("(&{}).lcm({}) = {}", x, y_old, (&x).lcm(y));
    }
}

fn demo_natural_lcm_ref_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        println!("(&{}).lcm(&{}) = {}", x, y, (&x).lcm(&y));
    }
}

fn demo_natural_lcm_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.lcm_assign(y);
        println!("x := {}; x.lcm_assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_natural_lcm_assign_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        x.lcm_assign(&y);
        println!("x := {}; x.lcm_assign(&{}); x = {}", x_old, y, x);
    }
}

fn benchmark_natural_lcm_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.lcm(Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.lcm(y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.lcm(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.lcm(&y))),
        ],
    );
}

fn benchmark_natural_lcm_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.lcm(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.lcm(Natural)", &mut |(x, y)| no_out!(x.lcm(y))),
            ("Natural.lcm(&Natural)", &mut |(x, y)| no_out!(x.lcm(&y))),
            ("&Natural.lcm(Natural)", &mut |(x, y)| no_out!((&x).lcm(y))),
            (
                "&Natural.lcm(&Natural)",
                &mut |(x, y)| no_out!((&x).lcm(&y)),
            ),
        ],
    );
}

fn benchmark_natural_lcm_assign_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.lcm_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.lcm(Natural)", &mut |(x, y)| no_out!(x.lcm(y))),
            ("Natural.lcm(&Natural)", &mut |(x, y)| no_out!(x.lcm(&y))),
        ],
    );
}
