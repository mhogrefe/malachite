use common::GenerationMode;
use malachite_base::traits::{AddMul, AddMulAssign};
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, BenchmarkOptions4,
                              BenchmarkOptions5, benchmark_1, benchmark_2, benchmark_4,
                              benchmark_5};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_triples_from_single, random_triples_from_single};
use std::cmp::max;

type It = Iterator<Item = (Integer, Integer, Integer)>;

pub fn exhaustive_inputs() -> Box<It> {
    Box::new(exhaustive_triples_from_single(exhaustive_integers()))
}

pub fn random_inputs(scale: u32) -> Box<It> {
    Box::new(random_triples_from_single(random_integers(
        &EXAMPLE_SEED,
        scale,
    )))
}

pub fn select_inputs(gm: GenerationMode) -> Box<It> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_inputs(),
        GenerationMode::Random(scale) => random_inputs(scale),
    }
}

pub fn demo_integer_add_mul_assign(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old, b_old, c_old, a
        );
    }
}

pub fn demo_integer_add_mul_assign_val_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, &c);
        println!(
            "a := {}; x.add_mul_assign({}, &{}); x = {}",
            a_old, b_old, c, a
        );
    }
}

pub fn demo_integer_add_mul_assign_ref_val(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.add_mul_assign(&b, c);
        println!(
            "a := {}; x.add_mul_assign(&{}, {}); x = {}",
            a_old, b, c_old, a
        );
    }
}

pub fn demo_integer_add_mul_assign_ref_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        a.add_mul_assign(&b, &c);
        println!(
            "a := {}; x.add_mul_assign(&{}, &{}); x = {}",
            a_old, b, c, a
        );
    }
}

pub fn demo_integer_add_mul(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.add_mul({}, {}) = {}",
            a_old,
            b_old,
            c_old,
            a.add_mul(b, c)
        );
    }
}

pub fn demo_integer_add_mul_val_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.add_mul({}, &{}) = {}",
            a_old,
            b_old,
            c,
            a.add_mul(b, &c)
        );
    }
}

pub fn demo_integer_add_mul_val_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.add_mul(&{}, {}) = {}",
            a_old,
            b,
            c_old,
            a.add_mul(&b, c)
        );
    }
}

pub fn demo_integer_add_mul_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, &{}) = {}", a_old, b, c, a.add_mul(&b, &c));
    }
}

pub fn demo_integer_add_mul_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in select_inputs(gm).take(limit) {
        println!(
            "(&{}).add_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).add_mul(&b, &c)
        );
    }
}

const X_AXIS_LABEL: &str = "max(a.significant\\\\_bits(), b.significant\\\\_bits(), \
                            c.significant\\\\_bits())";

pub fn benchmark_integer_add_mul_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.add_mul_assign(Integer, Integer)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(b, c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite",
        title: "Integer.add\\\\_mul\\\\_assign(Integer, Integer)",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul_assign(Integer, Integer) evaluation strategy",
        gm.name()
    );
    benchmark_4(BenchmarkOptions4 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(b, c)),
        function_g: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(b, &c)),
        function_h: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(&b, c)),
        function_i: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(&b, &c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        z_cons: &(|t| t.clone()),
        w_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul\\\\_assign(Integer, Integer)",
        g_name: "Integer.add\\\\_mul\\\\_assign(Integer, \\\\&Integer)",
        h_name: "Integer.add\\\\_mul\\\\_assign(\\\\&Integer, Integer)",
        i_name: "Integer.add\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer)",
        title: "Integer.add\\\\_mul\\\\_assign(Integer, Integer) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_assign_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul_assign(Integer, Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(b, c)),
        function_g: &(|(mut a, b, c): (Integer, Integer, Integer)| a += b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul\\\\_assign(Integer, Integer)",
        g_name: "Integer += Integer * Integer",
        title: "Integer.add\\\\_mul\\\\_assign(Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_assign_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul_assign(Integer, &Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(b, &c)),
        function_g: &(|(mut a, b, c): (Integer, Integer, Integer)| a += b * &c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul\\\\_assign(Integer, \\\\&Integer)",
        g_name: "Integer += Integer * \\\\&Integer",
        title: "Integer.add\\\\_mul\\\\_assign(Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_assign_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul_assign(&Integer, Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(&b, c)),
        function_g: &(|(mut a, b, c): (Integer, Integer, Integer)| a += &b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul\\\\_assign(\\\\&Integer, Integer)",
        g_name: "Integer += \\\\&Integer * Integer",
        title: "Integer.add\\\\_mul\\\\_assign(\\\\&Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_assign_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul_assign(&Integer, &Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(mut a, b, c): (Integer, Integer, Integer)| a.add_mul_assign(&b, &c)),
        function_g: &(|(mut a, b, c): (Integer, Integer, Integer)| a += &b * &c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer)",
        g_name: "Integer += \\\\&Integer * \\\\&Integer",
        title: "Integer.add\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.add_mul(Integer, Integer)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(b, c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite",
        title: "Integer.add\\\\_mul(Integer, Integer)",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul(Integer, Integer) evaluation strategy",
        gm.name()
    );
    benchmark_5(BenchmarkOptions5 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(b, &c)),
        function_h: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(&b, c)),
        function_i: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(&b, &c)),
        function_j: &(|(a, b, c): (Integer, Integer, Integer)| (&a).add_mul(&b, &c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        z_cons: &(|t| t.clone()),
        w_cons: &(|t| t.clone()),
        v_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul(Integer, Integer)",
        g_name: "Integer.add\\\\_mul(Integer, \\\\&Integer)",
        h_name: "Integer.add\\\\_mul(\\\\&Integer, Integer)",
        i_name: "Integer.add\\\\_mul(\\\\&Integer, \\\\&Integer)",
        j_name: "(\\\\&Integer).add\\\\_mul(\\\\&Integer, \\\\&Integer)",
        title: "Integer.add\\\\_mul(Integer, Integer) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.add_mul(Integer, Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (Integer, Integer, Integer)| a + b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul(Integer, Integer)",
        g_name: "Integer + Integer * Integer",
        title: "Integer.add\\\\_mul(Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_val_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul(Integer, &Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(b, &c)),
        function_g: &(|(a, b, c): (Integer, Integer, Integer)| a + b * &c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul(Integer, \\\\&Integer)",
        g_name: "Integer + Integer * \\\\&Integer",
        title: "Integer.add\\\\_mul(Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_val_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul(&Integer, Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(&b, c)),
        function_g: &(|(a, b, c): (Integer, Integer, Integer)| a + &b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul(\\\\&Integer, Integer)",
        g_name: "Integer + \\\\&Integer * Integer",
        title: "Integer.add\\\\_mul(\\\\&Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_val_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.add_mul(&Integer, &Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Integer, Integer, Integer)| a.add_mul(&b, &c)),
        function_g: &(|(a, b, c): (Integer, Integer, Integer)| a + &b * &c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.add\\\\_mul(\\\\&Integer, \\\\&Integer)",
        g_name: "Integer + \\\\&Integer * \\\\&Integer",
        title: "Integer.add\\\\_mul(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_add_mul_ref_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Integer).add_mul(&Integer, &Integer) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: select_inputs(gm),
        function_f: &(|(a, b, c): (Integer, Integer, Integer)| (&a).add_mul(&b, &c)),
        function_g: &(|(a, b, c): (Integer, Integer, Integer)| &a + &b * &c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "(\\\\&Integer).add\\\\_mul(\\\\&Integer, \\\\&Integer)",
        g_name: "\\\\&Integer + \\\\&Integer * \\\\&Integer",
        title: "(\\\\&Integer).add\\\\_mul(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
