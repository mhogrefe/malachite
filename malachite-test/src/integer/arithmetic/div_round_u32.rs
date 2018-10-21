use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_positive_unsigned, pairs_of_integer_and_positive_unsigned,
    rm_pairs_of_integer_and_positive_unsigned,
    triples_of_integer_positive_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_nonzero_integer_and_rounding_mode_var_1,
};
use malachite_base::num::{CeilingDivNegMod, DivRound, DivRoundAssign, SignificantBits};
use malachite_base::round::RoundingMode;
use num::{BigInt, Integer};
use rug::ops::DivRounding;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_round_assign_u32);
    register_demo!(registry, demo_integer_div_round_u32);
    register_demo!(registry, demo_integer_div_round_u32_ref);
    register_demo!(registry, demo_u32_div_round_integer);
    register_demo!(registry, demo_u32_div_round_integer_ref);
    register_bench!(registry, Large, benchmark_integer_div_round_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_u32_down_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_u32_floor_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_u32_ceiling_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_u32_ceiling_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_round_integer_evaluation_strategy
    );
}

pub fn num_div_round_u32_floor(x: BigInt, u: u32) -> BigInt {
    x.div_floor(&BigInt::from(u))
}

fn demo_integer_div_round_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u, rm) in
        triples_of_integer_positive_unsigned_and_rounding_mode_var_1::<u32>(gm).take(limit)
    {
        let n_old = n.clone();
        n.div_round_assign(u, rm);
        println!(
            "x := {}; x.div_round_assign({}, {}); x = {}",
            n_old, u, rm, n
        );
    }
}

fn demo_integer_div_round_u32(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in
        triples_of_integer_positive_unsigned_and_rounding_mode_var_1::<u32>(gm).take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.div_round({}, {}) = {}",
            n_old,
            u,
            rm,
            n.div_round(u, rm)
        );
    }
}

fn demo_integer_div_round_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in
        triples_of_integer_positive_unsigned_and_rounding_mode_var_1::<u32>(gm).take(limit)
    {
        let n_old = n.clone();
        println!(
            "(&{}).div_round({}, {}) = {}",
            n_old,
            u,
            rm,
            (&n).div_round(u, rm)
        );
    }
}

fn demo_u32_div_round_integer(gm: GenerationMode, limit: usize) {
    for (u, n, rm) in
        triples_of_unsigned_nonzero_integer_and_rounding_mode_var_1::<u32>(gm).take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.div_round({}, {}) = {}",
            u,
            n_old,
            rm,
            u.div_round(n, rm)
        );
    }
}

fn demo_u32_div_round_integer_ref(gm: GenerationMode, limit: usize) {
    for (u, n, rm) in
        triples_of_unsigned_nonzero_integer_and_rounding_mode_var_1::<u32>(gm).take(limit)
    {
        println!("{}.div_round(&{}, {}) = {}", u, n, rm, u.div_round(&n, rm));
    }
}

fn benchmark_integer_div_round_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_round_assign(u32, RoundingMode)",
        BenchmarkType::Single,
        triples_of_integer_positive_unsigned_and_rounding_mode_var_1::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y, rm)| x.div_round_assign(y, rm)),
        )],
    );
}

fn benchmark_integer_div_round_u32_down_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(u32, RoundingMode::Down)",
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
                &mut (|(_, (x, y))| no_out!(x.div_round(y, RoundingMode::Down))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x.div_trunc(y)))),
        ],
    );
}

fn benchmark_integer_div_round_u32_floor_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(u32, RoundingMode::Floor)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.div_round(y, RoundingMode::Floor))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_div_round_u32_floor(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_floor(y)))),
        ],
    );
}

fn benchmark_integer_div_round_u32_ceiling_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(u32, RoundingMode::Ceiling)",
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
                &mut (|(_, (x, y))| no_out!(x.div_round(y, RoundingMode::Ceiling))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x.div_ceil(y)))),
        ],
    );
}

fn benchmark_integer_div_round_u32_ceiling_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(u32, RoundingMode::Ceiling)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|(x, y)| no_out!(x.div_round(y, RoundingMode::Ceiling))),
            ),
            (
                "using ceiling_div_neg_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y).0)),
            ),
        ],
    );
}

fn benchmark_integer_div_round_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(u32, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_positive_unsigned_and_rounding_mode_var_1::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_round(u32, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))),
            ),
            (
                "(&Integer).div_round(u32, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).div_round(y, rm))),
            ),
        ],
    );
}

fn benchmark_u32_div_round_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_round(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_nonzero_integer_and_rounding_mode_var_1::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_round(Integer, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))),
            ),
            (
                "u32.div_round(&Integer, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(&y, rm))),
            ),
        ],
    );
}
