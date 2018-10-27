use common::test_properties_no_special;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::random::random_natural_up_to_bits::{
    limbs_random_up_to_bits, random_natural_up_to_bits,
};
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::{small_positive_unsigneds, small_unsigneds};
use rand::{IsaacRng, SeedableRng, StdRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;

#[test]
fn test_limbs_random_up_to_bits() {
    let test = |bits, out: &[u32]| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        assert_eq!(limbs_random_up_to_bits(&mut rng, bits), out);
    };
    test(1, &[0]);
    test(2, &[2]);
    test(3, &[2]);
    test(4, &[2]);
    test(5, &[18]);
    test(10, &[818]);
    test(32, &[2_562_717_490]);
    test(100, &[2_562_717_490, 103_053_517, 1_930_352_495, 10]);
}

#[test]
#[should_panic(expected = "assertion failed: `(left != right)")]
fn limbs_random_up_to_bits_fail() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    limbs_random_up_to_bits(&mut rng, 0);
}

#[test]
fn test_random_natural_up_to_bits() {
    let test = |bits, out| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let x = random_natural_up_to_bits(&mut rng, bits);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(1, "0");
    test(2, "2");
    test(3, "2");
    test(4, "2");
    test(5, "18");
    test(10, "818");
    test(32, "2562717490");
    test(100, "827890343590397684290531882802");
}

#[test]
fn limbs_random_up_to_bits_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties_no_special(small_positive_unsigneds, |&bits| {
        let mut cloned_rng = rng.clone();
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_random_up_to_bits(&mut rng, bits)),
            random_natural_up_to_bits(&mut cloned_rng, bits)
        );
    });
}

#[test]
fn random_natural_up_to_bits_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties_no_special(small_unsigneds, |&bits| {
        let n = random_natural_up_to_bits(&mut rng, bits);
        assert!(n.is_valid());
        assert!(n.significant_bits() <= bits);
    });
}
