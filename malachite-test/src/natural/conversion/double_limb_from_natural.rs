use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, OverflowingFrom, SaturatingFrom, WrappingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::platform::DoubleLimb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::naturals;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_double_limb_checked_from_natural);
    register_demo!(registry, demo_double_limb_checked_from_natural_ref);
    register_demo!(registry, demo_double_limb_wrapping_from_natural);
    register_demo!(registry, demo_double_limb_wrapping_from_natural_ref);
    register_demo!(registry, demo_double_limb_saturating_from_natural);
    register_demo!(registry, demo_double_limb_saturating_from_natural_ref);
    register_demo!(registry, demo_double_limb_overflowing_from_natural);
    register_demo!(registry, demo_double_limb_overflowing_from_natural_ref);
    register_demo!(registry, demo_double_limb_convertible_from_natural);
    register_demo!(registry, demo_double_limb_convertible_from_natural_ref);
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_checked_from_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_checked_from_natural_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_wrapping_from_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_wrapping_from_natural_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_saturating_from_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_overflowing_from_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_overflowing_from_natural_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_convertible_from_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_double_limb_convertible_from_natural_algorithms
    );
}

fn demo_double_limb_checked_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "DoubleLimb::checked_from({}) = {:?}",
            n_clone,
            DoubleLimb::checked_from(n)
        );
    }
}

fn demo_double_limb_checked_from_natural_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "DoubleLimb::checked_from(&{}) = {:?}",
            n,
            DoubleLimb::checked_from(&n)
        );
    }
}

fn demo_double_limb_wrapping_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "DoubleLimb::wrapping_from({}) = {}",
            n_clone,
            DoubleLimb::wrapping_from(n)
        );
    }
}

fn demo_double_limb_wrapping_from_natural_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "DoubleLimb::wrapping_from(&{}) = {}",
            n,
            DoubleLimb::wrapping_from(&n)
        );
    }
}

fn demo_double_limb_saturating_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "DoubleLimb::saturating_from({}) = {}",
            n_clone,
            DoubleLimb::saturating_from(n)
        );
    }
}

fn demo_double_limb_saturating_from_natural_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "DoubleLimb::saturating_from(&{}) = {}",
            n,
            DoubleLimb::saturating_from(&n)
        );
    }
}

fn demo_double_limb_overflowing_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "DoubleLimb::overflowing_from({}) = {:?}",
            n_clone,
            DoubleLimb::overflowing_from(n)
        );
    }
}

fn demo_double_limb_overflowing_from_natural_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "DoubleLimb::overflowing_from(&{}) = {:?}",
            n,
            DoubleLimb::overflowing_from(&n)
        );
    }
}

fn demo_double_limb_convertible_from_natural(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "{} is {}convertible to a DoubleLimb",
            n_clone,
            if DoubleLimb::convertible_from(n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_double_limb_convertible_from_natural_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "{} is {}convertible to a DoubleLimb",
            n,
            if DoubleLimb::convertible_from(&n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_double_limb_checked_from_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::checked_from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "DoubleLimb::checked_from(Natural)",
                &mut (|n| no_out!(DoubleLimb::checked_from(n))),
            ),
            (
                "DoubleLimb::checked_from(&Natural)",
                &mut (|n| no_out!(DoubleLimb::checked_from(&n))),
            ),
        ],
    );
}

fn benchmark_double_limb_checked_from_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::checked_from(Natural)",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(DoubleLimb::checked_from(n)))),
            (
                "using overflowing_from",
                &mut (|n| {
                    let (value, overflow) = DoubleLimb::overflowing_from(n);
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

fn benchmark_double_limb_wrapping_from_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::wrapping_from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "DoubleLimb::wrapping_from(Natural)",
                &mut (|n| no_out!(DoubleLimb::wrapping_from(n))),
            ),
            (
                "DoubleLimb::wrapping_from(&Natural)",
                &mut (|n| no_out!(DoubleLimb::wrapping_from(&n))),
            ),
        ],
    );
}

fn benchmark_double_limb_wrapping_from_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::wrapping_from(Natural)",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(DoubleLimb::wrapping_from(n)))),
            (
                "using overflowing_from",
                &mut (|n| {
                    DoubleLimb::overflowing_from(n).0;
                }),
            ),
        ],
    );
}

fn benchmark_double_limb_saturating_from_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::saturating_from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "DoubleLimb::saturating_from(Natural)",
                &mut (|n| no_out!(DoubleLimb::saturating_from(n))),
            ),
            (
                "DoubleLimb::saturating_from(&Natural)",
                &mut (|n| no_out!(DoubleLimb::saturating_from(&n))),
            ),
        ],
    );
}

fn benchmark_double_limb_overflowing_from_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::overflowing_from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "DoubleLimb::overflowing_from(Natural)",
                &mut (|n| no_out!(DoubleLimb::overflowing_from(n))),
            ),
            (
                "DoubleLimb::overflowing_from(&Natural)",
                &mut (|n| no_out!(DoubleLimb::overflowing_from(&n))),
            ),
        ],
    );
}

fn benchmark_double_limb_overflowing_from_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::overflowing_from(Natural)",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|n| no_out!(DoubleLimb::overflowing_from(n))),
            ),
            (
                "using wrapping_from and convertible_from",
                &mut (|n| {
                    no_out!((
                        DoubleLimb::wrapping_from(&n),
                        !DoubleLimb::convertible_from(n)
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_double_limb_convertible_from_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::convertible_from(Natural)",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "DoubleLimb::convertible_from(Natural)",
                &mut (|n| no_out!(DoubleLimb::convertible_from(n))),
            ),
            (
                "DoubleLimb::convertible_from(&Natural)",
                &mut (|n| no_out!(DoubleLimb::convertible_from(&n))),
            ),
        ],
    );
}

fn benchmark_double_limb_convertible_from_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "DoubleLimb::convertible_from(Natural)",
        BenchmarkType::Algorithms,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|n| no_out!(DoubleLimb::convertible_from(n))),
            ),
            (
                "using checked_from",
                &mut (|n| no_out!(DoubleLimb::checked_from(n).is_some())),
            ),
        ],
    );
}
