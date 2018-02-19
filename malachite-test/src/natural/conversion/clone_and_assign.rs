use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::natural::{naturals, pairs_of_naturals};
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use malachite_nz::natural::Natural;
use num::BigUint;
use rug;
use rug::Assign as rug_assign;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use std::cmp::max;

pub fn demo_natural_clone(gm: GenerationMode, limit: usize) {
    for n in naturals(gm).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

pub fn demo_natural_clone_from(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

pub fn demo_natural_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_natural_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_natural_clone(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.clone()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: naturals(gm),
        function_f: &mut (|n: Natural| n.clone()),
        function_g: &mut (|n: BigUint| n.clone()),
        function_h: &mut (|n: rug::Integer| n.clone()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| natural_to_biguint(x)),
        z_cons: &(|x| natural_to_rug_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural.clone()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_clone_from(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.clone_from(Natural)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_naturals(gm),
        function_f: &mut (|(mut x, y): (Natural, Natural)| x.clone_from(&y)),
        function_g: &mut (|(mut x, y): (BigUint, BigUint)| x.clone_from(&y)),
        function_h: &mut (|(mut x, y): (rug::Integer, rug::Integer)| x.clone_from(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_biguint(x), natural_to_biguint(y))),
        z_cons: &(|&(ref x, ref y)| (natural_to_rug_integer(x), natural_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural.clone\\\\_from(Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.assign(Natural)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_naturals(gm),
        function_f: &mut (|(mut x, y): (Natural, Natural)| x.assign(y)),
        function_g: &mut (|(mut x, y): (rug::Integer, rug::Integer)| x.assign(y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_rug_integer(x), natural_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Natural.assign(Natural)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.assign(Natural) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_naturals(gm),
        function_f: &mut (|(mut x, y): (Natural, Natural)| x.assign(y)),
        function_g: &mut (|(mut x, y): (Natural, Natural)| x.assign(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Natural.assign(Integer)",
        g_name: "Natural.assign(\\\\&Natural)",
        title: "Natural.assign(Natural) evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
