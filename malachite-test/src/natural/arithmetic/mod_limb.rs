use malachite_base::num::arithmetic::traits::{
    CeilingDivNegMod, DivMod, Mod, ModAssign, NegMod, NegModAssign,
};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::mod_limb::{
    _limbs_mod_limb_alt_1, _limbs_mod_limb_alt_2, _limbs_mod_limb_any_leading_zeros_1,
    _limbs_mod_limb_any_leading_zeros_2, _limbs_mod_limb_at_least_1_leading_zero,
    _limbs_mod_limb_at_least_2_leading_zeros, _limbs_mod_limb_small_normalized,
    _limbs_mod_limb_small_normalized_large, _limbs_mod_limb_small_small,
    _limbs_mod_limb_small_unnormalized, _limbs_mod_limb_small_unnormalized_large, limbs_mod_limb,
};
use malachite_nz::platform::Limb;
use num::{BigUint, ToPrimitive};
use rug::{self, ops::RemRounding};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1,
    pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2,
    pairs_of_nonempty_unsigned_vec_and_unsigned_var_1,
    pairs_of_unsigned_vec_and_positive_unsigned_var_1,
};
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_unsigned, rm_pairs_of_natural_and_positive_unsigned,
};
use inputs::natural::{
    pairs_of_natural_and_positive_unsigned, pairs_of_unsigned_and_positive_natural,
};

// For `Natural`s, `mod` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_mod_limb);
    register_demo!(registry, demo_limbs_mod_limb_small_normalized);
    register_demo!(registry, demo_limbs_mod_limb_small_unnormalized);
    register_demo!(registry, demo_limbs_mod_limb_any_leading_zeros_1);
    register_demo!(registry, demo_limbs_mod_limb_any_leading_zeros_2);
    register_demo!(registry, demo_limbs_mod_limb_at_least_1_leading_zero);
    register_demo!(registry, demo_limbs_mod_limb_at_least_2_leading_zeros);
    register_demo!(registry, demo_natural_rem_assign_limb);
    register_demo!(registry, demo_natural_rem_limb);
    register_demo!(registry, demo_natural_rem_limb_ref);
    register_demo!(registry, demo_natural_mod_assign_limb);
    register_demo!(registry, demo_natural_mod_limb);
    register_demo!(registry, demo_natural_mod_limb_ref);
    register_demo!(registry, demo_natural_neg_mod_assign_limb);
    register_demo!(registry, demo_natural_neg_mod_limb);
    register_demo!(registry, demo_natural_neg_mod_limb_ref);
    register_demo!(registry, demo_limb_rem_natural);
    register_demo!(registry, demo_limb_rem_natural_ref);
    register_demo!(registry, demo_limb_rem_assign_natural);
    register_demo!(registry, demo_limb_rem_assign_natural_ref);
    register_demo!(registry, demo_limb_mod_natural);
    register_demo!(registry, demo_limb_mod_natural_ref);
    register_demo!(registry, demo_limb_mod_assign_natural);
    register_demo!(registry, demo_limb_mod_assign_natural_ref);
    register_demo!(registry, demo_limb_neg_mod_natural);
    register_demo!(registry, demo_limb_neg_mod_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_mod_limb_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_limb_small_normalized_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_limb_small_unnormalized_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_limb_at_least_1_leading_zero_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_mod_limb_at_least_2_leading_zeros
    );
    register_bench!(registry, Large, benchmark_natural_rem_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_rem_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_limb_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_limb_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_neg_mod_assign_limb);
    #[cfg(feature = "32_bit_limbs")]
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_neg_mod_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_rem_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_rem_assign_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_mod_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_mod_assign_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_neg_mod_natural_evaluation_strategy
    );
}

pub fn num_rem_u32(x: BigUint, u: u32) -> u32 {
    (x % u).to_u32().unwrap()
}

pub fn rug_neg_mod_u32(x: rug::Integer, u: u32) -> u32 {
    (-x.rem_ceil(u)).to_u32_wrapping()
}

fn demo_limbs_mod_limb(gm: GenerationMode, limit: usize) {
    for (limbs, divisor) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_mod_limb({:?}, {}) = {}",
            limbs,
            divisor,
            limbs_mod_limb(&limbs, divisor)
        );
    }
}

fn demo_limbs_mod_limb_small_normalized(gm: GenerationMode, limit: usize) {
    for (limbs, divisor) in pairs_of_nonempty_unsigned_vec_and_unsigned_var_1(gm).take(limit) {
        println!(
            "_limbs_mod_limb_small_normalized({:?}, {}) = {}",
            limbs,
            divisor,
            _limbs_mod_limb_small_normalized(&limbs, divisor)
        );
    }
}

fn demo_limbs_mod_limb_small_unnormalized(gm: GenerationMode, limit: usize) {
    for (limbs, divisor) in
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit)
    {
        println!(
            "_limbs_mod_limb_small_unnormalized({:?}, {}) = {}",
            limbs,
            divisor,
            _limbs_mod_limb_small_unnormalized(&limbs, divisor)
        );
    }
}

