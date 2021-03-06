use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_base_test_util::common::TRIPLE_SIGNIFICANT_BITS_LABEL;
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::triples_of_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_checked_sub_mul);
    register_demo!(registry, demo_natural_checked_sub_mul_val_val_ref);
    register_demo!(registry, demo_natural_checked_sub_mul_val_ref_val);
    register_demo!(registry, demo_natural_checked_sub_mul_val_ref_ref);
    register_demo!(registry, demo_natural_checked_sub_mul_ref_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_mul_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_mul_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_mul_val_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_mul_val_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_mul_val_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_sub_mul_ref_ref_ref_algorithms
    );
}

fn demo_natural_checked_sub_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.checked_sub_mul({}, {}) = {:?}",
            a_old,
            b_old,
            c_old,
            a.checked_sub_mul(b, c)
        );
    }
}

fn demo_natural_checked_sub_mul_val_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.checked_sub_mul({}, &{}) = {:?}",
            a_old,
            b_old,
            c,
            a.checked_sub_mul(b, &c)
        );
    }
}

fn demo_natural_checked_sub_mul_val_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.checked_sub_mul(&{}, {}) = {:?}",
            a_old,
            b,
            c_old,
            a.checked_sub_mul(&b, c)
        );
    }
}

fn demo_natural_checked_sub_mul_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "{}.checked_sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            a.checked_sub_mul(&b, &c)
        );
    }
}

fn demo_natural_checked_sub_mul_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).checked_sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            (&a).checked_sub_mul(&b, &c)
        );
    }
}

triple_significant_bits_fn!(Natural, bucketing_function);

fn benchmark_natural_checked_sub_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.checked_sub_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.checked_sub_mul(Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub_mul(b, c))),
            ),
            (
                "Natural.checked_sub_mul(Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub_mul(b, &c))),
            ),
            (
                "Natural.checked_sub_mul(&Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub_mul(&b, c))),
            ),
            (
                "Natural.checked_sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub_mul(&b, &c))),
            ),
            (
                "(&Natural).checked_sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!((&a).checked_sub_mul(&b, &c))),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.checked_sub_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.sub_mul(Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub_mul(b, c))),
            ),
            (
                "Natural.checked_sub(Natural * Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub(b * c))),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_val_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.checked_sub_mul(Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.sub_mul(Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub_mul(b, &c))),
            ),
            (
                "Natural.checked_sub(Natural * &Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub(b * &c))),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_val_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.checked_sub_mul(&Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.sub_mul(&Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub_mul(&b, c))),
            ),
            (
                "Natural.checked_sub(&Natural * Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub(&b * c))),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_val_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.checked_sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub_mul(&b, &c))),
            ),
            (
                "Natural.checked_sub(&Natural * &Natural)",
                &mut (|(a, b, c)| no_out!(a.checked_sub(&b * &c))),
            ),
        ],
    );
}

fn benchmark_natural_checked_sub_mul_ref_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "(&Natural).checked_sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "(&Natural).checked_sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!((&a).checked_sub_mul(&b, &c))),
            ),
            (
                "(&Natural).checked_sub(&Natural * &Natural)",
                &mut (|(a, b, c)| no_out!((&a).checked_sub(&b * &c))),
            ),
        ],
    );
}
