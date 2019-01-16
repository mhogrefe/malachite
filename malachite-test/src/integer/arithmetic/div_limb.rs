use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
#[cfg(feature = "64_bit_limbs")]
use inputs::integer::nm_pairs_of_integer_and_positive_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::nrm_pairs_of_integer_and_positive_unsigned;
use inputs::integer::{
    pairs_of_integer_and_positive_unsigned, pairs_of_unsigned_and_nonzero_integer,
};
use malachite_base::num::{DivRem, SignificantBits};
use malachite_nz::platform::Limb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_assign_limb);
    register_demo!(registry, demo_integer_div_limb);
    register_demo!(registry, demo_integer_div_limb_ref);
    register_demo!(registry, demo_limb_div_integer);
    register_demo!(registry, demo_limb_div_integer_ref);
    register_bench!(registry, Large, benchmark_integer_div_assign_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_integer_evaluation_strategy
    );
}

fn demo_integer_div_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n /= u;
        println!("x := {}; x /= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_div_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} / {} = {}", n_old, u, n / u);
    }
}

fn demo_integer_div_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("&{} / {} = {}", n, u, &n / u);
    }
}

fn demo_limb_div_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} / {} = {}", u, n_old, u / n);
    }
}

fn demo_limb_div_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        println!("{} / &{} = {}", u, n, u / &n);
    }
}

fn benchmark_integer_div_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer /= Limb",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x /= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_div_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer / Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_unsigned::<Limb>(gm),
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
fn benchmark_integer_div_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer / Limb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_positive_unsigned::<Limb>(gm),
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

fn benchmark_integer_div_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer / Limb",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
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

fn benchmark_integer_div_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer / Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer / Limb", &mut (|(x, y)| no_out!(x / y))),
            ("&Integer / Limb", &mut (|(x, y)| no_out!(&x / y))),
        ],
    );
}

fn benchmark_limb_div_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb / Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Limb / Integer", &mut (|(x, y)| no_out!(x / y))),
            ("Limb / &Integer", &mut (|(x, y)| no_out!(x / &y))),
        ],
    );
}
