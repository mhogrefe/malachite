use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_nonempty_unsigned_vec_and_unsigned;
use inputs::natural::{
    nrm_pairs_of_natural_and_unsigned, pairs_of_natural_and_unsigned,
    pairs_of_unsigned_and_natural, rm_pairs_of_natural_and_unsigned,
    rm_pairs_of_unsigned_and_natural,
};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::logic::and_u32::limbs_and_limb;
use malachite_nz::natural::Natural;
use natural::logic::and::{natural_and_alt_1, natural_and_alt_2};
use num::{BigUint, ToPrimitive};

pub fn natural_and_u32_alt_1(n: &Natural, u: u32) -> u32 {
    u32::checked_from(&natural_and_alt_1(n, &Natural::from(u))).unwrap()
}

pub fn natural_and_u32_alt_2(n: &Natural, u: u32) -> u32 {
    u32::checked_from(&natural_and_alt_2(n, &Natural::from(u))).unwrap()
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_and_limb);
    register_demo!(registry, demo_natural_and_assign_u32);
    register_demo!(registry, demo_natural_and_u32);
    register_demo!(registry, demo_u32_and_natural);
    register_bench!(registry, Small, benchmark_limbs_and_limb);
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

fn demo_limbs_and_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_nonempty_unsigned_vec_and_unsigned(gm).take(limit) {
        println!(
            "limbs_and_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_and_limb(&limbs, limb)
        );
    }
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

fn benchmark_limbs_and_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_and_limb(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_and_limb(&limbs, limb))),
        )],
    );
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
        BenchmarkType::Algorithms,
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
                &mut (|(x, y)| no_out!(natural_and_u32_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(natural_and_u32_alt_2(&x, y))),
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
