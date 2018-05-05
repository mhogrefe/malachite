use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned,
    pairs_of_unsigned_and_natural, rm_pairs_of_natural_and_unsigned,
    rm_pairs_of_unsigned_and_natural,
};
use malachite_base::num::SignificantBits;
use num::BigUint;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mul_assign_u32);
    register_demo!(registry, demo_natural_mul_u32);
    register_demo!(registry, demo_natural_mul_u32_ref);
    register_demo!(registry, demo_u32_mul_natural);
    register_demo!(registry, demo_u32_mul_natural_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mul_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_mul_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_mul_natural_evaluation_strategy
    );
}

pub fn num_mul_u32(x: BigUint, u: u32) -> BigUint {
    x * BigUint::from(u)
}

fn demo_natural_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n *= u;
        println!("x := {}; x *= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_mul_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", n_old, u, n * u);
    }
}

fn demo_natural_mul_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} * {} = {}", n, u, &n * u);
    }
}

fn demo_u32_mul_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", u, n_old, u * n);
    }
}

fn demo_u32_mul_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * &{} = {}", u, n_old, u * &n);
    }
}

fn benchmark_natural_mul_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural *= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x *= y)),
            ("rug", &mut (|((mut x, y), _)| x *= y)),
        ],
    );
}

fn benchmark_natural_mul_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural * u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x * y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_mul_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x * y))),
        ],
    );
}

fn benchmark_natural_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural * u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural * u32", &mut (|(x, y)| no_out!(x * y))),
            ("&Natural * u32", &mut (|(x, y)| no_out!(&x * y))),
        ],
    );
}

fn benchmark_u32_mul_natural_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 * Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x * y))),
            ("rug", &mut (|((x, y), _)| no_out!(x * y))),
        ],
    );
}

fn benchmark_u32_mul_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 * Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 * Natural", &mut (|(x, y)| no_out!(x * y))),
            ("u32 * &Natural", &mut (|(x, y)| no_out!(x * &y))),
        ],
    );
}
