use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_signed, pairs_of_signed_and_integer,
                      rm_pairs_of_integer_and_signed, rm_pairs_of_signed_and_integer};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use std::iter::repeat;

//TODO limbs

pub fn integer_and_i32_alt(n: &Integer, i: i32) -> Integer {
    let n_negative = *n < 0;
    let i_negative = i < 0;
    let i = Integer::from(i);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.significant_bits() >= i.significant_bits() {
            Box::new(
                n.twos_complement_bits()
                    .zip(i.twos_complement_bits().chain(repeat(i_negative))),
            )
        } else {
            Box::new(
                n.twos_complement_bits()
                    .chain(repeat(n_negative))
                    .zip(i.twos_complement_bits()),
            )
        };
    let mut and_bits = Vec::new();
    for (b, c) in bit_zip {
        and_bits.push(b && c);
    }
    and_bits.push(n_negative && i_negative);
    Integer::from_twos_complement_bits_asc(&and_bits)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_and_assign_i32);
    register_demo!(registry, demo_integer_and_i32);
    register_demo!(registry, demo_i32_and_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_assign_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_i32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_and_i32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_i32_and_integer_library_comparison
    );
}
fn demo_integer_and_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n &= u;
        println!("x := {}; x &= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_and_i32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        println!("&{} & {} = {}", n, u, &n & u);
    }
}

fn demo_i32_and_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_signed_and_integer::<i32>(gm).take(limit) {
        println!("{} + &{} = {}", u, n, u & &n);
    }
}

fn benchmark_integer_and_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer &= i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x &= y)),
            ("rug", &mut (|((mut x, y), _)| x &= y)),
        ],
    );
}

fn benchmark_integer_and_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "&Integer & i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(&x & y))),
            ("rug", &mut (|((x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_integer_and_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "&Integer & i32",
        BenchmarkType::LibraryComparison,
        pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(&x & y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(integer_and_i32_alt(&x, y))),
            ),
        ],
    );
}

fn benchmark_i32_and_integer_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "i32 & &Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_signed_and_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x & &y))),
            ("rug", &mut (|((x, y), _)| no_out!(x & y))),
        ],
    );
}
