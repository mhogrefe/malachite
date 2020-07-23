use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::integer::logic::bit_access::{
    limbs_slice_clear_bit_neg, limbs_vec_clear_bit_neg,
};
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_and_small_u64_var_3, pairs_of_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::integer::pairs_of_integer_and_small_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_slice_clear_bit_neg);
    register_demo!(registry, demo_limbs_vec_clear_bit_neg);
    register_demo!(registry, demo_integer_clear_bit);
    register_bench!(registry, Small, benchmark_limbs_slice_clear_bit_neg);
    register_bench!(registry, Small, benchmark_limbs_vec_clear_bit_neg);
    register_bench!(registry, Large, benchmark_integer_clear_bit);
}

fn demo_limbs_slice_clear_bit_neg(gm: GenerationMode, limit: usize) {
    for (limbs, index) in pairs_of_limb_vec_and_small_u64_var_3(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_slice_clear_bit_neg(&mut mut_limbs, index);
        println!(
            "limbs := {:?}; limbs_slice_clear_bit_neg(&mut limbs, {}); limbs = {:?}",
            limbs, index, mut_limbs
        );
    }
}

fn demo_limbs_vec_clear_bit_neg(gm: GenerationMode, limit: usize) {
    for (limbs, index) in
        pairs_of_unsigned_vec_and_small_unsigned_var_1::<Limb, u64>(gm).take(limit)
    {
        let mut mut_limbs = limbs.clone();
        limbs_vec_clear_bit_neg(&mut mut_limbs, index);
        println!(
            "limbs := {:?}; limbs_vec_clear_bit_neg(&mut limbs, {}); limbs = {:?}",
            limbs, index, mut_limbs
        );
    }
}

fn demo_integer_clear_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_limbs_slice_clear_bit_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_slice_clear_bit_neg(&mut [Limb], u64)",
        BenchmarkType::Single,
        pairs_of_limb_vec_and_small_u64_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(ref mut limbs, index)| no_out!(limbs_slice_clear_bit_neg(limbs, index))),
        )],
    );
}

fn benchmark_limbs_vec_clear_bit_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_vec_clear_bit_neg(&mut [Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(ref mut limbs, index)| no_out!(limbs_vec_clear_bit_neg(limbs, index))),
        )],
    );
}

fn benchmark_integer_clear_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer.clear_bit(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, index)| usize::exact_from(max(n.significant_bits(), index))),
        "max(n.significant_bits(), index)",
        &mut [("malachite", &mut (|(mut n, index)| n.clear_bit(index)))],
    );
}
