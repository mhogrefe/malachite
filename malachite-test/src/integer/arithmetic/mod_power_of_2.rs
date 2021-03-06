use malachite_base::num::arithmetic::traits::{
    CeilingModPowerOf2, CeilingModPowerOf2Assign, ModPowerOf2, ModPowerOf2Assign, RemPowerOf2,
    RemPowerOf2Assign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::pairs_of_integer_and_small_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_mod_power_of_2_assign);
    register_demo!(registry, demo_integer_mod_power_of_2);
    register_demo!(registry, demo_integer_mod_power_of_2_ref);
    register_demo!(registry, demo_integer_rem_power_of_2_assign);
    register_demo!(registry, demo_integer_rem_power_of_2);
    register_demo!(registry, demo_integer_rem_power_of_2_ref);
    register_demo!(registry, demo_integer_ceiling_mod_power_of_2_assign);
    register_demo!(registry, demo_integer_ceiling_mod_power_of_2);
    register_demo!(registry, demo_integer_ceiling_mod_power_of_2_ref);
    register_bench!(registry, Large, benchmark_integer_mod_power_of_2_assign);
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_power_of_2_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_rem_power_of_2_assign);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_power_of_2_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_power_of_2_assign
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_power_of_2_evaluation_strategy
    );
}

fn demo_integer_mod_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_2_assign(u);
        println!("x := {}; x.mod_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_integer_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod_power_of_2({}) = {}", n_old, u, n.mod_power_of_2(u));
    }
}

fn demo_integer_mod_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).mod_power_of_2({}) = {}",
            n,
            u,
            (&n).mod_power_of_2(u)
        );
    }
}

fn demo_integer_rem_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.rem_power_of_2_assign(u);
        println!("x := {}; x.rem_power_of_2_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_integer_rem_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.rem_power_of_2({}) = {}", n_old, u, n.rem_power_of_2(u));
    }
}

fn demo_integer_rem_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).rem_power_of_2({}) = {}",
            n,
            u,
            (&n).rem_power_of_2(u)
        );
    }
}

fn demo_integer_ceiling_mod_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_power_of_2_assign(u);
        println!(
            "x := {}; x.ceiling_mod_power_of_2_assign({}); x = {}",
            n_old, u, n
        );
    }
}

fn demo_integer_ceiling_mod_power_of_2(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_mod_power_of_2({}) = {}",
            n_old,
            u,
            n.ceiling_mod_power_of_2(u)
        );
    }
}

fn demo_integer_ceiling_mod_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).ceiling_mod_power_of_2({}) = {}",
            n,
            u,
            (&n).ceiling_mod_power_of_2(u)
        );
    }
}

fn benchmark_integer_mod_power_of_2_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.mod_power_of_2_assign(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, u)| n.mod_power_of_2_assign(u)))],
    );
}

fn benchmark_integer_mod_power_of_2_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [
            (
                "Integer.mod_power_of_2(u64)",
                &mut (|(n, u)| no_out!(n.mod_power_of_2(u))),
            ),
            (
                "(&Integer).mod_power_of_2(u64)",
                &mut (|(n, u)| no_out!((&n).mod_power_of_2(u))),
            ),
        ],
    );
}

fn benchmark_integer_rem_power_of_2_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.rem_power_of_2_assign(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, u)| n.rem_power_of_2_assign(u)))],
    );
}

fn benchmark_integer_rem_power_of_2_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.rem_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [
            (
                "Integer.rem_power_of_2(u64)",
                &mut (|(n, u)| no_out!(n.rem_power_of_2(u))),
            ),
            (
                "(&Integer).rem_power_of_2(u64)",
                &mut (|(n, u)| no_out!((&n).rem_power_of_2(u))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_power_of_2_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.ceiling_mod_power_of_2_assign(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "Malachite",
            &mut (|(mut n, u)| n.ceiling_mod_power_of_2_assign(u)),
        )],
    );
}

fn benchmark_integer_ceiling_mod_power_of_2_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.ceiling_mod_power_of_2(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [
            (
                "Integer.ceiling_mod_power_of_2(u64)",
                &mut (|(n, u)| no_out!(n.ceiling_mod_power_of_2(u))),
            ),
            (
                "(&Integer).ceiling_mod_power_of_2(u64)",
                &mut (|(n, u)| no_out!((&n).ceiling_mod_power_of_2(u))),
            ),
        ],
    );
}
