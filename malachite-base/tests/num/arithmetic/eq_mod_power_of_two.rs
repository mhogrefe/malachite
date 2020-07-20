use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;

fn eq_mod_power_of_two_primitive_helper<T: PrimitiveInteger>() {
    let test = |n: T, other, pow, out| {
        assert_eq!(n.eq_mod_power_of_two(other, pow), out);
    };
    test(T::ZERO, T::power_of_two(T::WIDTH >> 1), T::WIDTH >> 1, true);
    test(
        T::ZERO,
        T::power_of_two(T::WIDTH >> 1),
        (T::WIDTH >> 1) + 1,
        false,
    );
    test(T::exact_from(13), T::exact_from(21), 0, true);
    test(T::exact_from(13), T::exact_from(21), 1, true);
    test(T::exact_from(13), T::exact_from(21), 2, true);
    test(T::exact_from(13), T::exact_from(21), 3, true);
    test(T::exact_from(13), T::exact_from(21), 4, false);
    test(T::exact_from(13), T::exact_from(21), 100, false);
    test(T::MAX, T::MAX, T::WIDTH, true);
    test(T::MAX, T::MAX, 100, true);
    if T::WIDTH >= u64::WIDTH {
        test(T::exact_from(1_000_000_000_001u64), T::ONE, 12, true);
        test(T::exact_from(1_000_000_000_001u64), T::ONE, 13, false);
        test(
            T::exact_from(281_474_976_710_672u64),
            T::exact_from(844_424_930_131_984u64),
            49,
            true,
        );
        test(
            T::exact_from(281_474_976_710_672u64),
            T::exact_from(844_424_930_131_984u64),
            50,
            false,
        );
    }
}

fn eq_mod_power_of_two_signed_helper<T: PrimitiveSigned>() {
    let test = |n: T, other, pow, out| {
        assert_eq!(n.eq_mod_power_of_two(other, pow), out);
    };
    test(
        T::ZERO,
        -T::power_of_two(T::WIDTH >> 1),
        T::WIDTH >> 1,
        true,
    );
    test(
        T::ZERO,
        -T::power_of_two(T::WIDTH >> 1),
        (T::WIDTH >> 1) + 1,
        false,
    );
    test(T::exact_from(-13), T::exact_from(27), 0, true);
    test(T::exact_from(-13), T::exact_from(27), 1, true);
    test(T::exact_from(-13), T::exact_from(27), 2, true);
    test(T::exact_from(-13), T::exact_from(27), 3, true);
    test(T::exact_from(-13), T::exact_from(27), 4, false);
    test(T::exact_from(-13), T::exact_from(27), 100, false);
    test(T::exact_from(13), T::exact_from(-27), 0, true);
    test(T::exact_from(13), T::exact_from(-27), 1, true);
    test(T::exact_from(13), T::exact_from(-27), 2, true);
    test(T::exact_from(13), T::exact_from(-27), 3, true);
    test(T::exact_from(13), T::exact_from(-27), 4, false);
    test(T::exact_from(13), T::exact_from(-27), 100, false);
    test(
        T::NEGATIVE_ONE,
        T::power_of_two(T::WIDTH >> 1) - T::ONE,
        T::WIDTH >> 1,
        true,
    );
    test(
        T::power_of_two(T::WIDTH >> 1) - T::ONE,
        T::NEGATIVE_ONE,
        T::WIDTH >> 1,
        true,
    );
    if T::WIDTH >= u64::WIDTH {
        test(
            T::exact_from(-1_000_000_000_001i64),
            T::exact_from(4_095),
            13,
            true,
        );
        test(
            T::exact_from(-1_000_000_000_001i64),
            T::exact_from(4_095),
            14,
            false,
        );
        test(
            T::exact_from(1_000_000_000_001i64),
            T::exact_from(-4_095),
            13,
            true,
        );
        test(
            T::exact_from(1_000_000_000_001i64),
            T::exact_from(-4_095),
            14,
            false,
        );
    }

    test(T::exact_from(-13), T::exact_from(-21), 0, true);
    test(T::exact_from(-13), T::exact_from(-21), 1, true);
    test(T::exact_from(-13), T::exact_from(-21), 2, true);
    test(T::exact_from(-13), T::exact_from(-21), 3, true);
    test(T::exact_from(-13), T::exact_from(-21), 4, false);
    test(T::exact_from(-13), T::exact_from(-21), 100, false);
    test(
        T::power_of_two(T::WIDTH >> 1) - T::ONE,
        T::power_of_two(T::WIDTH >> 1) - T::ONE,
        T::WIDTH >> 1,
        true,
    );
    if T::WIDTH >= u64::WIDTH {
        test(
            T::exact_from(-1_000_000_000_001i64),
            T::NEGATIVE_ONE,
            12,
            true,
        );
        test(
            T::exact_from(-1_000_000_000_001i64),
            T::NEGATIVE_ONE,
            13,
            false,
        );
        test(
            T::exact_from(-281_474_976_710_672i64),
            T::exact_from(-844_424_930_131_984i64),
            49,
            true,
        );
        test(
            T::exact_from(-281_474_976_710_672i64),
            T::exact_from(-844_424_930_131_984i64),
            50,
            false,
        );
    }

    if T::WIDTH >= u128::WIDTH {
        test(
            T::exact_from(1_311_693_408_901_639_117i128),
            T::exact_from(-17_135_050_664_807_912_499i128),
            64,
            true,
        );
        test(
            T::exact_from(1_311_693_408_901_639_117i128),
            T::exact_from(-17_135_050_663_395_328_000i128),
            64,
            false,
        );
        test(
            T::exact_from(1_311_693_408_901_639_117i128),
            T::exact_from(-17_135_050_664_807_912_499i128),
            65,
            false,
        );
        test(
            T::exact_from(1_311_693_408_901_639_117i128),
            T::exact_from(-17_135_050_664_807_912_499i128),
            128,
            false,
        );
        test(
            T::exact_from(5_633_680_281_231_555_440_641_310_720i128),
            T::exact_from(-5_634_717_283_396_403_096_794_955_776i128),
            80,
            true,
        );

        test(
            T::exact_from(-1_311_693_408_901_639_117i128),
            T::exact_from(17_135_050_664_807_912_499i128),
            64,
            true,
        );
        test(
            T::exact_from(-1_311_693_408_901_639_117i128),
            T::exact_from(17_135_050_663_395_328_000i128),
            64,
            false,
        );
        test(
            T::exact_from(-1_311_693_408_901_639_117i128),
            T::exact_from(17_135_050_664_807_912_499i128),
            65,
            false,
        );
        test(
            T::exact_from(-1_311_693_408_901_639_117i128),
            T::exact_from(17_135_050_664_807_912_499i128),
            128,
            false,
        );
        test(
            T::exact_from(-5_633_680_281_231_555_440_641_310_720i128),
            T::exact_from(5_634_717_283_396_403_096_794_955_776i128),
            80,
            true,
        );
    }
}

#[test]
fn test_eq_mod_power_of_two() {
    apply_fn_to_primitive_ints!(eq_mod_power_of_two_primitive_helper);
    apply_fn_to_signeds!(eq_mod_power_of_two_signed_helper);
}
