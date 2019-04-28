use common::test_properties;
use malachite_base::num::traits::Zero;
use malachite_nz::natural::random::random_natural_below::random_natural_below;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::positive_naturals;
use rand::{IsaacRng, SeedableRng, StdRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use std::str::FromStr;

#[test]
fn test_random_natural_below() {
    let test = |n, out| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let x = random_natural_below(&mut rng, &Natural::from_str(n).unwrap());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test("1", "0");
    test("10", "2");
    test("1000000", "293069");
    test("1000000000000", "883031013170");
}

#[test]
#[should_panic]
fn random_natural_below_fail() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    random_natural_below(&mut rng, &Natural::ZERO);
}

#[test]
fn random_natural_below_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties(positive_naturals, |n| {
        let x = random_natural_below(&mut rng, n);
        assert!(x.is_valid());
        assert!(x < *n);
    });
}
