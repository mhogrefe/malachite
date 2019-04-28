use malachite_base::num::traits::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod, SignificantBits,
};
use malachite_base::round::RoundingMode;
use malachite_nz::platform::SignedLimb;
use num::{BigInt, Integer};
use rug;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_signed, pairs_of_integer_and_nonzero_signed,
    pairs_of_signed_and_nonzero_integer, rm_pairs_of_integer_and_nonzero_signed,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_assign_mod_signed_limb);
    register_demo!(registry, demo_integer_div_mod_signed_limb);
    register_demo!(registry, demo_integer_div_mod_signed_limb_ref);
    register_demo!(registry, demo_integer_div_assign_rem_signed_limb);
    register_demo!(registry, demo_integer_div_rem_signed_limb);
    register_demo!(registry, demo_integer_div_rem_signed_limb_ref);
    register_demo!(registry, demo_integer_ceiling_div_assign_mod_signed_limb);
    register_demo!(registry, demo_integer_ceiling_div_mod_signed_limb);
    register_demo!(registry, demo_integer_ceiling_div_mod_signed_limb_ref);
    register_demo!(registry, demo_signed_limb_div_mod_integer);
    register_demo!(registry, demo_signed_limb_div_mod_integer_ref);
    register_demo!(registry, demo_signed_limb_div_rem_integer);
    register_demo!(registry, demo_signed_limb_div_rem_integer_ref);
    register_demo!(registry, demo_signed_limb_ceiling_div_mod_integer);
    register_demo!(registry, demo_signed_limb_ceiling_div_mod_integer_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_assign_mod_signed_limb
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_assign_rem_signed_limb
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_signed_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_signed_limb_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_signed_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_div_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_div_rem_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_signed_limb_ceiling_div_mod_integer_evaluation_strategy
    );
}

pub fn num_div_mod_signed_limb(x: BigInt, i: SignedLimb) -> (BigInt, BigInt) {
    x.div_mod_floor(&BigInt::from(i))
}

pub fn rug_div_mod_signed_limb(x: rug::Integer, i: SignedLimb) -> (rug::Integer, rug::Integer) {
    x.div_rem_floor(rug::Integer::from(i))
}

pub fn num_div_rem_signed_limb(x: BigInt, i: SignedLimb) -> (BigInt, BigInt) {
    x.div_rem(&BigInt::from(i))
}

pub fn rug_div_rem_signed_limb(x: rug::Integer, i: SignedLimb) -> (rug::Integer, rug::Integer) {
    x.div_rem(rug::Integer::from(i))
}

pub fn rug_ceiling_div_mod_signed_limb(
    x: rug::Integer,
    i: SignedLimb,
) -> (rug::Integer, rug::Integer) {
    x.div_rem_ceil(rug::Integer::from(i))
}

fn demo_integer_div_assign_mod_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_mod(u);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_div_mod_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", n_old, i, n.div_mod(i));
    }
}

fn demo_integer_div_mod_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        println!("(&{}).div_mod({}) = {:?}", n, i, (&n).div_mod(i));
    }
}

fn demo_integer_div_assign_rem_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_rem(i);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            n_old, i, remainder, n
        );
    }
}

fn demo_integer_div_rem_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", n_old, i, n.div_rem(i));
    }
}

fn demo_integer_div_rem_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        println!("(&{}).div_rem({}) = {:?}", n, i, (&n).div_rem(i));
    }
}

fn demo_integer_ceiling_div_assign_mod_signed_limb(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.ceiling_div_assign_mod(i);
        println!(
            "x := {}; x.ceiling_div_assign_mod({}) = {}; x = {}",
            n_old, i, remainder, n
        );
    }
}

fn demo_integer_ceiling_div_mod_signed_limb(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            n_old,
            i,
            n.ceiling_div_mod(i)
        );
    }
}

fn demo_integer_ceiling_div_mod_signed_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm).take(limit) {
        println!(
            "(&{}).ceiling_div_mod({}) = {:?}",
            n,
            i,
            (&n).ceiling_div_mod(i)
        );
    }
}

fn demo_signed_limb_div_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", i, n_old, i.div_mod(n));
    }
}

fn demo_signed_limb_div_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod(&{}) = {:?}", i, n_old, i.div_mod(&n));
    }
}

fn demo_signed_limb_div_rem_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", i, n_old, i.div_rem(n));
    }
}

fn demo_signed_limb_div_rem_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem(&{}) = {:?}", i, n_old, i.div_rem(&n));
    }
}

fn demo_signed_limb_ceiling_div_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            i,
            n_old,
            i.ceiling_div_mod(n)
        );
    }
}

fn demo_signed_limb_ceiling_div_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod(&{}) = {:?}",
            i,
            n_old,
            i.ceiling_div_mod(&n)
        );
    }
}

fn benchmark_integer_div_assign_mod_signed_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_assign_mod(SignedLimb)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
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

fn benchmark_integer_div_mod_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(SignedLimb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_mod(y)))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_div_mod_signed_limb(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_mod_signed_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_mod_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(SignedLimb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
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

fn benchmark_integer_div_mod_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(SignedLimb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_mod(SignedLimb)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "(&Integer).div_mod(SignedLimb)",
                &mut (|(x, y)| no_out!((&x).div_mod(y))),
            ),
        ],
    );
}

fn benchmark_integer_div_assign_rem_signed_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_assign_rem(SignedLimb)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
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

fn benchmark_integer_div_rem_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(SignedLimb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_rem(y)))),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_div_rem_signed_limb(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_rem_signed_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_rem_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(SignedLimb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
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

fn benchmark_integer_div_rem_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(SignedLimb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_rem(SignedLimb)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "(&Integer).div_rem(SignedLimb)",
                &mut (|(x, y)| no_out!((&x).div_rem(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_signed_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(SignedLimb)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_nonzero_signed(gm),
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
                &mut (|((x, y), _)| no_out!(rug_ceiling_div_mod_signed_limb(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_signed_limb_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(SignedLimb)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
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

fn benchmark_integer_ceiling_div_mod_signed_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(SignedLimb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_div_mod(SignedLimb)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "(&Integer).ceiling_div_mod(SignedLimb)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_mod(y))),
            ),
        ],
    );
}

fn benchmark_signed_limb_div_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb.div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "SignedLimb.div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(&y))),
            ),
        ],
    );
}

fn benchmark_signed_limb_div_rem_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.div_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb.div_rem(Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "SignedLimb.div_rem(&Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(&y))),
            ),
        ],
    );
}

fn benchmark_signed_limb_ceiling_div_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "SignedLimb.ceiling_div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<SignedLimb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "SignedLimb.ceiling_div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "SignedLimb.ceiling_div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(&y))),
            ),
        ],
    );
}
