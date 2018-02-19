use common::GenerationMode;
use inputs::integer::pairs_of_integer_and_small_u64;
use malachite_base::num::{BitAccess, SignificantBits};
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, benchmark_1};

pub fn demo_integer_clear_bit(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_integer_and_small_u64(gm).take(limit) {
        let n_old = n.clone();
        n.clear_bit(index);
        println!("x := {}; x.clear_bit({}); x = {}", n_old, index, n);
    }
}

pub fn benchmark_integer_clear_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.clear_bit(u64)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: pairs_of_integer_and_small_u64(gm),
        function_f: &mut (|(mut n, index): (Integer, u64)| n.clear_bit(index)),
        x_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.clear\\\\_bit(u64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
