// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::common::test_cmp_helper;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_27, unsigned_vec_gen, unsigned_vec_gen_var_3,
    unsigned_vec_pair_gen_var_19, unsigned_vec_pair_gen_var_6, unsigned_vec_pair_gen_var_7,
    unsigned_vec_triple_gen_var_29, unsigned_vec_triple_gen_var_30,
};
use malachite_nz::natural::comparison::cmp::{
    limbs_cmp, limbs_cmp_normalized, limbs_cmp_same_length,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_gen_var_2, natural_pair_gen, natural_pair_gen_var_9, natural_triple_gen,
    natural_triple_gen_var_6,
};
use malachite_nz::test_util::natural::comparison::cmp::natural_cmp_normalized_naive;
use num::BigUint;
use rug;
use std::cmp::Ordering::*;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_cmp_same_length() {
    let test = |xs: &[Limb], ys: &[Limb], out| {
        assert_eq!(limbs_cmp_same_length(xs, ys), out);
    };
    test(&[3], &[5], Less);
    test(&[3, 0], &[5, 0], Less);
    test(&[1, 2], &[2, 1], Greater);
    test(&[1, 2, 3], &[1, 2, 3], Equal);
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
    test(&[3], &[5], Less);
    test(&[3, 1], &[5], Greater);
    test(&[1, 2], &[2, 1, 3], Less);
    test(&[1, 2, 3], &[1, 2, 3], Equal);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_cmp_normalized() {
    let test = |xs: &[Limb], ys: &[Limb], out| {
        assert_eq!(limbs_cmp_normalized(xs, ys), out);
    };
    test(&[5], &[6], Less);
    test(&[1], &[8], Equal);
    test(&[0, 0, 1], &[8], Equal);
    test(&[17], &[3], Less);
    test(&[1, 1, 1], &[1, 1], Greater);
    test(&[1, 0, 1], &[1, 1], Less);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_cmp_normalized_fail_1() {
    limbs_cmp_normalized(&[], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_cmp_normalized_fail_2() {
    limbs_cmp_normalized(&[1, 2, 3], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_cmp_normalized_fail_3() {
    limbs_cmp_normalized(&[1, 0], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_cmp_normalized_fail_4() {
    limbs_cmp_normalized(&[1, 2, 3], &[1, 0]);
}

#[test]
fn test_cmp() {
    let strings = vec!["0", "1", "2", "123", "999999999999", "1000000000000", "1000000000001"];
    test_cmp_helper::<Natural>(&strings);
    test_cmp_helper::<BigUint>(&strings);
    test_cmp_helper::<rug::Integer>(&strings);
}

#[test]
fn test_cmp_normalized() {
    let test = |x, y, out| {
        let x = Natural::from_str(x).unwrap();
        let y = Natural::from_str(y).unwrap();
        assert_eq!(x.cmp_normalized(&y), out);
        assert_eq!(natural_cmp_normalized_naive(&x, &y), out);
    };
    test("1", "4", Equal);
    test("5", "6", Less);
    test("3", "17", Greater);
    test("9", "36", Equal);
    test("117886223846050103296", "409", Equal);
    test("117886223846050103295", "409", Less);
    test("117886223846050103297", "409", Greater);
}

#[test]
#[should_panic]
fn cmp_normalized_fail_1() {
    Natural::ZERO.cmp_normalized(&Natural::ONE);
}

#[test]
#[should_panic]
fn cmp_normalized_fail_2() {
    Natural::ONE.cmp_normalized(&Natural::ZERO);
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
        assert_eq!(limbs_cmp_same_length(&xs, &xs), Equal);
    });

    unsigned_vec_triple_gen_var_29().test_properties_with_config(&config, |(xs, ys, zs)| {
        if limbs_cmp_same_length(&xs, &ys) == Less && limbs_cmp_same_length(&ys, &zs) == Less {
            assert_eq!(limbs_cmp_same_length(&xs, &zs), Less);
        } else if limbs_cmp_same_length(&xs, &ys) == Greater
            && limbs_cmp_same_length(&ys, &zs) == Greater
        {
            assert_eq!(limbs_cmp_same_length(&xs, &zs), Greater);
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
        assert_eq!(limbs_cmp(&xs, &xs), Equal);
    });

    unsigned_vec_triple_gen_var_30().test_properties_with_config(&config, |(xs, ys, zs)| {
        if limbs_cmp(&xs, &ys) == Less && limbs_cmp(&ys, &zs) == Less {
            assert_eq!(limbs_cmp(&xs, &zs), Less);
        } else if limbs_cmp(&xs, &ys) == Greater && limbs_cmp(&ys, &zs) == Greater {
            assert_eq!(limbs_cmp(&xs, &zs), Greater);
        }
    });
}

#[test]
fn limbs_cmp_normalized_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_19().test_properties_with_config(&config, |(xs, ys)| {
        let cmp = limbs_cmp_normalized(&xs, &ys);
        assert_eq!(
            Natural::from_limbs_asc(&xs).cmp_normalized(&Natural::from_limbs_asc(&ys)),
            cmp
        );
        assert_eq!(limbs_cmp_normalized(&ys, &xs).reverse(), cmp);
    });
}

#[test]
fn cmp_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let cmp = x.cmp(&y);
        assert_eq!(BigUint::from(&x).cmp(&BigUint::from(&y)), cmp);
        assert_eq!(rug::Integer::from(&x).cmp(&rug::Integer::from(&y)), cmp);
        assert_eq!(y.cmp(&x).reverse(), cmp);
        assert_eq!(x == y, x.cmp(&y) == Equal);
        assert_eq!((-y).cmp(&(-x)), cmp);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(x.cmp(&x), Equal);
        assert!(x >= Natural::ZERO);
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

#[test]
fn cmp_normalized_properties() {
    natural_pair_gen_var_9().test_properties(|(x, y)| {
        let cmp = x.cmp_normalized(&y);
        assert_eq!(natural_cmp_normalized_naive(&x, &y), cmp);
        assert_eq!(y.cmp_normalized(&x).reverse(), cmp);
    });

    natural_gen_var_2().test_properties(|x| {
        assert_eq!(x.cmp_normalized(&x), Equal);
    });

    natural_triple_gen_var_6().test_properties(|(x, y, z)| {
        if x.cmp_normalized(&y) == Less && y.cmp_normalized(&z) == Less {
            assert_eq!(x.cmp_normalized(&z), Less);
        } else if x.cmp_normalized(&y) == Greater && y.cmp_normalized(&z) == Greater {
            assert_eq!(x.cmp_normalized(&z), Greater);
        }
    });
}
