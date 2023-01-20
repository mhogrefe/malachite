extern crate criterion;
extern crate malachite_base;
extern crate malachite_nz;
extern crate num;
extern crate rug;

use criterion::*;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::random::get_random_natural_with_bits;
use malachite_nz::natural::Natural;
use num::BigUint;
use std::str::FromStr;

pub fn BigUint::from(n: &Natural) -> BigUint {
    BigUint::from_str(n.to_string().as_ref()).unwrap()
}

pub fn rug::Integer::from(n: &Natural) -> rug::Integer {
    rug::Integer::from_str(n.to_string().as_ref()).unwrap()
}

fn bench_div(c: &mut Criterion) {
    let mut group = c.benchmark_group("Natural / Natural");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for &i in [1u64, 10, 100, 1000, 10000, 100000, 1000000, 10000000].iter() {
        let x = get_random_natural_with_bits(
            &mut random_primitive_ints(EXAMPLE_SEED.fork("a")),
            i << 1,
        );
        let y = get_random_natural_with_bits(&mut random_primitive_ints(EXAMPLE_SEED.fork("b")), i);
        let x_num = BigUint::from(&x);
        let y_num = BigUint::from(&y);
        let x_rug = rug::Integer::from(&x);
        let y_rug = rug::Integer::from(&y);
        group.bench_function(BenchmarkId::new("malachite", i), |b| {
            b.iter_with_setup(|| (x.clone(), y.clone()), |(x, y)| x / y)
        });
        group.bench_function(BenchmarkId::new("num", i), |b| {
            b.iter_with_setup(|| (x_num.clone(), y_num.clone()), |(x, y)| x / y)
        });
        group.bench_function(BenchmarkId::new("rug", i), |b| {
            b.iter_with_setup(|| (x_rug.clone(), y_rug.clone()), |(x, y)| x / y)
        });
    }
    group.finish();
}
