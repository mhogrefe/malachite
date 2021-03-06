use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::natural::{
    rm_triples_of_natural_small_u64_and_bool, triples_of_natural_small_u64_and_bool,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_assign_bit);
    register_bench!(
        registry,
        Large,
        benchmark_natural_assign_bit_library_comparison
    );
}

fn demo_natural_assign_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index, bit) in triples_of_natural_small_u64_and_bool(gm).take(limit) {
        let n_old = n.clone();
        n.assign_bit(index, bit);
        println!(
            "x := {}; x.assign_bit({}, {}); x = {}",
            n_old, index, bit, n
        );
    }
}

fn benchmark_natural_assign_bit_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Natural.assign_bit(u64, bool)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_natural_small_u64_and_bool(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, index, _))| usize::exact_from(index)),
        "index",
        &mut [
            (
                "Malachite",
                &mut (|(_, (mut n, index, bit))| n.assign_bit(index, bit)),
            ),
            (
                "rug",
                &mut (|((mut n, index, bit), _)| no_out!(n.set_bit(u32::exact_from(index), bit))),
            ),
        ],
    );
}
