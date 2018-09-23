use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    odd_u32s, pairs_of_u32_vec_and_positive_u32_var_2,
    triples_of_u32_vec_u32_vec_and_positive_u32_var_2,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_u32_var_2, pairs_of_natural_and_positive_u32_var_2,
    pairs_of_u32_and_positive_natural_var_2,
};
use malachite_base::num::{DivExact, DivExactAssign, SignificantBits};
use malachite_nz::natural::arithmetic::div_exact_u32::{
    limbs_div_exact_limb, limbs_div_exact_limb_in_place, limbs_div_exact_limb_to_out,
    limbs_invert_limb,
};
use rug;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_invert_limb);
    register_demo!(registry, demo_limbs_div_exact_limb);
    register_demo!(registry, demo_limbs_div_exact_limb_to_out);
    register_demo!(registry, demo_limbs_div_exact_limb_in_place);
    register_demo!(registry, demo_natural_div_exact_assign_u32);
    register_demo!(registry, demo_natural_div_exact_u32);
    register_demo!(registry, demo_natural_div_exact_u32_ref);
    register_demo!(registry, demo_u32_div_exact_natural);
    register_demo!(registry, demo_u32_div_exact_natural_ref);
    register_demo!(registry, demo_u32_div_exact_assign_natural);
    register_demo!(registry, demo_u32_div_exact_assign_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_invert_limb);
    register_bench!(registry, Small, benchmark_limbs_div_exact_limb);
    register_bench!(registry, Small, benchmark_limbs_div_exact_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_div_exact_limb_in_place);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_exact_assign_u32_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_exact_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_div_exact_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_exact_u32_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_u32_div_exact_natural_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_exact_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_exact_assign_natural_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_exact_assign_natural_evaluation_strategy
    );
}

pub fn rug_div_exact_u32(x: rug::Integer, u: u32) -> rug::Integer {
    x.div_exact(&rug::Integer::from(u))
}

fn demo_limbs_invert_limb(gm: GenerationMode, limit: usize) {
    for limb in odd_u32s(gm).take(limit) {
        println!("limbs_invert_limb({}) = {}", limb, limbs_invert_limb(limb));
    }
}

fn demo_limbs_div_exact_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_u32_vec_and_positive_u32_var_2(gm).take(limit) {
        println!(
            "limbs_div_exact_limb({:?}, {}) = {:?}",
            limbs,
            limb,
            limbs_div_exact_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_div_exact_limb_to_out(gm: GenerationMode, limit: usize) {
    for (out_limbs, in_limbs, limb) in
        triples_of_u32_vec_u32_vec_and_positive_u32_var_2(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        limbs_div_exact_limb_to_out(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_exact_div_limb_to_out(&mut out_limbs, {:?}, {}); \
             out_limbs = {:?}",
            out_limbs_old, in_limbs, limb, out_limbs
        );
    }
}

fn demo_limbs_div_exact_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_u32_vec_and_positive_u32_var_2(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_div_exact_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_div_exact_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_natural_div_exact_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_u32_var_2(gm).take(limit) {
        let n_old = n.clone();
        n.div_exact_assign(u);
        println!("x := {}; x.div_exact_assign({}); x = {}", n_old, u, n);
    }
}

fn demo_natural_div_exact_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_u32_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_exact({}) = {}", n_old, u, n.div_exact(u));
    }
}

fn demo_natural_div_exact_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_u32_var_2(gm).take(limit) {
        println!("(&{}).div_exact({}) = {}", n, u, (&n).div_exact(u));
    }
}

fn demo_u32_div_exact_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_u32_and_positive_natural_var_2(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.div_exact({}) = {}", u, n_old, u.div_exact(n));
    }
}

fn demo_u32_div_exact_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_u32_and_positive_natural_var_2(gm).take(limit) {
        println!("{}.div_exact(&{}) = {}", u, n, u.div_exact(&n));
    }
}

