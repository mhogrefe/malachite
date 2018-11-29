use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_positive_unsigned, pairs_of_integer_and_positive_unsigned,
    pairs_of_unsigned_and_nonzero_integer, rm_pairs_of_integer_and_positive_unsigned,
};
use malachite_base::num::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, Mod, ModAssign, SignificantBits,
};
use num::{BigInt, Integer, ToPrimitive};
use rug::ops::RemRounding;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_rem_assign_u32);
    register_demo!(registry, demo_integer_rem_u32);
    register_demo!(registry, demo_integer_rem_u32_ref);
    register_demo!(registry, demo_integer_mod_assign_u32);
    register_demo!(registry, demo_integer_mod_u32);
    register_demo!(registry, demo_integer_mod_u32_ref);
    register_demo!(registry, demo_integer_ceiling_mod_assign_u32);
    register_demo!(registry, demo_integer_ceiling_mod_u32);
    register_demo!(registry, demo_integer_ceiling_mod_u32_ref);
    register_demo!(registry, demo_u32_rem_integer);
    register_demo!(registry, demo_u32_rem_integer_ref);
    register_demo!(registry, demo_u32_rem_assign_integer);
    register_demo!(registry, demo_u32_rem_assign_integer_ref);
    register_demo!(registry, demo_u32_mod_integer);
    register_demo!(registry, demo_u32_mod_integer_ref);
    register_demo!(registry, demo_u32_ceiling_mod_integer);
    register_demo!(registry, demo_u32_ceiling_mod_integer_ref);
    register_bench!(registry, Large, benchmark_integer_rem_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_rem_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_mod_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_ceiling_mod_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_u32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_rem_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_rem_assign_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_mod_integer_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_ceiling_mod_integer_evaluation_strategy
    );
}

pub fn num_mod_u32(x: BigInt, u: u32) -> u32 {
    x.mod_floor(&BigInt::from(u)).to_u32().unwrap()
}

fn demo_integer_rem_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n %= u;
        println!("x := {}; x %= {}; x = {}", n_old, u, n);
    }
}

fn demo_integer_rem_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", n_old, u, n % u);
    }
}

fn demo_integer_rem_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("&{} % {} = {}", n, u, &n % u);
    }
}

fn demo_integer_mod_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n.mod_assign(u);
        println!("x := {}; x.mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_integer_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", n_old, u, n.mod_op(u));
    }
}

fn demo_integer_mod_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).mod({}) = {}", n, u, (&n).mod_op(u));
    }
}

fn demo_integer_ceiling_mod_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_assign(u);
        println!("x := {}; x.ceiling_mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_integer_ceiling_mod_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod({}) = {}", n_old, u, n.ceiling_mod(u));
    }
}

fn demo_integer_ceiling_mod_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("(&{}).ceiling_mod({}) = {}", n, u, (&n).ceiling_mod(u));
    }
}

fn demo_u32_rem_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", u, n_old, u % n);
    }
}

fn demo_u32_rem_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % &{} = {}", u, n_old, u % &n);
    }
}

fn demo_u32_rem_assign_integer(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u %= n;
        println!("x := {}; x %= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_u32_rem_assign_integer_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let u_old = u;
        u %= &n;
        println!("x := {}; x %= &{}; x = {}", u_old, n, u);
    }
}

fn demo_u32_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", u, n_old, u.mod_op(n));
    }
}

fn demo_u32_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod(&{}) = {}", u, n_old, u.mod_op(&n));
    }
}

fn demo_u32_ceiling_mod_integer(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod({}) = {}", u, n_old, u.ceiling_mod(n));
    }
}

fn demo_u32_ceiling_mod_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_nonzero_integer::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.ceiling_mod(&{}) = {}", u, n_old, u.ceiling_mod(&n));
    }
}

fn benchmark_integer_rem_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer %= u32",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x %= y))],
    );
}

fn benchmark_integer_rem_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer % u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_unsigned::<u32>(gm),
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

fn benchmark_integer_rem_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer % u32",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
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

fn benchmark_integer_rem_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer % u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer % u32", &mut (|(x, y)| no_out!(x % y))),
            ("&Integer % u32", &mut (|(x, y)| no_out!(&x % y))),
        ],
    );
}

fn benchmark_integer_mod_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.mod_assign(u32)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.mod_assign(y)))],
    );
}

fn benchmark_integer_mod_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.mod(u32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_mod_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.mod_u(y)))),
        ],
    );
}

fn benchmark_integer_mod_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.mod(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Integer.mod(u32)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            (
                "(&Integer).mod(u32)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.ceiling_mod_assign(u32)",
        BenchmarkType::Single,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.ceiling_mod_assign(y)))],
    );
}

fn benchmark_integer_ceiling_mod_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(u32)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_positive_unsigned::<u32>(gm),
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

fn benchmark_integer_ceiling_mod_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.ceiling_mod(u32)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
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

fn benchmark_integer_ceiling_mod_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_mod(u32)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "(&Integer).ceiling_mod(u32)",
                &mut (|(x, y)| no_out!((&x).ceiling_mod(y))),
            ),
        ],
    );
}

fn benchmark_u32_rem_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 % Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 % Integer", &mut (|(x, y)| no_out!(x % y))),
            ("u32 % &Integer", &mut (|(x, y)| no_out!(x % &y))),
        ],
    );
}

fn benchmark_u32_rem_assign_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 %= Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 %= Integer", &mut (|(mut x, y)| x %= y)),
            ("u32 %= &Integer", &mut (|(mut x, y)| x %= &y)),
        ],
    );
}

fn benchmark_u32_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32.mod(Integer)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("u32.mod(&Integer)", &mut (|(x, y)| no_out!(x.mod_op(&y)))),
        ],
    );
}

fn benchmark_u32_ceiling_mod_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.ceiling_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_nonzero_integer::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.ceiling_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "u32.ceiling_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(&y))),
            ),
        ],
    );
}
