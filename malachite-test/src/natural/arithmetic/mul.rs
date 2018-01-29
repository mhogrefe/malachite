use common::{natural_to_biguint, natural_to_rugint_integer, GenerationMode};
use inputs::natural::pairs_of_naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};

pub fn demo_natural_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= y.clone();
        println!("x := {}; x *= {}; x = {}", x_old, y, x);
    }
}

pub fn demo_natural_mul_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x *= &y;
        println!("x := {}; x *= &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_natural_mul(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} * {} = {}", x_old, y_old, x * y);
    }
}

pub fn demo_natural_mul_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} * &{} = {}", x_old, y, x * &y);
    }
}

pub fn demo_natural_mul_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} * {} = {}", x, y_old, &x * y);
    }
}

pub fn demo_natural_mul_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} * &{} = {}", x, y, &x * &y);
    }
}

pub fn benchmark_natural_mul_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural *= Natural", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_naturals(gm),
        function_f: &(|(mut x, y)| x *= y),
        function_g: &(|(mut x, y): (rugint::Integer, rugint::Integer)| x *= y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_rugint_integer(x), natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Natural *= Natural",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_mul_assign_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural *= Natural algorithms", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_naturals(gm),
        function_f: &(|(mut x, y)| x *= y),
        function_g: &(|(mut x, y): (Natural, Natural)| x._mul_assign_basecase_mem_opt(y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "combined",
        g_name: "basecase only",
        title: "Natural *= Natural algorithms",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural *= Natural evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_naturals(gm),
        function_f: &(|(mut x, y)| x *= y),
        function_g: &(|(mut x, y)| x *= &y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "Natural *= Natural",
        g_name: "Natural *= \\\\&Natural",
        title: "Natural *= Natural evaluation strategy",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_mul(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural * Natural", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_naturals(gm),
        function_f: &(|(x, y)| x * y),
        function_g: &(|(x, y)| x * y),
        function_h: &(|(x, y)| x * y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_biguint(x), natural_to_biguint(y))),
        z_cons: &(|&(ref x, ref y)| (natural_to_rugint_integer(x), natural_to_rugint_integer(y))),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rugint",
        title: "Natural * Natural",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural * Natural evaluation strategy",
        gm.name()
    );
    benchmark_4(BenchmarkOptions4 {
        xs: pairs_of_naturals(gm),
        function_f: &(|(x, y)| x * y),
        function_g: &(|(x, y)| x * &y),
        function_h: &(|(x, y)| &x * y),
        function_i: &(|(x, y)| &x * &y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        z_cons: &(|p| p.clone()),
        w_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| (x.significant_bits() + y.significant_bits()) as usize),
        limit,
        f_name: "Natural * Natural",
        g_name: "Natural * \\\\&Natural",
        h_name: "\\\\&Natural * Natural",
        i_name: "\\\\&Natural * \\\\&Natural",
        title: "Natural * Natural evaluation strategy",
        x_axis_label: "x.significant\\\\_bits() + y.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
