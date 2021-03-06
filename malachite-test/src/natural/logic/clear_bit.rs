use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use malachite_nz::natural::logic::bit_access::limbs_clear_bit;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_unsigned;
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_clear_bit);
    register_demo!(registry, demo_natural_clear_bit);
    register_bench!(registry, Small, benchmark_limbs_clear_bit);
    register_bench!(registry, Large, benchmark_natural_clear_bit);
}

fn demo_limbs_clear_bit(gm: GenerationMode, limit: usize) {
    for (limbs, index) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_clear_bit(&mut mut_limbs, index);
        println!(
            "limbs := {:?}; limbs_clear_bit(&mut limbs, {}); limbs = {:?}",
            limbs, index, mut_limbs
        );
    }
}

fn demo_natural_clear_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_limbs_clear_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "limbs_clear_bit(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "Malachite",
            &mut (|(ref mut limbs, index)| no_out!(limbs_clear_bit(limbs, index))),
        )],
    );
}

fn benchmark_natural_clear_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Natural.clear_bit(u64)",
        BenchmarkType::Single,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [("Malachite", &mut (|(mut n, index)| n.clear_bit(index)))],
    );
}
