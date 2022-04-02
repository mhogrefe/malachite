use malachite_base::num::arithmetic::traits::{CoprimeWith, Gcd};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::arithmetic::coprime_with::{
    coprime_with_check_2, coprime_with_check_2_3, coprime_with_check_2_3_5,
};
use malachite_nz::test_util::bench::bucketers::pair_natural_max_bit_bucketer;
use malachite_nz::test_util::generators::natural_pair_gen;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_coprime_with);
    register_demo!(runner, demo_natural_coprime_with_val_ref);
    register_demo!(runner, demo_natural_coprime_with_ref_val);
    register_demo!(runner, demo_natural_coprime_with_ref_ref);

    register_bench!(runner, benchmark_natural_coprime_with_algorithms);
    register_bench!(runner, benchmark_natural_coprime_with_evaluation_strategy);
}

fn demo_natural_coprime_with(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.coprime_with(y) {
            println!("{} is coprime with {}", x_old, y_old);
        } else {
            println!("{} is not coprime with {}", x_old, y_old);
        }
    }
}

fn demo_natural_coprime_with_val_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        if x.coprime_with(&y) {
            println!("{} is coprime with {}", x_old, y);
        } else {
            println!("{} is not coprime with {}", x_old, y);
        }
    }
}

fn demo_natural_coprime_with_ref_val(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        let y_old = y.clone();
        if (&x).coprime_with(y) {
            println!("{} is coprime with {}", x, y_old);
        } else {
            println!("{} is not coprime with {}", x, y_old);
        }
    }
}

fn demo_natural_coprime_with_ref_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in natural_pair_gen().get(gm, &config).take(limit) {
        if (&x).coprime_with(&y) {
            println!("{} is coprime with {}", x, y);
        } else {
            println!("{} is not coprime with {}", x, y);
        }
    }
}

#[allow(clippy::unnecessary_operation, unused_must_use)]
fn benchmark_natural_coprime_with_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.coprime_with(Natural)",
        BenchmarkType::Algorithms,
        natural_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(x, y)| no_out!(x.coprime_with(y))),
            ("no divisibility check", &mut |(x, y)| {
                no_out!(x.gcd(y) == 1)
            }),
            ("check divisibility by 2", &mut |(x, y)| {
                no_out!(coprime_with_check_2(x, y))
            }),
            ("check divisibility by 2 and 3", &mut |(x, y)| {
                no_out!(coprime_with_check_2_3(x, y))
            }),
            ("check divisibility by 2, 3, and 5", &mut |(x, y)| {
                no_out!(coprime_with_check_2_3_5(x, y))
            }),
        ],
    );
}

fn benchmark_natural_coprime_with_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.coprime_with(Natural)",
        BenchmarkType::EvaluationStrategy,
        natural_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Natural.coprime_with(Natural)", &mut |(x, y)| {
                no_out!(x.coprime_with(y))
            }),
            ("Natural.coprime_with(&Natural)", &mut |(x, y)| {
                no_out!(x.coprime_with(&y))
            }),
            ("&Natural.coprime_with(Natural)", &mut |(x, y)| {
                no_out!((&x).coprime_with(y))
            }),
            ("&Natural.coprime_with(&Natural)", &mut |(x, y)| {
                no_out!((&x).coprime_with(&y))
            }),
        ],
    );
}
