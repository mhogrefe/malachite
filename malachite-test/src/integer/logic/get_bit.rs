use common::{integer_to_rug_integer, GenerationMode};
use inputs::integer::pairs_of_integer_and_small_u64;
use malachite_base::num::BitAccess;
use malachite_nz::integer::Integer;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn demo_integer_get_bit(gm: GenerationMode, limit: usize) {
    for (n, index) in pairs_of_integer_and_small_u64(gm).take(limit) {
        println!("get_bit({}, {}) = {}", n, index, n.get_bit(index));
    }
}

pub fn benchmark_integer_get_bit(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.get_bit(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_small_u64(gm),
        function_f: &mut (|(n, index): (Integer, u64)| n.get_bit(index)),
        function_g: &mut (|(n, index): (rug::Integer, u64)| n.get_bit(index as u32)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (integer_to_rug_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer.get\\\\_bit(u64)",
        x_axis_label: "index",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
