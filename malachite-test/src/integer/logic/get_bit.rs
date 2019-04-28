use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_small_unsigned_var_1;
use inputs::integer::{pairs_of_integer_and_small_u64, rm_pairs_of_integer_and_small_u64};
use malachite_base::num::traits::BitAccess;
use malachite_nz::integer::logic::bit_access::limbs_get_bit_neg;
use malachite_nz::platform::Limb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_get_bit_neg);
    register_demo!(registry, demo_integer_get_bit);
    register_bench!(registry, Small, benchmark_limbs_get_bit_neg);
    register_bench!(
        registry,
        Large,
        benchmark_integer_get_bit_library_comparison
    );
}

fn demo_limbs_get_bit_neg(gm: GenerationMode, limit: usize) {
    for (limbs, index) in
        pairs_of_unsigned_vec_and_small_unsigned_var_1::<Limb, u64>(gm).take(limit)
    {
        println!(
            "limbs_get_bit_neg({:?}, {}) = {}",
            limbs,
            index,
            limbs_get_bit_neg(&limbs, index)
        );
    }
}

fn demo_integer_get_bit(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_integer_and_small_u64(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

fn benchmark_limbs_get_bit_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_get_bit_neg(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "index",
        &mut [(
            "malachite",
            &mut (|(ref limbs, index)| no_out!(limbs_get_bit_neg(limbs, index))),
        )],
    );
}

fn benchmark_integer_get_bit_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer.get_bit(u64)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_u64(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, index))| index as usize),
        "index",
        &mut [
            (
                "malachite",
                &mut (|(_, (n, index))| no_out!(n.get_bit(index))),
            ),
            (
                "rug",
                &mut (|((n, index), _)| no_out!(n.get_bit(index as u32))),
            ),
        ],
    );
}
