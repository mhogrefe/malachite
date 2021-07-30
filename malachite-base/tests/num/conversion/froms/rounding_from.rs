use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::{
    primitive_float_gen_var_13, primitive_float_gen_var_14, primitive_float_gen_var_15,
    primitive_float_gen_var_16, primitive_float_gen_var_17,
    primitive_float_rounding_mode_pair_gen_var_3, signed_gen_var_7, signed_gen_var_8,
    signed_gen_var_9, signed_rounding_mode_pair_gen_var_4, unsigned_gen_var_18,
    unsigned_gen_var_19, unsigned_gen_var_20, unsigned_rounding_mode_pair_gen_var_2,
};

fn rounding_from_helper_unsigned_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveUnsigned + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>() {
    primitive_float_rounding_mode_pair_gen_var_3::<U, T>().test_properties(|(f, rm)| {
        T::rounding_from(f, rm);
    });

    primitive_float_gen_var_13::<U, T>().test_properties(|f| {
        let n = T::rounding_from(f, RoundingMode::Exact);
        assert_eq!(n, T::rounding_from(f, RoundingMode::Floor));
        assert_eq!(n, T::rounding_from(f, RoundingMode::Ceiling));
        assert_eq!(n, T::rounding_from(f, RoundingMode::Down));
        assert_eq!(n, T::rounding_from(f, RoundingMode::Up));
        assert_eq!(n, T::rounding_from(f, RoundingMode::Nearest));
    });

    let f_max = U::rounding_from(T::MAX, RoundingMode::Down);
    primitive_float_gen_var_15::<U, T>().test_properties(|f| {
        if f >= U::ZERO && f <= f_max {
            let n_floor = T::rounding_from(f, RoundingMode::Floor);
            if let Some(n_ceiling) = n_floor.checked_add(T::ONE) {
                assert_eq!(n_ceiling, T::rounding_from(f, RoundingMode::Ceiling));
                assert_eq!(n_floor, T::rounding_from(f, RoundingMode::Down));
                assert_eq!(n_ceiling, T::rounding_from(f, RoundingMode::Up));
                let n_nearest = T::rounding_from(f, RoundingMode::Nearest);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
            }
        }
    });

    primitive_float_gen_var_16::<U, T>().test_properties(|f| {
        let floor = T::rounding_from(f, RoundingMode::Floor);
        let ceiling = floor + T::ONE;
        let nearest = T::rounding_from(f, RoundingMode::Nearest);
        assert_eq!(nearest, if floor.even() { floor } else { ceiling });
    });
}

fn rounding_from_helper_signed_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveSigned + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveFloat + RoundingFrom<T>,
>() {
    primitive_float_rounding_mode_pair_gen_var_3::<U, T>().test_properties(|(f, rm)| {
        T::rounding_from(f, rm);
    });

    primitive_float_gen_var_14::<U, T>().test_properties(|f| {
        let n = T::rounding_from(f, RoundingMode::Exact);
        assert_eq!(n, T::rounding_from(f, RoundingMode::Floor));
        assert_eq!(n, T::rounding_from(f, RoundingMode::Ceiling));
        assert_eq!(n, T::rounding_from(f, RoundingMode::Down));
        assert_eq!(n, T::rounding_from(f, RoundingMode::Up));
        assert_eq!(n, T::rounding_from(f, RoundingMode::Nearest));
    });

    let f_min = U::rounding_from(T::MIN, RoundingMode::Down);
    let f_max = U::rounding_from(T::MAX, RoundingMode::Down);
    primitive_float_gen_var_15::<U, T>().test_properties(|f| {
        if f >= f_min && f <= f_max {
            let n_floor = T::rounding_from(f, RoundingMode::Floor);
            if let Some(n_ceiling) = n_floor.checked_add(T::ONE) {
                assert_eq!(n_ceiling, T::rounding_from(f, RoundingMode::Ceiling));
                if f >= U::ZERO {
                    assert_eq!(n_floor, T::rounding_from(f, RoundingMode::Down));
                    assert_eq!(n_ceiling, T::rounding_from(f, RoundingMode::Up));
                } else {
                    assert_eq!(n_ceiling, T::rounding_from(f, RoundingMode::Down));
                    assert_eq!(n_floor, T::rounding_from(f, RoundingMode::Up));
                }
                let n_nearest = T::rounding_from(f, RoundingMode::Nearest);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
            }
        }
    });

    primitive_float_gen_var_17::<U, T>().test_properties(|f| {
        let floor = T::rounding_from(f, RoundingMode::Floor);
        let ceiling = floor + T::ONE;
        let nearest = T::rounding_from(f, RoundingMode::Nearest);
        assert_eq!(nearest, if floor.even() { floor } else { ceiling });
    });
}

