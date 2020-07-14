use malachite_base::num::arithmetic::traits::{Mod, ModSub, ModSubAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::natural::triples_of_naturals_var_4;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_sub_assign);
    register_demo!(registry, demo_natural_mod_sub_assign_val_ref);
    register_demo!(registry, demo_natural_mod_sub_assign_ref_val);
    register_demo!(registry, demo_natural_mod_sub_assign_ref_ref);
    register_demo!(registry, demo_natural_mod_sub);
    register_demo!(registry, demo_natural_mod_sub_val_val_ref);
    register_demo!(registry, demo_natural_mod_sub_val_ref_val);
    register_demo!(registry, demo_natural_mod_sub_val_ref_ref);
    register_demo!(registry, demo_natural_mod_sub_ref_val_val);
    register_demo!(registry, demo_natural_mod_sub_ref_val_ref);
    register_demo!(registry, demo_natural_mod_sub_ref_ref_val);
    register_demo!(registry, demo_natural_mod_sub_ref_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_sub_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_sub_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_sub_evaluation_strategy
    );
}

fn demo_natural_mod_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        x.mod_sub_assign(y, m);
        println!(
            "x := {}; x.mod_sub_assign({}, {}); x = {}",
            x_old, y_old, m_old, x
        );
    }
}

fn demo_natural_mod_sub_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let m_old = m.clone();
        let y_old = y.clone();
        x.mod_sub_assign(y, &m);
        println!(
            "x := {}; x.mod_sub_assign({}, &{}); x = {}",
            x, y_old, m_old, x
        );
    }
}

fn demo_natural_mod_sub_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        x.mod_sub_assign(&y, m);
        println!(
            "x := {}; x.mod_sub_assign(&{}, {}); x = {}",
            x_old, y, m_old, x
        );
    }
}

fn demo_natural_mod_sub_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        x.mod_sub_assign(&y, &m);
        println!(
            "x := {}; x.mod_sub_assign(&{}, &{}); x = {}",
            x_old, y, m, x
        );
    }
}

fn demo_natural_mod_sub(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        println!(
            "{} - {} === {} mod {}",
            x_old,
            y_old,
            x.mod_sub(y, m),
            m_old
        );
    }
}

fn demo_natural_mod_sub_val_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} === {} mod {}", x_old, y_old, x.mod_sub(y, &m), m);
    }
}

fn demo_natural_mod_sub_val_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!("{} - {} === {} mod {}", x_old, y, x.mod_sub(&y, m), m_old);
    }
}

fn demo_natural_mod_sub_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - {} === {} mod {}", x_old, y, x.mod_sub(&y, &m), m);
    }
}

fn demo_natural_mod_sub_ref_val_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        println!("{} - {} === {} mod {}", x, y_old, (&x).mod_sub(y, m), m_old);
    }
}

fn demo_natural_mod_sub_ref_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let y_old = y.clone();
        println!("{} - {} === {} mod {}", x, y_old, (&x).mod_sub(y, &m), m);
    }
}

fn demo_natural_mod_sub_ref_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let m_old = m.clone();
        println!("{} - {} === {} mod {}", x, y, (&x).mod_sub(&y, m), m_old);
    }
}

fn demo_natural_mod_sub_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        println!("{} - {} === {} mod {}", x, y, (&x).mod_sub(&y, &m), m);
    }
}

fn benchmark_natural_mod_sub_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_sub_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_sub_assign(Natural, Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_sub_assign(y, m))),
            ),
            (
                "Natural.mod_sub_assign(Natural, &Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_sub_assign(y, &m))),
            ),
            (
                "Natural.mod_sub_assign(&Natural, Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_sub_assign(&y, m))),
            ),
            (
                "Natural.mod_sub_assign(&Natural, &Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_sub_assign(&y, &m))),
            ),
        ],
    );
}

fn benchmark_natural_mod_sub_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_sub(Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            ("default", &mut (|(x, y, m)| no_out!(x.mod_sub(y, m)))),
            (
                "naive",
                &mut (|(x, y, m)| {
                    no_out!((Integer::from(x) - Integer::from(y)).mod_op(Integer::from(m)))
                }),
            ),
        ],
    );
}

fn benchmark_natural_mod_sub_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_sub(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_sub(Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_sub(y, m))),
            ),
            (
                "Natural.mod_sub(Natural, &Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_sub(y, &m))),
            ),
            (
                "Natural.mod_sub(&Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_sub(&y, m))),
            ),
            (
                "Natural.mod_sub(&Natural, &Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_sub(&y, &m))),
            ),
            (
                "(&Natural).mod_sub(Natural, Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_sub(y, m))),
            ),
            (
                "(&Natural).mod_sub(Natural, &Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_sub(y, &m))),
            ),
            (
                "(&Natural).mod_sub(&Natural, Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_sub(&y, m))),
            ),
            (
                "(&Natural).mod_sub(&Natural, &Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_sub(&y, &m))),
            ),
        ],
    );
}
