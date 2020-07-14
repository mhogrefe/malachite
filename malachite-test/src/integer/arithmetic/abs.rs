use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, UnsignedAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use num::Signed;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::integer::{integers, nrm_integers};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_abs_assign);
    register_demo!(registry, demo_integer_abs);
    register_demo!(registry, demo_integer_abs_ref);
    register_demo!(registry, demo_integer_unsigned_abs);
    register_demo!(registry, demo_integer_unsigned_abs_ref);
    register_demo!(registry, demo_integer_unsigned_abs_ref_out);
    register_bench!(registry, Large, benchmark_integer_abs_assign);
    register_bench!(registry, Large, benchmark_integer_abs_library_comparison);
    register_bench!(registry, Large, benchmark_integer_abs_evaluation_strategy);
    register_bench!(registry, Large, benchmark_integer_unsigned_abs);
    register_bench!(
        registry,
        Large,
        benchmark_integer_unsigned_abs_evaluation_strategy
    );
}

fn demo_integer_abs_assign(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!("n := {}; n.abs_assign(); n = {}", n_old, n);
    }
}

fn demo_integer_abs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("|{}| = {}", n.clone(), n.abs());
    }
}

fn demo_integer_abs_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("|&{}| = {}", n, (&n).abs());
    }
}

fn demo_integer_unsigned_abs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("unsigned_abs({}) = {}", n.clone(), n.unsigned_abs());
    }
}

fn demo_integer_unsigned_abs_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("unsigned_abs(&{}) = {}", n, (&n).unsigned_abs());
    }
}

fn demo_integer_unsigned_abs_ref_out(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("{}.unsigned_abs_ref() = {}", n, n.unsigned_abs_ref());
    }
}

fn benchmark_integer_abs_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.abs_assign()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.abs_assign()))],
    );
}

fn benchmark_integer_abs_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.abs()",
        BenchmarkType::LibraryComparison,
        nrm_integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, n)| no_out!(n.abs()))),
            ("num", &mut (|(n, _, _)| no_out!(n.abs()))),
            ("rug", &mut (|(_, n, _)| no_out!(n.abs().cmp0()))),
        ],
    );
}

fn benchmark_integer_abs_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.abs()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Integer.abs()", &mut (|n| no_out!(n.abs()))),
            ("(&Integer).abs()", &mut (|n| no_out!((&n).abs()))),
        ],
    );
}

fn benchmark_integer_unsigned_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.unsigned_abs()",
        BenchmarkType::Single,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("malachite", &mut (|n| no_out!(n.unsigned_abs())))],
    );
}

fn benchmark_integer_unsigned_abs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.unsigned_abs()",
        BenchmarkType::EvaluationStrategy,
        integers(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.unsigned_abs()",
                &mut (|n| no_out!(n.unsigned_abs())),
            ),
            (
                "(&Integer).unsigned_abs()",
                &mut (|n| no_out!((&n).unsigned_abs())),
            ),
            (
                "Integer.unsigned_abs_ref()",
                &mut (|n| no_out!(n.unsigned_abs_ref())),
            ),
        ],
    );
}
