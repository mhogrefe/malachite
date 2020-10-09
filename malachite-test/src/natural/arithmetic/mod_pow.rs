use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::triples_of_naturals_var_5;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_pow_assign);
    register_demo!(registry, demo_natural_mod_pow_assign_val_ref);
    register_demo!(registry, demo_natural_mod_pow_assign_ref_val);
    register_demo!(registry, demo_natural_mod_pow_assign_ref_ref);
    register_demo!(registry, demo_natural_mod_pow);
    register_demo!(registry, demo_natural_mod_pow_val_val_ref);
    register_demo!(registry, demo_natural_mod_pow_val_ref_val);
    register_demo!(registry, demo_natural_mod_pow_val_ref_ref);
    register_demo!(registry, demo_natural_mod_pow_ref_val_val);
    register_demo!(registry, demo_natural_mod_pow_ref_val_ref);
    register_demo!(registry, demo_natural_mod_pow_ref_ref_val);
    register_demo!(registry, demo_natural_mod_pow_ref_ref_ref);

    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_pow_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_pow_evaluation_strategy
    );
}

fn demo_natural_mod_pow_assign(gm: GenerationMode, limit: usize) {
    for (mut x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        let m_old = m.clone();
        x.mod_pow_assign(exp, m);
        println!(
            "x := {}; x.mod_pow_assign({}, {}); x = {}",
            x_old, exp_old, m_old, x
        );
    }
}

fn demo_natural_mod_pow_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let m_old = m.clone();
        let exp_old = exp.clone();
        x.mod_pow_assign(exp, &m);
        println!(
            "x := {}; x.mod_pow_assign({}, &{}); x = {}",
            x, exp_old, m_old, x
        );
    }
}

fn demo_natural_mod_pow_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        x.mod_pow_assign(&exp, m);
        println!(
            "x := {}; x.mod_pow_assign(&{}, {}); x = {}",
            x_old, exp, m_old, x
        );
    }
}

fn demo_natural_mod_pow_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let x_old = x.clone();
        x.mod_pow_assign(&exp, &m);
        println!(
            "x := {}; x.mod_pow_assign(&{}, &{}); x = {}",
            x_old, exp, m, x
        );
    }
}

fn demo_natural_mod_pow(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        let m_old = m.clone();
        println!(
            "{}.pow({}) === {} mod {}",
            x_old,
            exp_old,
            x.mod_pow(exp, m),
            m_old
        );
    }
}

fn demo_natural_mod_pow_val_val_ref(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let x_old = x.clone();
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) === {} mod {}",
            x_old,
            exp_old,
            x.mod_pow(exp, &m),
            m
        );
    }
}

fn demo_natural_mod_pow_val_ref_val(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!(
            "{}.pow({}) === {} mod {}",
            x_old,
            exp,
            x.mod_pow(&exp, m),
            m_old
        );
    }
}

fn demo_natural_mod_pow_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.pow({}) === {} mod {}",
            x_old,
            exp,
            x.mod_pow(&exp, &m),
            m
        );
    }
}

fn demo_natural_mod_pow_ref_val_val(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let exp_old = exp.clone();
        let m_old = m.clone();
        println!(
            "{}.pow({}) === {} mod {}",
            x,
            exp_old,
            (&x).mod_pow(exp, m),
            m_old
        );
    }
}

fn demo_natural_mod_pow_ref_val_ref(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let exp_old = exp.clone();
        println!(
            "{}.pow({}) === {} mod {}",
            x,
            exp_old,
            (&x).mod_pow(exp, &m),
            m
        );
    }
}

fn demo_natural_mod_pow_ref_ref_val(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        let m_old = m.clone();
        println!(
            "{}.pow({}) === {} mod {}",
            x,
            exp,
            (&x).mod_pow(&exp, m),
            m_old
        );
    }
}

fn demo_natural_mod_pow_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, exp, m) in triples_of_naturals_var_5(gm).take(limit) {
        println!(
            "{}.pow({}) === {} mod {}",
            x,
            exp,
            (&x).mod_pow(&exp, &m),
            m
        );
    }
}

fn benchmark_natural_mod_pow_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_pow_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_5(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_pow_assign(Natural, Natural)",
                &mut (|(mut x, exp, m)| no_out!(x.mod_pow_assign(exp, m))),
            ),
            (
                "Natural.mod_pow_assign(Natural, &Natural)",
                &mut (|(mut x, exp, m)| no_out!(x.mod_pow_assign(exp, &m))),
            ),
            (
                "Natural.mod_pow_assign(&Natural, Natural)",
                &mut (|(mut x, exp, m)| no_out!(x.mod_pow_assign(&exp, m))),
            ),
            (
                "Natural.mod_pow_assign(&Natural, &Natural)",
                &mut (|(mut x, exp, m)| no_out!(x.mod_pow_assign(&exp, &m))),
            ),
        ],
    );
}

fn benchmark_natural_mod_pow_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_pow(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_5(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_pow(Natural, Natural)",
                &mut (|(x, exp, m)| no_out!(x.mod_pow(exp, m))),
            ),
            (
                "Natural.mod_pow(Natural, &Natural)",
                &mut (|(x, exp, m)| no_out!(x.mod_pow(exp, &m))),
            ),
            (
                "Natural.mod_pow(&Natural, Natural)",
                &mut (|(x, exp, m)| no_out!(x.mod_pow(&exp, m))),
            ),
            (
                "Natural.mod_pow(&Natural, &Natural)",
                &mut (|(x, exp, m)| no_out!(x.mod_pow(&exp, &m))),
            ),
            (
                "(&Natural).mod_pow(Natural, Natural)",
                &mut (|(x, exp, m)| no_out!((&x).mod_pow(exp, m))),
            ),
            (
                "(&Natural).mod_pow(Natural, &Natural)",
                &mut (|(x, exp, m)| no_out!((&x).mod_pow(exp, &m))),
            ),
            (
                "(&Natural).mod_pow(&Natural, Natural)",
                &mut (|(x, exp, m)| no_out!((&x).mod_pow(&exp, m))),
            ),
            (
                "(&Natural).mod_pow(&Natural, &Natural)",
                &mut (|(x, exp, m)| no_out!((&x).mod_pow(&exp, &m))),
            ),
        ],
    );
}
