use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{triples_of_naturals, triples_of_naturals_var_1};
use malachite_base::num::SignificantBits;
use malachite_base::num::{SubMul, SubMulAssign};
use malachite_nz::natural::Natural;
use std::cmp::max;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_sub_mul_assign);
    register_demo!(registry, demo_natural_sub_mul);
    register_demo!(registry, demo_natural_sub_mul_ref);
    register_bench!(registry, Large, benchmark_natural_sub_mul_assign);
    register_bench!(registry, Large, benchmark_natural_sub_mul_assign_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_mul_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_sub_mul_algorithms);
    register_bench!(registry, Large, benchmark_natural_sub_mul_ref_algorithms);
}

fn demo_natural_sub_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_naturals_var_1(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, &c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, &{}); x = {}",
            a_old, b, c, a
        );
    }
}

fn demo_natural_sub_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "{}.sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            a.sub_mul(&b, &c)
        );
    }
}

fn demo_natural_sub_mul_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_naturals(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, &{}) = {:?}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, &c)
        );
    }
}

fn bucketing_function(t: &(Natural, Natural, Natural)) -> usize {
    max(
        max(t.0.significant_bits(), t.1.significant_bits()),
        t.2.significant_bits(),
    ) as usize
}

const BUCKETING_LABEL: &str = "max(a.significant_bits(), b.significant_bits(), \
                               c.significant_bits())";

fn benchmark_natural_sub_mul_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.sub_mul_assign(&Natural, &Natural)",
        BenchmarkType::Single,
        triples_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [(
            "Natural.sub_mul_assign(&Natural, &Natural)",
            &mut (|(mut a, b, c)| a.sub_mul_assign(&b, &c)),
        )],
    );
}

fn benchmark_natural_sub_mul_assign_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.sub_mul_assign(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.sub_mul_assign(&Natural, &Natural)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, &c)),
            ),
            (
                "Natural -= &(&Natural * &Natural)",
                &mut (|(mut a, b, c)| a -= &(&b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, &c))),
            ),
            (
                "(&Natural).sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, &c))),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.sub_mul(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "Natural.sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, &c))),
            ),
            (
                "Natural - &Natural * &Natural",
                &mut (|(a, b, c)| no_out!(a - &b * &c)),
            ),
        ],
    );
}

fn benchmark_natural_sub_mul_ref_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "(&Natural).sub_mul(&Natural, &Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &bucketing_function,
        BUCKETING_LABEL,
        &mut [
            (
                "(&Natural).sub_mul(&Natural, &Natural)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, &c))),
            ),
            (
                "(&Natural) - &Natural * &Natural",
                &mut (|(a, b, c)| no_out!((&a) - &b * &c)),
            ),
        ],
    );
}