fn demo_u32_div_exact_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_u32_and_positive_natural_var_2(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u.div_exact_assign(n);
        println!("x := {}; x.div_exact_assign({}); x = {}", u_old, n_old, u);
    }
}

fn demo_u32_div_exact_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_u32_and_positive_natural_var_2(gm).take(limit) {
        let u_old = u;
        u.div_exact_assign(&n);
        println!("x := {}; x.div_exact_assign(&{}); x = {}", u_old, n, u);
    }
}

fn benchmark_limbs_invert_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_invert_limb(u32)",
        BenchmarkType::Single,
        odd_u32s(gm),
        gm.name(),
        limit,
        file_name,
        &(|limb| limb.significant_bits() as usize),
        "limb.significant_bits()",
        &mut [("malachite", &mut (|limb| no_out!(limbs_invert_limb(limb))))],
    );
}

fn benchmark_limbs_div_exact_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_exact_limb(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_positive_u32_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(limbs, limb)| no_out!(limbs_div_exact_limb(&limbs, limb))),
        )],
    );
}

fn benchmark_limbs_div_exact_limb_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_exact_limb_to_out(&mut [u32], &[u32], u32)",
        BenchmarkType::Single,
        triples_of_u32_vec_u32_vec_and_positive_u32_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out_limbs, in_limbs, limb)| {
                limbs_div_exact_limb_to_out(&mut out_limbs, &in_limbs, limb)
            }),
        )],
    );
}

fn benchmark_limbs_div_exact_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_exact_limb_in_place(&mut [u32], u32)",
        BenchmarkType::Single,
        pairs_of_u32_vec_and_positive_u32_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, limb)| limbs_div_exact_limb_in_place(&mut limbs, limb)),
        )],
    );
}

fn benchmark_natural_div_exact_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_exact_assign(u32)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_u32_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(mut x, y)| x /= y)),
            ("exact division", &mut (|(mut x, y)| x.div_exact_assign(y))),
        ],
    );
}

fn benchmark_natural_div_exact_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_exact(u32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_u32_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("num", &mut (|((x, y), _, _)| no_out!(x / y))),
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.div_exact(y)))),
            (
                "rug",
                &mut (|(_, (x, y), _)| no_out!(rug_div_exact_u32(x, y))),
            ),
        ],
    );
}

fn benchmark_natural_div_exact_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.div_exact(u32)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_u32_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(x, y)| no_out!(x / y))),
            ("exact division", &mut (|(x, y)| no_out!(x.div_exact(y)))),
        ],
    );
}

fn benchmark_natural_div_exact_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_exact(u32)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_u32_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.div_exact(u32)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "(&Natural).div_exact(u32)",
                &mut (|(x, y)| no_out!((&x).div_exact(y))),
            ),
        ],
    );
}

fn benchmark_u32_div_exact_natural_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "u32.div_exact(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_u32_and_positive_natural_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(x, y)| no_out!(x / y))),
            ("exact division", &mut (|(x, y)| no_out!(x.div_exact(y)))),
        ],
    );
}

fn benchmark_u32_div_exact_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_exact(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_u32_and_positive_natural_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_exact(Natural)",
                &mut (|(x, y)| no_out!(x.div_exact(y))),
            ),
            (
                "u32.div_exact(&Natural)",
                &mut (|(x, y)| no_out!(x.div_exact(&y))),
            ),
        ],
    );
}

fn benchmark_u32_div_exact_assign_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_exact_assign(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_u32_and_positive_natural_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("ordinary division", &mut (|(mut x, y)| x /= y)),
            ("exact division", &mut (|(mut x, y)| x.div_exact_assign(y))),
        ],
    );
}

fn benchmark_u32_div_exact_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32.div_exact_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_u32_and_positive_natural_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "u32.div_exact_assign(Natural)",
                &mut (|(mut x, y)| x.div_exact_assign(y)),
            ),
            (
                "u32.div_exact_assign(&Natural)",
                &mut (|(mut x, y)| x.div_exact_assign(&y)),
            ),
        ],
    );
}
