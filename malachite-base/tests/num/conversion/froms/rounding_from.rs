// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    primitive_float_gen_var_13, primitive_float_gen_var_14, primitive_float_gen_var_15,
    primitive_float_gen_var_16, primitive_float_gen_var_17,
    primitive_float_rounding_mode_pair_gen_var_3, signed_gen_var_7, signed_gen_var_8,
    signed_gen_var_9, signed_rounding_mode_pair_gen_var_4, unsigned_gen_var_18,
    unsigned_gen_var_19, unsigned_gen_var_20, unsigned_rounding_mode_pair_gen_var_2,
};
use std::cmp::Ordering::{self, *};
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
    test_from_floating_point::<f32, u8>(0.0, Down, 0, Equal);
    test_from_floating_point::<f32, u8>(0.0, Floor, 0, Equal);
    test_from_floating_point::<f32, u8>(0.0, Up, 0, Equal);
    test_from_floating_point::<f32, u8>(0.0, Ceiling, 0, Equal);
    test_from_floating_point::<f32, u8>(0.0, Nearest, 0, Equal);
    test_from_floating_point::<f32, u8>(0.0, Exact, 0, Equal);

    test_from_floating_point::<f32, u8>(-0.0, Down, 0, Equal);
    test_from_floating_point::<f32, u8>(-0.0, Floor, 0, Equal);
    test_from_floating_point::<f32, u8>(-0.0, Up, 0, Equal);
    test_from_floating_point::<f32, u8>(-0.0, Ceiling, 0, Equal);
    test_from_floating_point::<f32, u8>(-0.0, Nearest, 0, Equal);
    test_from_floating_point::<f32, u8>(-0.0, Exact, 0, Equal);

    test_from_floating_point::<f32, u8>(100.0, Down, 100, Equal);
    test_from_floating_point::<f32, u8>(100.0, Floor, 100, Equal);
    test_from_floating_point::<f32, u8>(100.0, Up, 100, Equal);
    test_from_floating_point::<f32, u8>(100.0, Ceiling, 100, Equal);
    test_from_floating_point::<f32, u8>(100.0, Nearest, 100, Equal);
    test_from_floating_point::<f32, u8>(100.0, Exact, 100, Equal);

    test_from_floating_point::<f32, u8>(100.1, Down, 100, Less);
    test_from_floating_point::<f32, u8>(100.1, Floor, 100, Less);
    test_from_floating_point::<f32, u8>(100.1, Up, 101, Greater);
    test_from_floating_point::<f32, u8>(100.1, Ceiling, 101, Greater);
    test_from_floating_point::<f32, u8>(100.1, Nearest, 100, Less);

    test_from_floating_point::<f32, u8>(100.9, Down, 100, Less);
    test_from_floating_point::<f32, u8>(100.9, Floor, 100, Less);
    test_from_floating_point::<f32, u8>(100.9, Up, 101, Greater);
    test_from_floating_point::<f32, u8>(100.9, Ceiling, 101, Greater);
    test_from_floating_point::<f32, u8>(100.9, Nearest, 101, Greater);

    test_from_floating_point::<f32, u8>(100.5, Down, 100, Less);
    test_from_floating_point::<f32, u8>(100.5, Floor, 100, Less);
    test_from_floating_point::<f32, u8>(100.5, Up, 101, Greater);
    test_from_floating_point::<f32, u8>(100.5, Ceiling, 101, Greater);
    test_from_floating_point::<f32, u8>(100.5, Nearest, 100, Less);

    test_from_floating_point::<f32, u8>(101.5, Down, 101, Less);
    test_from_floating_point::<f32, u8>(101.5, Floor, 101, Less);
    test_from_floating_point::<f32, u8>(101.5, Up, 102, Greater);
    test_from_floating_point::<f32, u8>(101.5, Ceiling, 102, Greater);
    test_from_floating_point::<f32, u8>(101.5, Nearest, 102, Greater);

    test_from_floating_point::<f32, u8>(256.0, Down, 255, Less);
    test_from_floating_point::<f32, u8>(256.0, Floor, 255, Less);
    test_from_floating_point::<f32, u8>(256.0, Nearest, 255, Less);

    test_from_floating_point::<f32, u8>(-100.0, Down, 0, Greater);
    test_from_floating_point::<f32, u8>(-100.0, Ceiling, 0, Greater);
    test_from_floating_point::<f32, u8>(-100.0, Nearest, 0, Greater);

    test_from_floating_point::<f32, i8>(128.0, Down, 127, Less);
    test_from_floating_point::<f32, i8>(128.0, Floor, 127, Less);
    test_from_floating_point::<f32, i8>(128.0, Nearest, 127, Less);

    test_from_floating_point::<f32, i8>(-129.0, Down, -128, Greater);
    test_from_floating_point::<f32, i8>(-129.0, Ceiling, -128, Greater);
    test_from_floating_point::<f32, i8>(-129.0, Nearest, -128, Greater);

    test_from_floating_point::<f32, u8>(f32::INFINITY, Down, 255, Less);
    test_from_floating_point::<f32, u8>(f32::INFINITY, Floor, 255, Less);
    test_from_floating_point::<f32, u8>(f32::INFINITY, Nearest, 255, Less);
    test_from_floating_point::<f32, u8>(f32::NEGATIVE_INFINITY, Down, 0, Greater);
    test_from_floating_point::<f32, u8>(f32::NEGATIVE_INFINITY, Ceiling, 0, Greater);
    test_from_floating_point::<f32, u8>(f32::NEGATIVE_INFINITY, Nearest, 0, Greater);

    test_from_floating_point::<f32, i8>(f32::INFINITY, Down, 127, Less);
    test_from_floating_point::<f32, i8>(f32::INFINITY, Floor, 127, Less);
    test_from_floating_point::<f32, i8>(f32::INFINITY, Nearest, 127, Less);
    test_from_floating_point::<f32, i8>(f32::NEGATIVE_INFINITY, Down, -128, Greater);
    test_from_floating_point::<f32, i8>(f32::NEGATIVE_INFINITY, Ceiling, -128, Greater);
    test_from_floating_point::<f32, i8>(f32::NEGATIVE_INFINITY, Nearest, -128, Greater);

    fn test_from_primitive_int<T: PrimitiveInt, U: PrimitiveFloat + RoundingFrom<T>>(
        n_in: T,
        rm: RoundingMode,
        n_out: U,
        o: Ordering,
    ) {
        let (x, actual_o) = U::rounding_from(n_in, rm);
        assert_eq!((NiceFloat(x), actual_o), (NiceFloat(n_out), o));
    }
    test_from_primitive_int::<u8, f32>(0, Down, 0.0, Equal);
    test_from_primitive_int::<u8, f32>(0, Floor, 0.0, Equal);
    test_from_primitive_int::<u8, f32>(0, Up, 0.0, Equal);
    test_from_primitive_int::<u8, f32>(0, Ceiling, 0.0, Equal);
    test_from_primitive_int::<u8, f32>(0, Nearest, 0.0, Equal);
    test_from_primitive_int::<u8, f32>(0, Exact, 0.0, Equal);

    test_from_primitive_int::<u8, f32>(100, Down, 100.0, Equal);
    test_from_primitive_int::<u8, f32>(100, Floor, 100.0, Equal);
    test_from_primitive_int::<u8, f32>(100, Up, 100.0, Equal);
    test_from_primitive_int::<u8, f32>(100, Ceiling, 100.0, Equal);
    test_from_primitive_int::<u8, f32>(100, Nearest, 100.0, Equal);
    test_from_primitive_int::<u8, f32>(100, Exact, 100.0, Equal);

    test_from_primitive_int::<i8, f32>(-100, Down, -100.0, Equal);
    test_from_primitive_int::<i8, f32>(-100, Floor, -100.0, Equal);
    test_from_primitive_int::<i8, f32>(-100, Up, -100.0, Equal);
    test_from_primitive_int::<i8, f32>(-100, Ceiling, -100.0, Equal);
    test_from_primitive_int::<i8, f32>(-100, Nearest, -100.0, Equal);
    test_from_primitive_int::<i8, f32>(-100, Exact, -100.0, Equal);

    test_from_primitive_int::<i32, f32>(i32::MIN, Down, -2147483600.0, Equal);
    test_from_primitive_int::<i32, f32>(i32::MIN, Floor, -2147483600.0, Equal);
    test_from_primitive_int::<i32, f32>(i32::MIN, Up, -2147483600.0, Equal);
    test_from_primitive_int::<i32, f32>(i32::MIN, Ceiling, -2147483600.0, Equal);
    test_from_primitive_int::<i32, f32>(i32::MIN, Nearest, -2147483600.0, Equal);
    test_from_primitive_int::<i32, f32>(i32::MIN, Exact, -2147483600.0, Equal);

    test_from_primitive_int::<i32, f32>(i32::MAX, Down, 2147483500.0, Less);
    test_from_primitive_int::<i32, f32>(i32::MAX, Floor, 2147483500.0, Less);
    test_from_primitive_int::<i32, f32>(i32::MAX, Up, 2147483600.0, Greater);
    test_from_primitive_int::<i32, f32>(i32::MAX, Ceiling, 2147483600.0, Greater);
    test_from_primitive_int::<i32, f32>(i32::MAX, Nearest, 2147483600.0, Greater);

    test_from_primitive_int::<u128, f32>(u128::MAX, Down, 3.4028235e38, Less);
    test_from_primitive_int::<u128, f32>(u128::MAX, Floor, 3.4028235e38, Less);
    test_from_primitive_int::<u128, f32>(u128::MAX, Up, f32::INFINITY, Greater);
    test_from_primitive_int::<u128, f32>(u128::MAX, Ceiling, f32::INFINITY, Greater);
    test_from_primitive_int::<u128, f32>(u128::MAX, Nearest, 3.4028235e38, Less);
}

