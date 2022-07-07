use malachite_base::num::arithmetic::traits::{JacobiSymbol, KroneckerSymbol};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::kronecker_symbol::jacobi_symbol_simple;
use malachite_nz::test_util::bench::bucketers::pair_2_natural_bit_bucketer;
use malachite_nz::test_util::generators::{natural_pair_gen, natural_pair_gen_var_12};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_jacobi_symbol);
    register_demo!(runner, demo_natural_jacobi_symbol_val_ref);
    register_demo!(runner, demo_natural_jacobi_symbol_ref_val);
    register_demo!(runner, demo_natural_jacobi_symbol_ref_ref);
    register_demo!(runner, demo_natural_kronecker_symbol);
    register_demo!(runner, demo_natural_kronecker_symbol_val_ref);
    register_demo!(runner, demo_natural_kronecker_symbol_ref_val);
    register_demo!(runner, demo_natural_kronecker_symbol_ref_ref);

    register_bench!(runner, benchmark_natural_jacobi_symbol_evaluation_strategy);
    register_bench!(runner, benchmark_natural_jacobi_symbol_algorithms);
    register_bench!(
        runner,
        benchmark_natural_kronecker_symbol_evaluation_strategy
    );
}

fn demo_natural_jacobi_symbol(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_12().get(gm, &config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!(
            "{}.jacobi_symbol({}) = {}",
            n_old,
            m_old,
            n.jacobi_symbol(m)
        );
    }
}

fn demo_natural_jacobi_symbol_val_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_12().get(gm, &config).take(limit) {
        let n_old = n.clone();
        println!("{}.jacobi_symbol({}) = {}", n_old, m, n.jacobi_symbol(&m));
    }
}

fn demo_natural_jacobi_symbol_ref_val(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_12().get(gm, &config).take(limit) {
        let m_old = m.clone();
        println!("{}.jacobi_symbol({}) = {}", n, m_old, (&n).jacobi_symbol(m));
    }
}

fn demo_natural_jacobi_symbol_ref_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen_var_12().get(gm, &config).take(limit) {
        println!("{}.jacobi_symbol({}) = {}", n, m, (&n).jacobi_symbol(&m));
    }
}

fn demo_natural_kronecker_symbol(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen().get(gm, &config).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!(
            "{}.kronecker_symbol({}) = {}",
            n_old,
            m_old,
            n.kronecker_symbol(m)
        );
    }
}

fn demo_natural_kronecker_symbol_val_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen().get(gm, &config).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.kronecker_symbol({}) = {}",
            n_old,
            m,
            n.kronecker_symbol(&m)
        );
    }
}

fn demo_natural_kronecker_symbol_ref_val(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen().get(gm, &config).take(limit) {
        let m_old = m.clone();
        println!(
            "{}.kronecker_symbol({}) = {}",
            n,
            m_old,
            (&n).kronecker_symbol(m)
        );
    }
}

fn demo_natural_kronecker_symbol_ref_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, m) in natural_pair_gen().get(gm, &config).take(limit) {
        println!(
            "{}.kronecker_symbol({}) = {}",
            n,
            m,
            (&n).kronecker_symbol(&m)
        );
    }
}

fn benchmark_natural_jacobi_symbol_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.jacobi_symbol(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen_var_12().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.jacobi_symbol(Natural)", &mut |(n, m)| {
                no_out!(n.jacobi_symbol(m))
            }),
            ("Natural.jacobi_symbol(&Natural)", &mut |(n, m)| {
                no_out!(n.jacobi_symbol(&m))
            }),
            ("(&Natural).jacobi_symbol(Natural)", &mut |(n, m)| {
                no_out!((&n).jacobi_symbol(m))
            }),
            ("(&Natural).jacobi_symbol(&Natural)", &mut |(n, m)| {
                no_out!((&n).jacobi_symbol(&m))
            }),
        ],
    );
}

fn benchmark_natural_jacobi_symbol_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.jacobi_symbol(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen_var_12().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("default", &mut |(n, m)| no_out!(n.jacobi_symbol(m))),
            ("simple", &mut |(n, m)| no_out!(jacobi_symbol_simple(n, m))),
        ],
    );
}

fn benchmark_natural_kronecker_symbol_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.kronecker_symbol(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_natural_bit_bucketer("m"),
        &mut [
            ("Natural.kronecker_symbol(Natural)", &mut |(n, m)| {
                no_out!(n.kronecker_symbol(m))
            }),
            ("Natural.kronecker_symbol(&Natural)", &mut |(n, m)| {
                no_out!(n.kronecker_symbol(&m))
            }),
            ("(&Natural).kronecker_symbol(Natural)", &mut |(n, m)| {
                no_out!((&n).kronecker_symbol(m))
            }),
            ("(&Natural).kronecker_symbol(&Natural)", &mut |(n, m)| {
                no_out!((&n).kronecker_symbol(&m))
            }),
        ],
    );
}
