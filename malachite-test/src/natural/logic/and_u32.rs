use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned,
                      pairs_of_unsigned_and_natural, rm_pairs_of_natural_and_unsigned,
                      rm_pairs_of_unsigned_and_natural};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use num::{BigUint, ToPrimitive};
use std::iter::repeat;

pub fn natural_and_u32_alt(n: &Natural, u: u32) -> u32 {
    let u = Natural::from(u);
    let bit_zip: Box<Iterator<Item = (bool, bool)>> =
        if n.significant_bits() >= u.significant_bits() {
            Box::new(n.bits().zip(u.bits().chain(repeat(false))))
        } else {
            Box::new(n.bits().chain(repeat(false)).zip(u.bits()))
        };
    let mut and_bits = Vec::new();
    for (b, c) in bit_zip {
        and_bits.push(b && c);
    }
    u32::checked_from(&Natural::from_bits_asc(&and_bits)).unwrap()
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_and_assign_u32);
    register_demo!(registry, demo_natural_and_u32);
    register_demo!(registry, demo_u32_and_natural);
    register_bench!(
        registry,
        Large,
        benchmark_natural_and_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_and_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_and_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_u32_and_natural_library_comparison
    );
}

pub fn num_and_u32(x: BigUint, u: u32) -> u32 {
    (x & BigUint::from(u)).to_u32().unwrap()
}

fn demo_natural_and_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n &= u;
        println!("x := {}; x &= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_and_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} & {} = {}", n, u, &n & u);
    }
}

fn demo_u32_and_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        println!("{} + &{} = {}", u, n, u & &n);
    }
}

fn benchmark_natural_and_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural &= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_unsigned::<u32>(gm),
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

fn benchmark_natural_and_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "&Natural & u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(&x & y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_and_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_natural_and_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "&Natural & u32",
        BenchmarkType::LibraryComparison,
        pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(&x & y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(natural_and_u32_alt(&x, y))),
            ),
        ],
    );
}

fn benchmark_u32_and_natural_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32 & &Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<u32>(gm),
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
