use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};
use malachite_base::num::traits::{SaturatingSub, SaturatingSubAssign, SignificantBits};
use malachite_nz::platform::Limb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_saturating_sub_assign_limb);
    register_demo!(registry, demo_natural_saturating_sub_limb);
    register_demo!(registry, demo_natural_saturating_sub_limb_ref);
    register_demo!(registry, demo_limb_saturating_sub_assign_natural);
    register_demo!(registry, demo_limb_saturating_sub_assign_natural_ref);
    register_demo!(registry, demo_limb_saturating_sub_natural);
    register_demo!(registry, demo_limb_saturating_sub_natural_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_assign_limb
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_saturating_sub_assign_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_saturating_sub_natural_evaluation_strategy
    );
}

fn demo_natural_saturating_sub_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n.saturating_sub_assign(u);
        println!("x := {}; x.saturating_sub_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_saturating_sub_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.saturating_sub({}) = {}", n_old, u, n.saturating_sub(u));
    }
}

fn demo_natural_saturating_sub_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        println!(
            "(&{}).saturating_sub({}) = {}",
            n,
            u,
            (&n).saturating_sub(u)
        );
    }
}

fn demo_limb_saturating_sub_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.saturating_sub({}) = {}",
            u,
            n_old,
            SaturatingSub::saturating_sub(u, n)
        );
    }
}

fn demo_limb_saturating_sub_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.saturating_sub(&{}) = {}",
            u,
            n_old,
            SaturatingSub::saturating_sub(u, &n)
        );
    }
}

fn demo_limb_saturating_sub_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u.saturating_sub_assign(n);
        println!(
            "x := {}; x.saturating_sub_assign({}); x = {}",
            u_old, n_old, u
        );
    }
}

fn demo_limb_saturating_sub_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        u.saturating_sub_assign(&n);
        println!("x := {}; x.saturating_sub_assign(&{}); x = {}", u_old, n, u);
    }
}

fn benchmark_natural_saturating_sub_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.saturating_sub_assign(Limb)",
        BenchmarkType::Single,
        pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.saturating_sub_assign(y)))],
    );
}

fn benchmark_natural_saturating_sub_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.saturating_sub(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.saturating_sub(Limb)",
                &mut (|(x, y)| no_out!(x.saturating_sub(y))),
            ),
            (
                "(&Natural).saturating_sub(Limb)",
                &mut (|(x, y)| no_out!((&x).saturating_sub(y))),
            ),
        ],
    );
}

fn benchmark_limb_saturating_sub_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.saturating_sub_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.saturating_sub_assign(Natural)",
                &mut (|(mut x, y)| x.saturating_sub_assign(y)),
            ),
            (
                "Limb.saturating_sub_assign(&Natural)",
                &mut (|(mut x, y)| x.saturating_sub_assign(&y)),
            ),
        ],
    );
}

fn benchmark_limb_saturating_sub_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.saturating_sub(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.saturating_sub(Natural)",
                &mut (|(x, y)| no_out!(SaturatingSub::saturating_sub(x, y))),
            ),
            (
                "Limb.saturating_sub(&Natural)",
                &mut (|(x, y)| no_out!(SaturatingSub::saturating_sub(x, &y))),
            ),
        ],
    );
}
