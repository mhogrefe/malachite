use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::rm_integers;
use malachite_base::conversion::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::traits::SignificantBits;
use malachite_nz::platform::SignedLimb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_signed_limb_checked_from_integer);
    register_demo!(registry, demo_signed_limb_checked_from_integer_ref);
    register_demo!(registry, demo_signed_limb_wrapping_from_integer);
    register_demo!(registry, demo_signed_limb_wrapping_from_integer_ref);
    register_demo!(registry, demo_signed_limb_saturating_from_integer);
    register_demo!(registry, demo_signed_limb_saturating_from_integer_ref);
    register_demo!(registry, demo_signed_limb_overflowing_from_integer);
    register_demo!(registry, demo_signed_limb_overflowing_from_integer_ref);
    register_demo!(registry, demo_signed_limb_convertible_from_integer);
    register_demo!(registry, demo_signed_limb_convertible_from_integer_ref);
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
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_checked_from_integer_algorithms
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
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_saturating_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_overflowing_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_convertible_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_convertible_from_integer_algorithms
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

fn demo_signed_limb_saturating_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "SignedLimb::saturating_from({}) = {}",
            n_clone,
            SignedLimb::saturating_from(n)
        );
    }
}

fn demo_signed_limb_saturating_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "SignedLimb::saturating_from(&{}) = {}",
            n,
            SignedLimb::saturating_from(&n)
        );
    }
}

fn demo_signed_limb_overflowing_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "SignedLimb::overflowing_from({}) = {:?}",
            n_clone,
            SignedLimb::overflowing_from(n)
        );
    }
}

fn demo_signed_limb_overflowing_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "SignedLimb::overflowing_from(&{}) = {:?}",
            n,
            SignedLimb::overflowing_from(&n)
        );
    }
}

fn demo_signed_limb_convertible_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "{} is {}convertible to a SignedLimb",
            n_clone,
            if SignedLimb::convertible_from(n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_signed_limb_convertible_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "{} is {}convertible to a SignedLimb",
            n,
            if SignedLimb::convertible_from(&n) {
                ""
            } else {
                "not "
            },
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

fn benchmark_signed_limb_checked_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::checked_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(SignedLimb::checked_from(n)))),
            (
                "using overflowing_from",
                &mut (|n| {
                    let (value, overflow) = SignedLimb::overflowing_from(n);
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

fn benchmark_signed_limb_wrapping_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::wrapping_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(SignedLimb::wrapping_from(n)))),
            (
                "using overflowing_from",
                &mut (|n| {
                    SignedLimb::overflowing_from(n).0;
                }),
            ),
        ],
    );
}

fn benchmark_signed_limb_saturating_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::saturating_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb::saturating_from(Integer)",
                &mut (|n| no_out!(SignedLimb::saturating_from(n))),
            ),
            (
                "SignedLimb::saturating_from(&Integer)",
                &mut (|n| no_out!(SignedLimb::saturating_from(&n))),
            ),
        ],
    );
}

fn benchmark_signed_limb_overflowing_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::overflowing_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb::overflowing_from(Integer)",
                &mut (|n| no_out!(SignedLimb::overflowing_from(n))),
            ),
            (
                "SignedLimb::overflowing_from(&Integer)",
                &mut (|n| no_out!(SignedLimb::overflowing_from(&n))),
            ),
        ],
    );
}

fn benchmark_signed_limb_overflowing_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::overflowing_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|n| no_out!(SignedLimb::overflowing_from(n))),
            ),
            (
                "using checked_from and wrapping_from",
                &mut (|n| {
                    no_out!((
                        SignedLimb::wrapping_from(&n),
                        SignedLimb::checked_from(n).is_none()
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_signed_limb_convertible_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::convertible_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb::convertible_from(Integer)",
                &mut (|n| no_out!(SignedLimb::convertible_from(n))),
            ),
            (
                "SignedLimb::convertible_from(&Integer)",
                &mut (|n| no_out!(SignedLimb::convertible_from(&n))),
            ),
        ],
    );
}

fn benchmark_signed_limb_convertible_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb::convertible_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|n| no_out!(SignedLimb::convertible_from(n))),
            ),
            (
                "using checked_from",
                &mut (|n| no_out!(SignedLimb::checked_from(n).is_some())),
            ),
        ],
    );
}
