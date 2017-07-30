use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::gmp_natural_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::vecs::{exhaustive_vecs, random_vecs};

#[test]
fn test_from_limbs_le() {
    let test = |limbs: &[u32], out| {
        let x = native::Natural::from_limbs_le(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Natural::from_limbs_le(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[123, 0], "123");
    test(&[123, 0, 0, 0], "123");
    test(&[3567587328, 232], "1000000000000");
    test(&[3567587328, 232, 0], "1000000000000");
    test(&[1, 2, 3, 4, 5], "1701411834921604967429270619762735448065");
}

#[test]
fn test_from_limbs_be() {
    let test = |limbs: Vec<u32>, out| {
        let x = native::Natural::from_limbs_be(&limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Natural::from_limbs_be(&limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(vec![], "0");
    test(vec![0], "0");
    test(vec![0, 0, 0], "0");
    test(vec![123], "123");
    test(vec![0, 123], "123");
    test(vec![0, 0, 0, 123], "123");
    test(vec![232, 3567587328], "1000000000000");
    test(vec![0, 232, 3567587328], "1000000000000");
    test(vec![5, 4, 3, 2, 1],
         "1701411834921604967429270619762735448065");
}

#[test]
fn from_limbs_le_properties() {
    // Natural::from_limbs_le(limbs) is equivalent for malachite-gmp and malachite-native.
    // (Natural::from_limbs_le(limbs) == x) ==
    //      (x.limbs_le() == limbs.rev().skip_while(|u| u == 0).rev())
    // Natural::from_limbs_le(limbs.reverse()) == Natural::from_limbs_be(limbs)
    // if !limbs.is_empty() and limbs.last() != 0, Natural::from_limbs_le(limbs).limbs_le() == x
    let u32_slice = |limbs: &[u32]| {
        let x = native::Natural::from_limbs_le(limbs);
        assert_eq!(gmp_natural_to_native(&gmp::Natural::from_limbs_le(limbs)),
                   x);
        let mut trimmed_limbs: Vec<u32> = limbs.iter()
            .cloned()
            .rev()
            .skip_while(|&u| u == 0)
            .collect();
        trimmed_limbs.reverse();
        assert_eq!(x.limbs_le(), trimmed_limbs);
        assert_eq!(native::Natural::from_limbs_be(&limbs.iter()
                                                       .cloned()
                                                       .rev()
                                                       .collect::<Vec<u32>>()),
                   x);
        if !limbs.is_empty() && *limbs.last().unwrap() != 0 {
            assert_eq!(&x.limbs_le()[..], limbs);
        }
    };

    for xs in exhaustive_vecs(exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        u32_slice(&xs);
    }

    for xs in random_vecs(&EXAMPLE_SEED, 32, &(|seed| random_x::<u32>(seed))).take(LARGE_LIMIT) {
        u32_slice(&xs);
    }
}

#[test]
fn from_limbs_be_properties() {
    // Natural::from_limbs_be(limbs) is equivalent for malachite-gmp and malachite-native.
    // (Natural::from_limbs_be(limbs) == x) == (x.limbs_be() == limbs.skip_while(|u| u == 0))
    // Natural::from_limbs_be(limbs.reverse()) == Natural::from_limbs_le(limbs)
    // if !limbs.is_empty() and limbs[0] != 0, Natural::from_limbs_be(limbs).limbs_le() == x
    let u32_slice = |limbs: &[u32]| {
        let x = native::Natural::from_limbs_be(limbs);
        assert_eq!(gmp_natural_to_native(&gmp::Natural::from_limbs_be(limbs)),
                   x);
        assert_eq!(x.limbs_be(),
                   limbs.iter()
                       .cloned()
                       .skip_while(|&u| u == 0)
                       .collect::<Vec<u32>>());
        assert_eq!(native::Natural::from_limbs_le(&limbs.iter()
                                                       .cloned()
                                                       .rev()
                                                       .collect::<Vec<u32>>()),
                   x);
        if !limbs.is_empty() && limbs[0] != 0 {
            assert_eq!(&x.limbs_be()[..], limbs);
        }
    };

    for xs in exhaustive_vecs(exhaustive_u::<u32>()).take(LARGE_LIMIT) {
        u32_slice(&xs);
    }

    for xs in random_vecs(&EXAMPLE_SEED, 32, &(|seed| random_x::<u32>(seed))).take(LARGE_LIMIT) {
        u32_slice(&xs);
    }
}
