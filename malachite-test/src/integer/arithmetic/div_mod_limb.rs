use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_positive_unsigned, pairs_of_integer_and_positive_unsigned,
    pairs_of_unsigned_and_nonzero_integer, rm_pairs_of_integer_and_positive_unsigned,
};
use malachite_base::num::traits::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod, SignificantBits,
};
use malachite_base::round::RoundingMode;
use malachite_nz::platform::Limb;
use num::{BigInt, Integer, ToPrimitive};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_assign_mod_limb);
    register_demo!(registry, demo_integer_div_mod_limb);
    register_demo!(registry, demo_integer_div_mod_limb_ref);
    register_demo!(registry, demo_integer_div_assign_rem_limb);
    register_demo!(registry, demo_integer_div_rem_limb);
    register_demo!(registry, demo_integer_div_rem_limb_ref);
    register_demo!(registry, demo_integer_ceiling_div_assign_mod_limb);
    register_demo!(registry, demo_integer_ceiling_div_mod_limb);
    register_demo!(registry, demo_integer_ceiling_div_mod_limb_ref);
    register_demo!(registry, demo_limb_div_mod_integer);
    register_demo!(registry, demo_limb_div_mod_integer_ref);
    register_demo!(registry, demo_limb_div_rem_integer);
    register_demo!(registry, demo_limb_div_rem_integer_ref);
    register_demo!(registry, demo_limb_ceiling_div_mod_integer);
    register_demo!(registry, demo_limb_ceiling_div_mod_integer_ref);
    register_bench!(registry, Large, benchmark_integer_div_assign_mod_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_mod_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_div_assign_rem_limb);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_rem_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_rem_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_ceiling_div_mod_integer_evaluation_strategy
    );
}

pub fn num_div_mod_u32(x: BigInt, u: u32) -> (BigInt, u32) {
    let (quotient, remainder) = x.div_mod_floor(&BigInt::from(u));
    (quotient, remainder.to_u32().unwrap())
}

pub fn rug_div_mod_u32(x: rug::Integer, u: u32) -> (rug::Integer, u32) {
    let (quotient, remainder) = x.div_rem_floor(rug::Integer::from(u));
    (quotient, remainder.to_u32().unwrap())
}

pub fn num_div_rem_limb(x: BigInt, u: Limb) -> (BigInt, BigInt) {
    x.div_rem(&BigInt::from(u))
}

pub fn rug_div_rem_limb(x: rug::Integer, u: Limb) -> (rug::Integer, rug::Integer) {
    x.div_rem(rug::Integer::from(u))
}

pub fn rug_ceiling_div_mod_limb(x: rug::Integer, u: Limb) -> (rug::Integer, rug::Integer) {
    x.div_rem_ceil(rug::Integer::from(u))
}

fn demo_integer_div_assign_mod_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_mod(u);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_div_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", n_old, u, n.div_mod(u));
    }
}

fn demo_integer_div_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).div_mod({}) = {:?}", n, u, (&n).div_mod(u));
    }
}

fn demo_integer_div_assign_rem_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_rem(u);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_div_rem_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", n_old, u, n.div_rem(u));
    }
}

fn demo_integer_div_rem_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).div_rem({}) = {:?}", n, u, (&n).div_rem(u));
    }
}

fn demo_integer_ceiling_div_assign_mod_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.ceiling_div_assign_mod(u);
        println!(
            "x := {}; x.ceiling_div_assign_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_ceiling_div_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            n_old,
            u,
            n.ceiling_div_mod(u)
        );
    }
}

fn demo_integer_ceiling_div_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!(
            "(&{}).ceiling_div_mod({}) = {:?}",
            n,
            u,
            (&n).ceiling_div_mod(u)
        );
    }
}

fn demo_limb_div_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", u, n_old, u.div_mod(n));
    }
}

fn demo_limb_div_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod(&{}) = {:?}", u, n_old, u.div_mod(&n));
    }
}

fn demo_limb_div_rem_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", u, n_old, u.div_rem(n));
    }
}

fn demo_limb_div_rem_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem(&{}) = {:?}", u, n_old, u.div_rem(&n));
    }
}

fn demo_limb_ceiling_div_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            u,
            n_old,
            u.ceiling_div_mod(n)
        );
    }
}

fn demo_limb_ceiling_div_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod(&{}) = {:?}",
            u,
            n_old,
            u.ceiling_div_mod(&n)
        );
    }
}

fn benchmark_integer_div_assign_mod_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_assign_mod(Limb)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_mod(y))),
        )],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_integer_div_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_mod(y)))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_div_mod_u32(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_mod_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_mod_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_mod(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.div_mod(y)))),
            (
                "using / and %",
                &mut (|(x, y)| {
                    let remainder = (&x).mod_op(y);
                    (x / y, remainder);
                }),
            ),
        ],
    );
}

fn benchmark_integer_div_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_mod(Limb)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "(&Integer).div_mod(Limb)",
                &mut (|(x, y)| no_out!((&x).div_mod(y))),
            ),
        ],
    );
}

fn benchmark_integer_div_assign_rem_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_assign_rem(Limb)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.div_assign_rem(y))),
        )],
    );
}

fn benchmark_integer_div_rem_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_rem(y)))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_div_rem_limb(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_rem_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_rem_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_rem(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.div_rem(y)))),
            (
                "using / and %",
                &mut (|(x, y)| {
                    let remainder = &x % y;
                    (x / y, remainder);
                }),
            ),
        ],
    );
}

fn benchmark_integer_div_rem_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_rem(Limb)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "(&Integer).div_rem(Limb)",
                &mut (|(x, y)| no_out!((&x).div_rem(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(Limb)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "rug",
                &mut (|((x, y), _)| no_out!(rug_ceiling_div_mod_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.ceiling_div_mod(y)))),
            (
                "using div_round and %",
                &mut (|(x, y)| {
                    no_out!(((&x).div_round(y, RoundingMode::Ceiling), x.ceiling_mod(y)))
                }),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_div_mod(Limb)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "(&Integer).ceiling_div_mod(Limb)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_mod(y))),
            ),
        ],
    );
}

fn benchmark_limb_div_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "Limb.div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(&y))),
            ),
        ],
    );
}

fn benchmark_limb_div_rem_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_rem(Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "Limb.div_rem(&Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(&y))),
            ),
        ],
    );
}

fn benchmark_limb_ceiling_div_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.ceiling_div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.ceiling_div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "Limb.ceiling_div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(&y))),
            ),
        ],
    );
}
