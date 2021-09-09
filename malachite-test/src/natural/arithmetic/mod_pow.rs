use malachite_base::num::arithmetic::traits::{ModPow, ModPowAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::mod_pow::{
    limbs_mod_pow, limbs_mod_pow_odd, limbs_mod_pow_odd_scratch_len,
};
use malachite_nz_test_util::natural::arithmetic::mod_pow::_simple_binary_mod_pow;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    quadruples_of_unsigned_vec_var_1, quadruples_of_unsigned_vec_var_2,
};
use malachite_test::inputs::natural::triples_of_naturals_var_5;

pub fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_pow_odd);
    register_demo!(registry, demo_limbs_mod_pow);
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

    register_bench!(registry, Small, benchmark_limbs_mod_pow_odd);
    register_bench!(registry, Small, benchmark_limbs_mod_pow);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_pow_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_pow_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_pow_evaluation_strategy
    );
}

fn demo_limbs_mod_pow_odd(gm: GenerationMode, limit: usize) {
    for (mut out, xs, es, ms) in quadruples_of_unsigned_vec_var_1(gm.with_scale(32)).take(limit) {
        let out_old = out.clone();
        let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(ms.len())];
        limbs_mod_pow_odd(&mut out, &xs, &es, &ms, &mut scratch);
        println!(
            "out := {:?}; limbs_mod_pow_odd(&mut out, {:?}, {:?}, {:?}, &mut scratch); out = {:?}",
            out_old, xs, es, ms, out
        );
    }
}

fn demo_limbs_mod_pow(gm: GenerationMode, limit: usize) {
    for (mut out, xs, es, ms) in quadruples_of_unsigned_vec_var_2(gm.with_scale(32)).take(limit) {
        let out_old = out.clone();
        limbs_mod_pow(&mut out, &xs, &es, &ms);
        println!(
            "out := {:?}; limbs_mod_pow(&mut out, {:?}, {:?}, {:?}); out = {:?}",
            out_old, xs, es, ms, out
        );
    }
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

fn benchmark_limbs_mod_pow_odd(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_mod_pow_odd(&mut [Limb], &[Limb], &[Limb], &[Limb], &mut [Limb])",
        BenchmarkType::Single,
        quadruples_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, _, ref ms)| ms.len()),
        "ms.len()",
        &mut [(
            "Malachite",
            &mut (|(mut out, xs, es, ms)| {
                let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(ms.len())];
                limbs_mod_pow_odd(&mut out, &xs, &es, &ms, &mut scratch);
            }),
        )],
    );
}

fn benchmark_limbs_mod_pow(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_mod_pow(&mut [Limb], &[Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        quadruples_of_unsigned_vec_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, _, ref ms)| ms.len()),
        "ms.len()",
        &mut [(
            "Malachite",
            &mut (|(mut out, xs, es, ms)| {
                limbs_mod_pow(&mut out, &xs, &es, &ms);
            }),
        )],
    );
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

fn benchmark_natural_mod_pow_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.mod_pow(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_5(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref m)| usize::exact_from(m.significant_bits())),
        "m.significant_bits()",
        &mut [
            ("default", &mut (|(x, exp, m)| no_out!(x.mod_pow(exp, m)))),
            (
                "simple binary",
                &mut (|(x, exp, m)| no_out!(_simple_binary_mod_pow(&x, &exp, &m))),
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
