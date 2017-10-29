use common::gmp_natural_to_native;
use malachite_base::traits::{AddMul, AddMulAssign};
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use rust_wheels::benchmarks::{benchmark_2, BenchmarkOptions2, benchmark_4, BenchmarkOptions4,
                              benchmark_5, BenchmarkOptions5};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::tuples::{exhaustive_triples_from_single, random_triples_from_single};
use std::cmp::max;

pub fn demo_exhaustive_natural_add_mul_assign(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old,
            b_old,
            c_old,
            a
        );
    }
}

pub fn demo_random_natural_add_mul_assign(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32))
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        let c_old = c.clone();
        a.add_mul_assign(b, c);
        println!(
            "a := {}; x.add_mul_assign({}, {}); x = {}",
            a_old,
            b_old,
            c_old,
            a
        );
    }
}

pub fn demo_exhaustive_natural_add_mul_assign_val_ref(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, &c);
        println!(
            "a := {}; x.add_mul_assign({}, &{}); x = {}",
            a_old,
            b_old,
            c,
            a
        );
    }
}

pub fn demo_random_natural_add_mul_assign_val_ref(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32))
        .take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        a.add_mul_assign(b, &c);
        println!(
            "a := {}; x.add_mul_assign({}, &{}); x = {}",
            a_old,
            b_old,
            c,
            a
        );
    }
}

pub fn demo_exhaustive_natural_add_mul_assign_ref_val(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
        let a_old = a.clone();
        let c_old = c.clone();
        a.add_mul_assign(&b, c);
        println!(
            "a := {}; x.add_mul_assign(&{}, {}); x = {}",
            a_old,
            b,
            c_old,
            a
        );
    }
}

pub fn demo_random_natural_add_mul_assign_ref_val(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32))
        .take(limit)
    {
        let a_old = a.clone();
        let c_old = c.clone();
        a.add_mul_assign(&b, c);
        println!(
            "a := {}; x.add_mul_assign(&{}, {}); x = {}",
            a_old,
            b,
            c_old,
            a
        );
    }
}

pub fn demo_exhaustive_natural_add_mul_assign_ref_ref(limit: usize) {
    for (mut a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
        let a_old = a.clone();
        a.add_mul_assign(&b, &c);
        println!(
            "a := {}; x.add_mul_assign(&{}, &{}); x = {}",
            a_old,
            b,
            c,
            a
        );
    }
}

pub fn demo_random_natural_add_mul_assign_ref_ref(limit: usize) {
    for (mut a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32))
        .take(limit)
    {
        let a_old = a.clone();
        a.add_mul_assign(&b, &c);
        println!(
            "a := {}; x.add_mul_assign(&{}, &{}); x = {}",
            a_old,
            b,
            c,
            a
        );
    }
}

pub fn demo_exhaustive_natural_add_mul(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
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

pub fn demo_random_natural_add_mul(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
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

pub fn demo_exhaustive_natural_add_mul_val_val_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
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

pub fn demo_random_natural_add_mul_val_val_ref(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
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

pub fn demo_exhaustive_natural_add_mul_val_ref_val(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
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

pub fn demo_random_natural_add_mul_val_ref_val(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
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

pub fn demo_exhaustive_natural_add_mul_val_ref_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, &{}) = {}", a_old, b, c, a.add_mul(&b, &c));
    }
}

pub fn demo_random_natural_add_mul_val_ref_ref(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
        let a_old = a.clone();
        println!("{}.add_mul(&{}, &{}) = {}", a_old, b, c, a.add_mul(&b, &c));
    }
}

pub fn demo_exhaustive_natural_add_mul_ref_ref_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples_from_single(exhaustive_naturals()).take(limit) {
        println!(
            "(&{}).add_mul(&{}, &{}) = {}",
            a,
            b,
            c,
            (&a).add_mul(&b, &c)
        );
    }
}

