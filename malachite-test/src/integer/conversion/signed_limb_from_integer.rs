use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::traits::SignificantBits;
use malachite_nz::platform::SignedLimb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::rm_integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_signed_limb_checked_from_integer);
    register_demo!(registry, demo_signed_limb_checked_from_integer_ref);
    register_demo!(registry, demo_signed_limb_wrapping_from_integer);
    register_demo!(registry, demo_signed_limb_wrapping_from_integer_ref);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_checked_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_checked_from_integer_evaluation_strategy
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_wrapping_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_wrapping_from_integer_evaluation_strategy
    );
}

fn demo_signed_limb_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "SignedLimb::checked_from({}) = {:?}",
            n_clone,
            SignedLimb::checked_from(n)
        );
    }
}

fn demo_signed_limb_checked_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "SignedLimb::checked_from(&{}) = {:?}",
            n,
            SignedLimb::checked_from(&n)
        );
    }
}

fn demo_signed_limb_wrapping_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "SignedLimb::wrapping_from({}) = {}",
            n_clone,
            SignedLimb::wrapping_from(n)
        );
    }
}

fn demo_signed_limb_wrapping_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "SignedLimb::wrapping_from(&{}) = {}",
            n,
            SignedLimb::wrapping_from(&n)
        );
    }
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_signed_limb_checked_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::checked_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, n)| no_out!(SignedLimb::checked_from(&n))),
            ),
            ("rug", &mut (|(n, _)| no_out!(n.to_i32()))),
        ],
    );
}

fn benchmark_signed_limb_checked_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::checked_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb::checked_from(Integer)",
                &mut (|n| no_out!(SignedLimb::checked_from(n))),
            ),
            (
                "SignedLimb::checked_from(&Integer)",
                &mut (|n| no_out!(SignedLimb::checked_from(&n))),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_signed_limb_wrapping_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::wrapping_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, n)| no_out!(SignedLimb::wrapping_from(&n))),
            ),
            ("rug", &mut (|(n, _)| no_out!(n.to_i32_wrapping()))),
        ],
    );
}

fn benchmark_signed_limb_wrapping_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::wrapping_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb::wrapping_from(Integer)",
                &mut (|n| no_out!(SignedLimb::wrapping_from(n))),
            ),
            (
                "SignedLimb::wrapping_from(&Integer)",
                &mut (|n| no_out!(SignedLimb::wrapping_from(&n))),
            ),
        ],
    );
}
