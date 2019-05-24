use common::test_properties_no_special;
use malachite_base::num::traits::SignificantBits;
use malachite_nz::natural::random::special_random_natural_with_bits::*;
use malachite_test::inputs::base::small_unsigneds;
use rand::{IsaacRng, SeedableRng, StdRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;

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
    test(2, "11");
    test(3, "101");
    test(4, "1101");
    test(5, "10101");
    test(10, "1010011000");
    test(32, "11111111111111111111100000111000");
    test(
        100,
        "111111111111111111000000000000000000000000011111100000000000000000000001111111111111111111\
        1111110011",
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
