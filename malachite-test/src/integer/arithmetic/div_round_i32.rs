use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_signed, pairs_of_integer_and_nonzero_signed,
    rm_pairs_of_integer_and_nonzero_signed,
    triples_of_integer_nonzero_signed_and_rounding_mode_var_1,
    triples_of_signed_nonzero_integer_and_rounding_mode_var_1,
};
use malachite_base::num::{CeilingDivMod, DivRound, DivRoundAssign, SignificantBits};
use malachite_base::round::RoundingMode;
use num::{BigInt, Integer};
use rug::ops::DivRounding;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_div_round_assign_i32);
    register_demo!(registry, demo_integer_div_round_i32);
    register_demo!(registry, demo_integer_div_round_i32_ref);
    register_demo!(registry, demo_i32_div_round_integer);
    register_demo!(registry, demo_i32_div_round_integer_ref);
    register_bench!(registry, Large, benchmark_integer_div_round_assign_i32);
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_i32_down_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_i32_floor_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_i32_ceiling_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_i32_ceiling_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_div_round_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_i32_div_round_integer_evaluation_strategy
    );
}

pub fn num_div_round_i32_floor(x: BigInt, i: i32) -> BigInt {
    x.div_floor(&BigInt::from(i))
}

fn demo_integer_div_round_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i, rm) in
        triples_of_integer_nonzero_signed_and_rounding_mode_var_1::<i32>(gm).take(limit)
    {
        let n_old = n.clone();
        n.div_round_assign(i, rm);
        println!(
            "x := {}; x.div_round_assign({}, {}); x = {}",
            n_old, i, rm, n
        );
    }
}

fn demo_integer_div_round_i32(gm: GenerationMode, limit: usize) {
    for (n, i, rm) in
        triples_of_integer_nonzero_signed_and_rounding_mode_var_1::<i32>(gm).take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.div_round({}, {}) = {}",
            n_old,
            i,
            rm,
            n.div_round(i, rm)
        );
    }
}

fn demo_integer_div_round_i32_ref(gm: GenerationMode, limit: usize) {
    for (n, i, rm) in
        triples_of_integer_nonzero_signed_and_rounding_mode_var_1::<i32>(gm).take(limit)
    {
        let n_old = n.clone();
        println!(
            "(&{}).div_round({}, {}) = {}",
            n_old,
            i,
            rm,
            (&n).div_round(i, rm)
        );
    }
}

fn demo_i32_div_round_integer(gm: GenerationMode, limit: usize) {
    for (i, n, rm) in
        triples_of_signed_nonzero_integer_and_rounding_mode_var_1::<i32>(gm).take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.div_round({}, {}) = {}",
            i,
            n_old,
            rm,
            i.div_round(n, rm)
        );
    }
}

fn demo_i32_div_round_integer_ref(gm: GenerationMode, limit: usize) {
    for (i, n, rm) in
        triples_of_signed_nonzero_integer_and_rounding_mode_var_1::<i32>(gm).take(limit)
    {
        println!("{}.div_round(&{}, {}) = {}", i, n, rm, i.div_round(&n, rm));
    }
}

fn benchmark_integer_div_round_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.div_round_assign(i32, RoundingMode)",
        BenchmarkType::Single,
        triples_of_integer_nonzero_signed_and_rounding_mode_var_1::<i32>(gm),
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

fn benchmark_integer_div_round_i32_down_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(i32, RoundingMode::Down)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_div_round_i32_floor_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(i32, RoundingMode::Floor)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_signed(gm),
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
                &mut (|((x, y), _, _)| no_out!(num_div_round_i32_floor(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_floor(y)))),
        ],
    );
}

fn benchmark_integer_div_round_i32_ceiling_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(i32, RoundingMode::Ceiling)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_nonzero_signed::<i32>(gm),
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

fn benchmark_integer_div_round_i32_ceiling_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(i32, RoundingMode::Ceiling)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_signed::<i32>(gm),
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
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y).0)),
            ),
        ],
    );
}

fn benchmark_integer_div_round_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.div_round(i32, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_nonzero_signed_and_rounding_mode_var_1::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Integer.div_round(i32, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))),
            ),
            (
                "(&Integer).div_round(i32, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).div_round(y, rm))),
            ),
        ],
    );
}

fn benchmark_i32_div_round_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "i32.div_round(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_signed_nonzero_integer_and_rounding_mode_var_1::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "i32.div_round(Integer, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))),
            ),
            (
                "i32.div_round(&Integer, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(&y, rm))),
            ),
        ],
    );
}
