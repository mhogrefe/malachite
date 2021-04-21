use malachite_base::num::arithmetic::traits::{
    ModNeg, ModPowerOf2, ModPowerOf2Neg, ModPowerOf2NegAssign, PowerOf2,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::pairs_of_natural_and_u64_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_power_of_2_neg_assign);
    register_demo!(registry, demo_natural_mod_power_of_2_neg);
    register_demo!(registry, demo_natural_mod_power_of_2_neg_ref);
    register_bench!(registry, Large, benchmark_natural_mod_power_of_2_neg_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_neg_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_power_of_2_neg_algorithms
    );
}

fn demo_natural_mod_power_of_2_neg_assign(gm: GenerationMode, limit: usize) {
    for (mut n, pow) in pairs_of_natural_and_u64_var_1(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_2_neg_assign(pow);
        println!(
            "x := {}; x.mod_power_of_2_neg_assign({}); x = {}",
            n_old, pow, n
        );
    }
}

fn demo_natural_mod_power_of_2_neg(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_u64_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!("-{} === {} mod 2^{}", n_old, n.mod_power_of_2_neg(pow), pow);
    }
}

fn demo_natural_mod_power_of_2_neg_ref(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_u64_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "-(&{}) === {} mod 2^{}",
            n_old,
            n.mod_power_of_2_neg(pow),
            pow
        );
    }
}

fn benchmark_natural_mod_power_of_2_neg_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.mod_power_of_2_neg_assign(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [(
            "Natural.mod_power_of_2_neg_assign(u64)",
            &mut (|(mut n, pow)| n.mod_power_of_2_neg_assign(pow)),
        )],
    );
}

fn benchmark_natural_mod_power_of_2_neg_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_neg(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_2_neg(u64)",
                &mut (|(n, pow)| no_out!(n.mod_power_of_2_neg(pow))),
            ),
            (
                "(&Natural).mod_power_of_2_neg(u64)",
                &mut (|(n, pow)| no_out!((&n).mod_power_of_2_neg(pow))),
            ),
        ],
    );
}

fn benchmark_natural_mod_power_of_2_neg_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_power_of_2_neg(u64)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, pow)| usize::exact_from(pow)),
        "pow",
        &mut [
            (
                "Natural.mod_power_of_2_neg(u64)",
                &mut (|(n, pow)| no_out!(n.mod_power_of_2_neg(pow))),
            ),
            (
                "(-Natural).mod_power_of_2(u64)",
                &mut (|(n, pow)| no_out!((-n).mod_power_of_2(pow))),
            ),
            (
                "Natural.mod_neg(Natural::power_of_2(u64))",
                &mut (|(n, pow)| no_out!(n.mod_neg(Natural::power_of_2(pow)))),
            ),
        ],
    );
}
