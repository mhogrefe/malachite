use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer,
                      rm_pairs_of_integer_and_unsigned, rm_pairs_of_unsigned_and_integer};
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::iter::repeat;
use std::u32;

pub fn integer_and_u32_alt_1(n: &Integer, u: u32) -> Natural {
    let u = Natural::from(u);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.significant_bits() >= u.significant_bits() {
            Box::new(n.twos_complement_bits().zip(u.bits().chain(repeat(false))))
        } else {
            Box::new(n.twos_complement_bits().chain(repeat(*n < 0)).zip(u.bits()))
        };
    let mut and_bits = Vec::new();
    for (b, c) in bit_zip {
        and_bits.push(b && c);
    }
    Natural::from_bits_asc(&and_bits)
}

pub fn integer_and_u32_alt_2(n: &Integer, u: u32) -> Natural {
    let u = Natural::from(u);
    let limb_zip: Box<Iterator<Item = (u32, u32)>> =
        if n.twos_complement_limbs().count() as u64 >= u.limb_count() {
            Box::new(n.twos_complement_limbs().zip(u.limbs().chain(repeat(0))))
        } else {
            Box::new(
                n.twos_complement_limbs()
                    .chain(repeat(if *n < 0 { u32::MAX } else { 0 }))
                    .zip(u.limbs()),
            )
        };
    let mut and_limbs = Vec::new();
    for (x, y) in limb_zip {
        and_limbs.push(x & y);
    }
    Natural::from_owned_limbs_asc(and_limbs)
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_and_assign_u32);
    register_demo!(registry, demo_integer_and_u32);
    register_demo!(registry, demo_u32_and_integer);
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_and_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_u32_and_integer_library_comparison
    );
}
fn demo_integer_and_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n &= u;
        println!("x := {}; x &= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_and_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} & {} = {}", n, u, &n & u);
    }
}

fn demo_u32_and_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<u32>(gm).take(limit) {
        println!("{} + &{} = {}", u, n, u & &n);
    }
}

fn benchmark_integer_and_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer &= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<u32>(gm),
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

fn benchmark_integer_and_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "&Integer & u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<u32>(gm),
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

fn benchmark_integer_and_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "&Integer & u32",
        BenchmarkType::LibraryComparison,
        pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(&x & y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(integer_and_u32_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(integer_and_u32_alt_2(&x, y))),
            ),
        ],
    );
}

fn benchmark_u32_and_integer_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 & &Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_integer::<u32>(gm),
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
