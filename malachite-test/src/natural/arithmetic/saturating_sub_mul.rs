use malachite_base::num::arithmetic::traits::{SaturatingSubMul, SaturatingSubMulAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_base_test_util::common::TRIPLE_SIGNIFICANT_BITS_LABEL;
use malachite_nz::natural::Natural;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::triples_of_naturals_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_saturating_sub_mul_assign);
    register_demo!(registry, demo_natural_saturating_sub_mul_assign_val_ref);
    register_demo!(registry, demo_natural_saturating_sub_mul_assign_ref_val);
    register_demo!(registry, demo_natural_saturating_sub_mul_assign_ref_ref);
    register_demo!(registry, demo_natural_saturating_sub_mul);
    register_demo!(registry, demo_natural_saturating_sub_mul_val_val_ref);
    register_demo!(registry, demo_natural_saturating_sub_mul_val_ref_val);
    register_demo!(registry, demo_natural_saturating_sub_mul_val_ref_ref);
    register_demo!(registry, demo_natural_saturating_sub_mul_ref_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_mul_assign_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_mul_assign_val_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_mul_assign_ref_val_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_mul_assign_ref_ref_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_sub_mul_evaluation_strategy
    );
}

fn demo_natural_saturating_sub_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.saturating_sub_mul_assign(b, c);
        println!(
            "a := {}; x.saturating_sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c_old, a
        );
    }
}

fn demo_natural_saturating_sub_mul_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.saturating_sub_mul_assign(b, &c);
        println!(
            "a := {}; x.saturating_sub_mul_assign({}, &{}); x = {}",
            a_old, b_old, c, a
        );
    }
}

fn demo_natural_saturating_sub_mul_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.saturating_sub_mul_assign(&b, c);
        println!(
            "a := {}; x.saturating_sub_mul_assign(&{}, {}); x = {}",
            a_old, b, c_old, a
        );
    }
}

fn demo_natural_saturating_sub_mul_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        a.saturating_sub_mul_assign(&b, &c);
        println!(
            "a := {}; x.saturating_sub_mul_assign(&{}, &{}); x = {}",
            a_old, b, c, a
        );
    }
}

fn demo_natural_saturating_sub_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.saturating_sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c_old,
            a.saturating_sub_mul(b, c)
        );
    }
}

fn demo_natural_saturating_sub_mul_val_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.saturating_sub_mul({}, &{}) = {}",
            a_old,
            b_old,
            c,
            a.saturating_sub_mul(b, &c)
        );
    }
}

fn demo_natural_saturating_sub_mul_val_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.saturating_sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c_old,
            a.saturating_sub_mul(&b, c)
        );
    }
}

fn demo_natural_saturating_sub_mul_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "{}.saturating_sub_mul(&{}, &{}) = {}",
            a_old,
            b,
            c,
            a.saturating_sub_mul(&b, &c)
        );
    }
}

fn demo_natural_saturating_sub_mul_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).saturating_sub_mul(&{}, &{}) = {}",
            a_old,
            b,
            c,
            (&a).saturating_sub_mul(&b, &c)
        );
    }
}

triple_significant_bits_fn!(Natural, bucketing_function);

fn benchmark_natural_saturating_sub_mul_assign_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.saturating_sub_mul_assign(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.saturating_sub_mul_assign(Natural, Natural)",
                &mut (|(mut a, b, c)| a.saturating_sub_mul_assign(b, c)),
            ),
            (
                "Natural -= Natural * Natural",
                &mut (|(mut a, b, c)| a -= b * c),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_assign_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.saturating_sub_mul_assign(Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.saturating_sub_mul_assign(Natural, &Natural)",
                &mut (|(mut a, b, c)| a.saturating_sub_mul_assign(b, &c)),
            ),
            (
                "Natural -= Natural * &Natural",
                &mut (|(mut a, b, c)| a -= b * &c),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_assign_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.saturating_sub_mul_assign(&Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.saturating_sub_mul_assign(&Natural, Natural)",
                &mut (|(mut a, b, c)| a.saturating_sub_mul_assign(&b, c)),
            ),
            (
                "Natural -= &Natural * Natural",
                &mut (|(mut a, b, c)| a -= &b * c),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_assign_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.saturating_sub_mul_assign(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.saturating_sub_mul_assign(&Natural, &Natural)",
                &mut (|(mut a, b, c)| a.saturating_sub_mul_assign(&b, &c)),
            ),
            (
                "Natural -= &Natural * &Natural",
                &mut (|(mut a, b, c)| a -= &b * &c),
            ),
        ],
    );
}

fn benchmark_natural_saturating_sub_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.saturating_sub_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        TRIPLE_SIGNIFICANT_BITS_LABEL,
        &mut [
            (
                "Natural.saturating_sub_mul(Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.saturating_sub_mul(b, c))),
            ),
            (
                "Natural.saturating_sub_mul(Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.saturating_sub_mul(b, &c))),
            ),
            (
                "Natural.saturating_sub_mul(&Natural, Natural)",
                &mut (|(a, b, c)| no_out!(a.saturating_sub_mul(&b, c))),
            ),
            (
                "Natural.saturating_sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.saturating_sub_mul(&b, &c))),
            ),
            (
                "(&Natural).saturating_sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!((&a).saturating_sub_mul(&b, &c))),
            ),
        ],
    );
}
