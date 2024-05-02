// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeInfinity;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom, WrappingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    primitive_float_gen_var_13, primitive_float_gen_var_14, primitive_float_gen_var_15,
    primitive_float_gen_var_16, primitive_float_gen_var_17,
    primitive_float_rounding_mode_pair_gen_var_3, signed_gen_var_7, signed_gen_var_8,
    signed_gen_var_9, signed_rounding_mode_pair_gen_var_4, unsigned_gen_var_18,
    unsigned_gen_var_19, unsigned_gen_var_20, unsigned_rounding_mode_pair_gen_var_2,
};
use std::cmp::Ordering;
use std::panic::catch_unwind;

#[test]
pub fn test_rounding_from() {
    fn test_from_floating_point<T: PrimitiveFloat, U: PrimitiveInt + RoundingFrom<T>>(
        n_in: T,
        rm: RoundingMode,
        n_out: U,
        o: Ordering,
    ) {
        assert_eq!(U::rounding_from(n_in, rm), (n_out, o));
    }
    test_from_floating_point::<f32, u8>(0.0, RoundingMode::Down, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(0.0, RoundingMode::Floor, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(0.0, RoundingMode::Up, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(0.0, RoundingMode::Ceiling, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(0.0, RoundingMode::Nearest, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(0.0, RoundingMode::Exact, 0, Ordering::Equal);

    test_from_floating_point::<f32, u8>(-0.0, RoundingMode::Down, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(-0.0, RoundingMode::Floor, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(-0.0, RoundingMode::Up, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(-0.0, RoundingMode::Ceiling, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(-0.0, RoundingMode::Nearest, 0, Ordering::Equal);
    test_from_floating_point::<f32, u8>(-0.0, RoundingMode::Exact, 0, Ordering::Equal);

    test_from_floating_point::<f32, u8>(100.0, RoundingMode::Down, 100, Ordering::Equal);
    test_from_floating_point::<f32, u8>(100.0, RoundingMode::Floor, 100, Ordering::Equal);
    test_from_floating_point::<f32, u8>(100.0, RoundingMode::Up, 100, Ordering::Equal);
    test_from_floating_point::<f32, u8>(100.0, RoundingMode::Ceiling, 100, Ordering::Equal);
    test_from_floating_point::<f32, u8>(100.0, RoundingMode::Nearest, 100, Ordering::Equal);
    test_from_floating_point::<f32, u8>(100.0, RoundingMode::Exact, 100, Ordering::Equal);

    test_from_floating_point::<f32, u8>(100.1, RoundingMode::Down, 100, Ordering::Less);
    test_from_floating_point::<f32, u8>(100.1, RoundingMode::Floor, 100, Ordering::Less);
    test_from_floating_point::<f32, u8>(100.1, RoundingMode::Up, 101, Ordering::Greater);
    test_from_floating_point::<f32, u8>(100.1, RoundingMode::Ceiling, 101, Ordering::Greater);
    test_from_floating_point::<f32, u8>(100.1, RoundingMode::Nearest, 100, Ordering::Less);

    test_from_floating_point::<f32, u8>(100.9, RoundingMode::Down, 100, Ordering::Less);
    test_from_floating_point::<f32, u8>(100.9, RoundingMode::Floor, 100, Ordering::Less);
    test_from_floating_point::<f32, u8>(100.9, RoundingMode::Up, 101, Ordering::Greater);
    test_from_floating_point::<f32, u8>(100.9, RoundingMode::Ceiling, 101, Ordering::Greater);
    test_from_floating_point::<f32, u8>(100.9, RoundingMode::Nearest, 101, Ordering::Greater);

    test_from_floating_point::<f32, u8>(100.5, RoundingMode::Down, 100, Ordering::Less);
    test_from_floating_point::<f32, u8>(100.5, RoundingMode::Floor, 100, Ordering::Less);
    test_from_floating_point::<f32, u8>(100.5, RoundingMode::Up, 101, Ordering::Greater);
    test_from_floating_point::<f32, u8>(100.5, RoundingMode::Ceiling, 101, Ordering::Greater);
    test_from_floating_point::<f32, u8>(100.5, RoundingMode::Nearest, 100, Ordering::Less);

    test_from_floating_point::<f32, u8>(101.5, RoundingMode::Down, 101, Ordering::Less);
    test_from_floating_point::<f32, u8>(101.5, RoundingMode::Floor, 101, Ordering::Less);
    test_from_floating_point::<f32, u8>(101.5, RoundingMode::Up, 102, Ordering::Greater);
    test_from_floating_point::<f32, u8>(101.5, RoundingMode::Ceiling, 102, Ordering::Greater);
    test_from_floating_point::<f32, u8>(101.5, RoundingMode::Nearest, 102, Ordering::Greater);

    test_from_floating_point::<f32, u8>(256.0, RoundingMode::Down, 255, Ordering::Less);
    test_from_floating_point::<f32, u8>(256.0, RoundingMode::Floor, 255, Ordering::Less);
    test_from_floating_point::<f32, u8>(256.0, RoundingMode::Nearest, 255, Ordering::Less);

    test_from_floating_point::<f32, u8>(-100.0, RoundingMode::Down, 0, Ordering::Greater);
    test_from_floating_point::<f32, u8>(-100.0, RoundingMode::Ceiling, 0, Ordering::Greater);
    test_from_floating_point::<f32, u8>(-100.0, RoundingMode::Nearest, 0, Ordering::Greater);

    test_from_floating_point::<f32, i8>(128.0, RoundingMode::Down, 127, Ordering::Less);
    test_from_floating_point::<f32, i8>(128.0, RoundingMode::Floor, 127, Ordering::Less);
    test_from_floating_point::<f32, i8>(128.0, RoundingMode::Nearest, 127, Ordering::Less);

    test_from_floating_point::<f32, i8>(-129.0, RoundingMode::Down, -128, Ordering::Greater);
    test_from_floating_point::<f32, i8>(-129.0, RoundingMode::Ceiling, -128, Ordering::Greater);
    test_from_floating_point::<f32, i8>(-129.0, RoundingMode::Nearest, -128, Ordering::Greater);

    test_from_floating_point::<f32, u8>(f32::INFINITY, RoundingMode::Down, 255, Ordering::Less);
    test_from_floating_point::<f32, u8>(f32::INFINITY, RoundingMode::Floor, 255, Ordering::Less);
    test_from_floating_point::<f32, u8>(f32::INFINITY, RoundingMode::Nearest, 255, Ordering::Less);
    test_from_floating_point::<f32, u8>(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Down,
        0,
        Ordering::Greater,
    );
    test_from_floating_point::<f32, u8>(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Ceiling,
        0,
        Ordering::Greater,
    );
    test_from_floating_point::<f32, u8>(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Nearest,
        0,
        Ordering::Greater,
    );

    test_from_floating_point::<f32, i8>(f32::INFINITY, RoundingMode::Down, 127, Ordering::Less);
    test_from_floating_point::<f32, i8>(f32::INFINITY, RoundingMode::Floor, 127, Ordering::Less);
    test_from_floating_point::<f32, i8>(f32::INFINITY, RoundingMode::Nearest, 127, Ordering::Less);
    test_from_floating_point::<f32, i8>(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Down,
        -128,
        Ordering::Greater,
    );
    test_from_floating_point::<f32, i8>(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Ceiling,
        -128,
        Ordering::Greater,
    );
    test_from_floating_point::<f32, i8>(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Nearest,
        -128,
        Ordering::Greater,
    );

    fn test_from_primitive_int<T: PrimitiveInt, U: PrimitiveFloat + RoundingFrom<T>>(
        n_in: T,
        rm: RoundingMode,
        n_out: U,
        o: Ordering,
    ) {
        let (x, actual_o) = U::rounding_from(n_in, rm);
        assert_eq!((NiceFloat(x), actual_o), (NiceFloat(n_out), o));
    }
    test_from_primitive_int::<u8, f32>(0, RoundingMode::Down, 0.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(0, RoundingMode::Floor, 0.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(0, RoundingMode::Up, 0.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(0, RoundingMode::Ceiling, 0.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(0, RoundingMode::Nearest, 0.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(0, RoundingMode::Exact, 0.0, Ordering::Equal);

    test_from_primitive_int::<u8, f32>(100, RoundingMode::Down, 100.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(100, RoundingMode::Floor, 100.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(100, RoundingMode::Up, 100.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(100, RoundingMode::Ceiling, 100.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(100, RoundingMode::Nearest, 100.0, Ordering::Equal);
    test_from_primitive_int::<u8, f32>(100, RoundingMode::Exact, 100.0, Ordering::Equal);

    test_from_primitive_int::<i8, f32>(-100, RoundingMode::Down, -100.0, Ordering::Equal);
    test_from_primitive_int::<i8, f32>(-100, RoundingMode::Floor, -100.0, Ordering::Equal);
    test_from_primitive_int::<i8, f32>(-100, RoundingMode::Up, -100.0, Ordering::Equal);
    test_from_primitive_int::<i8, f32>(-100, RoundingMode::Ceiling, -100.0, Ordering::Equal);
    test_from_primitive_int::<i8, f32>(-100, RoundingMode::Nearest, -100.0, Ordering::Equal);
    test_from_primitive_int::<i8, f32>(-100, RoundingMode::Exact, -100.0, Ordering::Equal);

    test_from_primitive_int::<i32, f32>(
        i32::MIN,
        RoundingMode::Down,
        -2147483600.0,
        Ordering::Equal,
    );
    test_from_primitive_int::<i32, f32>(
        i32::MIN,
        RoundingMode::Floor,
        -2147483600.0,
        Ordering::Equal,
    );
    test_from_primitive_int::<i32, f32>(i32::MIN, RoundingMode::Up, -2147483600.0, Ordering::Equal);
    test_from_primitive_int::<i32, f32>(
        i32::MIN,
        RoundingMode::Ceiling,
        -2147483600.0,
        Ordering::Equal,
    );
    test_from_primitive_int::<i32, f32>(
        i32::MIN,
        RoundingMode::Nearest,
        -2147483600.0,
        Ordering::Equal,
    );
    test_from_primitive_int::<i32, f32>(
        i32::MIN,
        RoundingMode::Exact,
        -2147483600.0,
        Ordering::Equal,
    );

    test_from_primitive_int::<i32, f32>(i32::MAX, RoundingMode::Down, 2147483500.0, Ordering::Less);
    test_from_primitive_int::<i32, f32>(
        i32::MAX,
        RoundingMode::Floor,
        2147483500.0,
        Ordering::Less,
    );
    test_from_primitive_int::<i32, f32>(
        i32::MAX,
        RoundingMode::Up,
        2147483600.0,
        Ordering::Greater,
    );
    test_from_primitive_int::<i32, f32>(
        i32::MAX,
        RoundingMode::Ceiling,
        2147483600.0,
        Ordering::Greater,
    );
    test_from_primitive_int::<i32, f32>(
        i32::MAX,
        RoundingMode::Nearest,
        2147483600.0,
        Ordering::Greater,
    );

    test_from_primitive_int::<u128, f32>(
        u128::MAX,
        RoundingMode::Down,
        3.4028235e38,
        Ordering::Less,
    );
    test_from_primitive_int::<u128, f32>(
        u128::MAX,
        RoundingMode::Floor,
        3.4028235e38,
        Ordering::Less,
    );
    test_from_primitive_int::<u128, f32>(
        u128::MAX,
        RoundingMode::Up,
        f32::INFINITY,
        Ordering::Greater,
    );
    test_from_primitive_int::<u128, f32>(
        u128::MAX,
        RoundingMode::Ceiling,
        f32::INFINITY,
        Ordering::Greater,
    );
    test_from_primitive_int::<u128, f32>(
        u128::MAX,
        RoundingMode::Nearest,
        3.4028235e38,
        Ordering::Less,
    );
}

#[test]
fn exact_from_fail() {
    assert_panic!(u8::rounding_from(100.1f32, RoundingMode::Exact));

    assert_panic!(u8::rounding_from(256.0f32, RoundingMode::Exact));
    assert_panic!(u8::rounding_from(256.0f32, RoundingMode::Up));
    assert_panic!(u8::rounding_from(256.0f32, RoundingMode::Ceiling));
    assert_panic!(u8::rounding_from(-100.0f32, RoundingMode::Exact));
    assert_panic!(u8::rounding_from(-100.0f32, RoundingMode::Up));
    assert_panic!(u8::rounding_from(-100.0f32, RoundingMode::Floor));

    assert_panic!(i8::rounding_from(128.0f32, RoundingMode::Exact));
    assert_panic!(i8::rounding_from(128.0f32, RoundingMode::Up));
    assert_panic!(i8::rounding_from(128.0f32, RoundingMode::Ceiling));
    assert_panic!(i8::rounding_from(-129.0f32, RoundingMode::Exact));
    assert_panic!(i8::rounding_from(-129.0f32, RoundingMode::Up));
    assert_panic!(i8::rounding_from(-129.0f32, RoundingMode::Floor));

    assert_panic!(u8::rounding_from(f32::NAN, RoundingMode::Down));
    assert_panic!(u8::rounding_from(f32::NAN, RoundingMode::Floor));
    assert_panic!(u8::rounding_from(f32::NAN, RoundingMode::Up));
    assert_panic!(u8::rounding_from(f32::NAN, RoundingMode::Ceiling));
    assert_panic!(u8::rounding_from(f32::NAN, RoundingMode::Nearest));
    assert_panic!(u8::rounding_from(f32::NAN, RoundingMode::Exact));

    assert_panic!(u8::rounding_from(f32::INFINITY, RoundingMode::Up));
    assert_panic!(u8::rounding_from(f32::INFINITY, RoundingMode::Ceiling));
    assert_panic!(u8::rounding_from(f32::INFINITY, RoundingMode::Exact));
    assert_panic!(u8::rounding_from(f32::NEGATIVE_INFINITY, RoundingMode::Up));
    assert_panic!(u8::rounding_from(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(u8::rounding_from(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(i8::rounding_from(f32::INFINITY, RoundingMode::Up));
    assert_panic!(i8::rounding_from(f32::INFINITY, RoundingMode::Ceiling));
    assert_panic!(i8::rounding_from(f32::INFINITY, RoundingMode::Exact));
    assert_panic!(i8::rounding_from(f32::NEGATIVE_INFINITY, RoundingMode::Up));
    assert_panic!(i8::rounding_from(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Floor
    ));
    assert_panic!(i8::rounding_from(
        f32::NEGATIVE_INFINITY,
        RoundingMode::Exact
    ));

    assert_panic!(u8::rounding_from(f32::NAN, RoundingMode::Down));

    assert_panic!(f32::rounding_from(u32::MAX, RoundingMode::Exact));
    assert_panic!(f32::rounding_from(u128::MAX, RoundingMode::Exact));
}

fn rounding_from_helper_unsigned_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveUnsigned + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>()
where
    NiceFloat<U>: TryFrom<T>,
{
    primitive_float_rounding_mode_pair_gen_var_3::<U, T>().test_properties(|(f, rm)| {
        let o = T::rounding_from(f, rm).1;
        match (f >= U::ZERO, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    primitive_float_gen_var_13::<U, T>().test_properties(|f| {
        let no = T::rounding_from(f, RoundingMode::Exact);
        assert_eq!(no, T::rounding_from(f, RoundingMode::Floor));
        assert_eq!(no, T::rounding_from(f, RoundingMode::Ceiling));
        assert_eq!(no, T::rounding_from(f, RoundingMode::Down));
        assert_eq!(no, T::rounding_from(f, RoundingMode::Up));
        assert_eq!(no, T::rounding_from(f, RoundingMode::Nearest));
    });

    let f_max = U::rounding_from(T::MAX, RoundingMode::Down).0;
    primitive_float_gen_var_15::<U, T>().test_properties(|f| {
        if f >= U::ZERO && f <= f_max {
            let n_floor = T::rounding_from(f, RoundingMode::Floor);
            assert_eq!(n_floor.1, Ordering::Less);
            if let Some(n_ceiling) = n_floor.0.checked_add(T::ONE) {
                let n_ceiling = (n_ceiling, Ordering::Greater);
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
        let ceiling = (floor.0 + T::ONE, Ordering::Greater);
        let nearest = T::rounding_from(f, RoundingMode::Nearest);
        assert_eq!(nearest, if floor.0.even() { floor } else { ceiling });
    });
}

fn rounding_from_helper_signed_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveSigned + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>()
where
    NiceFloat<U>: TryFrom<T>,
{
    primitive_float_rounding_mode_pair_gen_var_3::<U, T>().test_properties(|(f, rm)| {
        let o = T::rounding_from(f, rm).1;
        match (f >= U::ZERO, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    primitive_float_gen_var_14::<U, T>().test_properties(|f| {
        let no = T::rounding_from(f, RoundingMode::Exact);
        assert_eq!(no, T::rounding_from(f, RoundingMode::Floor));
        assert_eq!(no, T::rounding_from(f, RoundingMode::Ceiling));
        assert_eq!(no, T::rounding_from(f, RoundingMode::Down));
        assert_eq!(no, T::rounding_from(f, RoundingMode::Up));
        assert_eq!(no, T::rounding_from(f, RoundingMode::Nearest));
    });

    let f_min = U::rounding_from(T::MIN, RoundingMode::Down).0;
    let f_max = U::rounding_from(T::MAX, RoundingMode::Down).0;
    primitive_float_gen_var_15::<U, T>().test_properties(|f| {
        if f >= f_min && f <= f_max {
            let n_floor = T::rounding_from(f, RoundingMode::Floor);
            if let Some(n_ceiling) = n_floor.0.checked_add(T::ONE) {
                let n_ceiling = (n_ceiling, Ordering::Greater);
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
        let ceiling = (floor.0 + T::ONE, Ordering::Greater);
        let nearest = T::rounding_from(f, RoundingMode::Nearest);
        assert_eq!(nearest, if floor.0.even() { floor } else { ceiling });
    });
}

fn rounding_from_helper_primitive_float_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: TryFrom<NiceFloat<T>> + PrimitiveUnsigned + RoundingFrom<T>,
>() {
    unsigned_rounding_mode_pair_gen_var_2::<U, T>().test_properties(|(u, rm)| {
        let o = T::rounding_from(u, rm).1;
        match rm {
            RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
            RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    unsigned_gen_var_18::<U, T>().test_properties(|u| {
        let (f, o) = T::rounding_from(u, RoundingMode::Exact);
        let (f_alt, o_alt) = T::rounding_from(u, RoundingMode::Floor);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(u, RoundingMode::Ceiling);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(u, RoundingMode::Down);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(u, RoundingMode::Up);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(u, RoundingMode::Nearest);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        assert_eq!(
            U::rounding_from(f, RoundingMode::Exact),
            (u, Ordering::Equal)
        );
    });

    if U::WIDTH > T::MANTISSA_WIDTH {
        unsigned_gen_var_19::<U, T>().test_properties(|u| {
            let (f_below, o) = T::rounding_from(u, RoundingMode::Floor);
            assert_eq!(o, Ordering::Less);
            let f_above = f_below.next_higher();
            let (f_alt, o) = T::rounding_from(u, RoundingMode::Ceiling);
            assert_eq!(NiceFloat(f_alt), NiceFloat(f_above));
            assert_eq!(o, Ordering::Greater);
            let (f_alt, o) = T::rounding_from(u, RoundingMode::Down);
            assert_eq!(NiceFloat(f_alt), NiceFloat(f_below));
            assert_eq!(o, Ordering::Less);
            let (f_alt, o) = T::rounding_from(u, RoundingMode::Up);
            assert_eq!(NiceFloat(f_alt), NiceFloat(f_above));
            assert_eq!(o, Ordering::Greater);
            let (f_nearest, o) = T::rounding_from(u, RoundingMode::Nearest);
            assert!(
                (NiceFloat(f_nearest), o) == (NiceFloat(f_below), Ordering::Less)
                    || (NiceFloat(f_nearest), o) == (NiceFloat(f_above), Ordering::Greater)
            );
        });

        unsigned_gen_var_20::<U, T>().test_properties(|u| {
            let (floor, o) = T::rounding_from(u, RoundingMode::Floor);
            assert_eq!(o, Ordering::Less);
            let ceiling = floor.next_higher();
            let (nearest, o) = T::rounding_from(u, RoundingMode::Nearest);
            assert_eq!(
                (NiceFloat(nearest), o),
                if floor.to_bits().even() {
                    (NiceFloat(floor), Ordering::Less)
                } else {
                    (NiceFloat(ceiling), Ordering::Greater)
                }
            );
        });
    }
}

fn rounding_from_helper_primitive_float_signed<
    T: ConvertibleFrom<S> + PrimitiveFloat + RoundingFrom<S>,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: TryFrom<NiceFloat<T>> + PrimitiveSigned + RoundingFrom<T> + WrappingFrom<U>,
>() {
    signed_rounding_mode_pair_gen_var_4::<S, T>().test_properties(|(i, rm)| {
        let o = T::rounding_from(i, rm).1;
        match (i >= S::ZERO, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
    });

    signed_gen_var_7::<S, T>().test_properties(|i| {
        let (f, o) = T::rounding_from(i, RoundingMode::Exact);
        let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Floor);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Ceiling);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Down);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Up);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Nearest);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        assert_eq!(
            S::rounding_from(f, RoundingMode::Exact),
            (i, Ordering::Equal)
        );
    });

    if S::WIDTH > T::MANTISSA_WIDTH {
        signed_gen_var_8::<U, S, T>().test_properties(|i| {
            let (f_below, o) = T::rounding_from(i, RoundingMode::Floor);
            assert_eq!(o, Ordering::Less);
            let f_above = f_below.next_higher();
            let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Ceiling);
            assert_eq!(NiceFloat(f_alt), NiceFloat(f_above));
            assert_eq!(o_alt, Ordering::Greater);
            if i >= S::ZERO {
                let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Down);
                assert_eq!(NiceFloat(f_below), NiceFloat(f_alt));
                assert_eq!(o_alt, Ordering::Less);
                let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Up);
                assert_eq!(NiceFloat(f_above), NiceFloat(f_alt));
                assert_eq!(o_alt, Ordering::Greater);
            } else {
                let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Down);
                assert_eq!(NiceFloat(f_above), NiceFloat(f_alt));
                assert_eq!(o_alt, Ordering::Greater);
                let (f_alt, o_alt) = T::rounding_from(i, RoundingMode::Up);
                assert_eq!(NiceFloat(f_below), NiceFloat(f_alt));
                assert_eq!(o_alt, Ordering::Less);
            }
            let (f_nearest, o_alt) = T::rounding_from(i, RoundingMode::Nearest);
            assert!(
                (NiceFloat(f_nearest), o_alt) == (NiceFloat(f_below), Ordering::Less)
                    || (NiceFloat(f_nearest), o_alt) == (NiceFloat(f_above), Ordering::Greater)
            );
        });

        signed_gen_var_9::<U, S, T>().test_properties(|i| {
            let (floor, o) = T::rounding_from(i, RoundingMode::Floor);
            assert_eq!(o, Ordering::Less);
            let ceiling = floor.next_higher();
            let (nearest, o) = T::rounding_from(i, RoundingMode::Nearest);
            assert_eq!(
                (NiceFloat(nearest), o),
                if floor.to_bits().even() {
                    (NiceFloat(floor), Ordering::Less)
                } else {
                    (NiceFloat(ceiling), Ordering::Greater)
                }
            );
        });
    }
}

#[test]
fn rounding_from_properties() {
    apply_fn_to_unsigneds_and_primitive_floats!(rounding_from_helper_unsigned_primitive_float);
    apply_fn_to_signeds_and_primitive_floats!(rounding_from_helper_signed_primitive_float);
    apply_fn_to_primitive_floats_and_unsigneds!(rounding_from_helper_primitive_float_unsigned);
    apply_fn_to_primitive_floats_and_unsigned_signed_pairs!(
        rounding_from_helper_primitive_float_signed
    );
}
