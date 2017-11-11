use common::gmp_integer_to_native;
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use rust_wheels::benchmarks::{benchmark_2, BenchmarkOptions2, benchmark_4, BenchmarkOptions4,
                              benchmark_5, BenchmarkOptions5};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::tuples::{exhaustive_triples_from_single, random_triples_from_single};
use std::cmp::max;

pub fn demo_exhaustive_integer_sub_mul_assign(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old,
            b_old,
            c_old,
            a
        );
    }
}

pub fn demo_random_integer_sub_mul_assign(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32))
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old,
            b_old,
            c_old,
            a
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul_assign_val_ref(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, &c);
        println!(
            "a := {}; x.sub_mul_assign({}, &{}); x = {}",
            a_old,
            b_old,
            c,
            a
        );
    }
}

pub fn demo_random_integer_sub_mul_assign_val_ref(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32))
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, &c);
        println!(
            "a := {}; x.sub_mul_assign({}, &{}); x = {}",
            a_old,
            b_old,
            c,
            a
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul_assign_ref_val(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.sub_mul_assign(&b, c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, {}); x = {}",
            a_old,
            b,
            c_old,
            a
        );
    }
}

pub fn demo_random_integer_sub_mul_assign_ref_val(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32))
        .take(limit)
    {
        let a_old = a.clone();
        let c_old = c.clone();
        a.sub_mul_assign(&b, c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, {}); x = {}",
            a_old,
            b,
            c_old,
            a
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul_assign_ref_ref(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, &c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, &{}); x = {}",
            a_old,
            b,
            c,
            a
        );
    }
}

pub fn demo_random_integer_sub_mul_assign_ref_ref(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32))
        .take(limit)
    {
        let a_old = a.clone();
        a.sub_mul_assign(&b, &c);
        println!(
            "a := {}; x.sub_mul_assign(&{}, &{}); x = {}",
            a_old,
            b,
            c,
            a
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c_old,
            a.sub_mul(b, c)
        );
    }
}

pub fn demo_random_integer_sub_mul(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        println!(
            "{}.sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c_old,
            a.sub_mul(b, c)
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul_val_val_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.sub_mul({}, &{}) = {}",
            a_old,
            b_old,
            c,
            a.sub_mul(b, &c)
        );
    }
}

pub fn demo_random_integer_sub_mul_val_val_ref(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "{}.sub_mul({}, &{}) = {}",
            a_old,
            b_old,
            c,
            a.sub_mul(b, &c)
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul_val_ref_val(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c_old,
            a.sub_mul(&b, c)
        );
    }
}

pub fn demo_random_integer_sub_mul_val_ref_val(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        println!(
            "{}.sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c_old,
            a.sub_mul(&b, c)
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul_val_ref_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, &{}) = {}", a_old, b, c, a.sub_mul(&b, &c));
    }
}

pub fn demo_random_integer_sub_mul_val_ref_ref(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, &{}) = {}", a_old, b, c, a.sub_mul(&b, &c));
    }
}

pub fn demo_exhaustive_integer_sub_mul_ref_ref_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_integers()).take(limit) {
        println!(
            "(&{}).sub_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).sub_mul(&b, &c)
        );
    }
}

pub fn demo_random_integer_sub_mul_ref_ref_ref(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_integers(&EXAMPLE_SEED, 32)).take(limit) {
        println!(
            "(&{}).sub_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).sub_mul(&b, &c)
        );
    }
}

const X_AXIS_LABEL: &str = "max(a.significant\\\\_bits(), b.significant\\\\_bits(), \
c.significant\\\\_bits())";

