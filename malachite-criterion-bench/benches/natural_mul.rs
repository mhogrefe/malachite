// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

extern crate criterion;
extern crate malachite_base;
extern crate malachite_nz;
extern crate num;
extern crate rug;

use criterion::*;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::get_random_natural_with_bits;
use num::BigUint;
use std::str::FromStr;

pub fn natural_to_biguint(n: &Natural) -> BigUint {
    BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn natural_to_rug_integer(n: &Natural) -> rug::Integer {
    rug::Integer::from_str(n.to_string().as_ref()).unwrap()
}

fn bench_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("Natural * Natural");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    let sizes = [
        1u64, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
    ];
    for &i in sizes.iter() {
        let x = get_random_natural_with_bits(&mut random_primitive_ints(EXAMPLE_SEED.fork("a")), i);
        let y = get_random_natural_with_bits(&mut random_primitive_ints(EXAMPLE_SEED.fork("b")), i);
        let x_num = natural_to_biguint(&x);
        let y_num = natural_to_biguint(&y);
        let x_rug = natural_to_rug_integer(&x);
        let y_rug = natural_to_rug_integer(&y);
        group.bench_function(BenchmarkId::new("malachite", i), |b| {
            b.iter_with_setup(|| (x.clone(), y.clone()), |(x, y)| x * y)
        });
        group.bench_function(BenchmarkId::new("num", i), |b| {
            b.iter_with_setup(|| (x_num.clone(), y_num.clone()), |(x, y)| x * y)
        });
        group.bench_function(BenchmarkId::new("rug", i), |b| {
            b.iter_with_setup(|| (x_rug.clone(), y_rug.clone()), |(x, y)| x * y)
        });
    }
    group.finish();
}
criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = bench_mul
}
criterion_main!(benches);
