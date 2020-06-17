#[cfg(not(feature = "32_bit_limbs"))]
use malachite_base::num::conversion::traits::VecFromOtherTypeSlice;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::random::special_random_natural_up_to_bits::*;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rand::{IsaacRng, SeedableRng, StdRng};
use rust_wheels::iterators::common::EXAMPLE_SEED;

use malachite_test::common::test_properties_no_special;
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
    test(1, &[0]);
    test(2, &[2]);
    test(3, &[2]);
    test(4, &[10]);
    test(5, &[10]);
    #[cfg(feature = "32_bit_limbs")]
    {
        test(10, &[305]);
        test(32, &[4_294_963_312]);
        test(100, &[1_073_741_799, 66_060_288, 4_294_443_008, 15]);
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(10, &[797]);
        test(32, &[2_113_966_064]);
        test(100, &[283_726_777_598_083_047, 68_718_952_448]);
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
    test(1, "0");
    test(2, "10");
    test(3, "10");
    test(4, "1010");
    test(5, "1010");
    test(10, "100110001");
    test(32, "11111111111111111111000001110000");
    test(
        100,
        "111111111111111110000000000000000000000000111111000000000000000000000011111111111111111111\
        1111100111",
    );
}

#[test]
fn limbs_special_random_up_to_bits_properties() {
    let mut rng = IsaacRng::from_seed(&EXAMPLE_SEED);
    test_properties_no_special(small_positive_unsigneds, |&bits| {
        let mut cloned_rng = rng.clone();
        let random_limbs: Vec<u32> = limbs_special_random_up_to_bits(&mut rng, bits);
        #[cfg(not(feature = "32_bit_limbs"))]
        let random_limbs = Limb::vec_from_other_type_slice(&random_limbs);
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
