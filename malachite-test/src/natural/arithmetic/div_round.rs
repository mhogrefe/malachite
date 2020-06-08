use malachite_base::num::arithmetic::traits::{CeilingDivNegMod, DivRound, DivRoundAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_mode::RoundingMode;
use malachite_nz::natural::arithmetic::div_round::limbs_limb_div_round_limbs;
use num::Integer;
use rug::ops::DivRounding;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1;
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural,
    rm_pairs_of_natural_and_positive_natural,
    triples_of_natural_positive_natural_and_rounding_mode_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_limb_div_round_limbs);
    register_demo!(registry, demo_natural_div_round_assign);
    register_demo!(registry, demo_natural_div_round_assign_ref);
    register_demo!(registry, demo_natural_div_round);
    register_demo!(registry, demo_natural_div_round_val_ref);
    register_demo!(registry, demo_natural_div_round_ref_val);
    register_demo!(registry, demo_natural_div_round_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_limb_div_round_limbs);
    register_bench!(registry, Large, benchmark_natural_div_round_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_down_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_floor_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_ceiling_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_ceiling_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_assign_evaluation_strategy
    );
}

fn demo_limbs_limb_div_round_limbs(gm: GenerationMode, limit: usize) {
    for (limb, limbs, rm) in
        triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1(gm).take(limit)
    {
        println!(
            "limbs_limb_div_round_limbs({}, {:?}, {}) = {:?}",
            limb,
            limbs,
            rm,
            limbs_limb_div_round_limbs(limb, &limbs, rm)
        );
    }
}

fn demo_natural_div_round_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y, rm) in
        triples_of_natural_positive_natural_and_rounding_mode_var_1(gm).take(limit)
    {
        let x_old = x.clone();
        let y_old = y.clone();
        x.div_round_assign(y, rm);
        println!(
            "x := {}; x.div_round_assign({}, {}); x = {}",
            x_old, y_old, rm, x
        );
    }
}

fn demo_natural_div_round_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y, rm) in
        triples_of_natural_positive_natural_and_rounding_mode_var_1(gm).take(limit)
    {
        let x_old = x.clone();
        x.div_round_assign(&y, rm);
        println!(
            "x := {}; x.div_round_assign(&{}, {}); x = {}",
            x_old, y, rm, x
        );
    }
}

fn demo_natural_div_round(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_natural_positive_natural_and_rounding_mode_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!(
            "{}.div_round({}, {}) = {}",
            x_old,
            y_old,
            rm,
            x.div_round(y, rm)
        );
    }
}

fn demo_natural_div_round_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_natural_positive_natural_and_rounding_mode_var_1(gm).take(limit) {
        let x_old = x.clone();
        println!(
            "{}.div_round(&{}, {}) = {}",
            x_old,
            y,
            rm,
            x.div_round(&y, rm)
        );
    }
}

fn demo_natural_div_round_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_natural_positive_natural_and_rounding_mode_var_1(gm).take(limit) {
        let y_old = y.clone();
        println!(
            "(&{}).div_round({}, {}) = {}",
            x,
            y_old,
            rm,
            (&x).div_round(y, rm)
        );
    }
}

fn demo_natural_div_round_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, rm) in triples_of_natural_positive_natural_and_rounding_mode_var_1(gm).take(limit) {
        println!(
            "(&{}).div_round(&{}, {}) = {}",
            x,
            y,
            rm,
            (&x).div_round(&y, rm)
        );
    }
}

fn benchmark_limbs_limb_div_round_limbs(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_limb_div_round_limbs(Limb, &[Limb], RoundingMode)",
        BenchmarkType::Single,
        triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limb, limbs, rm)| no_out!(limbs_limb_div_round_limbs(limb, &limbs, rm))),
        )],
    );
}

fn benchmark_natural_div_round_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_round_assign(Natural, RoundingMode)",
        BenchmarkType::Single,
        triples_of_natural_positive_natural_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [(
            "malachite",
            &mut (|(mut x, y, rm)| x.div_round_assign(y, rm)),
        )],
    );
}

fn benchmark_natural_div_round_down_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Natural, RoundingMode::Down)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::exact_from(n.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.div_round(y, RoundingMode::Down))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x.div_trunc(y)))),
        ],
    );
}

fn benchmark_natural_div_round_floor_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Natural, RoundingMode::Floor)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, _))| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.div_round(y, RoundingMode::Floor))),
            ),
            ("num", &mut (|((x, y), _, _)| no_out!(x.div_floor(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_floor(y)))),
        ],
    );
}

fn benchmark_natural_div_round_ceiling_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Natural, RoundingMode::Ceiling)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, _))| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.div_round(y, RoundingMode::Ceiling))),
            ),
            ("rug", &mut (|((x, y), _)| no_out!(x.div_ceil(y)))),
        ],
    );
}

fn benchmark_natural_div_round_ceiling_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Natural, RoundingMode::Ceiling)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
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

fn benchmark_natural_div_round_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Natural, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_positive_natural_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "Natural.div_round(Natural, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))),
            ),
            (
                "Natural.div_round(&Natural, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(&y, rm))),
            ),
            (
                "(&Natural).div_round(Natural, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).div_round(y, rm))),
            ),
            (
                "(&Natural).div_round(&Natural, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).div_round(&y, rm))),
            ),
        ],
    );
}

fn benchmark_natural_div_round_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round_assign(Natural, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_positive_natural_and_rounding_mode_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "Natural.div_round_assign(Natural, RoundingMode)",
                &mut (|(mut x, y, rm)| x.div_round_assign(y, rm)),
            ),
            (
                "Natural.div_round_assign(&Natural, RoundingMode)",
                &mut (|(mut x, y, rm)| x.div_round_assign(&y, rm)),
            ),
        ],
    );
}
