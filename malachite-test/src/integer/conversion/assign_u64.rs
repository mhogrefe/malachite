use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{nm_pairs_of_integer_and_unsigned, pairs_of_integer_and_unsigned};
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use num::BigInt;

pub fn num_assign_u64(x: &mut BigInt, u: u64) {
    *x = BigInt::from(u);
}

pub fn demo_integer_assign_u64(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<u64>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_integer_assign_u64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(u64)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            ("num", &mut (|((mut x, y), _)| num_assign_u64(&mut x, y))),
        ],
    );
}
