use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::triples_of_natural_natural_and_unsigned;
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{SaturatingSubMul, SaturatingSubMulAssign, SignificantBits};
use malachite_nz::platform::Limb;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_saturating_sub_mul_assign_limb);
    register_demo!(registry, demo_natural_saturating_sub_mul_assign_limb_ref);
    register_demo!(registry, demo_natural_saturating_sub_mul_limb);
    register_demo!(registry, demo_natural_saturating_sub_mul_limb_val_ref);
    register_demo!(registry, demo_natural_saturating_sub_mul_limb_ref_val);
    register_demo!(registry, demo_natural_saturating_sub_mul_limb_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_mul_limb_evaluation_strategy
    );
}

fn demo_natural_saturating_sub_mul_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.saturating_sub_mul_assign(b, c);
        println!(
            "a := {}; x.saturating_sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_natural_saturating_sub_mul_assign_limb_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        a.saturating_sub_mul_assign(&b, c);
        println!(
            "a := {}; x.saturating_sub_mul_assign(&{}, {}); x = {}",
            a_old, b, c, a
        );
    }
}

fn demo_natural_saturating_sub_mul_limb(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.saturating_sub_mul({}, {}) = {:?}",
            a_old,
            b_old,
            c,
            a.saturating_sub_mul(b, c)
        );
    }
}

fn demo_natural_saturating_sub_mul_limb_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<Limb>(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "{}.saturating_sub_mul(&{}, {}) = {:?}",
            a_old,
            b,
            c,
            a.saturating_sub_mul(&b, c)
        );
    }
}

fn demo_natural_saturating_sub_mul_limb_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<Limb>(gm).take(limit) {
        let b_old = a.clone();
        println!(
            "(&{}).saturating_sub_mul({}, {}) = {:?}",
            a,
            b_old,
            c,
            (&a).saturating_sub_mul(b, c)
        );
    }
}

fn demo_natural_saturating_sub_mul_limb_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<Limb>(gm).take(limit) {
        println!(
            "(&{}).saturating_sub_mul(&{}, {}) = {:?}",
            a,
            b,
            c,
            (&a).saturating_sub_mul(&b, c)
        );
    }
}

fn benchmark_natural_saturating_sub_mul_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.saturating_sub_mul(&Natural, Limb)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.saturating_sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.saturating_sub_mul(b, c))),
            ),
            (
                "Natural.saturating_sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.saturating_sub_mul(&b, c))),
            ),
            (
                "(&Natural).saturating_sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).saturating_sub_mul(b, c))),
            ),
            (
                "(&Natural).saturating_sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).saturating_sub_mul(&b, c))),
            ),
        ],
    );
}
