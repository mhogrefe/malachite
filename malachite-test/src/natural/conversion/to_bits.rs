use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::naturals;
use malachite_base::num::SignificantBits;

pub fn demo_natural_to_bits_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_bits_asc({}) = {:?}", n, n.to_bits_asc());
    }
}

pub fn demo_natural_to_bits_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_bits_desc({}) = {:?}", n, n.to_bits_desc());
    }
}

pub fn demo_natural_bits(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("bits({}) = {:?}", n, n.bits().collect::<Vec<bool>>());
    }
}

pub fn demo_natural_bits_rev(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "bits({}).rev() = {:?}",
            n,
            n.bits().rev().collect::<Vec<bool>>()
        );
    }
}

pub fn demo_natural_bits_size_hint(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("bits({}).size_hint() = {:?}", n, n.bits().size_hint());
    }
}

#[allow(unknown_lints, unused_collect)]
pub fn benchmark_natural_bits_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.bits()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural.to_bits_asc()", &mut (|n| no_out!(n.to_bits_asc()))),
            (
                "Natural.bits().collect::<Vec<bool>>()",
                &mut (|n| no_out!(n.bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

#[allow(unknown_lints, unused_collect)]
pub fn benchmark_natural_bits_rev_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.bits().rev()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.to_bits_desc()",
                &mut (|n| no_out!(n.to_bits_desc())),
            ),
            (
                "Natural.limbs().rev().collect::<Vec<u32>>()",
                &mut (|n| no_out!(n.limbs().rev().collect::<Vec<u32>>())),
            ),
        ],
    );
}

pub fn benchmark_natural_bits_size_hint(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.bits().size_hint()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.bits().size_hint()",
                &mut (|n| no_out!(n.bits().size_hint())),
            ),
        ],
    );
}
