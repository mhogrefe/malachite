// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::exhaustive_primitive_floats_with_sci_exponent_in_range;
use malachite_base::test_util::num::exhaustive::exhaustive_primitive_floats_helper_helper;
use std::panic::catch_unwind;

fn exhaustive_primitive_floats_with_sci_exponent_in_range_helper<T: PrimitiveFloat>(
    a: T,
    b: T,
    sci_exponent: i64,
    out: &[T],
) {
    exhaustive_primitive_floats_helper_helper(
        exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(a, b, sci_exponent),
        out,
    );
}

#[test]
fn test_exhaustive_primitive_floats_with_sci_exponent_in_range() {
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        1,
        &[
            3.0, 2.75, 2.875, 3.125, 2.8125, 2.9375, 3.0625, 2.71875, 2.78125, 2.84375, 2.90625,
            2.96875, 3.03125, 3.09375, 2.734375, 2.765625, 2.796875, 2.828125, 2.859375, 2.890625,
            2.921875, 2.953125, 2.984375, 3.015625, 3.046875, 3.078125, 3.109375, 3.140625,
            2.7265625, 2.7421875, 2.7578125, 2.7734375, 2.7890625, 2.8046875, 2.8203125, 2.8359375,
            2.8515625, 2.8671875, 2.8828125, 2.8984375, 2.9140625, 2.9296875, 2.9453125, 2.9609375,
            2.9765625, 2.9921875, 3.0078125, 3.0234375, 3.0390625, 3.0546875,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f32>(
        1900.0,
        2000.0,
        10,
        &[
            1920.0, 1984.0, 1952.0, 1904.0, 1936.0, 1968.0, 2000.0, 1912.0, 1928.0, 1944.0, 1960.0,
            1976.0, 1992.0, 1900.0, 1908.0, 1916.0, 1924.0, 1932.0, 1940.0, 1948.0, 1956.0, 1964.0,
            1972.0, 1980.0, 1988.0, 1996.0, 1902.0, 1906.0, 1910.0, 1914.0, 1918.0, 1922.0, 1926.0,
            1930.0, 1934.0, 1938.0, 1942.0, 1946.0, 1950.0, 1954.0, 1958.0, 1962.0, 1966.0, 1970.0,
            1974.0, 1978.0, 1982.0, 1986.0, 1990.0, 1994.0,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f32>(
        7.0e-45,
        1.0e-44,
        -147,
        &[8.0e-45, 7.0e-45, 1.0e-44],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f32>(
        1000000.0,
        1000001.0,
        19,
        &[
            1000000.0, 1000001.0, 1000000.5, 1000000.25, 1000000.75, 1000000.1, 1000000.4,
            1000000.6, 1000000.9, 1000000.06, 1000000.2, 1000000.3, 1000000.44, 1000000.56,
            1000000.7, 1000000.8, 1000000.94,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f32>(
        1.0,
        1.99,
        0,
        &[
            1.0, 1.5, 1.25, 1.75, 1.125, 1.375, 1.625, 1.875, 1.0625, 1.1875, 1.3125, 1.4375,
            1.5625, 1.6875, 1.8125, 1.9375, 1.03125, 1.09375, 1.15625, 1.21875, 1.28125, 1.34375,
            1.40625, 1.46875, 1.53125, 1.59375, 1.65625, 1.71875, 1.78125, 1.84375, 1.90625,
            1.96875, 1.015625, 1.046875, 1.078125, 1.109375, 1.140625, 1.171875, 1.203125,
            1.234375, 1.265625, 1.296875, 1.328125, 1.359375, 1.390625, 1.421875, 1.453125,
            1.484375, 1.515625, 1.546875,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f32>(5.0, 5.0, 2, &[5.0]);

    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        1,
        &[
            3.0, 2.75, 2.875, 3.125, 2.8125, 2.9375, 3.0625, 2.71875, 2.78125, 2.84375, 2.90625,
            2.96875, 3.03125, 3.09375, 2.734375, 2.765625, 2.796875, 2.828125, 2.859375, 2.890625,
            2.921875, 2.953125, 2.984375, 3.015625, 3.046875, 3.078125, 3.109375, 3.140625,
            2.7265625, 2.7421875, 2.7578125, 2.7734375, 2.7890625, 2.8046875, 2.8203125, 2.8359375,
            2.8515625, 2.8671875, 2.8828125, 2.8984375, 2.9140625, 2.9296875, 2.9453125, 2.9609375,
            2.9765625, 2.9921875, 3.0078125, 3.0234375, 3.0390625, 3.0546875,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f64>(
        1900.0,
        2000.0,
        10,
        &[
            1920.0, 1984.0, 1952.0, 1904.0, 1936.0, 1968.0, 2000.0, 1912.0, 1928.0, 1944.0, 1960.0,
            1976.0, 1992.0, 1900.0, 1908.0, 1916.0, 1924.0, 1932.0, 1940.0, 1948.0, 1956.0, 1964.0,
            1972.0, 1980.0, 1988.0, 1996.0, 1902.0, 1906.0, 1910.0, 1914.0, 1918.0, 1922.0, 1926.0,
            1930.0, 1934.0, 1938.0, 1942.0, 1946.0, 1950.0, 1954.0, 1958.0, 1962.0, 1966.0, 1970.0,
            1974.0, 1978.0, 1982.0, 1986.0, 1990.0, 1994.0,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f64>(
        7.0e-45,
        1.0e-44,
        -147,
        &[
            8.407790785948902e-45,
            7.006492321624085e-45,
            9.80908925027372e-45,
            7.707141553786494e-45,
            9.108440018111311e-45,
            7.35681693770529e-45,
            8.057466169867698e-45,
            8.758115402030107e-45,
            9.458764634192515e-45,
            7.181654629664687e-45,
            7.531979245745892e-45,
            7.882303861827096e-45,
            8.2326284779083e-45,
            8.582953093989505e-45,
            8.933277710070709e-45,
            9.283602326151913e-45,
            9.633926942233117e-45,
            9.984251558314322e-45,
            7.094073475644386e-45,
            7.269235783684989e-45,
            7.444398091725591e-45,
            7.619560399766193e-45,
            7.794722707806795e-45,
            7.969885015847397e-45,
            8.145047323887999e-45,
            8.320209631928601e-45,
            8.495371939969203e-45,
            8.670534248009806e-45,
            8.845696556050408e-45,
            9.02085886409101e-45,
            9.196021172131612e-45,
            9.371183480172214e-45,
            9.546345788212816e-45,
            9.721508096253418e-45,
            9.89667040429402e-45,
            7.050282898634236e-45,
            7.137864052654537e-45,
            7.225445206674838e-45,
            7.313026360695139e-45,
            7.40060751471544e-45,
            7.488188668735741e-45,
            7.575769822756042e-45,
            7.663350976776343e-45,
            7.750932130796644e-45,
            7.838513284816945e-45,
            7.926094438837247e-45,
            8.013675592857548e-45,
            8.101256746877849e-45,
            8.18883790089815e-45,
            8.276419054918451e-45,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f64>(
        1000000.0,
        1000001.0,
        19,
        &[
            1000000.0,
            1000001.0,
            1000000.5,
            1000000.25,
            1000000.75,
            1000000.125,
            1000000.375,
            1000000.625,
            1000000.875,
            1000000.0625,
            1000000.1875,
            1000000.3125,
            1000000.4375,
            1000000.5625,
            1000000.6875,
            1000000.8125,
            1000000.9375,
            1000000.03125,
            1000000.09375,
            1000000.15625,
            1000000.21875,
            1000000.28125,
            1000000.34375,
            1000000.40625,
            1000000.46875,
            1000000.53125,
            1000000.59375,
            1000000.65625,
            1000000.71875,
            1000000.78125,
            1000000.84375,
            1000000.90625,
            1000000.96875,
            1000000.015625,
            1000000.046875,
            1000000.078125,
            1000000.109375,
            1000000.140625,
            1000000.171875,
            1000000.203125,
            1000000.234375,
            1000000.265625,
            1000000.296875,
            1000000.328125,
            1000000.359375,
            1000000.390625,
            1000000.421875,
            1000000.453125,
            1000000.484375,
            1000000.515625,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f64>(
        1.0,
        1.99,
        0,
        &[
            1.0, 1.5, 1.25, 1.75, 1.125, 1.375, 1.625, 1.875, 1.0625, 1.1875, 1.3125, 1.4375,
            1.5625, 1.6875, 1.8125, 1.9375, 1.03125, 1.09375, 1.15625, 1.21875, 1.28125, 1.34375,
            1.40625, 1.46875, 1.53125, 1.59375, 1.65625, 1.71875, 1.78125, 1.84375, 1.90625,
            1.96875, 1.015625, 1.046875, 1.078125, 1.109375, 1.140625, 1.171875, 1.203125,
            1.234375, 1.265625, 1.296875, 1.328125, 1.359375, 1.390625, 1.421875, 1.453125,
            1.484375, 1.515625, 1.546875,
        ],
    );
    exhaustive_primitive_floats_with_sci_exponent_in_range_helper::<f64>(5.0, 5.0, 2, &[5.0]);
}

fn exhaustive_primitive_floats_with_sci_exponent_in_range_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(
        T::from(1.1),
        T::from(1.2),
        10000,
    ));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(
        T::from(1.1),
        T::from(1.2),
        -10000,
    ));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(
        T::from(-1.2),
        T::from(1.1),
        0,
    )
    .for_each(|_| {}));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(
        T::ONE,
        T::INFINITY,
        0,
    )
    .for_each(|_| {}));
    assert_panic!(
        exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(T::ONE, T::NAN, 0)
            .for_each(|_| {})
    );
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(
        T::from(1.2),
        T::from(1.1),
        0,
    )
    .for_each(|_| {}));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(
        T::from(1.1),
        T::from(1.2),
        1,
    )
    .for_each(|_| {}));
    assert_panic!(exhaustive_primitive_floats_with_sci_exponent_in_range::<T>(
        T::from(0.1),
        T::from(1.2),
        1,
    )
    .for_each(|_| {}));
}

#[test]
fn exhaustive_primitive_floats_with_sci_exponent_in_range_fail() {
    apply_fn_to_primitive_floats!(
        exhaustive_primitive_floats_with_sci_exponent_in_range_fail_helper
    );
}
