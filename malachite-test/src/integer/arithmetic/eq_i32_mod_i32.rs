use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    rm_triples_of_integer_signed_and_signed, triples_of_integer_signed_and_signed,
    triples_of_signed_integer_and_signed,
};
use malachite_base::num::{DivisibleBy, EqMod, Mod, SignificantBits};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_eq_i32_mod_i32);
    register_demo!(registry, demo_i32_eq_integer_mod_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_eq_i32_mod_i32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_eq_i32_mod_i32_algorithms);
    register_bench!(registry, Large, benchmark_i32_eq_integer_mod_i32_algorithms);
}

pub fn rug_eq_i32_mod_i32(n: rug::Integer, i: i32, m: i32) -> bool {
    n.is_congruent(&rug::Integer::from(i), &rug::Integer::from(m))
}

fn demo_integer_eq_i32_mod_i32(gm: GenerationMode, limit: usize) {
    for (n, i, modulus) in triples_of_integer_signed_and_signed::<i32>(gm).take(limit) {
        if n.eq_mod(i, modulus) {
            println!("{} is equal to {} mod {}", n, i, modulus);
        } else {
            println!("{} is not equal to {} mod {}", n, i, modulus);
        }
    }
}

fn demo_i32_eq_integer_mod_i32(gm: GenerationMode, limit: usize) {
    for (i, n, modulus) in triples_of_signed_integer_and_signed::<i32>(gm).take(limit) {
        if i.eq_mod(&n, modulus) {
            println!("{} is equal to {} mod {}", i, n, modulus);
        } else {
            println!("{} is not equal to {} mod {}", i, n, modulus);
        }
    }
}

fn benchmark_integer_eq_i32_mod_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.eq_mod(i32, i32)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_integer_signed_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (n, i, modulus))| no_out!(n.eq_mod(i, modulus))),
            ),
            (
                "rug",
                &mut (|((n, i, modulus), _)| no_out!(rug_eq_i32_mod_i32(n, i, modulus))),
            ),
        ],
    );
}

fn benchmark_integer_eq_i32_mod_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.eq_mod(i32, i32)",
        BenchmarkType::Algorithms,
        triples_of_integer_signed_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.eq_mod(i32, i32)",
                &mut (|(n, i, modulus)| no_out!(n.eq_mod(i, modulus))),
            ),
            (
                "Integer == i32 || i32 != 0 && Integer.mod(i32) == i32.mod(i32)",
                &mut (|(n, i, modulus)| {
                    no_out!(n == i || modulus != 0 && n.mod_op(modulus) == i.mod_op(modulus))
                }),
            ),
            (
                "(Integer - i32).divisible_by(i32)",
                &mut (|(n, i, modulus)| no_out!((n - i).divisible_by(modulus))),
            ),
        ],
    );
}

fn benchmark_i32_eq_integer_mod_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i32.eq_mod(&Integer, i32)",
        BenchmarkType::Algorithms,
        triples_of_signed_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i32.eq_mod(&Integer, i32)",
                &mut (|(i, ref n, modulus)| no_out!(i.eq_mod(n, modulus))),
            ),
            (
                "i32 == Integer || i32 != 0 && i32.mod_op(i32) == Integer.mod_op(i32)",
                &mut (|(n, i, modulus)| {
                    no_out!(i == n || modulus != 0 && i.mod_op(modulus) == n.mod_op(modulus))
                }),
            ),
        ],
    );
}
