use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_unsigned, pairs_of_integer_and_unsigned,
    pairs_of_unsigned_and_integer, rm_pairs_of_integer_and_unsigned,
    rm_pairs_of_unsigned_and_integer,
};
use malachite_base::num::SignificantBits;
use num::BigInt;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_mul_assign_u32);
    register_demo!(registry, demo_integer_mul_u32);
    register_demo!(registry, demo_integer_mul_u32_ref);
    register_demo!(registry, demo_u32_mul_integer);
    register_demo!(registry, demo_u32_mul_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_mul_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_mul_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_mul_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_mul_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_mul_integer_evaluation_strategy
    );
}

pub fn num_mul_u32(x: BigInt, u: u32) -> BigInt {
    x * BigInt::from(u)
}

fn demo_integer_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n *= u;
        println!("x := {}; x *= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_mul_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", n_old, u, n * u);
    }
}

fn demo_integer_mul_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} * {} = {}", n, u, &n * u);
    }
}

fn demo_u32_mul_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", u, n_old, u * n);
    }
}

fn demo_u32_mul_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * &{} = {}", u, n_old, u * &n);
    }
}

fn benchmark_integer_mul_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer *= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<u32>(gm),
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

fn benchmark_integer_mul_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer * u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_unsigned(gm),
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

fn benchmark_integer_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer * u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer * u32", &mut (|(x, y)| no_out!(x * y))),
            ("&Integer * u32", &mut (|(x, y)| no_out!(&x * y))),
        ],
    );
}

fn benchmark_u32_mul_integer_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 * Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_integer::<u32>(gm),
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

fn benchmark_u32_mul_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 * Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 * Integer", &mut (|(x, y)| no_out!(x * y))),
            ("u32 * &Integer", &mut (|(x, y)| no_out!(x * &y))),
        ],
    );
}
