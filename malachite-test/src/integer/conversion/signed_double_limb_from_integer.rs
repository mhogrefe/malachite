use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::integers;
use malachite_base::conversion::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::traits::SignificantBits;
use malachite_nz::platform::SignedDoubleLimb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_signed_double_limb_checked_from_integer);
    register_demo!(registry, demo_signed_double_limb_checked_from_integer_ref);
    register_demo!(registry, demo_signed_double_limb_wrapping_from_integer);
    register_demo!(registry, demo_signed_double_limb_wrapping_from_integer_ref);
    register_demo!(registry, demo_signed_double_limb_saturating_from_integer);
    register_demo!(
        registry,
        demo_signed_double_limb_saturating_from_integer_ref
    );
    register_demo!(registry, demo_signed_double_limb_overflowing_from_integer);
    register_demo!(
        registry,
        demo_signed_double_limb_overflowing_from_integer_ref
    );
    register_demo!(registry, demo_signed_double_limb_convertible_from_integer);
    register_demo!(
        registry,
        demo_signed_double_limb_convertible_from_integer_ref
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_checked_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_checked_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_wrapping_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_wrapping_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_saturating_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_overflowing_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_overflowing_from_integer_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_convertible_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_double_limb_convertible_from_integer_algorithms
    );
}

fn demo_signed_double_limb_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "SignedDoubleLimb::checked_from({}) = {:?}",
            n_clone,
            SignedDoubleLimb::checked_from(n)
        );
    }
}

fn demo_signed_double_limb_checked_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "SignedDoubleLimb::checked_from(&{}) = {:?}",
            n,
            SignedDoubleLimb::checked_from(&n)
        );
    }
}

fn demo_signed_double_limb_wrapping_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "SignedDoubleLimb::wrapping_from({}) = {}",
            n_clone,
            SignedDoubleLimb::wrapping_from(n)
        );
    }
}

fn demo_signed_double_limb_wrapping_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "SignedDoubleLimb::wrapping_from(&{}) = {}",
            n,
            SignedDoubleLimb::wrapping_from(&n)
        );
    }
}

fn demo_signed_double_limb_saturating_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "SignedDoubleLimb::saturating_from({}) = {}",
            n_clone,
            SignedDoubleLimb::saturating_from(n)
        );
    }
}

fn demo_signed_double_limb_saturating_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "SignedDoubleLimb::saturating_from(&{}) = {}",
            n,
            SignedDoubleLimb::saturating_from(&n)
        );
    }
}

fn demo_signed_double_limb_overflowing_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "SignedDoubleLimb::overflowing_from({}) = {:?}",
            n_clone,
            SignedDoubleLimb::overflowing_from(n)
        );
    }
}

fn demo_signed_double_limb_overflowing_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "SignedDoubleLimb::overflowing_from(&{}) = {:?}",
            n,
            SignedDoubleLimb::overflowing_from(&n)
        );
    }
}

fn demo_signed_double_limb_convertible_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "{} is {}convertible to a SignedDoubleLimb",
            n_clone,
            if SignedDoubleLimb::convertible_from(n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_signed_double_limb_convertible_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "{} is {}convertible to a SignedDoubleLimb",
            n,
            if SignedDoubleLimb::convertible_from(&n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_signed_double_limb_checked_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::checked_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedDoubleLimb::checked_from(Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::checked_from(n))),
            ),
            (
                "SignedDoubleLimb::checked_from(&Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::checked_from(&n))),
            ),
        ],
    );
}

fn benchmark_signed_double_limb_checked_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::checked_from(Integer)",
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
                &mut (|n| no_out!(SignedDoubleLimb::checked_from(n))),
            ),
            (
                "using overflowing_from",
                &mut (|n| {
                    let (value, overflow) = SignedDoubleLimb::overflowing_from(n);
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

fn benchmark_signed_double_limb_wrapping_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::wrapping_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedDoubleLimb::wrapping_from(Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::wrapping_from(n))),
            ),
            (
                "SignedDoubleLimb::wrapping_from(&Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::wrapping_from(&n))),
            ),
        ],
    );
}

fn benchmark_signed_double_limb_wrapping_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::wrapping_from(Integer)",
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
                &mut (|n| no_out!(SignedDoubleLimb::wrapping_from(n))),
            ),
            (
                "using overflowing_from",
                &mut (|n| {
                    SignedDoubleLimb::overflowing_from(n).0;
                }),
            ),
        ],
    );
}

fn benchmark_signed_double_limb_saturating_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::saturating_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedDoubleLimb::saturating_from(Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::saturating_from(n))),
            ),
            (
                "SignedDoubleLimb::saturating_from(&Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::saturating_from(&n))),
            ),
        ],
    );
}

fn benchmark_signed_double_limb_overflowing_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::overflowing_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedDoubleLimb::overflowing_from(Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::overflowing_from(n))),
            ),
            (
                "SignedDoubleLimb::overflowing_from(&Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::overflowing_from(&n))),
            ),
        ],
    );
}

fn benchmark_signed_double_limb_overflowing_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::overflowing_from(Integer)",
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
                &mut (|n| no_out!(SignedDoubleLimb::overflowing_from(n))),
            ),
            (
                "using wrapping_from and convertible_from",
                &mut (|n| {
                    no_out!((
                        SignedDoubleLimb::wrapping_from(&n),
                        !SignedDoubleLimb::convertible_from(n)
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_signed_double_limb_convertible_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::convertible_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "SignedDoubleLimb::convertible_from(Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::convertible_from(n))),
            ),
            (
                "SignedDoubleLimb::convertible_from(&Integer)",
                &mut (|n| no_out!(SignedDoubleLimb::convertible_from(&n))),
            ),
        ],
    );
}

fn benchmark_signed_double_limb_convertible_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedDoubleLimb::convertible_from(Integer)",
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
                &mut (|n| no_out!(SignedDoubleLimb::convertible_from(n))),
            ),
            (
                "using checked_from",
                &mut (|n| no_out!(SignedDoubleLimb::checked_from(n).is_some())),
            ),
        ],
    );
}
