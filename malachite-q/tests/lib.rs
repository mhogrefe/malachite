extern crate itertools;
#[macro_use]
extern crate malachite_base;
#[macro_use]
extern crate malachite_base_test_util;
extern crate malachite_nz;
extern crate malachite_nz_test_util;
extern crate malachite_q;
extern crate malachite_q_test_util;
extern crate num;
extern crate rug;

pub mod arithmetic {
    pub mod abs;
    pub mod add;
    pub mod approximate;
    pub mod ceiling;
    pub mod div;
    pub mod floor;
    pub mod is_power_of_2;
    pub mod log_base;
    pub mod log_base_2;
    pub mod log_base_power_of_2;
    pub mod mul;
    pub mod neg;
    pub mod next_power_of_2;
    pub mod pow;
    pub mod power_of_2;
    pub mod reciprocal;
    pub mod root;
    pub mod round_to_multiple;
    pub mod round_to_multiple_of_power_of_2;
    pub mod shl;
    pub mod shr;
    pub mod sign;
    pub mod simplest_rational_in_interval;
    pub mod sqrt;
    pub mod square;
    pub mod sub;
}
pub mod basic {
    pub mod constants;
    pub mod default;
    pub mod from_numerator_and_denominator;
    pub mod mutate_numerator_or_denominator;
    pub mod named;
    pub mod significant_bits;
    pub mod size;
    pub mod to_numerator_or_denominator;
}
pub mod comparison {
    pub mod cmp;
    pub mod cmp_abs;
    pub mod eq;
    pub mod hash;
    pub mod partial_cmp_abs_integer;
    pub mod partial_cmp_abs_natural;
    pub mod partial_cmp_abs_primitive_float;
    pub mod partial_cmp_abs_primitive_int;
    pub mod partial_cmp_integer;
    pub mod partial_cmp_natural;
    pub mod partial_cmp_primitive_float;
    pub mod partial_cmp_primitive_int;
    pub mod partial_eq_integer;
    pub mod partial_eq_natural;
    pub mod partial_eq_primitive_float;
    pub mod partial_eq_primitive_int;
}
pub mod conversion {
    pub mod clone;
    pub mod continued_fraction {
        pub mod convergents;
        pub mod from_continued_fraction;
        pub mod to_continued_fraction;
    }
    pub mod digits {
        #[allow(clippy::module_inception)]
        pub mod digits;
        pub mod from_digits;
        pub mod from_power_of_2_digits;
        pub mod power_of_2_digits;
        pub mod to_digits;
        pub mod to_power_of_2_digits;
    }
    pub mod floating_point_from_rational;
    pub mod from_float_simplest;
    pub mod from_floating_point;
    pub mod from_integer;
    pub mod from_natural;
    pub mod from_primitive_int;
    pub mod integer_from_rational;
    pub mod is_integer;
    pub mod natural_from_rational;
    pub mod primitive_int_from_rational;
    pub mod sci_mantissa_and_exponent;
    pub mod serde;
    pub mod string {
        pub mod from_sci_string;
        pub mod from_string;
        pub mod to_sci;
        pub mod to_string;
    }
}
pub mod exhaustive {
    pub mod exhaustive_negative_rationals;
    pub mod exhaustive_non_negative_rationals;
    pub mod exhaustive_nonzero_rationals;
    pub mod exhaustive_positive_rationals;
    pub mod exhaustive_rationals;
}
pub mod random {
    pub mod random_negative_rationals;
    pub mod random_non_negative_rationals;
    pub mod random_nonzero_rationals;
    pub mod random_positive_rationals;
    pub mod random_rationals;
    pub mod striped_random_negative_rationals;
    pub mod striped_random_non_negative_rationals;
    pub mod striped_random_nonzero_rationals;
    pub mod striped_random_positive_rationals;
    pub mod striped_random_rationals;
}
