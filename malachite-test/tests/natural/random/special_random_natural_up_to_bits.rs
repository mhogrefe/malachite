use malachite_base::num::traits::SignificantBits;
#[cfg(feature = "64_bit_limbs")]
use malachite_nz::natural::random::random_natural_up_to_bits::_transform_32_to_64_bit_limbs;
use malachite_nz::natural::random::special_random_natural_up_to_bits::*;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rand::{IsaacRng, SeedableRng, StdRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;

use common::test_properties_no_special;
use malachite_test::inputs::base::{small_positive_unsigneds, small_unsigneds};

#[test]
fn test_limbs_special_random_up_to_bits() {
    let test = |bits, out: &[Limb]| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        assert_eq!(
            limbs_special_random_up_to_bits::<Limb, _>(&mut rng, bits),
            out
        );
    };
    test(1, &[1]);
    test(2, &[1]);
    test(3, &[5]);
    test(4, &[5]);
    test(5, &[21]);
    #[cfg(feature = "32_bit_limbs")]
    {
        test(10, &[152]);
        test(32, &[4_294_965_304]);
        test(100, &[536_870_899, 33_030_144, 4_294_705_152, 15]);
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test(10, &[910]);
        test(32, &[1_056_983_032]);
        test(100, &[141_863_388_799_041_523, 68_719_214_592]);
    }
}

#[test]
#[should_panic]
fn limbs_special_random_up_to_bits_fail() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    limbs_special_random_up_to_bits::<Limb, _>(&mut rng, 0);
}

#[test]
fn test_special_random_natural_up_to_bits() {
    let test = |bits, out| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let x = special_random_natural_up_to_bits(&mut rng, bits);
        assert_eq!(format!("{:b}", x), out);
        assert!(x.is_valid());
    };
    test(1, "1");
    test(2, "1");
    test(3, "101");
    test(4, "101");
    test(5, "10101");
    test(10, "10011000");
    test(32, "11111111111111111111100000111000");
    test(
        100,
        "111111111111111111000000000000000000000000011111100000000000000000000001111111111111111111\
        1111110011",
    );
}

#[test]
fn limbs_special_random_up_to_bits_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties_no_special(small_positive_unsigneds, |&bits| {
        let mut cloned_rng = rng.clone();
        #[cfg(feature = "32_bit_limbs")]
        let random_limbs = limbs_special_random_up_to_bits(&mut rng, bits);
        #[cfg(feature = "64_bit_limbs")]
        let random_limbs =
            _transform_32_to_64_bit_limbs(&limbs_special_random_up_to_bits(&mut rng, bits));
        assert_eq!(
            Natural::from_owned_limbs_asc(random_limbs),
            special_random_natural_up_to_bits(&mut cloned_rng, bits)
        );
    });
}

#[test]
fn special_random_natural_up_to_bits_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties_no_special(small_unsigneds, |&bits| {
        let n = special_random_natural_up_to_bits(&mut rng, bits);
        assert!(n.is_valid());
        assert!(n.significant_bits() <= bits);
    });
}