fn demo_limbs_mod_limb_any_leading_zeros_1(gm: GenerationMode, limit: usize) {
    for (limbs, divisor) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "_limbs_mod_limb_any_leading_zeros_1({:?}, {}) = {}",
            limbs,
            divisor,
            _limbs_mod_limb_any_leading_zeros_1(&limbs, divisor)
        );
    }
}

fn demo_limbs_mod_limb_any_leading_zeros_2(gm: GenerationMode, limit: usize) {
    for (limbs, divisor) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "_limbs_mod_limb_any_leading_zeros_2({:?}, {}) = {}",
            limbs,
            divisor,
            _limbs_mod_limb_any_leading_zeros_2(&limbs, divisor)
        );
    }
}

fn demo_limbs_mod_limb_at_least_1_leading_zero(gm: GenerationMode, limit: usize) {
    for (limbs, divisor) in
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit)
    {
        println!(
            "_limbs_mod_limb_at_least_1_leading_zero({:?}, {}) = {}",
            limbs,
            divisor,
            _limbs_mod_limb_at_least_1_leading_zero(&limbs, divisor)
        );
    }
}

fn demo_limbs_mod_limb_at_least_2_leading_zeros(gm: GenerationMode, limit: usize) {
    for (limbs, divisor) in
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2(gm).take(limit)
    {
        println!(
            "_limbs_mod_limb_at_least_2_leading_zeros({:?}, {}) = {}",
            limbs,
            divisor,
            _limbs_mod_limb_at_least_2_leading_zeros(&limbs, divisor)
        );
    }
}

fn demo_natural_rem_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n %= u;
        println!("x := {}; x %= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_rem_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", n_old, u, n % u);
    }
}

fn demo_natural_rem_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("&{} % {} = {}", n, u, &n % u);
    }
}

fn demo_natural_mod_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n.mod_assign(u);
        println!("x := {}; x.mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {}", n_old, u, n.mod_op(u));
    }
}

fn demo_natural_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).mod({}) = {}", n, u, (&n).mod_op(u));
    }
}

fn demo_natural_neg_mod_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n.neg_mod_assign(u);
        println!("x := {}; x.neg_mod_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_neg_mod_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod({}) = {}", n_old, u, n.neg_mod(u));
    }
}

fn demo_natural_neg_mod_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("(&{}).neg_mod({}) = {}", n, u, (&n).neg_mod(u));
    }
}

fn demo_limb_rem_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % {} = {}", u, n_old, u % n);
    }
}

fn demo_limb_rem_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} % &{} = {}", u, n_old, u % &n);
    }
}

fn demo_limb_rem_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u %= n;
        println!("x := {}; x %= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_limb_rem_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        u %= &n;
        println!("x := {}; x %= &{}; x = {}", u_old, n, u);
    }
}

fn demo_limb_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod({}) = {:?}", u, n_old, u.mod_op(n));
    }
}

fn demo_limb_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.mod(&{}) = {:?}", u, n_old, u.mod_op(&n));
    }
}

fn demo_limb_mod_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u.mod_assign(n);
        println!("x := {}; x.mod_assign({}); x = {}", u_old, n_old, u);
    }
}

fn demo_limb_mod_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        u.mod_assign(&n);
        println!("x := {}; x.mod_assign(&{}); x = {}", u_old, n, u);
    }
}

fn demo_limb_neg_mod_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod({}) = {:?}", u, n_old, u.neg_mod(n));
    }
}

fn demo_limb_neg_mod_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.neg_mod(&{}) = {:?}", u, n_old, u.neg_mod(&n));
    }
}

