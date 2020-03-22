use malachite_base::num::arithmetic::traits::{Mod, ModNeg, ModNegAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::pairs_of_naturals_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_neg_assign);
    register_demo!(registry, demo_natural_mod_neg_assign_ref);
    register_demo!(registry, demo_natural_mod_neg);
    register_demo!(registry, demo_natural_mod_neg_val_ref);
    register_demo!(registry, demo_natural_mod_neg_ref_val);
    register_demo!(registry, demo_natural_mod_neg_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_neg_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_neg_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_neg_algorithms);
}

fn demo_natural_mod_neg_assign(gm: GenerationMode, limit: usize) {
    for (mut n, modulus) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        let modulus_old = modulus.clone();
        n.mod_neg_assign(modulus);
        println!(
            "x := {}; x.mod_neg_assign({}); x = {}",
            n_old, modulus_old, n
        );
    }
}

fn demo_natural_mod_neg_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut n, modulus) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        n.mod_neg_assign(&modulus);
        println!("x := {}; x.mod_neg_assign(&{}); x = {}", n_old, modulus, n);
    }
}

fn demo_natural_mod_neg(gm: GenerationMode, limit: usize) {
    for (n, modulus) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        let modulus_old = modulus.clone();
        println!("{} === {} mod {}", n_old, n.mod_neg(modulus), modulus_old);
    }
}

fn demo_natural_mod_neg_val_ref(gm: GenerationMode, limit: usize) {
    for (n, modulus) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{} === {} mod &{}", n_old, n.mod_neg(&modulus), modulus);
    }
}

fn demo_natural_mod_neg_ref_val(gm: GenerationMode, limit: usize) {
    for (n, modulus) in pairs_of_naturals_var_2(gm).take(limit) {
        let modulus_old = modulus.clone();
        println!("&{} === {} mod {}", n, (&n).mod_neg(modulus), modulus_old);
    }
}

fn demo_natural_mod_neg_ref_ref(gm: GenerationMode, limit: usize) {
    for (n, modulus) in pairs_of_naturals_var_2(gm).take(limit) {
        println!("&{} === {} mod &{}", n, (&n).mod_neg(&modulus), modulus);
    }
}

fn benchmark_natural_mod_neg_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_neg_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, modulus)| usize::exact_from(modulus.significant_bits())),
        "modulus.significant_bits()",
        &mut [
            (
                "Natural.mod_neg_assign(Natural)",
                &mut (|(mut n, modulus)| n.mod_neg_assign(modulus)),
            ),
            (
                "Natural.mod_neg_assign(&Natural)",
                &mut (|(mut n, modulus)| n.mod_neg_assign(&modulus)),
            ),
        ],
    );
}

fn benchmark_natural_mod_neg_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_neg(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, modulus)| usize::exact_from(modulus.significant_bits())),
        "modulus.significant_bits()",
        &mut [
            (
                "Natural.mod_neg(Natural)",
                &mut (|(n, modulus)| no_out!(n.mod_neg(modulus))),
            ),
            (
                "Natural.mod_neg(&Natural)",
                &mut (|(n, modulus)| no_out!(n.mod_neg(&modulus))),
            ),
            (
                "(&Natural).mod_neg(Natural)",
                &mut (|(n, modulus)| no_out!((&n).mod_neg(modulus))),
            ),
            (
                "(&Natural).mod_neg(&Natural)",
                &mut (|(n, modulus)| no_out!((&n).mod_neg(&modulus))),
            ),
        ],
    );
}

fn benchmark_natural_mod_neg_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_neg(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_naturals_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, modulus)| usize::exact_from(modulus.significant_bits())),
        "modulus.significant_bits()",
        &mut [
            (
                "Natural.mod_neg(Natural)",
                &mut (|(n, modulus)| no_out!(n.mod_neg(modulus))),
            ),
            (
                "(-Natural).mod(Natural)",
                &mut (|(n, modulus)| {
                    no_out!(Natural::exact_from((-n).mod_op(Integer::from(modulus))))
                }),
            ),
        ],
    );
}
