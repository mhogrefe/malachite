use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::{integers, natural_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_checked_from_integer);
    register_demo!(registry, demo_natural_checked_from_integer_ref);
    register_demo!(registry, demo_natural_exact_from_integer);
    register_demo!(registry, demo_natural_exact_from_integer_ref);
    register_demo!(registry, demo_natural_saturating_from_integer);
    register_demo!(registry, demo_natural_saturating_from_integer_ref);
    register_demo!(registry, demo_natural_convertible_from_integer);
    register_demo!(registry, demo_natural_convertible_from_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_checked_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_exact_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_saturating_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_convertible_from_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_convertible_from_integer_algorithms
    );
}

fn demo_natural_checked_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Natural::checked_from({}) = {:?}",
            n_clone,
            Natural::checked_from(n)
        );
    }
}

fn demo_natural_checked_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "Natural::checked_from(&{}) = {:?}",
            n,
            Natural::checked_from(&n)
        );
    }
}

fn demo_natural_exact_from_integer(gm: GenerationMode, limit: usize) {
    for n in natural_integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Natural::exact_from({}) = {}",
            n_clone,
            Natural::exact_from(n)
        );
    }
}

fn demo_natural_exact_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in natural_integers(gm).take(limit) {
        println!("Natural::exact_from(&{}) = {}", n, Natural::exact_from(&n));
    }
}

fn demo_natural_saturating_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "Natural::saturating_from({}) = {}",
            n_clone,
            Natural::saturating_from(n)
        );
    }
}

fn demo_natural_saturating_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "Natural::saturating_from(&{}) = {}",
            n,
            Natural::saturating_from(&n)
        );
    }
}

fn demo_natural_convertible_from_integer(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        let n_clone = n.clone();
        println!(
            "{} is {}convertible to a Natural",
            n_clone,
            if Natural::convertible_from(n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn demo_natural_convertible_from_integer_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!(
            "{} is {}convertible to a Natural",
            n,
            if Natural::convertible_from(&n) {
                ""
            } else {
                "not "
            },
        );
    }
}

fn benchmark_natural_checked_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::checked_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural::checked_from(Integer)",
                &mut (|n| no_out!(Natural::checked_from(n))),
            ),
            (
                "Natural::checked_from(&Integer)",
                &mut (|n| no_out!(Natural::checked_from(&n))),
            ),
        ],
    );
}

fn benchmark_natural_exact_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::exact_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural::exact_from(Integer)",
                &mut (|n| no_out!(Natural::exact_from(n))),
            ),
            (
                "Natural::exact_from(&Integer)",
                &mut (|n| no_out!(Natural::exact_from(&n))),
            ),
        ],
    );
}

fn benchmark_natural_saturating_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::saturating_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural::saturating_from(Integer)",
                &mut (|n| no_out!(Natural::saturating_from(n))),
            ),
            (
                "Natural::saturating_from(&Integer)",
                &mut (|n| no_out!(Natural::saturating_from(&n))),
            ),
        ],
    );
}

fn benchmark_natural_convertible_from_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::convertible_from(Integer)",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural::convertible_from(Integer)",
                &mut (|n| no_out!(Natural::convertible_from(n))),
            ),
            (
                "Natural::convertible_from(&Integer)",
                &mut (|n| no_out!(Natural::convertible_from(&n))),
            ),
        ],
    );
}

fn benchmark_natural_convertible_from_integer_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural::convertible_from(Integer)",
        BenchmarkType::Algorithms,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|n| no_out!(Natural::convertible_from(n)))),
            (
                "using checked_from",
                &mut (|n| no_out!(Natural::checked_from(n).is_some())),
            ),
        ],
    );
}
