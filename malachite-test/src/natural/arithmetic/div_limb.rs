use malachite_base::num::arithmetic::traits::DivRem;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::div_limb::{
    limbs_div_divisor_of_limb_max_with_carry_in_place,
    limbs_div_divisor_of_limb_max_with_carry_to_out, limbs_div_limb, limbs_div_limb_in_place,
    limbs_div_limb_to_out,
};
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3, triples_of_limb_vec_limb_and_limb_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
};
#[cfg(not(feature = "32_bit_limbs"))]
use inputs::natural::nm_pairs_of_natural_and_positive_unsigned;
#[cfg(feature = "32_bit_limbs")]
use inputs::natural::nrm_pairs_of_natural_and_positive_unsigned;
use inputs::natural::{
    pairs_of_natural_and_positive_unsigned, pairs_of_unsigned_and_positive_natural,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_div_limb);
    register_demo!(registry, demo_limbs_div_limb_to_out);
    register_demo!(registry, demo_limbs_div_limb_in_place);
    register_demo!(
        registry,
        demo_limbs_div_divisor_of_limb_max_with_carry_to_out
    );
    register_demo!(
        registry,
        demo_limbs_div_divisor_of_limb_max_with_carry_in_place
    );
    register_demo!(registry, demo_natural_div_assign_limb);
    register_demo!(registry, demo_natural_div_limb);
    register_demo!(registry, demo_natural_div_limb_ref);
    register_demo!(registry, demo_limb_div_natural);
    register_demo!(registry, demo_limb_div_natural_ref);
    register_demo!(registry, demo_limb_div_assign_natural);
    register_demo!(registry, demo_limb_div_assign_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_div_limb);
    register_bench!(registry, Small, benchmark_limbs_div_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_div_limb_in_place);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_divisor_of_limb_max_with_carry_to_out
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_div_divisor_of_limb_max_with_carry_in_place
    );
    register_bench!(registry, Large, benchmark_natural_div_assign_limb);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_limb_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_div_limb_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_limb_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_limb_div_assign_natural_evaluation_strategy
    );
}

fn demo_limbs_div_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_div_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_div_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_div_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_div_limb_to_out(&mut out, &in_limbs, limb);
        println!(
            "out := {:?}; limbs_div_limb_to_out(&mut out, {:?}, {}); out = {:?}",
            out_old, in_limbs, limb, out
        );
    }
}

fn demo_limbs_div_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_div_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_div_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_limbs_div_divisor_of_limb_max_with_carry_to_out(gm: GenerationMode, limit: usize) {
    for (out, xs, divisor, carry) in
        quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry_out =
            limbs_div_divisor_of_limb_max_with_carry_to_out(&mut out, &xs, divisor, carry);
        println!(
            "out := {:?}; limbs_div_divisor_of_limb_max_with_carry_to_out(&mut out, {:?}, {}, {}) \
             = {}; out = {:?}",
            out_old, xs, divisor, carry, carry_out, out
        );
    }
}

fn demo_limbs_div_divisor_of_limb_max_with_carry_in_place(gm: GenerationMode, limit: usize) {
    for (xs, divisor, carry) in triples_of_limb_vec_limb_and_limb_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let carry_out = limbs_div_divisor_of_limb_max_with_carry_in_place(&mut xs, divisor, carry);
        println!(
            "xs := {:?}; limbs_div_divisor_of_limb_max_with_carry_in_place(&mut xs, {}, {}) = {}; \
             xs = {:?}",
            xs_old, divisor, carry, carry_out, xs
        );
    }
}

fn demo_natural_div_assign_limb(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        n /= u;
        println!("x := {}; x /= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_div_limb(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} / {} = {}", n_old, u, n / u);
    }
}

fn demo_natural_div_limb_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<Limb>(gm).take(limit) {
        println!("&{} / {} = {}", n, u, &n / u);
    }
}

fn demo_limb_div_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} / {} = {}", u, n_old, u / n);
    }
}

fn demo_limb_div_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        println!("{} / &{} = {}", u, n, u / &n);
    }
}

fn demo_limb_div_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u /= n;
        println!("x := {}; x /= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_limb_div_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<Limb>(gm).take(limit) {
        let u_old = u;
        u /= &n;
        println!("x := {}; x /= &{}; x = {}", u_old, n, u);
    }
}

fn benchmark_limbs_div_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_div_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_div_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, in_limbs, limb)| limbs_div_limb_to_out(&mut out, &in_limbs, limb)),
        )],
    );
}

fn benchmark_limbs_div_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| limbs_div_limb_in_place(&mut limbs, limb)),
        )],
    );
}

fn benchmark_limbs_div_divisor_of_limb_max_with_carry_to_out(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_div_divisor_of_limb_max_with_carry_to_out(&mut [Limb], &[Limb], Limb, Limb)",
        BenchmarkType::Single,
        quadruples_of_limb_vec_limb_vec_limb_and_limb_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, xs, divisor, carry)| {
                no_out!(limbs_div_divisor_of_limb_max_with_carry_to_out(
                    &mut out, &xs, divisor, carry
                ))
            }),
        )],
    );
}

fn benchmark_limbs_div_divisor_of_limb_max_with_carry_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_div_divisor_of_limb_max_with_carry_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        triples_of_limb_vec_limb_and_limb_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, divisor, carry)| {
                no_out!(limbs_div_divisor_of_limb_max_with_carry_in_place(
                    &mut xs, divisor, carry
                ))
            }),
        )],
    );
}

fn benchmark_natural_div_assign_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural /= Limb",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x /= y))],
    );
}

#[cfg(feature = "32_bit_limbs")]
fn benchmark_natural_div_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural / Limb",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x / y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x / y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x / y))),
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
fn benchmark_natural_div_limb_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural / Limb",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x / y))),
            ("num", &mut (|((x, y), _)| no_out!(x / y))),
        ],
    );
}

fn benchmark_natural_div_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural / Limb",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x / y))),
            ("naive", &mut (|(x, y)| no_out!(x._div_limb_naive(y)))),
            ("using div_rem", &mut (|(x, y)| no_out!(x.div_rem(y).0))),
        ],
    );
}

fn benchmark_natural_div_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural / Limb",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural / Limb", &mut (|(x, y)| no_out!(x / y))),
            ("&Natural / Limb", &mut (|(x, y)| no_out!(&x / y))),
        ],
    );
}

fn benchmark_limb_div_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb / Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb / Natural", &mut (|(x, y)| no_out!(x / y))),
            ("Limb / &Natural", &mut (|(x, y)| no_out!(x / &y))),
        ],
    );
}

fn benchmark_limb_div_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Limb /= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<Limb>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Limb /= Natural", &mut (|(mut x, y)| x /= y)),
            ("Limb /= &Natural", &mut (|(mut x, y)| x /= &y)),
        ],
    );
}