#[test]
fn exact_from_fail() {
    assert_panic!(u8::rounding_from(100.1f32, Exact));

    assert_panic!(u8::rounding_from(256.0f32, Exact));
    assert_panic!(u8::rounding_from(256.0f32, Up));
    assert_panic!(u8::rounding_from(256.0f32, Ceiling));
    assert_panic!(u8::rounding_from(-100.0f32, Exact));
    assert_panic!(u8::rounding_from(-100.0f32, Up));
    assert_panic!(u8::rounding_from(-100.0f32, Floor));

    assert_panic!(i8::rounding_from(128.0f32, Exact));
    assert_panic!(i8::rounding_from(128.0f32, Up));
    assert_panic!(i8::rounding_from(128.0f32, Ceiling));
    assert_panic!(i8::rounding_from(-129.0f32, Exact));
    assert_panic!(i8::rounding_from(-129.0f32, Up));
    assert_panic!(i8::rounding_from(-129.0f32, Floor));

    assert_panic!(u8::rounding_from(f32::NAN, Down));
    assert_panic!(u8::rounding_from(f32::NAN, Floor));
    assert_panic!(u8::rounding_from(f32::NAN, Up));
    assert_panic!(u8::rounding_from(f32::NAN, Ceiling));
    assert_panic!(u8::rounding_from(f32::NAN, Nearest));
    assert_panic!(u8::rounding_from(f32::NAN, Exact));

    assert_panic!(u8::rounding_from(f32::INFINITY, Up));
    assert_panic!(u8::rounding_from(f32::INFINITY, Ceiling));
    assert_panic!(u8::rounding_from(f32::INFINITY, Exact));
    assert_panic!(u8::rounding_from(f32::NEGATIVE_INFINITY, Up));
    assert_panic!(u8::rounding_from(f32::NEGATIVE_INFINITY, Floor));
    assert_panic!(u8::rounding_from(f32::NEGATIVE_INFINITY, Exact));

    assert_panic!(i8::rounding_from(f32::INFINITY, Up));
    assert_panic!(i8::rounding_from(f32::INFINITY, Ceiling));
    assert_panic!(i8::rounding_from(f32::INFINITY, Exact));
    assert_panic!(i8::rounding_from(f32::NEGATIVE_INFINITY, Up));
    assert_panic!(i8::rounding_from(f32::NEGATIVE_INFINITY, Floor));
    assert_panic!(i8::rounding_from(f32::NEGATIVE_INFINITY, Exact));

    assert_panic!(u8::rounding_from(f32::NAN, Down));

    assert_panic!(f32::rounding_from(u32::MAX, Exact));
    assert_panic!(f32::rounding_from(u128::MAX, Exact));
}

