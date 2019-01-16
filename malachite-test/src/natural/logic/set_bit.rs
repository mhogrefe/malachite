use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_limb_vec_and_small_u64_var_2, pairs_of_unsigned_vec_and_small_unsigned,
};
use inputs::natural::{nm_pairs_of_natural_and_small_u64, pairs_of_natural_and_small_unsigned};
use malachite_base::num::BitAccess;
use malachite_nz::natural::logic::bit_access::{limbs_slice_set_bit, limbs_vec_set_bit};
use num::{BigUint, One};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_slice_set_bit);
    register_demo!(registry, demo_limbs_vec_set_bit);
    register_demo!(registry, demo_natural_set_bit);
    register_bench!(registry, Small, benchmark_limbs_slice_set_bit);
    register_bench!(registry, Small, benchmark_limbs_vec_set_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_set_bit_library_comparison
    );
}

pub fn num_set_bit(x: &mut BigUint, index: u64) {
    *x = x.clone() | (BigUint::one() << index as usize);
}

fn demo_limbs_slice_set_bit(gm: GenerationMode, limit: usize) {
    for (limbs, index) in pairs_of_limb_vec_and_small_u64_var_2(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_slice_set_bit(&mut mut_limbs, index);
        println!(
            "limbs := {:?}; limbs_slice_set_bit(&mut limbs, {}); limbs = {:?}",
            limbs, index, mut_limbs
        );
    }
}

fn demo_limbs_vec_set_bit(gm: GenerationMode, limit: usize) {
    for (limbs, index) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut mut_limbs = limbs.clone();
        limbs_vec_set_bit(&mut mut_limbs, index);
        println!(
            "limbs := {:?}; limbs_vec_set_bit(&mut limbs, {}); limbs = {:?}",
            limbs, index, mut_limbs
        );
    }
}

fn demo_natural_set_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.set_bit(index);
        println!("x := {}; x.set_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_limbs_slice_set_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_slice_set_bit(&mut [u32], u64)",
        BenchmarkType::Single,
        pairs_of_limb_vec_and_small_u64_var_2(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [(
            "malachite",
            &mut (|(ref mut limbs, index)| no_out!(limbs_slice_set_bit(limbs, index))),
        )],
    );
}

fn benchmark_limbs_vec_set_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_set_bit(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &mut [(
            "malachite",
            &mut (|(ref mut limbs, index)| no_out!(limbs_vec_set_bit(limbs, index))),
        )],
    );
}

fn benchmark_natural_set_bit_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.set_bit(u64)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_natural_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, index))| index as usize),
        "index",
        &mut [
            ("malachite", &mut (|(_, (mut n, index))| n.set_bit(index))),
            (
                "num",
                &mut (|((mut n, index), _)| num_set_bit(&mut n, index)),
            ),
        ],
    );
}
