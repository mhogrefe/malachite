use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::triples_of_integer_integer_and_rounding_mode_var_2;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_round_to_multiple_assign);
    register_demo!(registry, demo_integer_round_to_multiple_assign_ref);
    register_demo!(registry, demo_integer_round_to_multiple);
    register_demo!(registry, demo_integer_round_to_multiple_val_ref);
    register_demo!(registry, demo_integer_round_to_multiple_ref_val);
    register_demo!(registry, demo_integer_round_to_multiple_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_round_to_multiple_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_round_to_multiple_assign_evaluation_strategy
    );
}

fn demo_integer_round_to_multiple_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y, rm) in triples_of_integer_integer_and_rounding_mode_var_2(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.round_to_multiple_assign(y, rm);
        println!(
            "x := {}; x.round_to_multiple_assign({}, {}); x = {}",
            x_old, y_old, rm, x
        );
    }
}

fn demo_integer_round_to_multiple_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, rm) in triples_of_integer_integer_and_rounding_mode_var_2(gm).take(limit) {
        let x_old = x.clone();
        x.round_to_multiple_assign(&y, rm);
        println!(
            "x := {}; x.round_to_multiple_assign(&{}, {}); x = {}",
            x_old, y, rm, x
        );
    }
}

fn demo_integer_round_to_multiple(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_integer_integer_and_rounding_mode_var_2(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "({}).round_to_multiple({}, {}) = {}",
            x_old,
            y_old,
            rm,
            x.round_to_multiple(y, rm)
        );
    }
}

fn demo_integer_round_to_multiple_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_integer_integer_and_rounding_mode_var_2(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "({}).round_to_multiple(&{}, {}) = {}",
            x_old,
            y,
            rm,
            x.round_to_multiple(&y, rm)
        );
    }
}

fn demo_integer_round_to_multiple_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_integer_integer_and_rounding_mode_var_2(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).round_to_multiple({}, {}) = {}",
            x,
            y_old,
            rm,
            (&x).round_to_multiple(y, rm)
        );
    }
}

fn demo_integer_round_to_multiple_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_integer_integer_and_rounding_mode_var_2(gm).take(limit) {
        println!(
            "(&{}).round_to_multiple(&{}, {}) = {}",
            x,
            y,
            rm,
            (&x).round_to_multiple(&y, rm)
        );
    }
}

fn benchmark_integer_round_to_multiple_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.round_to_multiple(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_rounding_mode_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "Integer.round_to_multiple(Integer, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.round_to_multiple(y, rm))),
            ),
            (
                "Integer.round_to_multiple(&Integer, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.round_to_multiple(&y, rm))),
            ),
            (
                "(&Integer).round_to_multiple(Integer, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).round_to_multiple(y, rm))),
            ),
            (
                "(&Integer).round_to_multiple(&Integer, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).round_to_multiple(&y, rm))),
            ),
        ],
    );
}

fn benchmark_integer_round_to_multiple_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.round_to_multiple_assign(Integer, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_integer_integer_and_rounding_mode_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "Integer.round_to_multiple_assign(Integer, RoundingMode)",
                &mut (|(mut x, y, rm)| x.round_to_multiple_assign(y, rm)),
            ),
            (
                "Integer.round_to_multiple_assign(&Integer, RoundingMode)",
                &mut (|(mut x, y, rm)| x.round_to_multiple_assign(&y, rm)),
            ),
        ],
    );
}
