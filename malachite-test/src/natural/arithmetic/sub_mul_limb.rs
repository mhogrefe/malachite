use std::cmp::max;

use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{SignificantBits, SubMul, SubMulAssign};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::triples_of_natural_natural_and_limb_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_sub_mul_assign_limb);
    register_demo!(registry, demo_natural_sub_mul_assign_limb_ref);
    register_demo!(registry, demo_natural_sub_mul_limb);
    register_demo!(registry, demo_natural_sub_mul_limb_val_ref);
    register_demo!(registry, demo_natural_sub_mul_limb_ref_val);
    register_demo!(registry, demo_natural_sub_mul_limb_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_assign_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_assign_limb_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_sub_mul_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_limb_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_limb_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_limb_ref_ref_algorithms
    );
}

fn demo_natural_sub_mul_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_natural_sub_mul_assign_limb_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

fn demo_natural_sub_mul_limb(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.sub_mul({}, {}) = {:?}",
            a_old,
            b_old,
            c,
            a.sub_mul(b, c)
        );
    }
}

fn demo_natural_sub_mul_limb_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {:?}", a_old, b, c, a.sub_mul(&b, c));
    }
}

fn demo_natural_sub_mul_limb_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        let b_old = a.clone();
        println!(
            "(&{}).sub_mul({}, {}) = {:?}",
            a,
            b_old,
            c,
            (&a).sub_mul(b, c)
        );
    }
}

fn demo_natural_sub_mul_limb_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_limb_var_1(gm).take(limit) {
        println!(
            "(&{}).sub_mul(&{}, {}) = {:?}",
            a,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

fn benchmark_natural_sub_mul_assign_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul_assign(Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul_assign(Natural, Limb)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(b, c)),
            ),
            (
                "Natural -= Natural * Limb",
                &mut (|(mut a, b, c)| a -= b * c),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_assign_limb_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul_assign(&Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul_assign(&Natural, Limb)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
            (
                "Natural -= &Natural * Limb",
                &mut (|(mut a, b, c)| a -= &b * c),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul(&Natural, Limb)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Natural.sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "(&Natural).sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(b, c))),
            ),
            (
                "(&Natural).sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.sub_mul(Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(b, c))),
            ),
            (
                "Natural - Natural * Limb",
                &mut (|(a, b, c)| no_out!(a - b * c)),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul(&Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "Natural.sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "Natural - &Natural * Limb",
                &mut (|(a, b, c)| no_out!(a - &b * c)),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Natural).sub_mul(Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Natural).sub_mul(Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(b, c))),
            ),
            (
                "&Natural - Natural * Limb",
                &mut (|(a, b, c)| no_out!(&a - b * c)),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_limb_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Natural).sub_mul(&Natural, Limb)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| {
            usize::checked_from(max(a.significant_bits(), b.significant_bits())).unwrap()
        }),
        "max(a.significant_bits(), b.significant_bits())",
        &mut [
            (
                "(&Natural).sub_mul(&Natural, Limb)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
            (
                "&Natural - &Natural * Limb",
                &mut (|(a, b, c)| no_out!(&a - &b * c)),
            ),
        ],
    );
}
