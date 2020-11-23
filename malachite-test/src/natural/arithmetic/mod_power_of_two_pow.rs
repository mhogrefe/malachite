use malachite_base::num::arithmetic::traits::{ModPowerOfTwoPow, ModPowerOfTwoPowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz_test_util::natural::arithmetic::mod_power_of_two_pow::*;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::triples_of_natural_natural_and_u64_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_power_of_two_pow_assign);
    register_demo!(registry, demo_natural_mod_power_of_two_pow_assign_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_pow);
    register_demo!(registry, demo_natural_mod_power_of_two_pow_val_ref);
    register_demo!(registry, demo_natural_mod_power_of_two_pow_ref_val);
    register_demo!(registry, demo_natural_mod_power_of_two_pow_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_pow_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_pow_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_two_pow_evaluation_strategy
    );
}

fn demo_natural_mod_power_of_two_pow_assign(gm: GenerationMode, limit: usize) {
    for (mut x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        x.mod_power_of_two_pow_assign(exp, pow);
        println!(
            "x := {}; x.mod_power_of_two_pow_assign({}, {}); x = {}",
            x_old, exp_old, pow, x
        );
    }
}

fn demo_natural_mod_power_of_two_pow_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let x_old = x.clone();
        x.mod_power_of_two_pow_assign(&exp, pow);
        println!(
            "x := {}; x.mod_power_of_two_pow_assign(&{}, {}); x = {}",
            x_old, exp, pow, x
        );
    }
}

fn demo_natural_mod_power_of_two_pow(gm: GenerationMode, limit: usize) {
    for (x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x_old,
            exp_old,
            x.mod_power_of_two_pow(exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_two_pow_val_ref(gm: GenerationMode, limit: usize) {
    for (x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x_old,
            exp,
            x.mod_power_of_two_pow(&exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_two_pow_ref_val(gm: GenerationMode, limit: usize) {
    for (x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x,
            exp_old,
            (&x).mod_power_of_two_pow(exp, pow),
            pow
        );
    }
}

fn demo_natural_mod_power_of_two_pow_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, exp, pow) in triples_of_natural_natural_and_u64_var_2(gm).take(limit) {
        println!(
            "{}.pow({}) === {} mod 2^{}",
            x,
            exp,
            (&x).mod_power_of_two_pow(&exp, pow),
            pow
        );
    }
}

fn benchmark_natural_mod_power_of_two_pow_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_two_pow_assign(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_two_pow_assign(Natural, u64)",
                &mut (|(mut x, exp, pow)| no_out!(x.mod_power_of_two_pow_assign(exp, pow))),
            ),
            (
                "Natural.mod_power_of_two_pow_assign(&Natural, u64)",
                &mut (|(mut x, exp, pow)| no_out!(x.mod_power_of_two_pow_assign(&exp, pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_two_pow_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_two_pow(Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_u64_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "default",
                &mut (|(x, exp, pow)| no_out!(x.mod_power_of_two_pow(exp, pow))),
            ),
            (
                "simple binary",
                &mut (|(x, exp, pow)| no_out!(_simple_binary_mod_power_of_two_pow(&x, &exp, pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_two_pow_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_two_pow(Natural, u64)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_u64_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_two_pow(Natural, u64)",
                &mut (|(x, exp, pow)| no_out!(x.mod_power_of_two_pow(exp, pow))),
            ),
            (
                "Natural.mod_power_of_two_pow(&Natural, u64)",
                &mut (|(x, exp, pow)| no_out!(x.mod_power_of_two_pow(&exp, pow))),
            ),
            (
                "(&Natural).mod_power_of_two_pow(Natural, u64)",
                &mut (|(x, exp, pow)| no_out!((&x).mod_power_of_two_pow(exp, pow))),
            ),
            (
                "(&Natural).mod_power_of_two_pow(&Natural, u64)",
                &mut (|(x, exp, pow)| no_out!((&x).mod_power_of_two_pow(&exp, pow))),
            ),
        ],
    );
}
