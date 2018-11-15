use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_signed, pairs_of_integer_and_nonzero_signed,
    pairs_of_signed_and_nonzero_integer, rm_pairs_of_integer_and_nonzero_signed,
};
use malachite_base::num::{
    CeilingDivMod, CeilingDivNegMod, CeilingMod, CeilingModAssign, DivMod, Mod, ModAssign, NegMod,
    NegModAssign, SignificantBits, UnsignedAbs,
};
use num::{BigInt, Integer, ToPrimitive};
use rug::{self, ops::RemRounding};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_rem_assign_i32);
    register_demo!(registry, demo_integer_rem_i32);
    register_demo!(registry, demo_integer_rem_i32_ref);
    register_demo!(registry, demo_integer_mod_assign_i32);
    register_demo!(registry, demo_integer_mod_i32);
    register_demo!(registry, demo_integer_mod_i32_ref);
    register_demo!(registry, demo_integer_neg_mod_assign_i32);
    register_demo!(registry, demo_integer_neg_mod_i32);
    register_demo!(registry, demo_integer_neg_mod_i32_ref);
    register_demo!(registry, demo_integer_ceiling_mod_assign_i32);
    register_demo!(registry, demo_integer_ceiling_mod_i32);
    register_demo!(registry, demo_integer_ceiling_mod_i32_ref);
    register_demo!(registry, demo_i32_rem_integer);
    register_demo!(registry, demo_i32_rem_integer_ref);
    register_demo!(registry, demo_i32_mod_integer);
    register_demo!(registry, demo_i32_mod_integer_ref);
    register_demo!(registry, demo_i32_neg_mod_integer);
    register_demo!(registry, demo_i32_neg_mod_integer_ref);
    register_demo!(registry, demo_i32_ceiling_mod_integer);
    register_demo!(registry, demo_i32_ceiling_mod_integer_ref);
    register_bench!(registry, Large, benchmark_integer_rem_assign_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_i32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_rem_i32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_i32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_mod_assign_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_i32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_neg_mod_assign_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_neg_mod_i32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_neg_mod_i32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_neg_mod_i32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_ceiling_mod_assign_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_i32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_rem_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_neg_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_ceiling_mod_integer_evaluation_strategy
    );
}

pub fn num_mod_i32(x: BigInt, i: i32) -> u32 {
    x.mod_floor(&BigInt::from(i.unsigned_abs()))
        .to_u32()
        .unwrap()
}

pub fn rug_mod_i32(x: rug::Integer, i: i32) -> u32 {
    x.rem_floor(i.unsigned_abs()).to_u32_wrapping()
}

pub fn rug_neg_mod_i32(x: rug::Integer, i: i32) -> u32 {
    (-x.rem_ceil(i.unsigned_abs())).to_u32_wrapping()
}

pub fn rug_ceiling_mod_i32(x: rug::Integer, i: i32) -> rug::Integer {
    x.rem_ceil(i.unsigned_abs())
}

fn demo_integer_rem_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n %= i;
        println!("x := {}; x %= {}; x = {}", n_old, i, n);
    }
}

fn demo_integer_rem_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", n_old, i, n % i);
    }
}

fn demo_integer_rem_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        println!("&{} % {} = {}", n, i, &n % i);
    }
}

fn demo_integer_mod_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n.mod_assign(i);
        println!("x := {}; x.mod_assign({}); x = {}", n_old, i, n);
    }
}

fn demo_integer_mod_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", n_old, i, n.mod_op(i));
    }
}

fn demo_integer_mod_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        println!("(&{}).mod({}) = {}", n, i, (&n).mod_op(i));
    }
}

fn demo_integer_neg_mod_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n.neg_mod_assign(i);
        println!("x := {}; x.neg_mod_assign({}); x = {}", n_old, i, n);
    }
}

fn demo_integer_neg_mod_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod({}) = {}", n_old, i, n.neg_mod(i));
    }
}

fn demo_integer_neg_mod_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        println!("(&{}).neg_mod({}) = {}", n, i, (&n).neg_mod(i));
    }
}

fn demo_integer_ceiling_mod_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_assign(i);
        println!("x := {}; x.ceiling_mod_assign({}); x = {}", n_old, i, n);
    }
}

