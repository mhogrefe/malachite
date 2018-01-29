use common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_base::traits::NegAssign;
use malachite_nz::integer::Integer;
use num::BigInt;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, BenchmarkOptions3,
                              benchmark_1, benchmark_2, benchmark_3};

pub fn demo_integer_neg_assign(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.neg_assign();
        println!("n := {}; n.neg_assign(); n = {}", n_old, n);
    }
}

pub fn demo_integer_neg(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("-({}) = {}", n.clone(), -n);
    }
}

pub fn demo_integer_neg_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("-(&{}) = {}", n, -&n);
    }
}

pub fn benchmark_integer_neg_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.neg_assign()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &(|mut n: Integer| n.neg_assign()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.neg_assign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_neg(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} -Integer", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: integers(gm),
        function_f: &(|n: Integer| -n),
        function_g: &(|n: BigInt| -n),
        function_h: &(|n: rugint::Integer| -n),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_bigint(x)),
        z_cons: &(|x| integer_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "-Integer",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_neg_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking {} -Integer evaluation strategy", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &(|n: Integer| -n),
        function_g: &(|n: Integer| -&n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "-Integer",
        g_name: "-\\\\&Integer",
        title: "-Integer evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
