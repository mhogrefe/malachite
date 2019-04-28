use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
#[cfg(feature = "64_bit_limbs")]
use inputs::integer::nm_pairs_of_integer_and_nonzero_signed;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::nrm_pairs_of_integer_and_nonzero_signed;
use inputs::integer::{pairs_of_integer_and_nonzero_signed, pairs_of_signed_and_nonzero_integer};
use malachite_base::num::traits::{DivRem, SignificantBits};
use malachite_nz::platform::SignedLimb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_assign_signed_limb);
    register_demo!(registry, demo_integer_div_signed_limb);
    register_demo!(registry, demo_integer_div_signed_limb_ref);
    register_demo!(registry, demo_signed_limb_div_integer);
    register_demo!(registry, demo_signed_limb_div_integer_ref);
    register_bench!(registry, Large, benchmark_integer_div_assign_signed_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_div_integer_evaluation_strategy
    );
}

fn demo_integer_div_assign_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        n /= i;
        println!("x := {}; x /= {}; x = {}", n_old, i, n);
    }
}

fn demo_integer_div_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} / {} = {}", n_old, i, n / i);
    }
}

fn demo_integer_div_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        println!("&{} / {} = {}", n, i, &n / i);
    }
}

fn demo_signed_limb_div_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} / {} = {}", i, n_old, i / n);
    }
}

fn demo_signed_limb_div_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        println!("{} / &{} = {}", i, n, i / &n);
    }
}

fn benchmark_integer_div_assign_signed_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer /= SignedLimb",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x /= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_div_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer / SignedLimb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x / y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x / y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x / y))),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_integer_div_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer / SignedLimb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x / y))),
            ("num", &mut (|((x, y), _)| no_out!(x / y))),
        ],
    );
}

fn benchmark_integer_div_signed_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer / SignedLimb",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x / y))),
            ("using div_rem", &mut (|(x, y)| no_out!(x.div_rem(y).0))),
        ],
    );
}

fn benchmark_integer_div_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer / SignedLimb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer / SignedLimb", &mut (|(x, y)| no_out!(x / y))),
            ("&Integer / SignedLimb", &mut (|(x, y)| no_out!(&x / y))),
        ],
    );
}

fn benchmark_signed_limb_div_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb / Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("SignedLimb / Integer", &mut (|(x, y)| no_out!(x / y))),
            ("SignedLimb / &Integer", &mut (|(x, y)| no_out!(x / &y))),
        ],
    );
}
