use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::random::special_random_natural_with_bits::*;
use rand::{IsaacRng, SeedableRng, StdRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;

use malachite_test::common::test_properties_no_special;
use malachite_test::inputs::base::small_unsigneds;

#[test]
fn test_special_random_natural_with_bits() {
    let test = |bits, out| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let x = special_random_natural_with_bits(&mut rng, bits);
        assert_eq!(format!("{:b}", x), out);
        assert!(x.is_valid());
    };
    test(1, "1");
    test(2, "10");
    test(3, "110");
    test(4, "1010");
    test(5, "11010");
    test(10, "1100110001");
    test(32, "11111111111111111111000001110000");
    test(
        100,
        "111111111111111110000000000000000000000000111111000000000000000000000011111111111111111111\
        1111100111",
    );
}

#[test]
fn special_random_natural_with_bits_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties_no_special(small_unsigneds, |&bits| {
        let n = special_random_natural_with_bits(&mut rng, bits);
        assert!(n.is_valid());
        assert_eq!(n.significant_bits(), bits);
    });
}
