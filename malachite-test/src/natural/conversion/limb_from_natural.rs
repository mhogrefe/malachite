use malachite_base::conversion::{CheckedFrom, WrappingFrom};
use malachite_base::num::traits::SignificantBits;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::naturals;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::rm_naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limb_checked_from_natural);
    register_demo!(registry, demo_limb_checked_from_natural_ref);
    register_demo!(registry, demo_limb_wrapping_from_natural);
    register_demo!(registry, demo_limb_wrapping_from_natural_ref);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_checked_from_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_checked_from_natural_evaluation_strategy
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_wrapping_from_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_wrapping_from_natural_evaluation_strategy
    );
}

fn demo_limb_checked_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Limb::checked_from({}) = {:?}",
            n_clone,
            Limb::checked_from(n)
        );
    }
}

fn demo_limb_checked_from_natural_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("Limb::checked_from(&{}) = {:?}", n, Limb::checked_from(&n));
    }
}

fn demo_limb_wrapping_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Limb::wrapping_from({}) = {}",
            n_clone,
            Limb::wrapping_from(n)
        );
    }
}

fn demo_limb_wrapping_from_natural_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("Limb::wrapping_from(&{}) = {}", n, Limb::wrapping_from(&n));
    }
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_checked_from_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::checked_from(Natural)",
        BenchmarkType::LibraryComparison,
        rm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(Limb::checked_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32()))),
        ],
    );
}

fn benchmark_limb_checked_from_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::checked_from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb::checked_from(Natural)",
                &mut (|n| no_out!(Limb::checked_from(n))),
            ),
            (
                "Limb::checked_from(&Natural)",
                &mut (|n| no_out!(Limb::checked_from(&n))),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_wrapping_from_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::wrapping_from(&Natural)",
        BenchmarkType::LibraryComparison,
        rm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, n)| no_out!(Limb::wrapping_from(&n))),
            ),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32_wrapping()))),
        ],
    );
}

fn benchmark_limb_wrapping_from_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::wrapping_from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb::wrapping_from(Natural)",
                &mut (|n| no_out!(Limb::wrapping_from(n))),
            ),
            (
                "Limb::wrapping_from(&Natural)",
                &mut (|n| no_out!(Limb::wrapping_from(&n))),
            ),
        ],
    );
}