pub fn demo_random_natural_add_mul_ref_ref_ref(limit: usize) {
    for (a, b, c) in random_triples_from_single(random_naturals(&EXAMPLE_SEED, 32)).take(limit) {
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

pub fn benchmark_exhaustive_natural_add_mul_assign(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul_assign(Natural, Natural)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(mut a, b, c): (gmp::Natural, gmp::Natural, gmp::Natural)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
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
        title: "Natural.add\\\\_mul\\\\_assign(Natural, Natural)",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.add_mul_assign(Natural, Natural)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (gmp::Natural, gmp::Natural, gmp::Natural)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
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
        title: "Natural.add\\\\_mul\\\\_assign(Natural, Natural)",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_assign_evaluation_strategy(
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking exhaustive Natural.add_mul_assign(Natural, Natural) evaluation strategy"
    );
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, &c)
                      }),
        function_h: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(&b, c)
                      }),
        function_i: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        z_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        w_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, Natural)",
        g_name: "Natural.add\\\\_mul\\\\_assign(Natural, \\\\&Natural)",
        h_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, Natural)",
        i_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, Natural) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul_assign(Natural, Natural) evaluation strategy");
    benchmark_4(BenchmarkOptions4 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, &c)
                      }),
        function_h: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(&b, c)
                      }),
        function_i: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        z_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        w_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, Natural)",
        g_name: "Natural.add\\\\_mul\\\\_assign(Natural, \\\\&Natural)",
        h_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, Natural)",
        i_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, Natural) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_assign_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul_assign(Natural, Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a += b * c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, Natural)",
        g_name: "Natural += Natural * Natural",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul_assign(Natural, Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a += b * c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, Natural)",
        g_name: "Natural += Natural * Natural",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_assign_val_ref_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.add_mul_assign(Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a += b * &c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, \\\\&Natural)",
        g_name: "Natural += Natural * \\\\&Natural",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_val_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul_assign(Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a += b * &c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(Natural, \\\\&Natural)",
        g_name: "Natural += Natural * \\\\&Natural",
        title: "Natural.add\\\\_mul\\\\_assign(Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_assign_ref_val_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.add_mul_assign(&Natural, Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(&b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a += &b * c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, Natural)",
        g_name: "Natural += \\\\&Natural * Natural",
        title: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_ref_val_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul_assign(&Natural, Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(&b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a += &b * c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, Natural)",
        g_name: "Natural += \\\\&Natural * Natural",
        title: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_assign_ref_ref_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.add_mul_assign(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(&b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a += &b * &c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        g_name: "Natural += \\\\&Natural * \\\\&Natural",
        title: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_assign_ref_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul_assign(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul_assign(&b, &c)
                      }),
        function_g: &(|(mut a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a += &b * &c
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural)",
        g_name: "Natural += \\\\&Natural * \\\\&Natural",
        title: "Natural.add\\\\_mul\\\\_assign(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(Natural, Natural)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (gmp::Natural, gmp::Natural, gmp::Natural)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
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
        title: "Natural.add\\\\_mul(Natural, Natural)",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.add_mul(Natural, Natural)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (gmp::Natural, gmp::Natural, gmp::Natural)| a.add_mul(b, c)),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
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
        title: "Natural.add\\\\_mul(Natural, Natural)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(Natural, Natural) evaluation strategy");
    benchmark_5(BenchmarkOptions5 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, &c)
                      }),
        function_h: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(&b, c)
                      }),
        function_i: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(&b, &c)
                      }),
        function_j: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          (&a).add_mul(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        z_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        w_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        v_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, Natural)",
        g_name: "Natural.add\\\\_mul(Natural, \\\\&Natural)",
        h_name: "Natural.add\\\\_mul(\\\\&Natural, Natural)",
        i_name: "Natural.add\\\\_mul(\\\\&Natural, \\\\&Natural)",
        j_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, \\\\&Natural)",
        title: "Natural.add\\\\_mul(Natural, Natural) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Natural.add_mul(Natural, Natural) evaluation strategy");
    benchmark_5(BenchmarkOptions5 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, &c)
                      }),
        function_h: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(&b, c)
                      }),
        function_i: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(&b, &c)
                      }),
        function_j: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          (&a).add_mul(&b, &c)
                      }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        z_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        w_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        v_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, Natural)",
        g_name: "Natural.add\\\\_mul(Natural, \\\\&Natural)",
        h_name: "Natural.add\\\\_mul(\\\\&Natural, Natural)",
        i_name: "Natural.add\\\\_mul(\\\\&Natural, \\\\&Natural)",
        j_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, \\\\&Natural)",
        title: "Natural.add\\\\_mul(Natural, Natural) evaluation strategy",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(Natural, Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| a + b * c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, Natural)",
        g_name: "Natural + Natural * Natural",
        title: "Natural.add\\\\_mul(Natural, Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_algorithms(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.add_mul(Natural, Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| a + b * c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, Natural)",
        g_name: "Natural + Natural * Natural",
        title: "Natural.add\\\\_mul(Natural, Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_val_val_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| a + b * &c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, \\\\&Natural)",
        g_name: "Natural + Natural * \\\\&Natural",
        title: "Natural.add\\\\_mul(Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_val_val_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.add_mul(Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| a + b * &c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(Natural, \\\\&Natural)",
        g_name: "Natural + Natural * \\\\&Natural",
        title: "Natural.add\\\\_mul(Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_val_ref_val_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(&Natural, Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(&b, c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| a + &b * c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(\\\\&Natural, Natural)",
        g_name: "Natural + \\\\&Natural * Natural",
        title: "Natural.add\\\\_mul(\\\\&Natural, Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_val_ref_val_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.add_mul(&Natural, Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(&b, c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| a + &b * c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(\\\\&Natural, Natural)",
        g_name: "Natural + \\\\&Natural * Natural",
        title: "Natural.add\\\\_mul(\\\\&Natural, Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_val_ref_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.add_mul(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| a + &b * &c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "Natural + \\\\&Natural * \\\\&Natural",
        title: "Natural.add\\\\_mul(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_val_ref_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking exhaustive Natural.add_mul(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          a.add_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| a + &b * &c),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "Natural.add\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "Natural + \\\\&Natural * \\\\&Natural",
        title: "Natural.add\\\\_mul(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_add_mul_ref_ref_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Natural).add_mul(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples_from_single(exhaustive_naturals()),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          (&a).add_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
            &a + &b * &c
        }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "\\\\&Natural + \\\\&Natural * \\\\&Natural",
        title: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_add_mul_ref_ref_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking exhaustive (&Natural).add_mul(&Natural, &Natural) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples_from_single(random_naturals(&EXAMPLE_SEED, scale)),
        function_f: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
                          (&a).add_mul(&b, &c)
                      }),
        function_g: &(|(a, b, c): (native::Natural, native::Natural, native::Natural)| {
            &a + &b * &c
        }),
        x_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        y_cons: &(|&(ref a, ref b, ref c)| {
            (
                gmp_natural_to_native(a),
                gmp_natural_to_native(b),
                gmp_natural_to_native(c),
            )
        }),
        x_param: &(|&(ref a, ref b, ref c)| {
            max(
                max(a.significant_bits(), b.significant_bits()),
                c.significant_bits(),
            ) as usize
        }),
        limit,
        f_name: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, \\\\&Natural)",
        g_name: "\\\\&Natural + \\\\&Natural * \\\\&Natural",
        title: "(\\\\&Natural).add\\\\_mul(\\\\&Natural, \\\\&Natural) algorithms",
        x_axis_label: X_AXIS_LABEL,
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