fn rounding_from_helper_unsigned_primitive_float<
    T: ConvertibleFrom<U> + PrimitiveUnsigned + RoundingFrom<U>,
    U: PrimitiveFloat + RoundingFrom<T>,
>()
where
    NiceFloat<U>: TryFrom<T>,
{
    primitive_float_rounding_mode_pair_gen_var_3::<U, T>().test_properties(|(f, rm)| {
        let (rounded, o) = T::rounding_from(f, rm);
        match (f >= U::ZERO, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                assert_eq!(T::rounding_from(f, rm), (rounded, Equal));
            }
        } else {
            assert_panic!(T::rounding_from(f, Exact));
        }
    });

    primitive_float_gen_var_13::<U, T>().test_properties(|f| {
        let no = T::rounding_from(f, Exact);
        assert_eq!(no, T::rounding_from(f, Floor));
        assert_eq!(no, T::rounding_from(f, Ceiling));
        assert_eq!(no, T::rounding_from(f, Down));
        assert_eq!(no, T::rounding_from(f, Up));
        assert_eq!(no, T::rounding_from(f, Nearest));
    });

    let f_max = U::rounding_from(T::MAX, Down).0;
    primitive_float_gen_var_15::<U, T>().test_properties(|f| {
        if f >= U::ZERO && f <= f_max {
            let n_floor = T::rounding_from(f, Floor);
            assert_eq!(n_floor.1, Less);
            if let Some(n_ceiling) = n_floor.0.checked_add(T::ONE) {
                let n_ceiling = (n_ceiling, Greater);
                assert_eq!(n_ceiling, T::rounding_from(f, Ceiling));
                assert_eq!(n_floor, T::rounding_from(f, Down));
                assert_eq!(n_ceiling, T::rounding_from(f, Up));
                let n_nearest = T::rounding_from(f, Nearest);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
            }
        }
    });

    primitive_float_gen_var_16::<U, T>().test_properties(|f| {
        let floor = T::rounding_from(f, Floor);
        let ceiling = (floor.0 + T::ONE, Greater);
        let nearest = T::rounding_from(f, Nearest);
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
        let (rounded, o) = T::rounding_from(f, rm);
        match (f >= U::ZERO, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                assert_eq!(T::rounding_from(f, rm), (rounded, Equal));
            }
        } else {
            assert_panic!(T::rounding_from(f, Exact));
        }
    });

    primitive_float_gen_var_14::<U, T>().test_properties(|f| {
        let no = T::rounding_from(f, Exact);
        assert_eq!(no, T::rounding_from(f, Floor));
        assert_eq!(no, T::rounding_from(f, Ceiling));
        assert_eq!(no, T::rounding_from(f, Down));
        assert_eq!(no, T::rounding_from(f, Up));
        assert_eq!(no, T::rounding_from(f, Nearest));
    });

    let f_min = U::rounding_from(T::MIN, Down).0;
    let f_max = U::rounding_from(T::MAX, Down).0;
    primitive_float_gen_var_15::<U, T>().test_properties(|f| {
        if f >= f_min && f <= f_max {
            let n_floor = T::rounding_from(f, Floor);
            if let Some(n_ceiling) = n_floor.0.checked_add(T::ONE) {
                let n_ceiling = (n_ceiling, Greater);
                assert_eq!(n_ceiling, T::rounding_from(f, Ceiling));
                if f >= U::ZERO {
                    assert_eq!(n_floor, T::rounding_from(f, Down));
                    assert_eq!(n_ceiling, T::rounding_from(f, Up));
                } else {
                    assert_eq!(n_ceiling, T::rounding_from(f, Down));
                    assert_eq!(n_floor, T::rounding_from(f, Up));
                }
                let n_nearest = T::rounding_from(f, Nearest);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
            }
        }
    });

    primitive_float_gen_var_17::<U, T>().test_properties(|f| {
        let floor = T::rounding_from(f, Floor);
        let ceiling = (floor.0 + T::ONE, Greater);
        let nearest = T::rounding_from(f, Nearest);
        assert_eq!(nearest, if floor.0.even() { floor } else { ceiling });
    });
}

