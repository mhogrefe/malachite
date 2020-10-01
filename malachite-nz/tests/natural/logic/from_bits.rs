use malachite_base::num::logic::traits::BitConvertible;

use malachite_nz::natural::logic::bit_convertible::{
    limbs_asc_from_bits_asc, limbs_asc_from_bits_desc,
};
use malachite_nz::natural::Natural;

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
            vec![3567587328, 232],
        );
        test(
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
                true, false, false, false, true, false, true, false, false, true, false, true,
                false, false, true, false, true, false, true, true, false, false, false, true,
                false, true, true, true, false,
            ],
            vec![3567587328, 232],
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
            vec![3567587328, 232],
        );
        test(
            &[
                false, true, true, true, false, true, false, false, false, true, true, false, true,
                false, true, false, false, true, false, true, false, false, true, false, true,
                false, false, false, true, false, false, false, false, false, false, false, false,
                false, false, false, false,
            ],
            vec![3567587328, 232],
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
fn test_from_bits_asc_and_from_bit_iterator_asc() {
    let test = |bits: &[bool], out| {
        let x = Natural::from_bits_asc(bits);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Natural::from_bit_iterator_asc(bits.iter().cloned());
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
fn test_from_bits_desc_and_from_bit_iterator_desc() {
    let test = |bits: &[bool], out| {
        let x = Natural::from_bits_desc(bits);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);

        let x = Natural::from_bit_iterator_desc(bits.iter().cloned());
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