fn rounding_from_helper_primitive_float_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveUnsigned + RoundingFrom<T>,
>() {
    unsigned_rounding_mode_pair_gen_var_2::<U, T>().test_properties(|(u, rm)| {
        T::rounding_from(u, rm);
    });

    unsigned_gen_var_18::<U, T>().test_properties(|u| {
        let f = T::rounding_from(u, RoundingMode::Exact);
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(u, RoundingMode::Floor))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(u, RoundingMode::Ceiling))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(u, RoundingMode::Down))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(u, RoundingMode::Up))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(u, RoundingMode::Nearest))
        );
        assert_eq!(U::rounding_from(f, RoundingMode::Exact), u);
    });

    if U::WIDTH > T::MANTISSA_WIDTH {
        unsigned_gen_var_19::<U, T>().test_properties(|u| {
            let f_below = T::rounding_from(u, RoundingMode::Floor);
            let f_above = f_below.next_higher();
            assert_eq!(
                NiceFloat(f_above),
                NiceFloat(T::rounding_from(u, RoundingMode::Ceiling))
            );
            assert_eq!(
                NiceFloat(f_below),
                NiceFloat(T::rounding_from(u, RoundingMode::Down))
            );
            assert_eq!(
                NiceFloat(f_above),
                NiceFloat(T::rounding_from(u, RoundingMode::Up))
            );
            let f_nearest = T::rounding_from(u, RoundingMode::Nearest);
            assert!(
                NiceFloat(f_nearest) == NiceFloat(f_below)
                    || NiceFloat(f_nearest) == NiceFloat(f_above)
            );
        });

        unsigned_gen_var_20::<U, T>().test_properties(|u| {
            let floor = T::rounding_from(u, RoundingMode::Floor);
            let ceiling = floor.next_higher();
            let nearest = T::rounding_from(u, RoundingMode::Nearest);
            assert_eq!(
                NiceFloat(nearest),
                NiceFloat(if floor.to_bits().even() {
                    floor
                } else {
                    ceiling
                })
            );
        });
    }
}

fn rounding_from_helper_primitive_float_signed<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: CheckedFrom<T> + PrimitiveSigned + RoundingFrom<T>,
>() {
    signed_rounding_mode_pair_gen_var_4::<U, T>().test_properties(|(i, rm)| {
        T::rounding_from(i, rm);
    });

    signed_gen_var_7::<U, T>().test_properties(|i| {
        let f = T::rounding_from(i, RoundingMode::Exact);
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(i, RoundingMode::Floor))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(i, RoundingMode::Ceiling))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(i, RoundingMode::Down))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(i, RoundingMode::Up))
        );
        assert_eq!(
            NiceFloat(f),
            NiceFloat(T::rounding_from(i, RoundingMode::Nearest))
        );
        assert_eq!(U::rounding_from(f, RoundingMode::Exact), i);
    });

    if U::WIDTH > T::MANTISSA_WIDTH {
        signed_gen_var_8::<U, T>().test_properties(|i| {
            let f_below = T::rounding_from(i, RoundingMode::Floor);
            let f_above = f_below.next_higher();
            assert_eq!(
                NiceFloat(f_above),
                NiceFloat(T::rounding_from(i, RoundingMode::Ceiling))
            );
            if i >= U::ZERO {
                assert_eq!(
                    NiceFloat(f_below),
                    NiceFloat(T::rounding_from(i, RoundingMode::Down))
                );
                assert_eq!(
                    NiceFloat(f_above),
                    NiceFloat(T::rounding_from(i, RoundingMode::Up))
                );
            } else {
                assert_eq!(
                    NiceFloat(f_above),
                    NiceFloat(T::rounding_from(i, RoundingMode::Down))
                );
                assert_eq!(
                    NiceFloat(f_below),
                    NiceFloat(T::rounding_from(i, RoundingMode::Up))
                );
            }
            let f_nearest = T::rounding_from(i, RoundingMode::Nearest);
            assert_eq!(
                NiceFloat(T::rounding_from(i, RoundingMode::Nearest)),
                NiceFloat(f_nearest)
            );
            assert!(
                NiceFloat(f_nearest) == NiceFloat(f_below)
                    || NiceFloat(f_nearest) == NiceFloat(f_above)
            );
        });

        signed_gen_var_9::<U, T>().test_properties(|i| {
            let floor = T::rounding_from(i, RoundingMode::Floor);
            let ceiling = floor.next_higher();
            let nearest = T::rounding_from(i, RoundingMode::Nearest);
            assert_eq!(
                NiceFloat(nearest),
                NiceFloat(if floor.to_bits().even() {
                    floor
                } else {
                    ceiling
                })
            );
        });
    }
}

#[test]
fn rounding_from_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(rounding_from_helper_unsigned_primitive_float);
    apply_fn_to_signeds_and_primitive_floats!(rounding_from_helper_signed_primitive_float);
    apply_fn_to_primitive_floats_and_unsigneds!(rounding_from_helper_primitive_float_unsigned);
    apply_fn_to_primitive_floats_and_signeds!(rounding_from_helper_primitive_float_signed);
}
