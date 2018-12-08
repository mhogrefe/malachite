use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::pairs_of_integer_and_small_unsigned;
use malachite_base::num::{
    CeilingModPowerOfTwo, CeilingModPowerOfTwoAssign, ModPowerOfTwo, ModPowerOfTwoAssign,
    RemPowerOfTwo, RemPowerOfTwoAssign,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_mod_power_of_two_assign);
    register_demo!(registry, demo_integer_mod_power_of_two);
    register_demo!(registry, demo_integer_mod_power_of_two_ref);
    register_demo!(registry, demo_integer_rem_power_of_two_assign);
    register_demo!(registry, demo_integer_rem_power_of_two);
    register_demo!(registry, demo_integer_rem_power_of_two_ref);
    register_demo!(registry, demo_integer_ceiling_mod_power_of_two_assign);
    register_demo!(registry, demo_integer_ceiling_mod_power_of_two);
    register_demo!(registry, demo_integer_ceiling_mod_power_of_two_ref);
    register_bench!(registry, Large, benchmark_integer_mod_power_of_two_assign);
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_power_of_two_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_rem_power_of_two_assign);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_power_of_two_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_power_of_two_assign
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_power_of_two_evaluation_strategy
    );
}

fn demo_integer_mod_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_two_assign(u);
        println!(
            "x := {}; x.mod_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

fn demo_integer_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.mod_power_of_two({}) = {}",
            n_old,
            u,
            n.mod_power_of_two(u)
        );
    }
}

fn demo_integer_mod_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).mod_power_of_two({}) = {}",
            n,
            u,
            (&n).mod_power_of_two(u)
        );
    }
}

fn demo_integer_rem_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.rem_power_of_two_assign(u);
        println!(
            "x := {}; x.rem_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

fn demo_integer_rem_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.rem_power_of_two({}) = {}",
            n_old,
            u,
            n.rem_power_of_two(u)
        );
    }
}

fn demo_integer_rem_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).rem_power_of_two({}) = {}",
            n,
            u,
            (&n).rem_power_of_two(u)
        );
    }
}

fn demo_integer_ceiling_mod_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.ceiling_mod_power_of_two_assign(u);
        println!(
            "x := {}; x.ceiling_mod_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

fn demo_integer_ceiling_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.ceiling_mod_power_of_two({}) = {}",
            n_old,
            u,
            n.ceiling_mod_power_of_two(u)
        );
    }
}

fn demo_integer_ceiling_mod_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        println!(
            "(&{}).ceiling_mod_power_of_two({}) = {}",
            n,
            u,
            (&n).ceiling_mod_power_of_two(u)
        );
    }
}

fn benchmark_integer_mod_power_of_two_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.mod_power_of_two_assign(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [(
            "malachite",
            &mut (|(mut n, u)| n.mod_power_of_two_assign(u)),
        )],
    );
}

fn benchmark_integer_mod_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.mod_power_of_two(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [
            (
                "Integer.mod_power_of_two(u64)",
                &mut (|(n, u)| no_out!(n.mod_power_of_two(u))),
            ),
            (
                "(&Integer).mod_power_of_two(u64)",
                &mut (|(n, u)| no_out!((&n).mod_power_of_two(u))),
            ),
        ],
    );
}

fn benchmark_integer_rem_power_of_two_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.rem_power_of_two_assign(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [(
            "malachite",
            &mut (|(mut n, u)| n.rem_power_of_two_assign(u)),
        )],
    );
}

fn benchmark_integer_rem_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.rem_power_of_two(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [
            (
                "Integer.rem_power_of_two(u64)",
                &mut (|(n, u)| no_out!(n.rem_power_of_two(u))),
            ),
            (
                "(&Integer).rem_power_of_two(u64)",
                &mut (|(n, u)| no_out!((&n).rem_power_of_two(u))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod_power_of_two_assign(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [(
            "malachite",
            &mut (|(mut n, u)| n.ceiling_mod_power_of_two_assign(u)),
        )],
    );
}

fn benchmark_integer_ceiling_mod_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.ceiling_mod_power_of_two(u64)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [
            (
                "Integer.ceiling_mod_power_of_two(u64)",
                &mut (|(n, u)| no_out!(n.ceiling_mod_power_of_two(u))),
            ),
            (
                "(&Integer).ceiling_mod_power_of_two(u64)",
                &mut (|(n, u)| no_out!((&n).ceiling_mod_power_of_two(u))),
            ),
        ],
    );
}
