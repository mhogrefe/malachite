use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_nonempty_unsigned_vec_and_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::{
    nrm_pairs_of_natural_and_unsigned, rm_pairs_of_natural_and_unsigned,
    rm_pairs_of_unsigned_and_natural,
};
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::SignificantBits;
use malachite_nz::natural::logic::and_limb::limbs_and_limb;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use natural::logic::and::{natural_and_alt_1, natural_and_alt_2};
use num::{BigUint, ToPrimitive};

pub fn natural_and_limb_alt_1(n: &Natural, u: Limb) -> Limb {
    Limb::checked_from(natural_and_alt_1(n, &Natural::from(u))).unwrap()
}

pub fn natural_and_limb_alt_2(n: &Natural, u: Limb) -> Limb {
    Limb::checked_from(natural_and_alt_2(n, &Natural::from(u))).unwrap()
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_and_limb);
    register_demo!(registry, demo_natural_and_assign_limb);
    register_demo!(registry, demo_natural_and_limb);
    register_demo!(registry, demo_natural_and_limb_ref);
    register_demo!(registry, demo_limb_and_natural);
    register_demo!(registry, demo_limb_and_natural_ref);
    register_demo!(registry, demo_limb_and_assign_natural);
    register_demo!(registry, demo_limb_and_assign_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_and_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_and_assign_limb_library_comparison
    );
    #[cfg(feature = "64_bit_limbs")]
    register_bench!(registry, Large, benchmark_natural_and_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_and_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_and_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_and_limb_algorithms);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_and_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_and_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_and_assign_natural_evaluation_strategy
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

fn demo_natural_and_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n &= u;
        println!("x := {}; x &= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_and_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} & {} = {}", n_old, u, n & u);
    }
}

fn demo_natural_and_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<Limb>(gm).take(limit) {
        println!("&{} & {} = {}", n, u, &n & u);
    }
}

fn demo_limb_and_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} + {} = {}", u, n_old, u & n);
    }
}

fn demo_limb_and_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        println!("{} + &{} = {}", u, n, u & &n);
    }
}

fn demo_limb_and_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u &= n;
        println!("x := {}; x &= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_limb_and_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        u &= &n;
        println!("x := {}; x &= &{}; x = {}", u_old, n, u);
    }
}

fn benchmark_limbs_and_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_and_limb(&[Limb], Limb)",
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

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_and_assign_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural &= Limb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x &= y)),
            ("rug", &mut (|((mut x, y), _)| x &= y)),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_natural_and_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural &= Limb",
        BenchmarkType::Single,
        pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x &= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_and_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural & Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x & y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_and_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_natural_and_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural & Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural & Limb", &mut (|(x, y)| no_out!(x & y))),
            ("&Natural & Limb", &mut (|(x, y)| no_out!(&x & y))),
        ],
    );
}

fn benchmark_natural_and_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "&Natural & Limb",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(x & y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(natural_and_limb_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(natural_and_limb_alt_2(&x, y))),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_and_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb & &Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x & y))),
            ("rug", &mut (|((x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_limb_and_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb & Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb & Natural", &mut (|(x, y)| no_out!(x & y))),
            ("Limb & &Natural", &mut (|(x, y)| no_out!(x & &y))),
        ],
    );
}

fn benchmark_limb_and_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb &= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb &= Natural", &mut (|(mut x, y)| x &= y)),
            ("Limb &= &Natural", &mut (|(mut x, y)| no_out!(x &= &y))),
        ],
    );
}
