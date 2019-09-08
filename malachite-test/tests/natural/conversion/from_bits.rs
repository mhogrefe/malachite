use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_nz::natural::conversion::from_bits::{
    limbs_asc_from_bits_asc, limbs_asc_from_bits_desc,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::vecs_of_bool;

#[test]
fn test_limbs_asc_from_bits_asc() {
    let test = |bits: &[bool], out| {
        assert_eq!(limbs_asc_from_bits_asc(bits), out);
    };
    test(&[], vec![]);
    test(&[false], vec![0]);
    test(&[false, false, false], vec![0]);
    test(&[true], vec![1]);
    test(&[false, true, true], vec![6]);
    test(&[true, false, false, true, false, true, true], vec![105]);
    test(
        &[true, false, false, true, false, true, true, false],
        vec![105],
    );
    test(
        &[
            true, false, false, true, false, true, true, false, false, false,
        ],
        vec![105],
    );
    #[cfg(feature = "32_bit_limbs")]
    {
        test(
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false, true, false, true, false, false, true, false, true,
                false, false, true, false, true, false, true, true, false, false, false, true,
                false, true, true, true,
            ],
            vec![3_567_587_328, 232],
        );
        test(
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false, true, false, true, false, false, true, false, true,
                false, false, true, false, true, false, true, true, false, false, false, true,
                false, true, true, true, false,
            ],
            vec![3_567_587_328, 232],
        );
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false, true, false, true, false, false, true, false, true,
                false, false, true, false, true, false, true, true, false, false, false, true,
                false, true, true, true,
            ],
            vec![1000000000000],
        );
        test(
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false, true, false, true, false, false, true, false, true,
                false, false, true, false, true, false, true, true, false, false, false, true,
                false, true, true, true, false,
            ],
            vec![1000000000000],
        );
    }
}

#[test]
fn test_limbs_asc_from_bits_desc() {
    let test = |bits: &[bool], out| {
        assert_eq!(limbs_asc_from_bits_desc(bits), out);
    };
    test(&[], vec![]);
    test(&[false], vec![0]);
    test(&[false, false, false], vec![0]);
    test(&[true], vec![1]);
    test(&[true, true, false], vec![6]);
    test(&[true, true, false, true, false, false, true], vec![105]);
    test(
        &[false, true, true, false, true, false, false, true],
        vec![105],
    );
    test(
        &[
            false, false, false, true, true, false, true, false, false, true,
        ],
        vec![105],
    );
    #[cfg(feature = "32_bit_limbs")]
    {
        test(
            &[
                true, true, true, false, true, false, false, false, true, true, false, true, false,
                true, false, false, true, false, true, false, false, true, false, true, false,
                false, false, true, false, false, false, false, false, false, false, false, false,
                false, false, false,
            ],
            vec![3_567_587_328, 232],
        );
        test(
            &[
                false, true, true, true, false, true, false, false, false, true, true, false, true,
                false, true, false, false, true, false, true, false, false, true, false, true,
                false, false, false, true, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            vec![3_567_587_328, 232],
        );
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(
            &[
                true, true, true, false, true, false, false, false, true, true, false, true, false,
                true, false, false, true, false, true, false, false, true, false, true, false,
                false, false, true, false, false, false, false, false, false, false, false, false,
                false, false, false,
            ],
            vec![1000000000000],
        );
        test(
            &[
                false, true, true, true, false, true, false, false, false, true, true, false, true,
                false, true, false, false, true, false, true, false, false, true, false, true,
                false, false, false, true, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            vec![1000000000000],
        );
    }
}

#[test]
fn test_from_bits_asc() {
    let test = |bits: &[bool], out| {
        let x = Natural::from_bits_asc(bits);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(&[], "0");
    test(&[false], "0");
    test(&[false, false, false], "0");
    test(&[true], "1");
    test(&[false, true, true], "6");
    test(&[true, false, false, true, false, true, true], "105");
    test(&[true, false, false, true, false, true, true, false], "105");
    test(
        &[
            true, false, false, true, false, true, true, false, false, false,
        ],
        "105",
    );
    test(
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true,
        ],
        "1000000000000",
    );
    test(
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true, false,
        ],
        "1000000000000",
    );
}

#[test]
fn test_from_bits_desc() {
    let test = |bits: &[bool], out| {
        let x = Natural::from_bits_desc(bits);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(&[], "0");
    test(&[false], "0");
    test(&[false, false, false], "0");
    test(&[true], "1");
    test(&[true, true, false], "6");
    test(&[true, true, false, true, false, false, true], "105");
    test(&[false, true, true, false, true, false, false, true], "105");
    test(
        &[
            false, false, false, true, true, false, true, false, false, true,
        ],
        "105",
    );
    test(
        &[
            true, true, true, false, true, false, false, false, true, true, false, true, false,
            true, false, false, true, false, true, false, false, true, false, true, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ],
        "1000000000000",
    );
    test(
        &[
            false, true, true, true, false, true, false, false, false, true, true, false, true,
            false, true, false, false, true, false, true, false, false, true, false, true, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ],
        "1000000000000",
    );
}

#[test]
fn limbs_asc_from_bits_asc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let limbs = limbs_asc_from_bits_asc(bits);
        assert_eq!(
            Natural::from_limbs_asc(&limbs),
            Natural::from_bits_asc(bits)
        );
        let mut limb_count = bits.len() >> Limb::LOG_WIDTH;
        if limb_count << Limb::LOG_WIDTH != bits.len() {
            limb_count += 1;
        }
        assert_eq!(limbs.len(), limb_count);
    });
}

#[test]
fn limbs_asc_from_bits_desc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let limbs = limbs_asc_from_bits_desc(bits);
        assert_eq!(
            Natural::from_limbs_asc(&limbs),
            Natural::from_bits_desc(bits)
        );
        let mut limb_count = bits.len() >> Limb::LOG_WIDTH;
        if limb_count << Limb::LOG_WIDTH != bits.len() {
            limb_count += 1;
        }
        assert_eq!(limbs.len(), limb_count);
    });
}

#[test]
fn from_bits_asc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let x = Natural::from_bits_asc(bits);
        assert!(x.is_valid());
        let mut trimmed_bits: Vec<bool> =
            bits.iter().cloned().rev().skip_while(|&bit| !bit).collect();
        trimmed_bits.reverse();
        assert_eq!(x.to_bits_asc(), trimmed_bits);
        assert_eq!(
            Natural::from_bits_desc(&bits.iter().cloned().rev().collect::<Vec<bool>>()),
            x
        );
        if !bits.is_empty() && *bits.last().unwrap() {
            assert_eq!(x.to_bits_asc(), *bits);
        }
        assert_eq!(bits.iter().all(|b| !b), x == 0 as Limb);
    });
}

#[test]
fn from_bits_desc_properties() {
    test_properties(vecs_of_bool, |bits| {
        let x = Natural::from_bits_desc(bits);
        assert!(x.is_valid());
        assert_eq!(
            x.to_bits_desc(),
            bits.iter()
                .cloned()
                .skip_while(|&b| !b)
                .collect::<Vec<bool>>()
        );
        assert_eq!(
            Natural::from_bits_asc(&bits.iter().cloned().rev().collect::<Vec<bool>>()),
            x
        );
        if !bits.is_empty() && bits[0] {
            assert_eq!(x.to_bits_desc(), *bits);
        }
        assert_eq!(bits.iter().all(|b| !b), x == 0 as Limb);
    });
}
