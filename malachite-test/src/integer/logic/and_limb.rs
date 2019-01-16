use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{pairs_of_integer_and_unsigned, pairs_of_unsigned_and_integer};
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::{rm_pairs_of_integer_and_unsigned, rm_pairs_of_unsigned_and_integer};
use integer::logic::and::{integer_and_alt_1, integer_and_alt_2};
use malachite_base::misc::CheckedFrom;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

pub fn integer_and_limb_alt_1(n: &Integer, u: Limb) -> Limb {
    Limb::checked_from(integer_and_alt_1(n, &Integer::from(u))).unwrap()
}

pub fn integer_and_limb_alt_2(n: &Integer, u: Limb) -> Limb {
    Limb::checked_from(integer_and_alt_2(n, &Integer::from(u))).unwrap()
}

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_and_assign_limb);
    register_demo!(registry, demo_integer_and_limb);
    register_demo!(registry, demo_integer_and_limb_ref);
    register_demo!(registry, demo_limb_and_integer);
    register_demo!(registry, demo_limb_and_integer_ref);
    register_demo!(registry, demo_limb_and_assign_integer);
    register_demo!(registry, demo_limb_and_assign_integer_ref);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_assign_limb_library_comparison
    );
    #[cfg(feature = "64_bit_limbs")]
    register_bench!(registry, Large, benchmark_integer_and_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_and_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_and_limb_algorithms);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_limb_and_integer_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_and_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_and_assign_integer_evaluation_strategy
    );
}

fn demo_integer_and_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n &= u;
        println!("x := {}; x &= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_and_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} & {} = {}", n_old, u, n & u);
    }
}

fn demo_integer_and_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_unsigned::<Limb>(gm).take(limit) {
        println!("&{} & {} = {}", n, u, &n & u);
    }
}

fn demo_limb_and_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} & {} = {}", u, n_old, u & n);
    }
}

fn demo_limb_and_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        println!("{} & &{} = {}", u, n, u & &n);
    }
}

fn demo_limb_and_assign_integer(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u &= n;
        println!("x := {}; x &= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_limb_and_assign_integer_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_integer::<Limb>(gm).take(limit) {
        let u_old = u;
        u &= &n;
        println!("x := {}; x &= &{}; x = {}", u_old, n, u);
    }
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_and_assign_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer &= Limb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<Limb>(gm),
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

#[cfg(feature = "64_bit_limbs")]
fn benchmark_integer_and_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer &= Limb",
        BenchmarkType::Single,
        pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x &= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_and_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer & Limb",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x & y))),
            ("rug", &mut (|((x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_integer_and_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer & Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer & Limb", &mut (|(x, y)| no_out!(x & y))),
            ("&Integer & Limb", &mut (|(x, y)| no_out!(&x & y))),
        ],
    );
}

fn benchmark_integer_and_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "&Integer & Limb",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("default", &mut (|(x, y)| no_out!(x & y))),
            (
                "using bits explicitly",
                &mut (|(x, y)| no_out!(integer_and_limb_alt_1(&x, y))),
            ),
            (
                "using limbs explicitly",
                &mut (|(x, y)| no_out!(integer_and_limb_alt_2(&x, y))),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_limb_and_integer_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb & &Integer",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, ref n))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x & y))),
            ("rug", &mut (|((x, y), _)| no_out!(x & y))),
        ],
    );
}

fn benchmark_limb_and_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb & Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Limb & Integer", &mut (|(x, y)| no_out!(x & y))),
            ("Limb & &Integer", &mut (|(x, y)| no_out!(x & &y))),
        ],
    );
}

fn benchmark_limb_and_assign_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb &= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Limb &= Integer", &mut (|(mut x, y)| x &= y)),
            ("Limb &= &Integer", &mut (|(mut x, y)| no_out!(x &= &y))),
        ],
    );
}
