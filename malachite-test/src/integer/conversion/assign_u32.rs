use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::pairs_of_integer_and_unsigned;
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;
use rug::Assign as rug_assign;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

pub fn num_assign_u32(x: &mut BigInt, u: u32) {
    *x = BigInt::from(u);
}

pub fn demo_integer_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

pub fn benchmark_integer_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(u32)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integer_and_unsigned::<u32>(gm),
        function_f: &mut (|(mut n, u): (Integer, u32)| n.assign(u)),
        function_g: &mut (|(mut n, u): (BigInt, u32)| num_assign_u32(&mut n, u)),
        function_h: &mut (|(mut n, u): (rug::Integer, u32)| n.assign(u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (integer_to_bigint(n), u)),
        z_cons: &(|&(ref n, u)| (integer_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer.assign(u32)",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
