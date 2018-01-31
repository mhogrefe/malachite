use common::{integer_to_rugint_integer, natural_to_rugint_integer, GenerationMode};
use inputs::integer::pairs_of_natural_and_natural_integer;
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use rugint;
use rugint::Assign as rugint_assign;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};
use std::cmp::max;

pub fn demo_natural_assign_integer(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_natural_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_natural_assign_integer_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_natural_integer(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_natural_assign_integer(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.assign(Integer)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_natural_integer(gm),
        function_f: &(|(mut x, y): (Natural, Integer)| x.assign(y)),
        function_g: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x.assign(y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_rugint_integer(x), integer_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Natural.assign(Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_assign_integer_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.assign(Integer) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_natural_integer(gm),
        function_f: &(|(mut x, y): (Natural, Integer)| x.assign(y)),
        function_g: &(|(mut x, y): (Natural, Integer)| x.assign(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Natural.assign(Integer)",
        g_name: "Natural.assign(\\\\&Integer)",
        title: "Natural.assign(Integer) evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
