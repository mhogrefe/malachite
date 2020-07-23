use malachite_base::num::arithmetic::traits::{Mod, ModNeg, ModNegAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::pairs_of_naturals_var_2;

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
    for (mut n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        n.mod_neg_assign(m);
        println!("x := {}; x.mod_neg_assign({}); x = {}", n_old, m_old, n);
    }
}

fn demo_natural_mod_neg_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        n.mod_neg_assign(&m);
        println!("x := {}; x.mod_neg_assign(&{}); x = {}", n_old, m, n);
    }
}

fn demo_natural_mod_neg(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!("{} === {} mod {}", n_old, n.mod_neg(m), m_old);
    }
}

fn demo_natural_mod_neg_val_ref(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{} === {} mod &{}", n_old, n.mod_neg(&m), m);
    }
}

fn demo_natural_mod_neg_ref_val(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let m_old = m.clone();
        println!("&{} === {} mod {}", n, (&n).mod_neg(m), m_old);
    }
}

fn demo_natural_mod_neg_ref_ref(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        println!("&{} === {} mod &{}", n, (&n).mod_neg(&m), m);
    }
}

fn benchmark_natural_mod_neg_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_neg_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_neg_assign(Natural)",
                &mut (|(mut n, m)| n.mod_neg_assign(m)),
            ),
            (
                "Natural.mod_neg_assign(&Natural)",
                &mut (|(mut n, m)| n.mod_neg_assign(&m)),
            ),
        ],
    );
}

fn benchmark_natural_mod_neg_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.mod_neg(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_neg(Natural)",
                &mut (|(n, m)| no_out!(n.mod_neg(m))),
            ),
            (
                "Natural.mod_neg(&Natural)",
                &mut (|(n, m)| no_out!(n.mod_neg(&m))),
            ),
            (
                "(&Natural).mod_neg(Natural)",
                &mut (|(n, m)| no_out!((&n).mod_neg(m))),
            ),
            (
                "(&Natural).mod_neg(&Natural)",
                &mut (|(n, m)| no_out!((&n).mod_neg(&m))),
            ),
        ],
    );
}

fn benchmark_natural_mod_neg_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural.mod_neg(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_naturals_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_neg(Natural)",
                &mut (|(n, m)| no_out!(n.mod_neg(m))),
            ),
            (
                "(-Natural).mod(Natural)",
                &mut (|(n, m)| no_out!(Natural::exact_from((-n).mod_op(Integer::from(m))))),
            ),
        ],
    );
}
