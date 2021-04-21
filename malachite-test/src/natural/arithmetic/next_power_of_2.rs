use malachite_base::num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::arithmetic::next_power_of_2::{
    limbs_next_power_of_2, limbs_slice_next_power_of_2_in_place, limbs_vec_next_power_of_2_in_place,
};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::vecs_of_unsigned_var_1;
use malachite_test::inputs::natural::{naturals, rm_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_next_power_of_2);
    register_demo!(registry, demo_limbs_slice_next_power_of_2_in_place);
    register_demo!(registry, demo_limbs_vec_next_power_of_2_in_place);
    register_demo!(registry, demo_natural_next_power_of_2_assign);
    register_demo!(registry, demo_natural_next_power_of_2);
    register_demo!(registry, demo_natural_next_power_of_2_ref);
    register_bench!(registry, Small, benchmark_limbs_next_power_of_2);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_slice_next_power_of_2_in_place
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_vec_next_power_of_2_in_place
    );
    register_bench!(registry, Large, benchmark_natural_next_power_of_2_assign);
    register_bench!(
        registry,
        Large,
        benchmark_natural_next_power_of_2_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_next_power_of_2_evaluation_strategy
    );
}

fn demo_limbs_next_power_of_2(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_next_power_of_2({:?}) = {:?}",
            limbs,
            limbs_next_power_of_2(&limbs)
        );
    }
}

fn demo_limbs_slice_next_power_of_2_in_place(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let carry = limbs_slice_next_power_of_2_in_place(&mut limbs);
        println!(
            "limbs := {:?}; limbs_slice_next_power_of_2_in_place(&mut limbs) = {}; limbs = {:?}",
            limbs_old, carry, limbs
        );
    }
}

fn demo_limbs_vec_next_power_of_2_in_place(gm: GenerationMode, limit: usize) {
    for limbs in vecs_of_unsigned_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_next_power_of_2_in_place(&mut limbs);
        println!(
            "limbs := {:?}; limbs_vec_next_power_of_2_in_place(&mut limbs); limbs = {:?}",
            limbs_old, limbs
        );
    }
}

fn demo_natural_next_power_of_2_assign(gm: GenerationMode, limit: usize) {
    for mut n in naturals(gm).take(limit) {
        let n_old = n.clone();
        n.next_power_of_2_assign();
        println!("x := {}; x.next_power_of_2_assign(); x = {}", n_old, n);
    }
}

fn demo_natural_next_power_of_2(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        let n_old = n.clone();
        println!("{}.next_power_of_2() = {}", n_old, n.next_power_of_2());
    }
}

fn demo_natural_next_power_of_2_ref(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("(&{}).next_power_of_2() = {}", n, (&n).next_power_of_2());
    }
}

fn benchmark_limbs_next_power_of_2(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_next_power_of_2(&[u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|ref limbs| no_out!(limbs_next_power_of_2(limbs))),
        )],
    );
}

fn benchmark_limbs_slice_next_power_of_2_in_place(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "limbs_slice_next_power_of_2_in_place(&mut [u32])",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|ref mut limbs| no_out!(limbs_slice_next_power_of_2_in_place(limbs))),
        )],
    );
}

fn benchmark_limbs_vec_next_power_of_2_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_vec_next_power_of_2_in_place(&mut Vec<u32>)",
        BenchmarkType::Single,
        vecs_of_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref limbs| limbs.len()),
        "limbs.len()",
        &mut [(
            "Malachite",
            &mut (|ref mut limbs| limbs_vec_next_power_of_2_in_place(limbs)),
        )],
    );
}

fn benchmark_natural_next_power_of_2_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.next_power_of_2_assign()",
        BenchmarkType::Single,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [("Malachite", &mut (|mut n| n.next_power_of_2_assign()))],
    );
}

fn benchmark_natural_next_power_of_2_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.next_power_of_2()",
        BenchmarkType::LibraryComparison,
        rm_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref n)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, n)| no_out!(n.next_power_of_2()))),
            ("rug", &mut (|(n, _)| no_out!(n.next_power_of_two()))),
        ],
    );
}

fn benchmark_natural_next_power_of_2_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.next_power_of_2()",
        BenchmarkType::EvaluationStrategy,
        naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|ref n| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Natural.next_power_of_2()",
                &mut (|n| no_out!(n.next_power_of_2())),
            ),
            (
                "(&Natural).next_power_of_2()",
                &mut (|n| no_out!((&n).next_power_of_2())),
            ),
        ],
    );
}
