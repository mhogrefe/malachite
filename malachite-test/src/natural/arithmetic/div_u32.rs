use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
};
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_unsigned, pairs_of_natural_and_positive_unsigned,
    pairs_of_unsigned_and_positive_natural,
};
use malachite_base::num::{DivMod, SignificantBits};
use malachite_nz::natural::arithmetic::div_u32::{
    limbs_div_limb, limbs_div_limb_in_place, limbs_div_limb_to_out,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_div_limb);
    register_demo!(registry, demo_limbs_div_limb_to_out);
    register_demo!(registry, demo_limbs_div_limb_in_place);
    register_demo!(registry, demo_natural_div_assign_u32);
    register_demo!(registry, demo_natural_div_u32);
    register_demo!(registry, demo_natural_div_u32_ref);
    register_demo!(registry, demo_u32_div_natural);
    register_demo!(registry, demo_u32_div_natural_ref);
    register_demo!(registry, demo_u32_div_assign_natural);
    register_demo!(registry, demo_u32_div_assign_natural_ref);
    register_bench!(registry, Small, benchmark_limbs_div_limb);
    register_bench!(registry, Small, benchmark_limbs_div_limb_to_out);
    register_bench!(registry, Small, benchmark_limbs_div_limb_in_place);
    register_bench!(registry, Large, benchmark_natural_div_assign_u32);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_u32_library_comparison
    );
    register_bench!(registry, Large, benchmark_natural_div_u32_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_div_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_natural_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_u32_div_assign_natural_evaluation_strategy
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
    for (out_limbs, in_limbs, limb) in
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit)
    {
        let mut out_limbs = out_limbs.to_vec();
        let mut out_limbs_old = out_limbs.clone();
        limbs_div_limb_to_out(&mut out_limbs, &in_limbs, limb);
        println!(
            "out_limbs := {:?}; limbs_div_limb_to_out(&mut out_limbs, {:?}, {}); out_limbs = {:?}",
            out_limbs_old, in_limbs, limb, out_limbs
        );
    }
}

fn demo_limbs_div_limb_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_div_limb_in_place(&mut limbs, limb);
        println!(
            "limbs := {:?}; limbs_div_limb_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, limb, limbs
        );
    }
}

fn demo_natural_div_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n /= u;
        println!("x := {}; x /= {}; x = {}", n_old, u, n);
    }
}

fn demo_natural_div_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} / {} = {}", n_old, u, n / u);
    }
}

fn demo_natural_div_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_positive_unsigned::<u32>(gm).take(limit) {
        println!("&{} / {} = {}", n, u, &n / u);
    }
}

fn demo_u32_div_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} / {} = {}", u, n_old, u / n);
    }
}

fn demo_u32_div_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        println!("{} / &{} = {}", u, n, u / &n);
    }
}

fn demo_u32_div_assign_natural(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        let n_old = n.clone();
        u /= n;
        println!("x := {}; x /= {}; x = {}", u_old, n_old, u);
    }
}

fn demo_u32_div_assign_natural_ref(gm: GenerationMode, limit: usize) {
    for (mut u, n) in pairs_of_unsigned_and_positive_natural::<u32>(gm).take(limit) {
        let u_old = u;
        u /= &n;
        println!("x := {}; x /= &{}; x = {}", u_old, n, u);
    }
}

fn benchmark_limbs_div_limb(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb(&[u32], u32)",
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
        "limbs_div_limb_to_out(&mut [u32], &[u32], u32)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out_limbs, in_limbs, limb)| {
                limbs_div_limb_to_out(&mut out_limbs, &in_limbs, limb)
            }),
        )],
    );
}

fn benchmark_limbs_div_limb_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_div_limb_in_place(&mut [u32], u32)",
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

fn benchmark_natural_div_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural /= u32",
        BenchmarkType::Single,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|(mut x, y)| x /= y))],
    );
}

fn benchmark_natural_div_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural / u32",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x / y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x / y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x / y))),
        ],
    );
}

fn benchmark_natural_div_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural / u32",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x / y))),
            ("naive", &mut (|(x, y)| no_out!(x._div_u32_naive(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).0))),
        ],
    );
}

fn benchmark_natural_div_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural / u32",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("Natural / u32", &mut (|(x, y)| no_out!(x / y))),
            ("&Natural / u32", &mut (|(x, y)| no_out!(&x / y))),
        ],
    );
}

fn benchmark_u32_div_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 / Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 / Natural", &mut (|(x, y)| no_out!(x / y))),
            ("u32 / &Natural", &mut (|(x, y)| no_out!(x / &y))),
        ],
    );
}

fn benchmark_u32_div_assign_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "u32 /= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_and_positive_natural::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("u32 /= Natural", &mut (|(mut x, y)| x /= y)),
            ("u32 /= &Natural", &mut (|(mut x, y)| x /= &y)),
        ],
    );
}
