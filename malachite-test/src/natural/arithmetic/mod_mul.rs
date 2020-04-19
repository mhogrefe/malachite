use std::cmp::max;

use malachite_base::num::arithmetic::traits::{ModMul, ModMulAssign, ModMulPrecomputed};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::mod_mul::{
    _limbs_mod_mul_two_limbs, _limbs_mod_mul_two_limbs_naive, _limbs_precompute_mod_mul_two_limbs,
    _limbs_precompute_mod_mul_two_limbs_alt,
};
use malachite_nz::natural::logic::significant_bits::limbs_significant_bits;
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{nonuples_of_limbs_var_1, pairs_of_unsigneds_var_6};
use inputs::natural::triples_of_naturals_var_4;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_precompute_mod_mul_two_limbs);
    register_demo!(registry, demo_limbs_mod_mul_two_limbs);
    register_demo!(registry, demo_natural_mod_mul_assign);
    register_demo!(registry, demo_natural_mod_mul_assign_val_ref);
    register_demo!(registry, demo_natural_mod_mul_assign_ref_val);
    register_demo!(registry, demo_natural_mod_mul_assign_ref_ref);
    register_demo!(registry, demo_natural_mod_mul);
    register_demo!(registry, demo_natural_mod_mul_val_val_ref);
    register_demo!(registry, demo_natural_mod_mul_val_ref_val);
    register_demo!(registry, demo_natural_mod_mul_val_ref_ref);
    register_demo!(registry, demo_natural_mod_mul_ref_val_val);
    register_demo!(registry, demo_natural_mod_mul_ref_val_ref);
    register_demo!(registry, demo_natural_mod_mul_ref_ref_val);
    register_demo!(registry, demo_natural_mod_mul_ref_ref_ref);

    register_bench!(
        registry,
        None,
        benchmark_limbs_precompute_mod_mul_two_limbs_algorithms
    );
    register_bench!(registry, None, benchmark_limbs_mod_mul_two_limbs);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_mul_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_mul_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_mul_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_mul_precomputed_algorithms
    );
}

fn demo_limbs_precompute_mod_mul_two_limbs(gm: GenerationMode, limit: usize) {
    for (m_1, m_0) in pairs_of_unsigneds_var_6(gm).take(limit) {
        println!(
            "_limbs_precompute_mod_mul_two_limbs({}, {}) = {:?}",
            m_1,
            m_0,
            _limbs_precompute_mod_mul_two_limbs(m_1, m_0)
        );
    }
}

fn demo_limbs_mod_mul_two_limbs(gm: GenerationMode, limit: usize) {
    for (x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0) in
        nonuples_of_limbs_var_1(gm).take(limit)
    {
        println!(
            "_limbs_mod_mul_two_limbs({}, {}, {}, {}, {}, {}, {}, {}, {}) = {:?}",
            x_1,
            x_0,
            y_1,
            y_0,
            m_1,
            m_0,
            inv_2,
            inv_1,
            inv_0,
            _limbs_mod_mul_two_limbs(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0)
        );
    }
}

fn demo_natural_mod_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        x.mod_mul_assign(y, m);
        println!(
            "x := {}; x.mod_mul_assign({}, {}); x = {}",
            x_old, y_old, m_old, x
        );
    }
}

fn demo_natural_mod_mul_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let m_old = m.clone();
        let y_old = y.clone();
        x.mod_mul_assign(y, &m);
        println!(
            "x := {}; x.mod_mul_assign({}, &{}); x = {}",
            x, y_old, m_old, x
        );
    }
}

fn demo_natural_mod_mul_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        x.mod_mul_assign(&y, m);
        println!(
            "x := {}; x.mod_mul_assign(&{}, {}); x = {}",
            x_old, y, m_old, x
        );
    }
}

fn demo_natural_mod_mul_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        x.mod_mul_assign(&y, &m);
        println!(
            "x := {}; x.mod_mul_assign(&{}, &{}); x = {}",
            x_old, y, m, x
        );
    }
}

fn demo_natural_mod_mul(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        println!(
            "{} * {} === {} mod {}",
            x_old,
            y_old,
            x.mod_mul(y, m),
            m_old
        );
    }
}

fn demo_natural_mod_mul_val_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} === {} mod {}", x_old, y_old, x.mod_mul(y, &m), m);
    }
}

fn demo_natural_mod_mul_val_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        println!("{} * {} === {} mod {}", x_old, y, x.mod_mul(&y, m), m_old);
    }
}

fn demo_natural_mod_mul_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let x_old = x.clone();
        println!("{} * {} === {} mod {}", x_old, y, x.mod_mul(&y, &m), m);
    }
}

