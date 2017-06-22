use malachite::natural::Natural;
use malachite::natural::random::random_natural_below::random_natural_below;
use rand::{SeedableRng, StdRng};
use std::str::FromStr;

#[test]
fn test_random_below() {
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
