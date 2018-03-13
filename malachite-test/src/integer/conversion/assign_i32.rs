use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::integer::{nrm_pairs_of_integer_and_signed, pairs_of_integer_and_signed};
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use num::BigInt;
use rug::Assign as rug_assign;

pub fn num_assign_i32(x: &mut BigInt, i: i32) {
    *x = BigInt::from(i);
}

pub fn demo_integer_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_signed::<i32>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(i);
        println!("x := {}; x.assign({}); x = {}", n_old, i, n);
    }
}

pub fn benchmark_integer_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(i32)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_signed(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &[
            ("malachite", &mut (|(_, _, (mut x, y))| x.assign(y))),
            ("num", &mut (|((mut x, y), _, _)| num_assign_i32(&mut x, y))),
            ("rug", &mut (|(_, (mut x, y), _)| x.assign(y))),
        ],
    );
}
