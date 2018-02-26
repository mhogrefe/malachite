use common::test_properties_no_special;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::random::special_random_natural_with_bits::*;
use malachite_test::inputs::base::small_u64s;
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
    test(10, "1001101110");
    test(32, "11111111111111111110000011100000");
    test(
        100,
        "111111111000000000000000000000000001111111111000000000110000000000000000000000000000111111\
        1111111111",
    );
}

#[test]
fn special_random_natural_with_bits_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties_no_special(small_u64s, |&bits| {
        let n = special_random_natural_with_bits(&mut rng, bits);
        assert!(n.is_valid());
        assert_eq!(n.significant_bits(), bits);
    });
}
