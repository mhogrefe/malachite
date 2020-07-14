use std::cmp::max;

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_nz::integer::logic::bit_access::limbs_set_bit_neg;

use malachite_test::common::{
    m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType,
};
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_unsigned_var_1;
use malachite_test::inputs::integer::pairs_of_integer_and_small_unsigned;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_set_bit_neg);
    register_demo!(registry, demo_integer_set_bit);
    register_bench!(registry, Small, benchmark_limbs_set_bit_neg);
    register_bench!(registry, Large, benchmark_integer_set_bit);
}

fn demo_limbs_set_bit_neg(gm: GenerationMode, limit: usize) {
    for (limbs, index) in pairs_of_unsigned_vec_and_small_unsigned_var_1(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_set_bit_neg(&mut mut_limbs, index);
        println!(
            "limbs := {:?}; limbs_set_bit_neg(&mut limbs, {}); limbs = {:?}",
            limbs, index, mut_limbs
        );
    }
}

fn demo_integer_set_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_integer_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_limbs_set_bit_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_set_bit_neg(&mut [u32], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| usize::exact_from(index)),
        "index",
        &mut [(
            "malachite",
            &mut (|(ref mut limbs, index)| limbs_set_bit_neg(limbs, index)),
        )],
    );
}

fn benchmark_integer_set_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.set_bit(u64)",
        BenchmarkType::Single,
        pairs_of_integer_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, index)| usize::exact_from(max(n.significant_bits(), index))),
        "max(n.significant_bits(), index)",
        &mut [("malachite", &mut (|(mut n, index)| n.set_bit(index)))],
    );
}
