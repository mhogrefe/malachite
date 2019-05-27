use malachite_base::conversion::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::traits::SignificantBits;
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::rm_integers;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limb_checked_from_integer);
    register_demo!(registry, demo_limb_checked_from_integer_ref);
    register_demo!(registry, demo_limb_wrapping_from_integer);
    register_demo!(registry, demo_limb_wrapping_from_integer_ref);
    register_demo!(registry, demo_limb_saturating_from_integer);
    register_demo!(registry, demo_limb_saturating_from_integer_ref);
    register_demo!(registry, demo_limb_overflowing_from_integer);
    register_demo!(registry, demo_limb_overflowing_from_integer_ref);
    register_demo!(registry, demo_limb_convertible_from_integer);
    register_demo!(registry, demo_limb_convertible_from_integer_ref);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_checked_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_checked_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_checked_from_integer_algorithms
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_wrapping_from_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_wrapping_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_saturating_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_overflowing_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_convertible_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_convertible_from_integer_algorithms
    );
}

fn demo_limb_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Limb::checked_from({}) = {:?}",
            n_clone,
            Limb::checked_from(n)
        );
    }
}

fn demo_limb_checked_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("Limb::checked_from(&{}) = {:?}", n, Limb::checked_from(&n));
    }
}

fn demo_limb_wrapping_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Limb::wrapping_from({}) = {}",
            n_clone,
            Limb::wrapping_from(n)
        );
    }
}

fn demo_limb_wrapping_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("Limb::wrapping_from(&{}) = {}", n, Limb::wrapping_from(&n));
    }
}

fn demo_limb_saturating_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Limb::saturating_from({}) = {}",
            n_clone,
            Limb::saturating_from(n)
        );
    }
}

fn demo_limb_saturating_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "Limb::saturating_from(&{}) = {}",
            n,
            Limb::saturating_from(&n)
        );
    }
}

fn demo_limb_overflowing_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Limb::overflowing_from({}) = {:?}",
            n_clone,
            Limb::overflowing_from(n)
        );
    }
}

fn demo_limb_overflowing_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "Limb::overflowing_from(&{}) = {:?}",
            n,
            Limb::overflowing_from(&n)
        );
    }
}

fn demo_limb_convertible_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "{} is {}convertible to a Limb",
            n_clone,
            if Limb::convertible_from(n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_limb_convertible_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "{} is {}convertible to a Limb",
            n,
            if Limb::convertible_from(&n) {
                ""
            } else {
                "not "
            },
        );
    }
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_checked_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::checked_from(&Integer)",
        BenchmarkType::LibraryComparison,
        rm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(Limb::checked_from(&n)))),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32()))),
        ],
    );
}

fn benchmark_limb_checked_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::checked_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb::checked_from(Integer)",
                &mut (|n| no_out!(Limb::checked_from(n))),
            ),
            (
                "Limb::checked_from(&Integer)",
                &mut (|n| no_out!(Limb::checked_from(&n))),
            ),
        ],
    );
}

fn benchmark_limb_checked_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::checked_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(Limb::checked_from(n)))),
            (
                "using overflowing_from",
                &mut (|n| {
                    let (value, overflow) = Limb::overflowing_from(n);
                    if overflow {
                        None
                    } else {
                        Some(value)
                    };
                }),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_wrapping_from_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::wrapping_from(&Integer)",
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
                &mut (|(_, n)| no_out!(Limb::wrapping_from(&n))),
            ),
            ("rug", &mut (|(n, _)| no_out!(n.to_u32_wrapping()))),
        ],
    );
}

fn benchmark_limb_wrapping_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::wrapping_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb::wrapping_from(Integer)",
                &mut (|n| no_out!(Limb::wrapping_from(n))),
            ),
            (
                "Limb::wrapping_from(&Integer)",
                &mut (|n| no_out!(Limb::wrapping_from(&n))),
            ),
        ],
    );
}

fn benchmark_limb_wrapping_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::wrapping_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(Limb::wrapping_from(n)))),
            (
                "using overflowing_from",
                &mut (|n| {
                    Limb::overflowing_from(n).0;
                }),
            ),
        ],
    );
}

fn benchmark_limb_saturating_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::saturating_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb::saturating_from(Integer)",
                &mut (|n| no_out!(Limb::saturating_from(n))),
            ),
            (
                "Limb::saturating_from(&Integer)",
                &mut (|n| no_out!(Limb::saturating_from(&n))),
            ),
        ],
    );
}

fn benchmark_limb_overflowing_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::overflowing_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb::overflowing_from(Integer)",
                &mut (|n| no_out!(Limb::overflowing_from(n))),
            ),
            (
                "Limb::overflowing_from(&Integer)",
                &mut (|n| no_out!(Limb::overflowing_from(&n))),
            ),
        ],
    );
}

fn benchmark_limb_overflowing_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::overflowing_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(Limb::overflowing_from(n)))),
            (
                "using wrapping_from and convertible_from",
                &mut (|n| no_out!((Limb::wrapping_from(&n), !Limb::convertible_from(n)))),
            ),
        ],
    );
}

fn benchmark_limb_convertible_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::convertible_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb::convertible_from(Integer)",
                &mut (|n| no_out!(Limb::convertible_from(n))),
            ),
            (
                "Limb::convertible_from(&Integer)",
                &mut (|n| no_out!(Limb::convertible_from(&n))),
            ),
        ],
    );
}

fn benchmark_limb_convertible_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb::convertible_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(Limb::convertible_from(n)))),
            (
                "using checked_from",
                &mut (|n| no_out!(Limb::checked_from(n).is_some())),
            ),
        ],
    );
}
