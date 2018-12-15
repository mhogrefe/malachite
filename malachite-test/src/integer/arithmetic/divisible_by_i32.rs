use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_signed, pairs_of_integer_and_signed, pairs_of_signed_and_integer,
};
use malachite_base::num::{DivisibleBy, SignificantBits};
use num::{BigInt, Integer, Zero};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_divisible_by_i32);
    register_demo!(registry, demo_i32_divisible_by_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_divisible_by_i32_library_comparison
    );
    register_bench!(registry, Large, benchmark_i32_divisible_by_integer);
}

pub fn num_divisible_by_i32(x: BigInt, i: i32) -> bool {
    x == BigInt::zero() || i != 0 && x.is_multiple_of(&BigInt::from(i))
}

pub fn rug_divisible_by_i32(x: rug::Integer, i: i32) -> bool {
    x.is_divisible(&rug::Integer::from(i))
}

fn demo_integer_divisible_by_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        if n.divisible_by(i) {
            println!("{} is divisible by {}", n, i);
        } else {
            println!("{} is not divisible by {}", n, i);
        }
    }
}

fn demo_i32_divisible_by_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        if i.divisible_by(&n) {
            println!("{} is divisible by {}", i, n);
        } else {
            println!("{} is not divisible by {}", i, n);
        }
    }
}

fn benchmark_integer_divisible_by_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.divisible_by(i32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed(gm),
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
                &mut (|((x, y), _, _)| no_out!(num_divisible_by_i32(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_divisible_by_i32(x, y))),
            ),
        ],
    );
}

fn benchmark_i32_divisible_by_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i32.divisible_by(&Integer)",
        BenchmarkType::Single,
        pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "i32.divisible_by(&Integer)",
            &mut (|(x, ref y)| no_out!(x.divisible_by(y))),
        )],
    );
}
