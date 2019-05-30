use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::random::random_natural_with_bits::random_natural_with_bits;
use rand::{IsaacRng, SeedableRng, StdRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;

use common::test_properties_no_special;
use malachite_test::inputs::base::small_unsigneds;

#[test]
fn test_random_natural_with_bits() {
    let test = |bits, out| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let x = random_natural_with_bits(&mut rng, bits);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(1, "1");
    test(2, "2");
    test(3, "6");
    test(4, "10");
    test(5, "18");
    test(10, "818");
    test(32, "2562717490");
    test(100, "827890343590397684290531882802");
}

#[test]
fn random_natural_with_bits_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties_no_special(small_unsigneds, |&bits| {
        let n = random_natural_with_bits(&mut rng, bits);
        assert!(n.is_valid());
        assert_eq!(n.significant_bits(), bits);
    });
}