fn demo_natural_mod_mul_ref_val_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        println!("{} * {} === {} mod {}", x, y_old, (&x).mod_mul(y, m), m_old);
    }
}

fn demo_natural_mod_mul_ref_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let y_old = y.clone();
        println!("{} * {} === {} mod {}", x, y_old, (&x).mod_mul(y, &m), m);
    }
}

fn demo_natural_mod_mul_ref_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        let m_old = m.clone();
        println!("{} * {} === {} mod {}", x, y, (&x).mod_mul(&y, m), m_old);
    }
}

fn demo_natural_mod_mul_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals_var_4(gm).take(limit) {
        println!("{} * {} === {} mod {}", x, y, (&x).mod_mul(&y, &m), m);
    }
}

fn benchmark_limbs_precompute_mod_mul_two_limbs_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_precompute_mod_mul_two_limbs(Limb, Limb)",
        BenchmarkType::Algorithms,
        pairs_of_unsigneds_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(m_1, m_0)| usize::exact_from(limbs_significant_bits(&[m_0, m_1]))),
        "m.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(m_1, m_0)| no_out!(_limbs_precompute_mod_mul_two_limbs(m_1, m_0))),
            ),
            (
                "alt",
                &mut (|(m_1, m_0)| no_out!(_limbs_precompute_mod_mul_two_limbs_alt(m_1, m_0))),
            ),
        ],
    );
}

fn benchmark_limbs_mod_mul_two_limbs(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_mul_two_limbs(Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb, Limb)",
        BenchmarkType::Algorithms,
        nonuples_of_limbs_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(x_1, x_0, y_1, y_0, _, _, _, _, _)| {
            usize::exact_from(max(
                limbs_significant_bits(&[x_0, x_1]),
                limbs_significant_bits(&[y_0, y_1]),
            ))
        }),
        "m.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0)| {
                    no_out!(_limbs_mod_mul_two_limbs(
                        x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0
                    ))
                }),
            ),
            (
                "naive",
                &mut (|(x_1, x_0, y_1, y_0, m_1, m_0, _, _, _)| {
                    no_out!(_limbs_mod_mul_two_limbs_naive(x_1, x_0, y_1, y_0, m_1, m_0))
                }),
            ),
        ],
    );
}

fn benchmark_natural_mod_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_mul_assign(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_mul_assign(Natural, Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_mul_assign(y, m))),
            ),
            (
                "Natural.mod_mul_assign(Natural, &Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_mul_assign(y, &m))),
            ),
            (
                "Natural.mod_mul_assign(&Natural, Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_mul_assign(&y, m))),
            ),
            (
                "Natural.mod_mul_assign(&Natural, &Natural)",
                &mut (|(mut x, y, m)| no_out!(x.mod_mul_assign(&y, &m))),
            ),
        ],
    );
}

fn benchmark_natural_mod_mul_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_mul(Natural, u64)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            ("default", &mut (|(x, y, m)| no_out!(x.mod_mul(y, m)))),
            ("naive", &mut (|(x, y, m)| no_out!((x * y) % m))),
        ],
    );
}

fn benchmark_natural_mod_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "Natural.mod_mul(Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_mul(y, m))),
            ),
            (
                "Natural.mod_mul(Natural, &Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_mul(y, &m))),
            ),
            (
                "Natural.mod_mul(&Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_mul(&y, m))),
            ),
            (
                "Natural.mod_mul(&Natural, &Natural)",
                &mut (|(x, y, m)| no_out!(x.mod_mul(&y, &m))),
            ),
            (
                "(&Natural).mod_mul(Natural, Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_mul(y, m))),
            ),
            (
                "(&Natural).mod_mul(Natural, &Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_mul(y, &m))),
            ),
            (
                "(&Natural).mod_mul(&Natural, Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_mul(&y, m))),
            ),
            (
                "(&Natural).mod_mul(&Natural, &Natural)",
                &mut (|(x, y, m)| no_out!((&x).mod_mul(&y, &m))),
            ),
        ],
    );
}

fn benchmark_natural_mod_mul_precomputed_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_4(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            (
                "default",
                &mut (|(x, y, m)| {
                    for _ in 0..10 {
                        (&x).mod_mul(&y, &m);
                    }
                }),
            ),
            (
                "precomputed",
                &mut (|(x, y, m)| {
                    let data = ModMulPrecomputed::<Natural>::precompute_mod_mul_data(&m);
                    for _ in 0..10 {
                        (&x).mod_mul_precomputed(&y, &m, &data);
                    }
                }),
            ),
        ],
    );
}
