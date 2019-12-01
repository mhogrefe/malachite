#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::eq_limb_mod_limb::{
    _combined_limbs_eq_limb_mod_limb, limbs_eq_limb_mod_limb,
};
use malachite_nz::natural::arithmetic::mod_limb::limbs_mod_limb;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
#[cfg(feature = "32_bit_limbs")]
use rug;

use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::base::{
    pairs_of_unsigneds, triples_of_limb_vec_limb_and_positive_limb_var_3,
    triples_of_limb_vec_limb_and_positive_limb_var_4,
    triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1, triples_of_unsigneds,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_unsigned, triples_of_natural_limb_and_limb_var_2,
    triples_of_natural_unsigned_and_unsigned, triples_of_natural_unsigned_and_unsigned_var_1,
    triples_of_unsigned_unsigned_and_natural,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_limb_mod_limb() {
    let test = |limbs: &[Limb], limb: Limb, modulus: Limb, equal: bool| {
        assert_eq!(limbs_eq_limb_mod_limb(limbs, limb, modulus), equal);
        assert_eq!(limbs_mod_limb(limbs, modulus) == limb % modulus, equal);
        assert_eq!(
            _combined_limbs_eq_limb_mod_limb(limbs, limb, modulus),
            equal
        );
    };
    test(&[6, 7], 4, 2, true);
    test(&[7, 7], 4, 2, false);
    test(&[6, 7], 3, 2, false);
    test(&[7, 7], 3, 2, true);
    test(&[2, 2], 7, 13, true);
    test(&[100, 101, 102], 1_238, 10, true);
    test(&[100, 101, 102], 1_239, 10, false);
    test(&[123, 456], 636, 789, true);
    test(&[123, 456], 1_000, 789, false);
    test(&[0xffff_ffff, 0xffff_ffff], 101, 2, true);
    test(&[0xffff_ffff, 0xffff_ffff], 100, 2, false);
    test(&[0xffff_ffff, 0xffff_ffff], 120, 3, true);
    test(&[0xffff_ffff, 0xffff_ffff], 110, 3, false);
    test(
        &[
            957355272, 2717966866, 2284391330, 238149753, 3607703304, 23463007, 1388955612,
            3269479240, 881285075, 2493741919, 360635652, 2851492229, 3590429614, 2528168680,
            215334077, 3509222230, 1825157855, 3737409852, 4151389929, 2692167062, 1409227805,
            2060445344, 1453537438, 3186146035, 1159656442, 954576963, 2935313630, 2288694644,
            400433986, 3182217800, 3929694465, 3346806449, 131165877,
        ],
        1529684314,
        1469269654,
        false,
    );
    test(
        &[
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            511, 0, 0, 0, 4227858432, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            511, 0, 0, 0, 3221225472, 63, 0, 0, 0, 0, 0, 0, 0, 4294443008, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295,
        ],
        4294963200,
        4294967295,
        false,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_limb_fail_1() {
    limbs_eq_limb_mod_limb(&[10], 10, 15);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_limb_fail_2() {
    limbs_eq_limb_mod_limb(&[6, 7], 4, 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_eq_limb_mod_limb() {
    let test = |n, u, modulus, out| {
        assert_eq!(Natural::from_str(n).unwrap().eq_mod(u, modulus), out);
        assert_eq!(u.eq_mod(&Natural::from_str(n).unwrap(), modulus), out);
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .is_congruent_u(u, modulus),
            out
        );
    };
    test("0", 0, 0, true);
    test("0", 1, 0, false);
    test("57", 57, 0, true);
    test("57", 58, 0, false);
    test("1000000000000", 57, 0, false);
    test("0", 256, 256, true);
    test("0", 256, 512, false);
    test("13", 23, 10, true);
    test("13", 24, 10, false);
    test("13", 21, 1, true);
    test("13", 21, 2, true);
    test("13", 21, 4, true);
    test("13", 21, 8, true);
    test("13", 21, 16, false);
    test("13", 21, 3, false);
    test("1000000000001", 1, 4_096, true);
    test("1000000000001", 1, 8_192, false);
    test("12345678987654321", 321, 1_000, true);
    test("12345678987654321", 322, 1_000, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limb_eq_limb_mod_natural() {
    let test = |u: u32, v, modulus, out| {
        assert_eq!(u.eq_mod(v, &Natural::from_str(modulus).unwrap()), out);
    };
    test(0, 0, "0", true);
    test(0, 1, "0", false);
    test(57, 57, "0", true);
    test(57, 58, "0", false);
    test(57, 57, "1000000000000", true);
    test(57, 58, "1000000000000", false);
    test(0, 256, "256", true);
    test(0, 256, "512", false);
    test(13, 23, "10", true);
    test(13, 24, "10", false);
    test(13, 21, "1", true);
    test(13, 21, "2", true);
    test(13, 21, "4", true);
    test(13, 21, "8", true);
    test(13, 21, "16", false);
    test(13, 21, "3", false);
}

#[test]
fn limbs_eq_limb_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
        |&(ref limbs, limb, modulus)| {
            let equal = limbs_eq_limb_mod_limb(limbs, limb, modulus);
            assert_eq!(Natural::from_limbs_asc(limbs).eq_mod(limb, modulus), equal);
            assert_eq!(limbs_mod_limb(limbs, modulus) == limb % modulus, equal);
            assert_eq!(
                _combined_limbs_eq_limb_mod_limb(limbs, limb, modulus),
                equal
            );
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_positive_limb_var_3,
        |&(ref limbs, limb, modulus)| {
            assert!(limbs_eq_limb_mod_limb(limbs, limb, modulus));
            assert!(Natural::from_limbs_asc(limbs).eq_mod(limb, modulus));
            assert_eq!(limbs_mod_limb(limbs, modulus), limb % modulus);
            assert!(_combined_limbs_eq_limb_mod_limb(limbs, limb, modulus));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_positive_limb_var_4,
        |&(ref limbs, limb, modulus)| {
            assert!(!limbs_eq_limb_mod_limb(limbs, limb, modulus));
            assert!(!Natural::from_limbs_asc(limbs).eq_mod(limb, modulus));
            assert_ne!(limbs_mod_limb(limbs, modulus), limb % modulus);
            assert!(!_combined_limbs_eq_limb_mod_limb(limbs, limb, modulus));
        },
    );
}

#[test]
fn eq_limb_mod_limb_properties() {
    test_properties(
        triples_of_natural_unsigned_and_unsigned,
        |&(ref n, u, modulus): &(Natural, Limb, Limb)| {
            let equal = n.eq_mod(u, modulus);
            assert_eq!(u.eq_mod(n, modulus), equal);
            assert_eq!(*n == u || modulus != 0 && n % modulus == u % modulus, equal);

            //TODO assert_eq!(n.eq_mod(Natural::from(u), modulus), equal);

            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(natural_to_rug_integer(n).is_congruent_u(u, modulus), equal);
        },
    );

    test_properties(
        triples_of_natural_unsigned_and_unsigned_var_1,
        |&(ref n, u, modulus): &(Natural, Limb, Limb)| {
            assert!(n.eq_mod(u, modulus));
            assert!(u.eq_mod(n, modulus));
            assert!(*n == u || modulus != 0 && n % modulus == u % modulus);

            //TODO assert!(n.eq_mod(Natural::from(u), modulus));

            #[cfg(feature = "32_bit_limbs")]
            assert!(natural_to_rug_integer(n).is_congruent_u(u, modulus));
        },
    );

    test_properties(
        triples_of_natural_limb_and_limb_var_2,
        |&(ref n, u, modulus): &(Natural, Limb, Limb)| {
            assert!(!n.eq_mod(u, modulus));
            assert!(!u.eq_mod(n, modulus));
            assert!(*n != u && (modulus == 0 || n % modulus != u % modulus));

            //TODO assert!(!n.eq_mod(Natural::from(u), modulus));

            #[cfg(feature = "32_bit_limbs")]
            assert!(!natural_to_rug_integer(n).is_congruent_u(u, modulus));
        },
    );

    test_properties(pairs_of_natural_and_unsigned::<Limb>, |&(ref n, u)| {
        assert!(n.eq_mod(u, 1));
        assert!(u.eq_mod(n, 1));
        assert_eq!(n.eq_mod(0, u), n.divisible_by(u));
        assert_eq!(0.eq_mod(n, u), n.divisible_by(u));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(u, modulus)| {
        assert!(Natural::from(u).eq_mod(u, modulus));
        assert!(u.eq_mod(&Natural::from(u), modulus));
        assert_eq!(Natural::ZERO.eq_mod(u, modulus), u.divisible_by(modulus));
        assert_eq!(u.eq_mod(&Natural::ZERO, modulus), u.divisible_by(modulus));
    });

    test_properties(triples_of_unsigneds::<Limb>, |&(u, v, modulus)| {
        let equal = u.eq_mod(v, modulus);
        assert_eq!(Natural::from(u).eq_mod(v, modulus), equal);
        assert_eq!(EqMod::eq_mod(u, &Natural::from(v), modulus), equal);
    });
}

#[test]
fn limb_eq_limb_mod_natural_properties() {
    test_properties(
        triples_of_unsigned_unsigned_and_natural,
        |&(u, v, ref modulus): &(Limb, Limb, Natural)| {
            let equal = u.eq_mod(v, modulus);
            assert_eq!(v.eq_mod(u, modulus), equal);
            assert_eq!(
                u == v || *modulus != 0 as Limb && u % modulus == v % modulus,
                equal
            );

            //TODO assert_eq!(Natural::from(u).eq_mod(v, modulus), equal);
        },
    );

    test_properties(pairs_of_natural_and_unsigned::<Limb>, |&(ref n, u)| {
        assert_eq!(u.eq_mod(0, n), u.divisible_by(n));
        assert_eq!(0.eq_mod(u, n), u.divisible_by(n));
        assert!(u.eq_mod(u, n));
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(u, v)| {
        assert!(u.eq_mod(v, &Natural::ONE));
        assert!(v.eq_mod(u, &Natural::ONE));
        assert_eq!(u.eq_mod(v, &Natural::ZERO), u == v);
        assert_eq!(v.eq_mod(u, &Natural::ZERO), u == v);
    });

    test_properties(triples_of_unsigneds::<Limb>, |&(u, v, modulus)| {
        let equal = u.eq_mod(v, modulus);
        assert_eq!(EqMod::eq_mod(u, v, &Natural::from(modulus)), equal);
    });
}
