use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_unsigned, pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer,
};
use malachite_base::num::{DivisibleBy, SignificantBits};
use num::{BigInt, Integer, Zero};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_divisible_by_u32);
    register_demo!(registry, demo_u32_divisible_by_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_u32_divisible_by_integer);
}

pub fn num_divisible_by_u32(x: BigInt, u: u32) -> bool {
    x == BigInt::zero() || u != 0 && x.is_multiple_of(&BigInt::from(u))
}

fn demo_integer_divisible_by_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        if n.divisible_by(u) {
            println!("{} is divisible by {}", n, u);
        } else {
            println!("{} is not divisible by {}", n, u);
        }
    }
}

fn demo_u32_divisible_by_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        if u.divisible_by(&n) {
            println!("{} is divisible by {}", u, n);
        } else {
            println!("{} is not divisible by {}", u, n);
        }
    }
}

fn benchmark_integer_divisible_by_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by(u32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.divisible_by(y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_divisible_by_u32(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.is_divisible_u(y)))),
        ],
    );
}

fn benchmark_u32_divisible_by_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32.divisible_by(&Integer)",
        BenchmarkType::Single,
        pairs_of_unsigned_and_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "u32.divisible_by(&Integer)",
            &mut (|(x, ref y)| no_out!(x.divisible_by(y))),
        )],
    );
}
