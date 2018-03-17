use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::{triples_of_natural_natural_and_unsigned,
                      triples_of_natural_natural_and_u32_var_1};
use malachite_base::num::SignificantBits;
use malachite_base::num::{SubMul, SubMulAssign};
use std::cmp::max;

pub fn demo_natural_sub_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_natural_natural_and_u32_var_1(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_natural_sub_mul_u32(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {:?}", a_old, b, c, a.sub_mul(&b, c));
    }
}

pub fn demo_natural_sub_mul_u32_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_natural_natural_and_unsigned::<u32>(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {:?}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

pub fn benchmark_natural_sub_mul_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.sub_mul_assign(&Natural, u32)",
        BenchmarkType::Single,
        triples_of_natural_natural_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Natural.sub_mul_assign(&Natural, u32)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
        ],
    );
}

pub fn benchmark_natural_sub_mul_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul_assign(&Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_u32_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Natural.sub_mul_assign(&Natural, u32)",
                &mut (|(mut a, b, c)| a.sub_mul_assign(&b, c)),
            ),
            (
                "Natural -= &(&Natural * u32)",
                &mut (|(mut a, b, c)| a -= &(&b * c)),
            ),
        ],
    );
}

pub fn benchmark_natural_sub_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.sub_mul(&Natural, u32)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Natural.sub_mul(&Natural, u32)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "(&Natural).sub_mul(&Natural, u32)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
        ],
    );
}

pub fn benchmark_natural_sub_mul_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.sub_mul(&Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "Natural.sub_mul(&Natural, u32)",
                &mut (|(a, b, c)| no_out!(a.sub_mul(&b, c))),
            ),
            (
                "Natural - &(&Natural * u32)",
                &mut (|(a, b, c)| no_out!(a - &(&b * c))),
            ),
        ],
    );
}

pub fn benchmark_natural_sub_mul_u32_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "(&Natural).sub_mul(&Natural, u32)",
        BenchmarkType::Algorithms,
        triples_of_natural_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        "max(a.significant_bits(), b.significant_bits())",
        &[
            (
                "(&Natural).sub_mul(&Natural, u32)",
                &mut (|(a, b, c)| no_out!((&a).sub_mul(&b, c))),
            ),
            (
                "&Natural - &(&Natural * u32)",
                &mut (|(a, b, c)| no_out!(&a - &(&b * c))),
            ),
        ],
    );
}
