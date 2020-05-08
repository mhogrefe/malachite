use std::cmp::max;

use malachite_base::num::arithmetic::traits::{
    RoundToMultipleOfPowerOfTwo, RoundToMultipleOfPowerOfTwoAssign, ShrRound,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::triples_of_integer_small_unsigned_and_rounding_mode_var_1;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(
        registry,
        demo_integer_round_to_multiple_of_power_of_two_assign
    );
    register_demo!(registry, demo_integer_round_to_multiple_of_power_of_two);
    register_demo!(registry, demo_integer_round_to_multiple_of_power_of_two_ref);

    register_bench!(
        registry,
        Large,
        benchmark_integer_round_to_multiple_of_power_of_two_assign
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_round_to_multiple_of_power_of_two_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_round_to_multiple_of_power_of_two_evaluation_strategy
    );
}

fn demo_integer_round_to_multiple_of_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, pow, rm) in
        triples_of_integer_small_unsigned_and_rounding_mode_var_1(gm).take(limit)
    {
        let n_old = n.clone();
        n.round_to_multiple_of_power_of_two_assign(pow, rm);
        println!(
            "x := {}; x.round_to_multiple_of_power_of_two_assign({}, {}); x = {}",
            n_old, pow, rm, n
        );
    }
}

fn demo_integer_round_to_multiple_of_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, pow, rm) in triples_of_integer_small_unsigned_and_rounding_mode_var_1(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.round_to_multiple_of_power_of_two({}, {}) = {}",
            n_old,
            pow,
            rm,
            n.round_to_multiple_of_power_of_two(pow, rm)
        );
    }
}

fn demo_integer_round_to_multiple_of_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, pow, rm) in triples_of_integer_small_unsigned_and_rounding_mode_var_1(gm).take(limit) {
        println!(
            "(&{}).round_to_multiple_of_power_of_two({}, {}) = {}",
            n,
            pow,
            rm,
            (&n).round_to_multiple_of_power_of_two(pow, rm)
        );
    }
}

fn benchmark_integer_round_to_multiple_of_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.round_to_multiple_of_power_of_two_assign(u64, RoundingMode)",
        BenchmarkType::Single,
        triples_of_integer_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, pow, _)| usize::exact_from(max(n.significant_bits(), pow))),
        "max(self.significant_bits(), pow)",
        &mut [(
            "malachite",
            &mut (|(mut x, y, rm)| x.round_to_multiple_of_power_of_two_assign(y, rm)),
        )],
    );
}

fn benchmark_integer_round_to_multiple_of_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.round_to_multiple_of_power_of_two(u64, RoundingMode)",
        BenchmarkType::Algorithms,
        triples_of_integer_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, pow, _)| usize::exact_from(max(n.significant_bits(), pow))),
        "max(self.significant_bits(), pow)",
        &mut [
            (
                "default",
                &mut (|(x, y, rm)| no_out!(x.round_to_multiple_of_power_of_two(y, rm))),
            ),
            (
                "using shr_round",
                &mut (|(x, y, rm)| no_out!(x.shr_round(y, rm) << y)),
            ),
        ],
    );
}

fn benchmark_integer_round_to_multiple_of_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.round_to_multiple_of_power_of_two(u64, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_small_unsigned_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, pow, _)| usize::exact_from(max(n.significant_bits(), pow))),
        "max(self.significant_bits(), pow)",
        &mut [
            (
                "Integer.round_to_multiple_of_power_of_two(u64, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.round_to_multiple_of_power_of_two(y, rm))),
            ),
            (
                "(&Integer).round_to_multiple_of_power_of_two(u64, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).round_to_multiple_of_power_of_two(y, rm))),
            ),
        ],
    );
}
