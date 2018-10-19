use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_positive_unsigned, pairs_of_integer_and_positive_unsigned,
    pairs_of_unsigned_and_nonzero_integer, rm_pairs_of_integer_and_positive_unsigned,
};
use malachite_base::num::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignRem, DivMod, DivRem, Mod, SignificantBits,
};
use num::{BigInt, Integer, ToPrimitive};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_assign_mod_u32);
    register_demo!(registry, demo_integer_div_mod_u32);
    register_demo!(registry, demo_integer_div_mod_u32_ref);
    register_demo!(registry, demo_integer_div_assign_rem_u32);
    register_demo!(registry, demo_integer_div_rem_u32);
    register_demo!(registry, demo_integer_div_rem_u32_ref);
    register_demo!(registry, demo_integer_ceiling_div_assign_neg_mod_u32);
    register_demo!(registry, demo_integer_ceiling_div_neg_mod_u32);
    register_demo!(registry, demo_integer_ceiling_div_neg_mod_u32_ref);
    register_demo!(registry, demo_integer_ceiling_div_assign_mod_u32);
    register_demo!(registry, demo_integer_ceiling_div_mod_u32);
    register_demo!(registry, demo_integer_ceiling_div_mod_u32_ref);
    register_demo!(registry, demo_u32_div_mod_integer);
    register_demo!(registry, demo_u32_div_mod_integer_ref);
    register_demo!(registry, demo_u32_div_rem_integer);
    register_demo!(registry, demo_u32_div_rem_integer_ref);
    register_demo!(registry, demo_u32_ceiling_div_neg_mod_integer);
    register_demo!(registry, demo_u32_ceiling_div_neg_mod_integer_ref);
    register_demo!(registry, demo_u32_ceiling_div_mod_integer);
    register_demo!(registry, demo_u32_ceiling_div_mod_integer_ref);
    register_bench!(registry, Large, benchmark_integer_div_assign_mod_u32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_mod_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_div_assign_rem_u32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_rem_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_assign_neg_mod_u32
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_neg_mod_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_neg_mod_u32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_neg_mod_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_u32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_rem_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_ceiling_div_neg_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_ceiling_div_mod_integer_evaluation_strategy
    );
}

pub fn num_div_mod_u32(x: BigInt, u: u32) -> (BigInt, u32) {
    let (quotient, remainder) = x.div_mod_floor(&BigInt::from(u));
    (quotient, remainder.to_u32().unwrap())
}

pub fn rug_div_mod_u32(x: rug::Integer, u: u32) -> (rug::Integer, u32) {
    let (quotient, remainder) = x.div_rem_euc(rug::Integer::from(u));
    (quotient, remainder.to_u32_wrapping())
}

pub fn num_div_rem_u32(x: BigInt, u: u32) -> (BigInt, BigInt) {
    x.div_rem(&BigInt::from(u))
}

pub fn rug_div_rem_u32(x: rug::Integer, u: u32) -> (rug::Integer, rug::Integer) {
    x.div_rem(rug::Integer::from(u))
}

pub fn rug_ceiling_div_neg_mod_u32(x: rug::Integer, u: u32) -> (rug::Integer, u32) {
    let (quotient, remainder) = x.div_rem_ceil(rug::Integer::from(u));
    (quotient, (-remainder).to_u32_wrapping())
}

pub fn rug_ceiling_div_mod_u32(x: rug::Integer, u: u32) -> (rug::Integer, rug::Integer) {
    x.div_rem_ceil(rug::Integer::from(u))
}

fn demo_integer_div_assign_mod_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_mod(u);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_div_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", n_old, u, n.div_mod(u));
    }
}

fn demo_integer_div_mod_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).div_mod({}) = {:?}", n, u, (&n).div_mod(u));
    }
}

fn demo_integer_div_assign_rem_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_rem(u);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_div_rem_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", n_old, u, n.div_rem(u));
    }
}

fn demo_integer_div_rem_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).div_rem({}) = {:?}", n, u, (&n).div_rem(u));
    }
}

fn demo_integer_ceiling_div_assign_neg_mod_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.ceiling_div_assign_neg_mod(u);
        println!(
            "x := {}; x.ceiling_div_assign_neg_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_ceiling_div_neg_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_neg_mod({}) = {:?}",
            n_old,
            u,
            n.ceiling_div_neg_mod(u)
        );
    }
}

fn demo_integer_ceiling_div_neg_mod_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        println!(
            "(&{}).ceiling_div_neg_mod({}) = {:?}",
            n,
            u,
            (&n).ceiling_div_neg_mod(u)
        );
    }
}

