use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, Mod, ModAssign, SignificantBits,
};
use malachite_nz::platform::Limb;
use num::{BigInt, Integer, ToPrimitive};
#[cfg(feature = "32_bit_limbs")]
use rug::ops::RemRounding;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
#[cfg(feature = "64_bit_limbs")]
use inputs::integer::nm_pairs_of_integer_and_positive_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::{
    nrm_pairs_of_integer_and_positive_unsigned, rm_pairs_of_integer_and_positive_unsigned,
};
use inputs::integer::{
    pairs_of_integer_and_positive_unsigned, pairs_of_unsigned_and_nonzero_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_rem_assign_limb);
    register_demo!(registry, demo_integer_rem_limb);
    register_demo!(registry, demo_integer_rem_limb_ref);
    register_demo!(registry, demo_integer_mod_assign_limb);
    register_demo!(registry, demo_integer_mod_limb);
    register_demo!(registry, demo_integer_mod_limb_ref);
    register_demo!(registry, demo_integer_ceiling_mod_assign_limb);
    register_demo!(registry, demo_integer_ceiling_mod_limb);
    register_demo!(registry, demo_integer_ceiling_mod_limb_ref);
    register_demo!(registry, demo_limb_rem_integer);
    register_demo!(registry, demo_limb_rem_integer_ref);
    register_demo!(registry, demo_limb_rem_assign_integer);
    register_demo!(registry, demo_limb_rem_assign_integer_ref);
    register_demo!(registry, demo_limb_mod_integer);
    register_demo!(registry, demo_limb_mod_integer_ref);
    register_demo!(registry, demo_limb_ceiling_mod_integer);
    register_demo!(registry, demo_limb_ceiling_mod_integer_ref);
    register_bench!(registry, Large, benchmark_integer_rem_assign_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_rem_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_mod_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_ceiling_mod_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_rem_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_rem_assign_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_ceiling_mod_integer_evaluation_strategy
    );
}

pub fn num_mod_u32(x: BigInt, u: u32) -> u32 {
    x.mod_floor(&BigInt::from(u)).to_u32().unwrap()
}

fn demo_integer_rem_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n %= u;
        println!("x := {}; x %= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_rem_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", n_old, u, n % u);
    }
}

fn demo_integer_rem_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("&{} % {} = {}", n, u, &n % u);
    }
}

fn demo_integer_mod_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n.mod_assign(u);
        println!("x := {}; x.mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_integer_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", n_old, u, n.mod_op(u));
    }
}

fn demo_integer_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).mod({}) = {}", n, u, (&n).mod_op(u));
    }
}

fn demo_integer_ceiling_mod_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_assign(u);
        println!("x := {}; x.ceiling_mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_integer_ceiling_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod({}) = {}", n_old, u, n.ceiling_mod(u));
    }
}

fn demo_integer_ceiling_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).ceiling_mod({}) = {}", n, u, (&n).ceiling_mod(u));
    }
}

fn demo_limb_rem_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", u, n_old, u % n);
    }
}

fn demo_limb_rem_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % &{} = {}", u, n_old, u % &n);
    }
}

fn demo_limb_rem_assign_integer(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u %= n;
        println!("x := {}; x %= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_limb_rem_assign_integer_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let u_old = u;
        u %= &n;
        println!("x := {}; x %= &{}; x = {}", u_old, n, u);
    }
}

fn demo_limb_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", u, n_old, u.mod_op(n));
    }
}

fn demo_limb_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod(&{}) = {}", u, n_old, u.mod_op(&n));
    }
}

fn demo_limb_ceiling_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod({}) = {}", u, n_old, u.ceiling_mod(n));
    }
}

fn demo_limb_ceiling_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod(&{}) = {}", u, n_old, u.ceiling_mod(&n));
    }
}

fn benchmark_integer_rem_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer %= Limb",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x %= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_rem_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer % Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x % y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_integer_rem_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer % Limb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_integer_rem_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer % Limb",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x % y))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_integer_rem_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer % Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Integer % Limb", &mut (|(x, y)| no_out!(x % y))),
            ("&Integer % Limb", &mut (|(x, y)| no_out!(&x % y))),
        ],
    );
}

fn benchmark_integer_mod_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.mod_assign(Limb)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.mod_assign(y)))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.mod(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_mod_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.mod_u(y)))),
        ],
    );
}

fn benchmark_integer_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Integer.mod(Limb)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            (
                "(&Integer).mod(Limb)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.ceiling_mod_assign(Limb)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.ceiling_mod_assign(y)))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_ceiling_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(Limb)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.ceiling_mod(y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.rem_ceil(y)))),
        ],
    );
}

fn benchmark_integer_ceiling_mod_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.ceiling_mod(y)))),
            (
                "using ceiling_div_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y).1)),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_mod(Limb)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "(&Integer).ceiling_mod(Limb)",
                &mut (|(x, y)| no_out!((&x).ceiling_mod(y))),
            ),
        ],
    );
}

fn benchmark_limb_rem_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb % Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb % Integer", &mut (|(x, y)| no_out!(x % y))),
            ("Limb % &Integer", &mut (|(x, y)| no_out!(x % &y))),
        ],
    );
}

fn benchmark_limb_rem_assign_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb %= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb %= Integer", &mut (|(mut x, y)| x %= y)),
            ("Limb %= &Integer", &mut (|(mut x, y)| x %= &y)),
        ],
    );
}

fn benchmark_limb_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb.mod(Integer)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("Limb.mod(&Integer)", &mut (|(x, y)| no_out!(x.mod_op(&y)))),
        ],
    );
}

fn benchmark_limb_ceiling_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.ceiling_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.ceiling_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "Limb.ceiling_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(&y))),
            ),
        ],
    );
}
