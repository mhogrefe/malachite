// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::bucketers::{float_size, Bucketer};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use std::cmp::max;

pub fn pair_1_float_complexity_bucketer<T>(var_name: &str) -> Bucketer<(Float, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_pair_1_float_complexity_bucketer<T, U>(var_name: &str) -> Bucketer<(T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, _))| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_float_complexity_bucketer<T>(var_name: &str) -> Bucketer<(T, Float)> {
    Bucketer {
        bucketing_function: &|(_, x)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn triple_1_float_complexity_bucketer<T, U>(var_name: &str) -> Bucketer<(Float, T, U)> {
    Bucketer {
        bucketing_function: &|(x, _, _)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_triple_1_float_complexity_bucketer<T, U, V>(
    var_name: &str,
) -> Bucketer<(V, (Float, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, _, _))| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn float_complexity_bucketer(var_name: &str) -> Bucketer<Float> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_float_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Float)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn triple_1_2_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_2_triple_1_2_float_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (U, (Float, Float, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, _))| {
            usize::exact_from(max(x.complexity(), y.complexity()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn triple_1_2_float_rational_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Rational, T)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_triple_1_2_float_rational_max_complexity_bucketer<'a, T, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (U, (Float, Rational, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, _))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T)> {
    Bucketer {
        bucketing_function: &|(x, z)| usize::exact_from(max!(x.complexity(), z.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits)"),
    }
}

pub fn pair_2_pair_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (U, (Float, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, z))| {
            usize::exact_from(max!(x.complexity(), z.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits)"),
    }
}

pub fn triple_float_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits)"
        ),
    }
}

pub fn pair_2_triple_float_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (U, (Float, Float, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z))| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits)"
        ),
    }
}

pub fn triple_1_2_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, _)| {
            usize::exact_from(max!(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_triple_1_2_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U, V>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (V, (Float, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, _))| {
            usize::exact_from(max!(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt, U>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Float, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits())"
        ),
    }
}

pub fn pair_2_quadruple_1_2_3_float_float_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
    V,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (V, (Float, Float, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, _))| {
            usize::exact_from(max!(x.complexity(), y.complexity(), z.significant_bits()))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.complexity(), {z_name}.significant_bits())"
        ),
    }
}

pub fn quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Rational, T, U)> {
    Bucketer {
        bucketing_function: &|(x, y, z, _)| {
            usize::exact_from(max!(
                x.complexity(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {z_name}.significant_bits())"
        ),
    }
}

pub fn pair_2_quadruple_1_2_3_float_rational_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
    V,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (V, (Float, Rational, T, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z, _))| {
            usize::exact_from(max!(
                x.complexity(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {z_name}.significant_bits())"
        ),
    }
}

pub fn triple_float_rational_primitive_int_max_complexity_bucketer<'a, T: PrimitiveInt>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (Float, Rational, T)> {
    Bucketer {
        bucketing_function: &|(x, y, z)| {
            usize::exact_from(max!(
                x.complexity(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {z_name}.significant_bits)"
        ),
    }
}

pub fn pair_2_triple_float_rational_primitive_int_max_complexity_bucketer<
    'a,
    T: PrimitiveInt,
    U,
>(
    x_name: &'a str,
    y_name: &'a str,
    z_name: &'a str,
) -> Bucketer<'a, (U, (Float, Rational, T))> {
    Bucketer {
        bucketing_function: &|(_, (x, y, z))| {
            usize::exact_from(max!(
                x.complexity(),
                y.significant_bits(),
                z.significant_bits()
            ))
        },
        bucketing_label: format!(
            "max({x_name}.complexity(), {y_name}.significant_bits(), {z_name}.significant_bits)"
        ),
    }
}

pub fn pair_2_pair_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Float))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_float_integer_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Integer)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_natural_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_rational_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Rational)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_integer_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Integer))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_natural_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Natural))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_rational_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Rational))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_unsigned_max_complexity_bucketer<'a, T, U: PrimitiveUnsigned>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_signed_max_complexity_bucketer<'a, T, U: PrimitiveSigned>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_primitive_float_max_complexity_bucketer<'a, T, U: PrimitiveFloat>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), u64::exact_from(float_size(*y))))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_primitive_float_max_complexity_bucketer<'a, T: PrimitiveFloat>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.complexity(), u64::exact_from(float_size(*y))))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn max_triple_1_float_complexity_triple_2_bucketer<'a, T>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (Float, u64, T)> {
    Bucketer {
        bucketing_function: &|(x, p, _)| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn pair_2_max_triple_1_float_complexity_triple_2_bucketer<'a, T, U>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (T, (Float, u64, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, p, _))| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn max_pair_1_complexity_pair_2_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, u64)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), *y)),
        bucketing_label: format!("max({x_name}.complexity(), {y_name})"),
    }
}

pub fn pair_2_max_pair_1_complexity_pair_2_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, u64))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| usize::exact_from(max(x.complexity(), *y)),
        bucketing_label: format!("max({x_name}.complexity(), {y_name})"),
    }
}