fn demo_integer_ceiling_div_assign_mod_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.ceiling_div_assign_mod(u);
        println!(
            "x := {}; x.ceiling_div_assign_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_ceiling_div_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            n_old,
            u,
            n.ceiling_div_mod(u)
        );
    }
}

fn demo_integer_ceiling_div_mod_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        println!(
            "(&{}).ceiling_div_mod({}) = {:?}",
            n,
            u,
            (&n).ceiling_div_mod(u)
        );
    }
}

fn demo_u32_div_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", u, n_old, u.div_mod(n));
    }
}

fn demo_u32_div_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod(&{}) = {:?}", u, n_old, u.div_mod(&n));
    }
}

fn demo_u32_div_rem_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", u, n_old, u.div_rem(n));
    }
}

fn demo_u32_div_rem_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem(&{}) = {:?}", u, n_old, u.div_rem(&n));
    }
}

fn demo_u32_ceiling_div_neg_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_neg_mod({}) = {:?}",
            u,
            n_old,
            u.ceiling_div_neg_mod(n)
        );
    }
}

fn demo_u32_ceiling_div_neg_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_neg_mod(&{}) = {:?}",
            u,
            n_old,
            u.ceiling_div_neg_mod(&n)
        );
    }
}

fn demo_u32_ceiling_div_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            u,
            n_old,
            u.ceiling_div_mod(n)
        );
    }
}

fn demo_u32_ceiling_div_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod(&{}) = {:?}",
            u,
            n_old,
            u.ceiling_div_mod(&n)
        );
    }
}

fn benchmark_integer_div_assign_mod_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_assign_mod(u32)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
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

fn benchmark_integer_div_mod_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(u32)",
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

fn benchmark_integer_div_mod_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_mod(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
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

fn benchmark_integer_div_mod_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_mod(u32)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "(&Integer).div_mod(u32)",
                &mut (|(x, y)| no_out!((&x).div_mod(y))),
            ),
        ],
    );
}

fn benchmark_integer_div_assign_rem_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_assign_rem(u32)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
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

fn benchmark_integer_div_rem_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(u32)",
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
                &mut (|((x, y), _, _)| no_out!(num_div_rem_u32(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_rem_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_rem_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_rem(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
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

fn benchmark_integer_div_rem_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_rem(u32)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "(&Integer).div_rem(u32)",
                &mut (|(x, y)| no_out!((&x).div_rem(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_assign_neg_mod_u32(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_assign_neg_mod(u32)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y)| no_out!(x.ceiling_div_assign_neg_mod(y))),
        )],
    );
}

fn benchmark_integer_ceiling_div_neg_mod_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_neg_mod(u32)",
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
                &mut (|(_, (x, y))| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "rug",
                &mut (|((x, y), _)| no_out!(rug_ceiling_div_neg_mod_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_neg_mod_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_neg_mod(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            //TODO(
            //    "using div_round and %",
            //    &mut (|(x, y)| {
            //        let remainder = (&x).neg_mod(y);
            //        (x.div_round(y, RoundingMode::Ceiling), remainder);
            //    }),
            //),
        ],
    );
}

fn benchmark_integer_ceiling_div_neg_mod_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_neg_mod(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_div_neg_mod(u32)",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "(&Integer).ceiling_div_neg_mod(u32)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_neg_mod(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(u32)",
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
                &mut (|((x, y), _)| no_out!(rug_ceiling_div_mod_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.ceiling_div_mod(y)))),
            //TODO(
            //    "using div_round and %",
            //    &mut (|(x, y)| {
            //        let remainder = -(&x).neg_mod(y);
            //        (x.div_round(y, RoundingMode::Ceiling), remainder);
            //    }),
            //),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_div_mod(u32)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "(&Integer).ceiling_div_mod(u32)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_mod(y))),
            ),
        ],
    );
}

fn benchmark_u32_div_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "u32.div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(&y))),
            ),
        ],
    );
}

fn benchmark_u32_div_rem_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_rem(Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "u32.div_rem(&Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(&y))),
            ),
        ],
    );
}

fn benchmark_u32_ceiling_div_neg_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.ceiling_div_neg_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.ceiling_div_neg_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "u32.ceiling_div_neg_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(&y))),
            ),
        ],
    );
}

fn benchmark_u32_ceiling_div_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.ceiling_div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.ceiling_div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "u32.ceiling_div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(&y))),
            ),
        ],
    );
}
