use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::triples_of_naturals_var_4;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_add_assign);
    register_demo!(registry, demo_natural_mod_add_assign_val_ref);
    register_demo!(registry, demo_natural_mod_add_assign_ref_val);
    register_demo!(registry, demo_natural_mod_add_assign_ref_ref);
    register_demo!(registry, demo_natural_mod_add);
    register_demo!(registry, demo_natural_mod_add_val_val_ref);
    register_demo!(registry, demo_natural_mod_add_val_ref_val);
    register_demo!(registry, demo_natural_mod_add_val_ref_ref);
    register_demo!(registry, demo_natural_mod_add_ref_val_val);
    register_demo!(registry, demo_natural_mod_add_ref_val_ref);
    register_demo!(registry, demo_natural_mod_add_ref_ref_val);
    register_demo!(registry, demo_natural_mod_add_ref_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_add_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_add_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_add_evaluation_strategy
    );
}

fn demo_natural_mod_add_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        x.mod_add_assign(y, m);
        println!(
            "x := {}; x.mod_add_assign({}, {}); x = {}",
            x_old, y_old, m_old, x
        );
    }
}

fn demo_natural_mod_add_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let m_old = m.clone();
        let y_old = y.clone();
        x.mod_add_assign(y, &m);
        println!(
            "x := {}; x.mod_add_assign({}, &{}); x = {}",
            x, y_old, m_old, x
        );
    }
}

fn demo_natural_mod_add_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        x.mod_add_assign(&y, m);
        println!(
            "x := {}; x.mod_add_assign(&{}, {}); x = {}",
            x_old, y, m_old, x
        );
    }
}

fn demo_natural_mod_add_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        x.mod_add_assign(&y, &m);
        println!(
            "x := {}; x.mod_add_assign(&{}, &{}); x = {}",
            x_old, y, m, x
        );
    }
}

fn demo_natural_mod_add(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        println!(
            "{} + {} === {} mod {}",
            x_old,
            y_old,
            x.mod_add(y, m),
            m_old
        );
    }
}

fn demo_natural_mod_add_val_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} + {} === {} mod {}", x_old, y_old, x.mod_add(y, &m), m);
    }
}

fn demo_natural_mod_add_val_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!("{} + {} === {} mod {}", x_old, y, x.mod_add(&y, m), m_old);
    }
}

fn demo_natural_mod_add_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        println!("{} + {} === {} mod {}", x_old, y, x.mod_add(&y, &m), m);
    }
}

fn demo_natural_mod_add_ref_val_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        println!("{} + {} === {} mod {}", x, y_old, (&x).mod_add(y, m), m_old);
    }
}

fn demo_natural_mod_add_ref_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let y_old = y.clone();
        println!("{} + {} === {} mod {}", x, y_old, (&x).mod_add(y, &m), m);
    }
}

fn demo_natural_mod_add_ref_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let m_old = m.clone();
        println!("{} + {} === {} mod {}", x, y, (&x).mod_add(&y, m), m_old);
    }
}

fn demo_natural_mod_add_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        println!("{} + {} === {} mod {}", x, y, (&x).mod_add(&y, &m), m);
    }
}

fn benchmark_natural_mod_add_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_add_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_add_assign(Natural, Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_add_assign(y, m))),
            ),
            (
                "Natural.mod_add_assign(Natural, &Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_add_assign(y, &m))),
            ),
            (
                "Natural.mod_add_assign(&Natural, Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_add_assign(&y, m))),
            ),
            (
                "Natural.mod_add_assign(&Natural, &Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_add_assign(&y, &m))),
            ),
        ],
    );
}

fn benchmark_natural_mod_add_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.mod_add(Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            ("default", &mut (|(x, y, m)| no_out!(x.mod_add(y, m)))),
            ("naive", &mut (|(x, y, m)| no_out!((x + y) % m))),
        ],
    );
}

fn benchmark_natural_mod_add_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.mod_add(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_add(Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_add(y, m))),
            ),
            (
                "Natural.mod_add(Natural, &Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_add(y, &m))),
            ),
            (
                "Natural.mod_add(&Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_add(&y, m))),
            ),
            (
                "Natural.mod_add(&Natural, &Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_add(&y, &m))),
            ),
            (
                "(&Natural).mod_add(Natural, Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_add(y, m))),
            ),
            (
                "(&Natural).mod_add(Natural, &Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_add(y, &m))),
            ),
            (
                "(&Natural).mod_add(&Natural, Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_add(&y, m))),
            ),
            (
                "(&Natural).mod_add(&Natural, &Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_add(&y, &m))),
            ),
        ],
    );
}