fn rounding_from_helper_primitive_float_unsigned<
    T: ConvertibleFrom<U> + PrimitiveFloat + RoundingFrom<U>,
    U: TryFrom<NiceFloat<T>> + PrimitiveUnsigned + RoundingFrom<T>,
>() {
    unsigned_rounding_mode_pair_gen_var_2::<U, T>().test_properties(|(u, rm)| {
        let (rounded, o) = T::rounding_from(u, rm);
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                assert_eq!(T::rounding_from(u, rm), (rounded, Equal));
            }
        } else {
            assert_panic!(T::rounding_from(u, Exact));
        }
    });

    unsigned_gen_var_18::<U, T>().test_properties(|u| {
        let (f, o) = T::rounding_from(u, Exact);
        let (f_alt, o_alt) = T::rounding_from(u, Floor);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(u, Ceiling);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(u, Down);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(u, Up);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(u, Nearest);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        assert_eq!(U::rounding_from(f, Exact), (u, Equal));
    });

    if U::WIDTH > T::MANTISSA_WIDTH {
        unsigned_gen_var_19::<U, T>().test_properties(|u| {
            let (f_below, o) = T::rounding_from(u, Floor);
            assert_eq!(o, Less);
            let f_above = f_below.next_higher();
            let (f_alt, o) = T::rounding_from(u, Ceiling);
            assert_eq!(NiceFloat(f_alt), NiceFloat(f_above));
            assert_eq!(o, Greater);
            let (f_alt, o) = T::rounding_from(u, Down);
            assert_eq!(NiceFloat(f_alt), NiceFloat(f_below));
            assert_eq!(o, Less);
            let (f_alt, o) = T::rounding_from(u, Up);
            assert_eq!(NiceFloat(f_alt), NiceFloat(f_above));
            assert_eq!(o, Greater);
            let (f_nearest, o) = T::rounding_from(u, Nearest);
            assert!(
                (NiceFloat(f_nearest), o) == (NiceFloat(f_below), Less)
                    || (NiceFloat(f_nearest), o) == (NiceFloat(f_above), Greater)
            );
        });

        unsigned_gen_var_20::<U, T>().test_properties(|u| {
            let (floor, o) = T::rounding_from(u, Floor);
            assert_eq!(o, Less);
            let ceiling = floor.next_higher();
            let (nearest, o) = T::rounding_from(u, Nearest);
            assert_eq!(
                (NiceFloat(nearest), o),
                if floor.to_bits().even() {
                    (NiceFloat(floor), Less)
                } else {
                    (NiceFloat(ceiling), Greater)
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
        let (rounded, o) = T::rounding_from(i, rm);
        match (i >= S::ZERO, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                assert_eq!(T::rounding_from(i, rm), (rounded, Equal));
            }
        } else {
            assert_panic!(T::rounding_from(i, Exact));
        }
    });

    signed_gen_var_7::<S, T>().test_properties(|i| {
        let (f, o) = T::rounding_from(i, Exact);
        let (f_alt, o_alt) = T::rounding_from(i, Floor);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(i, Ceiling);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(i, Down);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(i, Up);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        let (f_alt, o_alt) = T::rounding_from(i, Nearest);
        assert_eq!(NiceFloat(f_alt), NiceFloat(f));
        assert_eq!(o_alt, o);
        assert_eq!(S::rounding_from(f, Exact), (i, Equal));
    });

    if S::WIDTH > T::MANTISSA_WIDTH {
        signed_gen_var_8::<U, S, T>().test_properties(|i| {
            let (f_below, o) = T::rounding_from(i, Floor);
            assert_eq!(o, Less);
            let f_above = f_below.next_higher();
            let (f_alt, o_alt) = T::rounding_from(i, Ceiling);
            assert_eq!(NiceFloat(f_alt), NiceFloat(f_above));
            assert_eq!(o_alt, Greater);
            if i >= S::ZERO {
                let (f_alt, o_alt) = T::rounding_from(i, Down);
                assert_eq!(NiceFloat(f_below), NiceFloat(f_alt));
                assert_eq!(o_alt, Less);
                let (f_alt, o_alt) = T::rounding_from(i, Up);
                assert_eq!(NiceFloat(f_above), NiceFloat(f_alt));
                assert_eq!(o_alt, Greater);
            } else {
                let (f_alt, o_alt) = T::rounding_from(i, Down);
                assert_eq!(NiceFloat(f_above), NiceFloat(f_alt));
                assert_eq!(o_alt, Greater);
                let (f_alt, o_alt) = T::rounding_from(i, Up);
                assert_eq!(NiceFloat(f_below), NiceFloat(f_alt));
                assert_eq!(o_alt, Less);
            }
            let (f_nearest, o_alt) = T::rounding_from(i, Nearest);
            assert!(
                (NiceFloat(f_nearest), o_alt) == (NiceFloat(f_below), Less)
                    || (NiceFloat(f_nearest), o_alt) == (NiceFloat(f_above), Greater)
            );
        });

        signed_gen_var_9::<U, S, T>().test_properties(|i| {
            let (floor, o) = T::rounding_from(i, Floor);
            assert_eq!(o, Less);
            let ceiling = floor.next_higher();
            let (nearest, o) = T::rounding_from(i, Nearest);
            assert_eq!(
                (NiceFloat(nearest), o),
                if floor.to_bits().even() {
                    (NiceFloat(floor), Less)
                } else {
                    (NiceFloat(ceiling), Greater)
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
