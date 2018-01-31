use common::{integer_to_bigint, integer_to_rugint_integer, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_base::num::AbsAssign;
use malachite_nz::integer::Integer;
use num::{BigInt, Signed};
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, BenchmarkOptions3,
                              benchmark_1, benchmark_2, benchmark_3};

pub fn demo_integer_abs_assign(gm: GenerationMode, limit: usize) {
    for mut n in integers(gm).take(limit) {
        let n_old = n.clone();
        n.abs_assign();
        println!("n := {}; n.abs_assign(); n = {}", n_old, n);
    }
}

pub fn demo_integer_abs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("abs({}) = {}", n.clone(), n.abs());
    }
}

pub fn demo_integer_abs_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("abs_ref(&{}) = {}", n, n.abs_ref());
    }
}

pub fn demo_integer_natural_abs(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("natural_abs({}) = {}", n.clone(), n.natural_abs());
    }
}

pub fn demo_integer_natural_abs_ref(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("natural_abs_ref(&{}) = {}", n, n.natural_abs_ref());
    }
}

pub fn benchmark_integer_abs_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.abs_assign()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.abs()),
        function_g: &(|n: BigInt| n.abs()),
        function_h: &(|mut n: rugint::Integer| n.abs().sign()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_bigint(x)),
        z_cons: &(|x| integer_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Integer.abs\\\\_assign()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.abs()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.abs()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.abs()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_abs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.abs() evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.abs()),
        function_g: &(|n: Integer| n.abs_ref()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "Integer.abs()",
        g_name: "Integer.abs_ref()",
        title: "Integer.abs() evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_natural_abs(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.natural_abs()", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.natural_abs()),
        x_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        title: "Integer.natural\\\\_abs()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_natural_abs_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.natural_abs() evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.natural_abs()),
        function_g: &(|n: Integer| n.natural_abs_ref()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| x.clone()),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "Integer.natural_abs()",
        g_name: "Integer.natural_abs_ref()",
        title: "Integer.natural\\\\_abs() evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
