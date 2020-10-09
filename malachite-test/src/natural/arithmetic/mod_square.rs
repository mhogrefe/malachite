use malachite_base::num::arithmetic::traits::{ModSquare, ModSquareAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::pairs_of_naturals_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_square_assign);
    register_demo!(registry, demo_natural_mod_square_assign_ref);
    register_demo!(registry, demo_natural_mod_square);
    register_demo!(registry, demo_natural_mod_square_val_ref);
    register_demo!(registry, demo_natural_mod_square_ref_val);
    register_demo!(registry, demo_natural_mod_square_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_square_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_square_evaluation_strategy
    );
}

fn demo_natural_mod_square_assign(gm: GenerationMode, limit: usize) {
    for (mut n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        n.mod_square_assign(m);
        println!("x := {}; x.mod_square_assign({}); x = {}", n_old, m_old, n);
    }
}

fn demo_natural_mod_square_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        n.mod_square_assign(&m);
        println!("x := {}; x.mod_square_assign(&{}); x = {}", n_old, m, n);
    }
}

fn demo_natural_mod_square(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        let m_old = m.clone();
        println!("{}.square() === {} mod {}", n_old, n.mod_square(m), m_old);
    }
}

fn demo_natural_mod_square_val_ref(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.square() === {} mod &{}", n_old, n.mod_square(&m), m);
    }
}

fn demo_natural_mod_square_ref_val(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        let m_old = m.clone();
        println!("(&{}).square() === {} mod {}", n, (&n).mod_square(m), m_old);
    }
}

fn demo_natural_mod_square_ref_ref(gm: GenerationMode, limit: usize) {
    for (n, m) in pairs_of_naturals_var_2(gm).take(limit) {
        println!("(&{}).square() === {} mod &{}", n, (&n).mod_square(&m), m);
    }
}

fn benchmark_natural_mod_square_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_square_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_square_assign(Natural)",
                &mut (|(mut n, m)| n.mod_square_assign(m)),
            ),
            (
                "Natural.mod_square_assign(&Natural)",
                &mut (|(mut n, m)| n.mod_square_assign(&m)),
            ),
        ],
    );
}

fn benchmark_natural_mod_square_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_square(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|(_, m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_square(Natural)",
                &mut (|(n, m)| no_out!(n.mod_square(m))),
            ),
            (
                "Natural.mod_square(&Natural)",
                &mut (|(n, m)| no_out!(n.mod_square(&m))),
            ),
            (
                "(&Natural).mod_square(Natural)",
                &mut (|(n, m)| no_out!((&n).mod_square(m))),
            ),
            (
                "(&Natural).mod_square(&Natural)",
                &mut (|(n, m)| no_out!((&n).mod_square(&m))),
            ),
        ],
    );
}