fn demo_integer_ceiling_mod_i32(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod({}) = {}", n_old, i, n.ceiling_mod(i));
    }
}

fn demo_integer_ceiling_mod_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i) in pairs_of_integer_and_nonzero_signed::<i32>(gm).take(limit) {
        println!("(&{}).ceiling_mod({}) = {}", n, i, (&n).ceiling_mod(i));
    }
}

fn demo_i32_rem_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", i, n_old, i % n);
    }
}

fn demo_i32_rem_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % &{} = {}", i, n_old, i % &n);
    }
}

fn demo_i32_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {:?}", i, n_old, i.mod_op(n));
    }
}

fn demo_i32_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod(&{}) = {:?}", i, n_old, i.mod_op(&n));
    }
}

fn demo_i32_neg_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod({}) = {:?}", i, n_old, i.neg_mod(n));
    }
}

fn demo_i32_neg_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod(&{}) = {:?}", i, n_old, i.neg_mod(&n));
    }
}

fn demo_i32_ceiling_mod_integer(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod({}) = {:?}", i, n_old, i.ceiling_mod(n));
    }
}

fn demo_i32_ceiling_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n) in pairs_of_signed_and_nonzero_integer::<i32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod(&{}) = {:?}", i, n_old, i.ceiling_mod(&n));
    }
}

fn benchmark_integer_rem_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer %= i32",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x %= y))],
    );
}

fn benchmark_integer_rem_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer % i32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_rem_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer % i32",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_rem_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer % i32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer % i32", &mut (|(x, y)| no_out!(x % y))),
            ("&Integer % i32", &mut (|(x, y)| no_out!(&x % y))),
        ],
    );
}

fn benchmark_integer_mod_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.mod_assign(i32)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.mod_assign(y)))],
    );
}

fn benchmark_integer_mod_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.mod(i32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_mod_i32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(rug_mod_i32(x, y)))),
        ],
    );
}

fn benchmark_integer_mod_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.mod(i32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer.mod(i32)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            (
                "(&Integer).mod(i32)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
        ],
    );
}

fn benchmark_integer_neg_mod_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.neg_mod_assign(i32)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.neg_mod_assign(y)))],
    );
}

fn benchmark_integer_neg_mod_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.neg_mod(i32)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_nonzero_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.neg_mod(y)))),
            ("rug", &mut (|((x, y), _)| no_out!(rug_neg_mod_i32(x, y)))),
        ],
    );
}

fn benchmark_integer_neg_mod_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.neg_mod(i32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.neg_mod(y)))),
            (
                "using ceiling_div_neg_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y).1)),
            ),
        ],
    );
}

fn benchmark_integer_neg_mod_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.neg_mod(i32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.neg_mod(i32)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "(&Integer).neg_mod(i32)",
                &mut (|(x, y)| no_out!((&x).neg_mod(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.ceiling_mod_assign(i32)",
        BenchmarkType::Single,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.ceiling_mod_assign(y)))],
    );
}

fn benchmark_integer_ceiling_mod_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(i32)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_ceiling_mod_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.ceiling_mod(i32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_ceiling_mod_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(i32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_mod(i32)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "(&Integer).ceiling_mod(i32)",
                &mut (|(x, y)| no_out!((&x).ceiling_mod(y))),
            ),
        ],
    );
}

fn benchmark_i32_rem_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32 % Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("i32 % Integer", &mut (|(x, y)| no_out!(x % y))),
            ("i32 % &Integer", &mut (|(x, y)| no_out!(x % &y))),
        ],
    );
}

fn benchmark_i32_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("i32.mod(Integer)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("i32.mod(&Integer)", &mut (|(x, y)| no_out!(x.mod_op(&y)))),
        ],
    );
}

fn benchmark_i32_neg_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.neg_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i32.neg_mod(Integer)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "i32.neg_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.neg_mod(&y))),
            ),
        ],
    );
}

fn benchmark_i32_ceiling_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.ceiling_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_signed_and_nonzero_integer::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i32.ceiling_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "i32.ceiling_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(&y))),
            ),
        ],
    );
}
