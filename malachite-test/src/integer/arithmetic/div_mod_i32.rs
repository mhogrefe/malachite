use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_signed, pairs_of_integer_and_nonzero_signed,
    pairs_of_signed_and_nonzero_integer, rm_pairs_of_integer_and_nonzero_signed,
};
use malachite_base::num::{
    CeilingDivAssignMod, CeilingDivMod, CeilingMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
    DivRound, Mod, SignificantBits,
};
use malachite_base::round::RoundingMode;
use num::{BigInt, Integer};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_assign_mod_i32);
    register_demo!(registry, demo_integer_div_mod_i32);
    register_demo!(registry, demo_integer_div_mod_i32_ref);
    register_demo!(registry, demo_integer_div_assign_rem_i32);
    register_demo!(registry, demo_integer_div_rem_i32);
    register_demo!(registry, demo_integer_div_rem_i32_ref);
    register_demo!(registry, demo_integer_ceiling_div_assign_mod_i32);
    register_demo!(registry, demo_integer_ceiling_div_mod_i32);
    register_demo!(registry, demo_integer_ceiling_div_mod_i32_ref);
    register_demo!(registry, demo_i32_div_mod_integer);
    register_demo!(registry, demo_i32_div_mod_integer_ref);
    register_demo!(registry, demo_i32_div_rem_integer);
    register_demo!(registry, demo_i32_div_rem_integer_ref);
    register_demo!(registry, demo_i32_ceiling_div_mod_integer);
    register_demo!(registry, demo_i32_ceiling_div_mod_integer_ref);
    register_bench!(registry, Large, benchmark_integer_div_assign_mod_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_i32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_mod_i32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_mod_i32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_div_assign_rem_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_i32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_div_rem_i32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_rem_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_i32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_div_mod_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_div_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_div_rem_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_ceiling_div_mod_integer_evaluation_strategy
    );
}

pub fn num_div_mod_i32(x: BigInt, i: i32) -> (BigInt, BigInt) {
    x.div_mod_floor(&BigInt::from(i))
}

pub fn rug_div_mod_i32(x: rug::Integer, i: i32) -> (rug::Integer, rug::Integer) {
    x.div_rem_floor(rug::Integer::from(i))
}

pub fn num_div_rem_i32(x: BigInt, i: i32) -> (BigInt, BigInt) {
    x.div_rem(&BigInt::from(i))
}

pub fn rug_div_rem_i32(x: rug::Integer, i: i32) -> (rug::Integer, rug::Integer) {
    x.div_rem(rug::Integer::from(i))
}

pub fn rug_ceiling_div_mod_i32(x: rug::Integer, i: i32) -> (rug::Integer, rug::Integer) {
    x.div_rem_ceil(rug::Integer::from(i))
}

fn demo_integer_div_assign_mod_i32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_mod(u);
        println!(
            "x := {}; x.div_assign_mod({}) = {}; x = {}",
            n_old, u, remainder, n
        );
    }
}

fn demo_integer_div_mod_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", n_old, i, n.div_mod(i));
    }
}

fn demo_integer_div_mod_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        println!("(&{}).div_mod({}) = {:?}", n, i, (&n).div_mod(i));
    }
}

fn demo_integer_div_assign_rem_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.div_assign_rem(i);
        println!(
            "x := {}; x.div_assign_rem({}) = {}; x = {}",
            n_old, i, remainder, n
        );
    }
}

fn demo_integer_div_rem_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", n_old, i, n.div_rem(i));
    }
}

fn demo_integer_div_rem_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        println!("(&{}).div_rem({}) = {:?}", n, i, (&n).div_rem(i));
    }
}

fn demo_integer_ceiling_div_assign_mod_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        let remainder = n.ceiling_div_assign_mod(i);
        println!(
            "x := {}; x.ceiling_div_assign_mod({}) = {}; x = {}",
            n_old, i, remainder, n
        );
    }
}

fn demo_integer_ceiling_div_mod_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            n_old,
            i,
            n.ceiling_div_mod(i)
        );
    }
}

fn demo_integer_ceiling_div_mod_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        println!(
            "(&{}).ceiling_div_mod({}) = {:?}",
            n,
            i,
            (&n).ceiling_div_mod(i)
        );
    }
}

fn demo_i32_div_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod({}) = {:?}", i, n_old, i.div_mod(n));
    }
}

fn demo_i32_div_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_mod(&{}) = {:?}", i, n_old, i.div_mod(&n));
    }
}

fn demo_i32_div_rem_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem({}) = {:?}", i, n_old, i.div_rem(n));
    }
}

fn demo_i32_div_rem_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_rem(&{}) = {:?}", i, n_old, i.div_rem(&n));
    }
}

fn demo_i32_ceiling_div_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod({}) = {:?}",
            i,
            n_old,
            i.ceiling_div_mod(n)
        );
    }
}

fn demo_i32_ceiling_div_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_div_mod(&{}) = {:?}",
            i,
            n_old,
            i.ceiling_div_mod(&n)
        );
    }
}

fn benchmark_integer_div_assign_mod_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_assign_mod(i32)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_div_mod_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(i32)",
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
                &mut (|((x, y), _, _)| no_out!(num_div_mod_i32(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_mod_i32(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_mod_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_mod(i32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_div_mod_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_mod(i32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_mod(i32)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "(&Integer).div_mod(i32)",
                &mut (|(x, y)| no_out!((&x).div_mod(y))),
            ),
        ],
    );
}

fn benchmark_integer_div_assign_rem_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_assign_rem(i32)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_div_rem_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(i32)",
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
                &mut (|((x, y), _, _)| no_out!(num_div_rem_i32(x, y))),
            ),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_rem_i32(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_div_rem_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_rem(i32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_div_rem_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_rem(i32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_rem(i32)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "(&Integer).div_rem(i32)",
                &mut (|(x, y)| no_out!((&x).div_rem(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(i32)",
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
                &mut (|((x, y), _)| no_out!(rug_ceiling_div_mod_i32(x, y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_div_mod_i32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(i32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_ceiling_div_mod_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_div_mod(i32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_div_mod(i32)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "(&Integer).ceiling_div_mod(i32)",
                &mut (|(x, y)| no_out!((&x).ceiling_div_mod(y))),
            ),
        ],
    );
}

fn benchmark_i32_div_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i32.div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(y))),
            ),
            (
                "i32.div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.div_mod(&y))),
            ),
        ],
    );
}

fn benchmark_i32_div_rem_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.div_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i32.div_rem(Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(y))),
            ),
            (
                "i32.div_rem(&Integer)",
                &mut (|(x, y)| no_out!(x.div_rem(&y))),
            ),
        ],
    );
}

fn benchmark_i32_ceiling_div_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.ceiling_div_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i32.ceiling_div_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y))),
            ),
            (
                "i32.ceiling_div_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(&y))),
            ),
        ],
    );
}
