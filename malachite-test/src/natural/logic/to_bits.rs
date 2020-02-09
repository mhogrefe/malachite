use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::integers::_to_bits_asc_alt;
use malachite_base::num::logic::traits::{BitAccess, BitConvertible, SignificantBits};
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_to_bits_asc);
    register_demo!(registry, demo_natural_to_bits_desc);
    register_demo!(registry, demo_natural_bits);
    register_demo!(registry, demo_natural_bits_rev);
    register_demo!(registry, demo_natural_bits_size_hint);
    register_bench!(
        registry,
        Large,
        benchmark_natural_to_bits_asc_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_to_bits_asc_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_to_bits_desc_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_to_bits_desc_algorithms);
    register_bench!(registry, Large, benchmark_natural_bits_size_hint);
}

pub fn _to_bits_asc_naive(n: &Natural) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..n.significant_bits() {
        bits.push(n.get_bit(i));
    }
    bits
}

pub fn _to_bits_desc_naive(n: &Natural) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in (0..n.significant_bits()).rev() {
        bits.push(n.get_bit(i));
    }
    bits
}

fn demo_natural_to_bits_asc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_bits_asc({}) = {:?}", n, n.to_bits_asc());
    }
}

fn demo_natural_to_bits_desc(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("to_bits_desc({}) = {:?}", n, n.to_bits_desc());
    }
}

fn demo_natural_bits(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("bits({}) = {:?}", n, n.bits().collect::<Vec<bool>>());
    }
}

fn demo_natural_bits_rev(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "bits({}).rev() = {:?}",
            n,
            n.bits().rev().collect::<Vec<bool>>()
        );
    }
}

fn demo_natural_bits_size_hint(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("bits({}).size_hint() = {:?}", n, n.bits().size_hint());
    }
}

#[allow(unused_collect)]
fn benchmark_natural_to_bits_asc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.to_bits_asc()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
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

#[allow(unused_collect)]
fn benchmark_natural_to_bits_asc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.to_bits_asc()",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.to_bits_asc()))),
            ("alt", &mut (|n| no_out!(_to_bits_asc_alt(&n)))),
            ("naive", &mut (|n| no_out!(_to_bits_asc_naive(&n)))),
        ],
    );
}

#[allow(unused_collect)]
fn benchmark_natural_to_bits_desc_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.to_bits_desc()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.to_bits_desc()",
                &mut (|n| no_out!(n.to_bits_desc())),
            ),
            (
                "Natural.bits().rev().collect::<Vec<bool>>()",
                &mut (|n| no_out!(n.bits().collect::<Vec<bool>>())),
            ),
        ],
    );
}

#[allow(unused_collect)]
fn benchmark_natural_to_bits_desc_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.to_bits_desc()",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|n| no_out!(n.to_bits_desc()))),
            ("alt", &mut (|n| no_out!(n._to_bits_desc_alt()))),
            ("naive", &mut (|n| no_out!(_to_bits_desc_naive(&n)))),
        ],
    );
}

fn benchmark_natural_bits_size_hint(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.bits().size_hint()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [(
            "Natural.bits().size_hint()",
            &mut (|n| no_out!(n.bits().size_hint())),
        )],
    );
}
