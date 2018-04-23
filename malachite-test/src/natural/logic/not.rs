use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{pairs_of_unsigned_vec_var_3, vecs_of_unsigned};
use inputs::natural::{naturals, rm_naturals};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::logic::not::{limbs_not, limbs_not_in_place};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_not);
    register_demo!(registry, demo_limbs_not_in_place);
    register_demo!(registry, demo_natural_not);
    register_demo!(registry, demo_natural_not_ref);
    register_bench!(registry, Small, benchmark_limbs_not);
    register_bench!(registry, Small, benchmark_limbs_not_in_place);
    register_bench!(registry, Large, benchmark_natural_not_library_comparison);
    register_bench!(registry, Large, benchmark_natural_not_evaluation_strategy);
}

fn demo_limbs_not(gm: GenerationMode, limit: usize) {
    for (limbs_out, limbs_in) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        let mut mut_limbs_out = limbs_out.clone();
        limbs_not(&mut mut_limbs_out, &limbs_in);
        println!(
            "limbs_out := {:?}; limbs_not(&mut limbs_out, &{:?}); limbs_out = {:?}",
            &limbs_out, &limbs_in, &mut_limbs_out
        );
    }
}

fn demo_limbs_not_in_place(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_not_in_place(&mut mut_limbs);
        println!(
            "limbs := {:?}; limbs_not_in_place(&mut limbs); limbs = {:?}",
            limbs, mut_limbs
        );
    }
}

fn demo_natural_not(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("!({}) = {}", n.clone(), !n);
    }
}

fn demo_natural_not_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("!(&{}) = {}", n, !&n);
    }
}

fn benchmark_limbs_not(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_not(&mut [u32], &[u32])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref limbs_in)| limbs_in.len() as usize),
        "limbs_in.len()",
        &mut [(
            "malachite",
            &mut (|(ref mut limbs_out, ref limbs_in)| limbs_not(limbs_out, limbs_in)),
        )],
    );
}

fn benchmark_limbs_not_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_not_in_place(&mut [u32])",
        BenchmarkType::Single,
        vecs_of_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|limbs| limbs.len() as usize),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|mut limbs| limbs_not_in_place(&mut limbs)),
        )],
    );
}

fn benchmark_natural_not_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "!Natural",
        BenchmarkType::LibraryComparison,
        rm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(!n))),
            ("rug", &mut (|(n, _)| no_out!(!n))),
        ],
    );
}

fn benchmark_natural_not_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "!Natural",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("-Natural", &mut (|n| no_out!(!n))),
            ("-&Natural", &mut (|n| no_out!(!&n))),
        ],
    );
}
