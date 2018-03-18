use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;

pub fn demo_integer_to_sign_and_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "to_sign_and_limbs_asc({}) = {:?}",
            n,
            n.to_sign_and_limbs_asc()
        );
    }
}

pub fn demo_integer_to_sign_and_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "to_sign_and_limbs_desc({}) = {:?}",
            n,
            n.to_sign_and_limbs_desc()
        );
    }
}

pub fn demo_integer_into_sign_and_limbs_asc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "into_sign_and_limbs_asc({}) = {:?}",
            n,
            n.clone().into_sign_and_limbs_asc()
        );
    }
}

pub fn demo_integer_into_sign_and_limbs_desc(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "into_sign_and_limbs_desc({}) = {:?}",
            n,
            n.clone().into_sign_and_limbs_desc()
        );
    }
}

pub fn benchmark_integer_to_sign_and_limbs_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_sign_and_limbs_asc()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.to_sign_and_limbs_asc()",
                &mut (|n| no_out!(n.to_sign_and_limbs_asc())),
            ),
            (
                "Integer.into_sign_and_limbs_asc()",
                &mut (|n| no_out!(n.into_sign_and_limbs_asc())),
            ),
        ],
    );
}

pub fn benchmark_integer_to_sign_and_limbs_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.to_sign_and_limbs_desc()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.to_sign_and_limbs_desc()",
                &mut (|n| no_out!(n.to_sign_and_limbs_desc())),
            ),
            (
                "Integer.into_sign_and_limbs_desc()",
                &mut (|n| no_out!(n.into_sign_and_limbs_desc())),
            ),
        ],
    );
}
