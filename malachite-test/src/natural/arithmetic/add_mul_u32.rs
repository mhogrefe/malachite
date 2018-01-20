use common::GenerationMode;
use malachite_base::num::SignificantBits;
use malachite_base::traits::{AddMul, AddMulAssign};
use malachite_nz::natural::Natural;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, BenchmarkOptions4,
                              benchmark_1, benchmark_2, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_triples, random_triples};
use std::cmp::max;

type It = Iterator<Item = (Natural, Natural, u32)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_triples(
        exhaustive_naturals(),
        exhaustive_naturals(),
        exhaustive_u(),
    ))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_naturals(seed, scale)),
        &(|seed| random_x(seed)),
    ))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_natural_add_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

pub fn demo_natural_add_mul_assign_u32_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        a.add_mul_assign(&b, c);
        println!("a := {}; x.add_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_natural_add_mul_u32(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.add_mul({}, {}) = {}", a_old, b_old, c, a.add_mul(b, c));
    }
}

pub fn demo_natural_add_mul_u32_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, {}) = {}", a_old, b, c, a.add_mul(&b, c));
    }
}

pub fn demo_natural_add_mul_u32_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).add_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).add_mul(b, c)
        );
    }
}

pub fn demo_natural_add_mul_u32_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).add_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).add_mul(&b, c)
        );
    }
}

pub fn benchmark_natural_add_mul_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.add_mul_assign(Natural, u32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Natural, Natural, u32)| a.add_mul_assign(b, c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_assign_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.add_mul_assign(Natural, u32) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Natural, Natural, u32)| a.add_mul_assign(b, c)),
        function_g: &(|(mut a, b, c): (Natural, Natural, u32)| a.add_mul_assign(&b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        g_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_assign_u32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.add_mul_assign(Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Natural, Natural, u32)| a.add_mul_assign(b, c)),
        function_g: &(|(mut a, b, c)| a += b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, u32)",
        g_name: "Natural += Natural * u32",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_assign_u32_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.add_mul_assign(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Natural, Natural, u32)| a.add_mul_assign(&b, c)),
        function_g: &(|(mut a, b, c)| a += &b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32)",
        g_name: "Natural += \\\\&Natural * u32",
        title: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.add_mul(Natural, u32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Natural, Natural, u32)| a.add_mul(b, c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        title: "Natural.add\\\\_mul(Natural, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.add_mul(Natural, u32) evaluation strategy",
        gm.name()
    );
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Natural, Natural, u32)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (Natural, Natural, u32)| a.add_mul(&b, c)),
        function_h: &(|(a, b, c): (Natural, Natural, u32)| (&a).add_mul(b, c)),
        function_i: &(|(a, b, c): (Natural, Natural, u32)| (&a).add_mul(&b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        z_cons: &(|t| t.clone()),
        w_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, u32)",
        g_name: "Natural.add\\\\_mul(\\\\&Natural, u32)",
        h_name: "(\\\\&Natural).add\\\\_mul(Natural, u32)",
        i_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32)",
        title: "Natural.add\\\\_mul(\\\\&Natural, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_u32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Natural.add_mul(Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Natural, Natural, u32)| a.add_mul(b, c)),
        function_g: &(|(a, b, c)| a + b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, u32)",
        g_name: "Natural + Natural * u32",
        title: "Natural.add\\\\_mul(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_u32_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural.add_mul(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Natural, Natural, u32)| a.add_mul(&b, c)),
        function_g: &(|(a, b, c)| a + &b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Natural.add\\\\_mul(\\\\&Natural, u32)",
        g_name: "Natural + \\\\&Natural * u32",
        title: "Natural.add\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_u32_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Natural).add_mul(Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Natural, Natural, u32)| (&a).add_mul(b, c)),
        function_g: &(|(a, b, c)| &a + b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Natural).add\\\\_mul(Natural, u32)",
        g_name: "(\\\\&Natural) + Natural * u32",
        title: "(\\\\&Natural).add\\\\_mul(Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_mul_u32_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Natural).add_mul(&Natural, u32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Natural, Natural, u32)| (&a).add_mul(&b, c)),
        function_g: &(|(a, b, c)| &a + &b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32)",
        g_name: "(\\\\&Natural) + \\\\&Natural * u32",
        title: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
