use malachite_base::slices::slice_test_zero::slice_test_zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::vecs_of_unsigned;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_limbs_asc() {
    let test = |limbs: &[Limb], out| {
        let x = Natural::from_limbs_asc(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Natural::from_owned_limbs_asc(limbs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[123, 0], "123");
    test(&[123, 0, 0, 0], "123");
    test(&[3_567_587_328, 232], "1000000000000");
    test(&[3_567_587_328, 232, 0], "1000000000000");
    test(&[1, 2, 3, 4, 5], "1701411834921604967429270619762735448065");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_limbs_desc() {
    let test = |limbs: Vec<Limb>, out| {
        let x = Natural::from_limbs_desc(&limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Natural::from_owned_limbs_desc(limbs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(vec![], "0");
    test(vec![0], "0");
    test(vec![0, 0, 0], "0");
    test(vec![123], "123");
    test(vec![0, 123], "123");
    test(vec![0, 0, 0, 123], "123");
    test(vec![232, 3_567_587_328], "1000000000000");
    test(vec![0, 232, 3_567_587_328], "1000000000000");
    test(
        vec![5, 4, 3, 2, 1],
        "1701411834921604967429270619762735448065",
    );
}

#[test]
fn from_limbs_asc_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        let x = Natural::from_limbs_asc(limbs);
        assert!(x.is_valid());
        assert_eq!(Natural::from_owned_limbs_asc(limbs.clone()), x);
        let mut trimmed_limbs: Vec<Limb> = limbs
            .iter()
            .cloned()
            .rev()
            .skip_while(|&limb| limb == 0)
            .collect();
        trimmed_limbs.reverse();
        assert_eq!(x.to_limbs_asc(), trimmed_limbs);
        assert_eq!(
            Natural::from_limbs_desc(&limbs.iter().cloned().rev().collect::<Vec<Limb>>()),
            x
        );
        if !limbs.is_empty() && *limbs.last().unwrap() != 0 {
            assert_eq!(x.to_limbs_asc(), *limbs);
        }
        assert_eq!(slice_test_zero(limbs), x == 0);
    });
}

#[test]
fn from_limbs_desc_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        let x = Natural::from_limbs_desc(limbs);
        assert!(x.is_valid());
        assert_eq!(Natural::from_owned_limbs_desc(limbs.clone()), x);
        assert_eq!(
            x.to_limbs_desc(),
            limbs
                .iter()
                .cloned()
                .skip_while(|&limb| limb == 0)
                .collect::<Vec<Limb>>()
        );
        assert_eq!(
            Natural::from_limbs_asc(&limbs.iter().cloned().rev().collect::<Vec<Limb>>()),
            x
        );
        if !limbs.is_empty() && limbs[0] != 0 {
            assert_eq!(x.to_limbs_desc(), *limbs);
        }
        assert_eq!(slice_test_zero(limbs), x == 0);
    });
}
