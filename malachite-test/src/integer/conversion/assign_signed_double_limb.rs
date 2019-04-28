use malachite_base::num::traits::Assign;
use malachite_base::num::traits::SignificantBits;
use num::BigInt;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{nm_pairs_of_integer_and_signed, pairs_of_integer_and_signed};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_assign_i64);
    register_bench!(
        registry,
        Large,
        benchmark_integer_assign_i64_library_comparison
    );
}

pub fn num_assign_i64(x: &mut BigInt, i: i64) {
    *x = BigInt::from(i);
}

fn demo_integer_assign_i64(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_signed::<i64>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(i);
        println!("x := {}; x.assign({}); x = {}", n_old, i, n);
    }
}

fn benchmark_integer_assign_i64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(i64)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            ("num", &mut (|((mut x, y), _)| num_assign_i64(&mut x, y))),
        ],
    );
}
