use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base_test_util::common::test_cmp_helper;
use malachite_base_test_util::generators::common::GenConfig;
use malachite_base_test_util::generators::{
    unsigned_pair_gen_var_27, unsigned_vec_gen, unsigned_vec_gen_var_3,
    unsigned_vec_pair_gen_var_6, unsigned_vec_pair_gen_var_7, unsigned_vec_triple_gen_var_29,
    unsigned_vec_triple_gen_var_30,
};
use malachite_nz::natural::comparison::cmp::{limbs_cmp, limbs_cmp_same_length};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_nz_test_util::generators::{natural_gen, natural_pair_gen, natural_triple_gen};
use num::BigUint;
use rug;
use std::cmp::Ordering;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_cmp_same_length() {
    let test = |xs: &[Limb], ys: &[Limb], out| {
        assert_eq!(limbs_cmp_same_length(xs, ys), out);
    };
    test(&[3], &[5], Ordering::Less);
    test(&[3, 0], &[5, 0], Ordering::Less);
    test(&[1, 2], &[2, 1], Ordering::Greater);
    test(&[1, 2, 3], &[1, 2, 3], Ordering::Equal);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_cmp_same_length_fail() {
    limbs_cmp_same_length(&[1], &[2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_cmp() {
    let test = |xs: &[Limb], ys: &[Limb], out| {
        assert_eq!(limbs_cmp(xs, ys), out);
    };
    test(&[3], &[5], Ordering::Less);
    test(&[3, 1], &[5], Ordering::Greater);
    test(&[1, 2], &[2, 1, 3], Ordering::Less);
    test(&[1, 2, 3], &[1, 2, 3], Ordering::Equal);
}

#[test]
fn test_cmp() {
    let strings = vec!["0", "1", "2", "123", "999999999999", "1000000000000", "1000000000001"];
    test_cmp_helper::<Natural>(&strings);
    test_cmp_helper::<BigUint>(&strings);
    test_cmp_helper::<rug::Integer>(&strings);
}

#[test]
fn limbs_cmp_same_length_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_6().test_properties_with_config(&config, |(xs, ys)| {
        let cmp = limbs_cmp_same_length(&xs, &ys);
        assert_eq!(
            Natural::from_limbs_asc(&xs).cmp(&Natural::from_limbs_asc(&ys)),
            cmp
        );
        assert_eq!(limbs_cmp_same_length(&ys, &xs).reverse(), cmp);
    });

    unsigned_vec_gen().test_properties_with_config(&config, |xs| {
        assert_eq!(limbs_cmp_same_length(&xs, &xs), Ordering::Equal);
    });

    unsigned_vec_triple_gen_var_29().test_properties_with_config(&config, |(xs, ys, zs)| {
        if limbs_cmp_same_length(&xs, &ys) == Ordering::Less
            && limbs_cmp_same_length(&ys, &zs) == Ordering::Less
        {
            assert_eq!(limbs_cmp_same_length(&xs, &zs), Ordering::Less);
        } else if limbs_cmp_same_length(&xs, &ys) == Ordering::Greater
            && limbs_cmp_same_length(&ys, &zs) == Ordering::Greater
        {
            assert_eq!(limbs_cmp_same_length(&xs, &zs), Ordering::Greater);
        }
    });
}

#[test]
fn limbs_cmp_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_7().test_properties_with_config(&config, |(xs, ys)| {
        let cmp = limbs_cmp(&xs, &ys);
        assert_eq!(
            Natural::from_limbs_asc(&xs).cmp(&Natural::from_limbs_asc(&ys)),
            cmp
        );
        assert_eq!(limbs_cmp(&ys, &xs).reverse(), cmp);
    });

    unsigned_vec_gen_var_3().test_properties_with_config(&config, |xs| {
        assert_eq!(limbs_cmp(&xs, &xs), Ordering::Equal);
    });

    unsigned_vec_triple_gen_var_30().test_properties_with_config(&config, |(xs, ys, zs)| {
        if limbs_cmp(&xs, &ys) == Ordering::Less && limbs_cmp(&ys, &zs) == Ordering::Less {
            assert_eq!(limbs_cmp(&xs, &zs), Ordering::Less);
        } else if limbs_cmp(&xs, &ys) == Ordering::Greater
            && limbs_cmp(&ys, &zs) == Ordering::Greater
        {
            assert_eq!(limbs_cmp(&xs, &zs), Ordering::Greater);
        }
    });
}

#[test]
fn cmp_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let cmp = x.cmp(&y);
        assert_eq!(natural_to_biguint(&x).cmp(&natural_to_biguint(&y)), cmp);
        assert_eq!(
            natural_to_rug_integer(&x).cmp(&natural_to_rug_integer(&y)),
            cmp
        );
        assert_eq!(y.cmp(&x).reverse(), cmp);
        assert_eq!((-y).cmp(&(-x)), cmp);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(x.cmp(&x), Ordering::Equal);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x).cmp(&Natural::from(y)), x.cmp(&y));
    });
}
