use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_unsigned, rm_pairs_of_natural_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_flip_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_flip_bit_library_comparison
    );
}

fn demo_natural_flip_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        let n_old = n.clone();
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_natural_flip_bit_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.flip_bit(u64)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, index))| usize::exact_from(index)),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, (mut n, index))| n.flip_bit(index))),
            (
                "rug",
                &mut (|((mut n, index), _)| no_out!(n.toggle_bit(u32::exact_from(index)))),
            ),
        ],
    );
}
