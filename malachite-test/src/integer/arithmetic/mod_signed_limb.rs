use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
#[cfg(feature = "64_bit_limbs")]
use inputs::integer::nm_pairs_of_integer_and_nonzero_signed;
#[cfg(feature = "32_bit_limbs")]
use inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_signed, rm_pairs_of_integer_and_nonzero_signed,
};
use inputs::integer::{pairs_of_integer_and_nonzero_signed, pairs_of_signed_and_nonzero_integer};
use malachite_base::num::traits::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, Mod, ModAssign, SignificantBits,
};
use malachite_nz::platform::SignedLimb;
use num::{BigInt, Integer};
#[cfg(feature = "32_bit_limbs")]
use rug::ops::RemRounding;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_rem_assign_signed_limb);
    register_demo!(registry, demo_integer_rem_signed_limb);
    register_demo!(registry, demo_integer_rem_signed_limb_ref);
    register_demo!(registry, demo_integer_mod_assign_signed_limb);
    register_demo!(registry, demo_integer_mod_signed_limb);
    register_demo!(registry, demo_integer_mod_signed_limb_ref);
    register_demo!(registry, demo_integer_ceiling_mod_assign_signed_limb);
    register_demo!(registry, demo_integer_ceiling_mod_signed_limb);
    register_demo!(registry, demo_integer_ceiling_mod_signed_limb_ref);
    register_demo!(registry, demo_signed_limb_rem_integer);
    register_demo!(registry, demo_signed_limb_rem_integer_ref);
    register_demo!(registry, demo_signed_limb_mod_integer);
    register_demo!(registry, demo_signed_limb_mod_integer_ref);
    register_demo!(registry, demo_signed_limb_ceiling_mod_integer);
    register_demo!(registry, demo_signed_limb_ceiling_mod_integer_ref);
    register_bench!(registry, Large, benchmark_integer_rem_assign_signed_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_signed_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_mod_assign_signed_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_assign_signed_limb
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_rem_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_ceiling_mod_integer_evaluation_strategy
    );
}

pub fn num_mod_signed_limb(x: BigInt, i: SignedLimb) -> BigInt {
    x.mod_floor(&BigInt::from(i))
}

fn demo_integer_rem_assign_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        n %= i;
        println!("x := {}; x %= {}; x = {}", n_old, i, n);
    }
}

fn demo_integer_rem_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", n_old, i, n % i);
    }
}

fn demo_integer_rem_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        println!("&{} % {} = {}", n, i, &n % i);
    }
}

fn demo_integer_mod_assign_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        n.mod_assign(i);
        println!("x := {}; x.mod_assign({}); x = {}", n_old, i, n);
    }
}

fn demo_integer_mod_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", n_old, i, n.mod_op(i));
    }
}

fn demo_integer_mod_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        println!("(&{}).mod({}) = {}", n, i, (&n).mod_op(i));
    }
}

fn demo_integer_ceiling_mod_assign_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_assign(i);
        println!("x := {}; x.ceiling_mod_assign({}); x = {}", n_old, i, n);
    }
}

fn demo_integer_ceiling_mod_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod({}) = {}", n_old, i, n.ceiling_mod(i));
    }
}

fn demo_integer_ceiling_mod_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        println!("(&{}).ceiling_mod({}) = {}", n, i, (&n).ceiling_mod(i));
    }
}

fn demo_signed_limb_rem_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", i, n_old, i % n);
    }
}

fn demo_signed_limb_rem_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % &{} = {}", i, n_old, i % &n);
    }
}

fn demo_signed_limb_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {:?}", i, n_old, i.mod_op(n));
    }
}

fn demo_signed_limb_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod(&{}) = {:?}", i, n_old, i.mod_op(&n));
    }
}

fn demo_signed_limb_ceiling_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod({}) = {:?}", i, n_old, i.ceiling_mod(n));
    }
}

fn demo_signed_limb_ceiling_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod(&{}) = {:?}", i, n_old, i.ceiling_mod(&n));
    }
}

fn benchmark_integer_rem_assign_signed_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer %= SignedLimb",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x %= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_rem_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer % SignedLimb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x % y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_integer_rem_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer % SignedLimb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_integer_rem_signed_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer % SignedLimb",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x % y))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_integer_rem_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer % SignedLimb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer % SignedLimb", &mut (|(x, y)| no_out!(x % y))),
            ("&Integer % SignedLimb", &mut (|(x, y)| no_out!(&x % y))),
        ],
    );
}

fn benchmark_integer_mod_assign_signed_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.mod_assign(SignedLimb)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.mod_assign(y)))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_mod_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.mod(SignedLimb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_mod_signed_limb(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.rem_floor(y)))),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_integer_mod_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.mod(SignedLimb)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_nonzero_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.mod_op(y)))),
            (
                "num",
                &mut (|((x, y), _)| no_out!(num_mod_signed_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_mod_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.mod(SignedLimb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.mod(SignedLimb)",
                &mut (|(x, y)| no_out!(x.mod_op(y))),
            ),
            (
                "(&Integer).mod(SignedLimb)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_assign_signed_limb(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod_assign(SignedLimb)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.ceiling_mod_assign(y)))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_ceiling_mod_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(SignedLimb)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.ceiling_mod(y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.rem_ceil(y)))),
        ],
    );
}

fn benchmark_integer_ceiling_mod_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(SignedLimb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
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

fn benchmark_integer_ceiling_mod_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(SignedLimb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_mod(SignedLimb)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "(&Integer).ceiling_mod(SignedLimb)",
                &mut (|(x, y)| no_out!((&x).ceiling_mod(y))),
            ),
        ],
    );
}

fn benchmark_signed_limb_rem_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb % Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("SignedLimb % Integer", &mut (|(x, y)| no_out!(x % y))),
            ("SignedLimb % &Integer", &mut (|(x, y)| no_out!(x % &y))),
        ],
    );
}

fn benchmark_signed_limb_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb.mod(Integer)",
                &mut (|(x, y)| no_out!(x.mod_op(y))),
            ),
            (
                "SignedLimb.mod(&Integer)",
                &mut (|(x, y)| no_out!(x.mod_op(&y))),
            ),
        ],
    );
}

fn benchmark_signed_limb_ceiling_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.ceiling_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb.ceiling_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "SignedLimb.ceiling_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(&y))),
            ),
        ],
    );
}