fn benchmark_limbs_mod_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_mod_limb(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "standard",
                &mut (|(limbs, divisor)| no_out!(limbs_mod_limb(&limbs, divisor))),
            ),
            (
                "alt 1",
                &mut (|(limbs, divisor)| no_out!(_limbs_mod_limb_alt_1(&limbs, divisor))),
            ),
            (
                "alt 2",
                &mut (|(limbs, divisor)| no_out!(_limbs_mod_limb_alt_2(&limbs, divisor))),
            ),
            (
                "_limbs_mod_limb_any_leading_zeros_1",
                &mut (|(limbs, divisor)| {
                    no_out!(_limbs_mod_limb_any_leading_zeros_1(&limbs, divisor))
                }),
            ),
            (
                "_limbs_mod_limb_any_leading_zeros_2",
                &mut (|(limbs, divisor)| {
                    no_out!(_limbs_mod_limb_any_leading_zeros_2(&limbs, divisor))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mod_limb_small_normalized_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mod_limb_small_normalized(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        pairs_of_nonempty_unsigned_vec_and_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len() - 1),
        "limbs.len() - 1",
        &mut [
            (
                "small",
                &mut (|(limbs, divisor)| {
                    let mut len = limbs.len();
                    let mut remainder = limbs[len - 1];
                    if remainder >= divisor {
                        remainder -= divisor;
                    }
                    len -= 1;
                    if len == 0 {
                        return;
                    }
                    let limbs = &limbs[..len];
                    _limbs_mod_limb_small_small(&limbs, divisor, remainder);
                }),
            ),
            (
                "large",
                &mut (|(limbs, divisor)| {
                    let mut len = limbs.len();
                    let mut remainder = limbs[len - 1];
                    if remainder >= divisor {
                        remainder -= divisor;
                    }
                    len -= 1;
                    if len == 0 {
                        return;
                    }
                    let limbs = &limbs[..len];
                    _limbs_mod_limb_small_normalized_large(&limbs, divisor, remainder);
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mod_limb_small_unnormalized_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mod_limb_small_unnormalized(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, divisor)| {
            if *limbs.last().unwrap() < divisor {
                limbs.len() - 1
            } else {
                limbs.len()
            }
        }),
        "adjusted limbs.len()",
        &mut [
            (
                "small",
                &mut (|(limbs, divisor)| {
                    let mut len = limbs.len();
                    let mut remainder = limbs[len - 1];
                    if remainder < divisor {
                        len -= 1;
                        if len == 0 {
                            return;
                        }
                    } else {
                        remainder = 0;
                    }
                    let limbs = &limbs[..len];
                    _limbs_mod_limb_small_small(limbs, divisor, remainder);
                }),
            ),
            (
                "large",
                &mut (|(limbs, divisor)| {
                    let mut len = limbs.len();
                    let mut remainder = limbs[len - 1];
                    if remainder < divisor {
                        len -= 1;
                        if len == 0 {
                            return;
                        }
                    } else {
                        remainder = 0;
                    }
                    let limbs = &limbs[..len];
                    _limbs_mod_limb_small_unnormalized_large(limbs, divisor, remainder);
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mod_limb_at_least_1_leading_zero_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mod_limb_at_least_1_leading_zero(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "_limbs_mod_limb_small_unnormalized",
                &mut (|(limbs, divisor)| {
                    no_out!(_limbs_mod_limb_small_unnormalized(&limbs, divisor))
                }),
            ),
            (
                "_limbs_mod_limb_at_least_1_leading_zero",
                &mut (|(limbs, divisor)| {
                    no_out!(_limbs_mod_limb_at_least_1_leading_zero(&limbs, divisor))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_mod_limb_at_least_2_leading_zeros(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "_limbs_mod_limb_at_least_2_leading_zeros(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_nonempty_unsigned_vec_and_positive_unsigned_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, divisor)| {
                no_out!(_limbs_mod_limb_at_least_2_leading_zeros(&limbs, divisor))
            }),
        )],
    );
}

fn benchmark_natural_rem_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural %= Limb",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x %= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_rem_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural % Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_rem_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_natural_rem_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural % Limb",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x % y))),
            ("naive", &mut (|(x, y)| no_out!(x._mod_limb_naive(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_natural_rem_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural % Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural % Limb", &mut (|(x, y)| no_out!(x % y))),
            ("&Natural % Limb", &mut (|(x, y)| no_out!(&x % y))),
        ],
    );
}

fn benchmark_natural_mod_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_assign(Limb)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.mod_assign(y)))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod(Limb)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(num_rem_u32(x, y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_natural_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural.mod(Limb)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            (
                "(&Natural).mod(Limb)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.neg_mod_assign(Limb)",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x.neg_mod_assign(y)))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_neg_mod_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod(Limb)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.neg_mod(y)))),
            ("rug", &mut (|((x, y), _)| no_out!(rug_neg_mod_u32(x, y)))),
        ],
    );
}

fn benchmark_natural_neg_mod_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.neg_mod(Limb)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.neg_mod(y)))),
            (
                "using ceiling_div_neg_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y).1)),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod(Limb)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.neg_mod(Limb)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "(&Natural).neg_mod(Limb)",
                &mut (|(x, y)| no_out!((&x).neg_mod(y))),
            ),
        ],
    );
}

fn benchmark_limb_rem_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb % Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb % Natural", &mut (|(x, y)| no_out!(x % y))),
            ("Limb % &Natural", &mut (|(x, y)| no_out!(x % &y))),
        ],
    );
}

fn benchmark_limb_rem_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb %= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb %= Natural", &mut (|(mut x, y)| x %= y)),
            ("Limb %= &Natural", &mut (|(mut x, y)| x %= &y)),
        ],
    );
}

fn benchmark_limb_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb.mod(Natural)", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("Limb.mod(&Natural)", &mut (|(x, y)| no_out!(x.mod_op(&y)))),
        ],
    );
}

fn benchmark_limb_mod_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.mod_assign(Natural)",
                &mut (|(mut x, y)| x.mod_assign(y)),
            ),
            (
                "Limb.mod_assign(&Natural)",
                &mut (|(mut x, y)| x.mod_assign(&y)),
            ),
        ],
    );
}

fn benchmark_limb_neg_mod_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb.neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Limb.neg_mod(Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "Limb.neg_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(&y))),
            ),
        ],
    );
}
