use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::pairs_of_integer_and_signed;
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;
use rug::Assign as rug_assign;
use rust_wheels::benchmarks::{BenchmarkOptions3, benchmark_3};

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

pub fn benchmark_integer_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(i32)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integer_and_signed::<i32>(gm),
        function_f: &mut (|(mut n, i): (Integer, i32)| n.assign(i)),
        function_g: &mut (|(mut n, i): (BigInt, i32)| num_assign_i32(&mut n, i)),
        function_h: &mut (|(mut n, i): (rug::Integer, i32)| n.assign(i)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (integer_to_bigint(n), i)),
        z_cons: &(|&(ref n, i)| (integer_to_rug_integer(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer.assign(i32)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