pub fn benchmark_exhaustive_integer_sub_mul_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(Integer, Integer)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(mut a, b, c): (gmp::Integer, gmp::Integer, gmp::Integer)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer)",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.sub_mul_assign(Integer, Integer)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (gmp::Integer, gmp::Integer, gmp::Integer)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer)",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_evaluation_strategy(
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking exhaustive Integer.sub_mul_assign(Integer, Integer) evaluation strategy"
    );
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, &c)
                      }),
        function_h: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(&b, c)
                      }),
        function_i: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        z_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        w_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer)",
        g_name: "Integer.sub\\\\_mul\\\\_assign(Integer, \\\\&Integer)",
        h_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, Integer)",
        i_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer)",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul_assign(Integer, Integer) evaluation strategy");
    benchmark_4(BenchmarkOptions4 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, &c)
                      }),
        function_h: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(&b, c)
                      }),
        function_i: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        z_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        w_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer)",
        g_name: "Integer.sub\\\\_mul\\\\_assign(Integer, \\\\&Integer)",
        h_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, Integer)",
        i_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer)",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(Integer, Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a -= b * c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer)",
        g_name: "Integer -= Integer * Integer",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul_assign(Integer, Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a -= b * c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer)",
        g_name: "Integer -= Integer * Integer",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_val_ref_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a -= b * &c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, \\\\&Integer)",
        g_name: "Integer -= Integer * \\\\&Integer",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_val_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul_assign(Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a -= b * &c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, \\\\&Integer)",
        g_name: "Integer -= Integer * \\\\&Integer",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_ref_val_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(&Integer, Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(&b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a -= &b * c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, Integer)",
        g_name: "Integer -= \\\\&Integer * Integer",
        title: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_ref_val_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul_assign(&Integer, Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(&b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a -= &b * c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, Integer)",
        g_name: "Integer -= \\\\&Integer * Integer",
        title: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_ref_ref_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(&Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a -= &b * &c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer)",
        g_name: "Integer -= \\\\&Integer * \\\\&Integer",
        title: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_ref_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul_assign(&Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul_assign(&b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a -= &b * &c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer)",
        g_name: "Integer -= \\\\&Integer * \\\\&Integer",
        title: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(Integer, Integer)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(a, b, c): (gmp::Integer, gmp::Integer, gmp::Integer)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul(Integer, Integer)",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.sub_mul(Integer, Integer)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (gmp::Integer, gmp::Integer, gmp::Integer)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul(Integer, Integer)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(Integer, Integer) evaluation strategy");
    benchmark_5(BenchmarkOptions5 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, &c)
                      }),
        function_h: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(&b, c)
                      }),
        function_i: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(&b, &c)
                      }),
        function_j: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          (&a).sub_mul(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        z_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        w_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        v_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(Integer, Integer)",
        g_name: "Integer.sub\\\\_mul(Integer, \\\\&Integer)",
        h_name: "Integer.sub\\\\_mul(\\\\&Integer, Integer)",
        i_name: "Integer.sub\\\\_mul(\\\\&Integer, \\\\&Integer)",
        j_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, \\\\&Integer)",
        title: "Integer.sub\\\\_mul(Integer, Integer) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul(Integer, Integer) evaluation strategy");
    benchmark_5(BenchmarkOptions5 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, &c)
                      }),
        function_h: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(&b, c)
                      }),
        function_i: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(&b, &c)
                      }),
        function_j: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          (&a).sub_mul(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        z_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        w_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        v_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(Integer, Integer)",
        g_name: "Integer.sub\\\\_mul(Integer, \\\\&Integer)",
        h_name: "Integer.sub\\\\_mul(\\\\&Integer, Integer)",
        i_name: "Integer.sub\\\\_mul(\\\\&Integer, \\\\&Integer)",
        j_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, \\\\&Integer)",
        title: "Integer.sub\\\\_mul(Integer, Integer) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(Integer, Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| a - b * c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(Integer, Integer)",
        g_name: "Integer - Integer * Integer",
        title: "Integer.sub\\\\_mul(Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_algorithms(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.sub_mul(Integer, Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| a - b * c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(Integer, Integer)",
        g_name: "Integer - Integer * Integer",
        title: "Integer.sub\\\\_mul(Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_val_val_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| a - b * &c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(Integer, \\\\&Integer)",
        g_name: "Integer - Integer * \\\\&Integer",
        title: "Integer.sub\\\\_mul(Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_val_val_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul(Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| a - b * &c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(Integer, \\\\&Integer)",
        g_name: "Integer - Integer * \\\\&Integer",
        title: "Integer.sub\\\\_mul(Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_val_ref_val_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(&Integer, Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(&b, c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| a - &b * c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(\\\\&Integer, Integer)",
        g_name: "Integer - \\\\&Integer * Integer",
        title: "Integer.sub\\\\_mul(\\\\&Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_val_ref_val_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul(&Integer, Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(&b, c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| a - &b * c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(\\\\&Integer, Integer)",
        g_name: "Integer - \\\\&Integer * Integer",
        title: "Integer.sub\\\\_mul(\\\\&Integer, Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_val_ref_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(&Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| a - &b * &c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(\\\\&Integer, \\\\&Integer)",
        g_name: "Integer - \\\\&Integer * \\\\&Integer",
        title: "Integer.sub\\\\_mul(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_val_ref_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul(&Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          a.sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| a - &b * &c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Integer.sub\\\\_mul(\\\\&Integer, \\\\&Integer)",
        g_name: "Integer - \\\\&Integer * \\\\&Integer",
        title: "Integer.sub\\\\_mul(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_ref_ref_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Integer).sub_mul(&Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_integers()),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          (&a).sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
            &a - &b * &c
        }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, \\\\&Integer)",
        g_name: "\\\\&Integer - \\\\&Integer * \\\\&Integer",
        title: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_ref_ref_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random (&Integer).sub_mul(&Integer, &Integer) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_integers(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
                          (&a).sub_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, native::Integer)| {
            &a - &b * &c
        }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_integer_to_native(a),
                gmp_integer_to_native(b),
                gmp_integer_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, \\\\&Integer)",
        g_name: "\\\\&Integer - \\\\&Integer * \\\\&Integer",
        title: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, \\\\&Integer) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
