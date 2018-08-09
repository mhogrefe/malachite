use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::vecs_of_unsigned_var_1;
use inputs::natural::{naturals, rm_naturals};
use malachite_base::num::{NextPowerOfTwo, NextPowerOfTwoAssign, SignificantBits};
use malachite_nz::natural::arithmetic::next_power_of_two::{
    limbs_next_power_of_two, limbs_slice_next_power_of_two_in_place,
    limbs_vec_next_power_of_two_in_place,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_next_power_of_two);
    register_demo!(registry, demo_limbs_slice_next_power_of_two_in_place);
    register_demo!(registry, demo_limbs_vec_next_power_of_two_in_place);
    register_demo!(registry, demo_natural_next_power_of_two_assign);
    register_demo!(registry, demo_natural_next_power_of_two);
    register_demo!(registry, demo_natural_next_power_of_two_ref);
    register_bench!(registry, Small, benchmark_limbs_next_power_of_two);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_next_power_of_two_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_next_power_of_two_in_place
    );
    register_bench!(registry, Large, benchmark_natural_next_power_of_two_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_next_power_of_two_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_next_power_of_two_evaluation_strategy
    );
}

fn demo_limbs_next_power_of_two(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_next_power_of_two({:?}) = {:?}",
            limbs,
            limbs_next_power_of_two(&limbs)
        );
    }
}

fn demo_limbs_slice_next_power_of_two_in_place(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        let carry = limbs_slice_next_power_of_two_in_place(&mut limbs);
        println!(
            "limbs := {:?}; limbs_slice_next_power_of_two_in_place(&mut limbs) = {}; limbs = {:?}",
            limbs_old, carry, limbs
        );
    }
}

fn demo_limbs_vec_next_power_of_two_in_place(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let mut limbs_old = limbs.clone();
        limbs_vec_next_power_of_two_in_place(&mut limbs);
        println!(
            "limbs := {:?}; limbs_vec_next_power_of_two_in_place(&mut limbs); limbs = {:?}",
            limbs_old, limbs
        );
    }
}

fn demo_natural_next_power_of_two_assign(gm: GenerationMode, limit: usize) {
    for mut n in naturals(gm).take(limit) {
        let n_old = n.clone();
        n.next_power_of_two_assign();
        println!("x := {}; x.next_power_of_two_assign(); x = {}", n_old, n);
    }
}

fn demo_natural_next_power_of_two(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.next_power_of_two() = {}", n_old, n.next_power_of_two());
    }
}

fn demo_natural_next_power_of_two_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!(
            "(&{}).next_power_of_two() = {}",
            n,
            (&n).next_power_of_two()
        );
    }
}

fn benchmark_limbs_next_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_next_power_of_two(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|ref limbs| no_out!(limbs_next_power_of_two(limbs))),
        )],
    );
}

fn benchmark_limbs_slice_next_power_of_two_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_slice_next_power_of_two_in_place(&mut [u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|ref mut limbs| no_out!(limbs_slice_next_power_of_two_in_place(limbs))),
        )],
    );
}

fn benchmark_limbs_vec_next_power_of_two_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_vec_next_power_of_two_in_place(&mut Vec<u32>)",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|ref mut limbs| limbs_vec_next_power_of_two_in_place(limbs)),
        )],
    );
}

fn benchmark_natural_next_power_of_two_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.next_power_of_two_assign()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [("malachite", &mut (|mut n| n.next_power_of_two_assign()))],
    );
}

fn benchmark_natural_next_power_of_two_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.next_power_of_two()",
        BenchmarkType::LibraryComparison,
        rm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, n)| no_out!(n.next_power_of_two()))),
            ("rug", &mut (|(n, _)| no_out!(n.next_power_of_two()))),
        ],
    );
}

fn benchmark_natural_next_power_of_two_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.next_power_of_two()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.next_power_of_two()",
                &mut (|n| no_out!(n.next_power_of_two())),
            ),
            (
                "(&Natural).next_power_of_two()",
                &mut (|n| no_out!((&n).next_power_of_two())),
            ),
        ],
    );
}
