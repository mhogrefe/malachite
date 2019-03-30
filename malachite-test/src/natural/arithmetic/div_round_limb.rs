use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::triples_of_unsigned_unsigned_vec_and_rounding_mode_var_1;
#[cfg(feature = "64_bit_limbs")]
use inputs::natural::nm_pairs_of_natural_and_positive_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_unsigned, rm_pairs_of_natural_and_positive_unsigned,
};
use inputs::natural::{
    pairs_of_natural_and_positive_unsigned,
    triples_of_natural_positive_unsigned_and_rounding_mode_var_1,
    triples_of_unsigned_positive_natural_and_rounding_mode_var_1,
};
use malachite_base::num::{CeilingDivNegMod, DivRound, DivRoundAssign, SignificantBits};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::arithmetic::div_round_limb::limbs_limb_div_round_limbs;
use malachite_nz::platform::Limb;
use num::{BigUint, Integer};
#[cfg(feature = "32_bit_limbs")]
use rug::ops::DivRounding;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_limb_div_round_limbs);
    register_demo!(registry, demo_natural_div_round_assign_limb);
    register_demo!(registry, demo_natural_div_round_limb);
    register_demo!(registry, demo_natural_div_round_limb_ref);
    register_demo!(registry, demo_limb_div_round_natural);
    register_demo!(registry, demo_limb_div_round_natural_ref);
    register_demo!(registry, demo_limb_div_round_assign_natural);
    register_demo!(registry, demo_limb_div_round_assign_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_limb_div_round_limbs);
    register_bench!(registry, Large, benchmark_natural_div_round_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_limb_down_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_limb_floor_library_comparison
    );
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_limb_ceiling_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_limb_ceiling_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_round_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_round_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_round_assign_natural_evaluation_strategy
    );
}

pub fn num_div_round_limb_floor(x: BigUint, u: Limb) -> BigUint {
    x.div_floor(&BigUint::from(u))
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

fn demo_natural_div_round_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u, rm) in
        triples_of_natural_positive_unsigned_and_rounding_mode_var_1::<Limb>(gm).take(limit)
    {
        let n_old = n.clone();
        n.div_round_assign(u, rm);
        println!(
            "x := {}; x.div_round_assign({}, {}); x = {}",
            n_old, u, rm, n
        );
    }
}

fn demo_natural_div_round_limb(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in
        triples_of_natural_positive_unsigned_and_rounding_mode_var_1::<Limb>(gm).take(limit)
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

fn demo_natural_div_round_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u, rm) in
        triples_of_natural_positive_unsigned_and_rounding_mode_var_1::<Limb>(gm).take(limit)
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

fn demo_limb_div_round_natural(gm: GenerationMode, limit: usize) {
    for (u, n, rm) in
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1::<Limb>(gm).take(limit)
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

fn demo_limb_div_round_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n, rm) in
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1::<Limb>(gm).take(limit)
    {
        println!("{}.div_round(&{}, {}) = {}", u, n, rm, u.div_round(&n, rm));
    }
}

fn demo_limb_div_round_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n, rm) in
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1::<Limb>(gm).take(limit)
    {
        let u_old = u;
        let n_old = n.clone();
        u.div_round_assign(n, rm);
        println!(
            "x := {}; x.div_round_assign({}, {}); x = {}",
            u_old, n_old, rm, u
        );
    }
}

fn demo_limb_div_round_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n, rm) in
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1::<Limb>(gm).take(limit)
    {
        let u_old = u;
        u.div_round_assign(&n, rm);
        println!(
            "x := {}; x.div_round_assign({}, {}); x = {}",
            u_old, n, rm, u
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

fn benchmark_natural_div_round_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_round_assign(Limb, RoundingMode)",
        BenchmarkType::Single,
        triples_of_natural_positive_unsigned_and_rounding_mode_var_1::<Limb>(gm),
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

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_div_round_limb_down_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Limb, RoundingMode::Down)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_unsigned(gm),
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

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_div_round_limb_floor_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Limb, RoundingMode::Floor)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
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
                &mut (|((x, y), _, _)| no_out!(num_div_round_limb_floor(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.div_floor(y)))),
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
fn benchmark_natural_div_round_limb_floor_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Limb, RoundingMode::Floor)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, (x, y))| no_out!(x.div_round(y, RoundingMode::Floor))),
            ),
            (
                "num",
                &mut (|((x, y), _)| no_out!(num_div_round_limb_floor(x, y))),
            ),
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_div_round_limb_ceiling_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Limb, RoundingMode::Ceiling)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_unsigned(gm),
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

fn benchmark_natural_div_round_limb_ceiling_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Limb, RoundingMode::Ceiling)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
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

fn benchmark_natural_div_round_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_round(Limb, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_natural_positive_unsigned_and_rounding_mode_var_1::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_round(Limb, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))),
            ),
            (
                "(&Natural).div_round(Limb, RoundingMode)",
                &mut (|(x, y, rm)| no_out!((&x).div_round(y, rm))),
            ),
        ],
    );
}

fn benchmark_limb_div_round_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_round(Natural, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_round(Natural, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(y, rm))),
            ),
            (
                "Limb.div_round(&Natural, RoundingMode)",
                &mut (|(x, y, rm)| no_out!(x.div_round(&y, rm))),
            ),
        ],
    );
}

fn benchmark_limb_div_round_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.div_round_assign(Natural, RoundingMode)",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_positive_natural_and_rounding_mode_var_1::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Limb.div_round_assign(Natural, RoundingMode)",
                &mut (|(mut x, y, rm)| x.div_round_assign(y, rm)),
            ),
            (
                "Limb.div_round_assign(&Natural, RoundingMode)",
                &mut (|(mut x, y, rm)| x.div_round_assign(&y, rm)),
            ),
        ],
    );
}
