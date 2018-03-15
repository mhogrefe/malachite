use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::natural::pairs_of_natural_and_small_u32;

pub fn demo_natural_mod_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n.mod_power_of_two_assign(u);
        println!(
            "x := {}; x.mod_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

pub fn demo_natural_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.mod_power_of_two({}) = {}",
            n_old,
            u,
            n.mod_power_of_two(u)
        );
    }
}

pub fn demo_natural_mod_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        println!(
            "{}.mod_power_of_two_ref({}) = {}",
            n,
            u,
            n.mod_power_of_two_ref(u)
        );
    }
}

pub fn demo_natural_neg_mod_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        n.neg_mod_power_of_two_assign(u);
        println!(
            "x := {}; x.neg_mod_power_of_two_assign({}); x = {}",
            n_old, u, n
        );
    }
}

pub fn demo_natural_neg_mod_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        let n_old = n.clone();
        println!(
            "{}.neg_mod_power_of_two({}) = {}",
            n_old,
            u,
            n.neg_mod_power_of_two(u)
        );
    }
}

pub fn demo_natural_neg_mod_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_small_u32(gm).take(limit) {
        println!(
            "{}.neg_mod_power_of_two_ref({}) = {}",
            n,
            u,
            n.neg_mod_power_of_two_ref(u)
        );
    }
}

pub fn benchmark_natural_mod_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_power_of_two_assign(u32)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "other",
        &[
            (
                "malachite",
                &mut (|(mut n, u)| n.mod_power_of_two_assign(u)),
            ),
        ],
    );
}

pub fn benchmark_natural_mod_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_power_of_two(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "other",
        &[
            (
                "Natural.mod_power_of_two(u32)",
                &mut (|(n, u)| no_out!(n.mod_power_of_two(u))),
            ),
            (
                "Natural.mod_power_of_two_ref(u32)",
                &mut (|(n, u)| no_out!(n.mod_power_of_two_ref(u))),
            ),
        ],
    );
}

pub fn benchmark_natural_neg_mod_power_of_two_assign(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod_power_of_two_assign(u32)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "other",
        &[
            (
                "malachite",
                &mut (|(mut n, u)| n.neg_mod_power_of_two_assign(u)),
            ),
        ],
    );
}

pub fn benchmark_natural_neg_mod_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod_power_of_two(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_small_u32(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "other",
        &[
            (
                "Natural.mod_power_of_two(u32)",
                &mut (|(n, u)| no_out!(n.neg_mod_power_of_two(u))),
            ),
            (
                "Natural.mod_power_of_two_ref(u32)",
                &mut (|(n, u)| no_out!(n.neg_mod_power_of_two_ref(u))),
            ),
        ],
    );
}
