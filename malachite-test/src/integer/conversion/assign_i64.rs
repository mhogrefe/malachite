use common::{integer_to_bigint, GenerationMode};
use inputs::integer::pairs_of_integer_and_signed;
use malachite_base::num::SignificantBits;
use malachite_base::traits::Assign;
use malachite_nz::integer::Integer;
use num::BigInt;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn num_assign_i64(x: &mut BigInt, i: i64) {
    *x = BigInt::from(i);
}

pub fn demo_integer_assign_i64(gm: GenerationMode, limit: usize) {
    for (mut n, i) in pairs_of_integer_and_signed::<i64>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(i);
        println!("x := {}; x.assign({}); x = {}", n_old, i, n);
    }
}

pub fn benchmark_integer_assign_i64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(i64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integer_and_signed::<i64>(gm),
        function_f: &(|(mut n, i): (Integer, i64)| n.assign(i)),
        function_g: &(|(mut n, i): (BigInt, i64)| num_assign_i64(&mut n, i)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, i)| (integer_to_bigint(n), i)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Integer.assign(i64)",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
