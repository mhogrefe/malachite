extern crate malachite;
extern crate malachite_gmp;
extern crate malachite_native;
extern crate malachite_test;
extern crate num;
extern crate rand;
extern crate rugint;
extern crate rust_wheels;

pub mod common;
pub mod integer {
    pub mod arithmetic {
        pub mod abs;
        pub mod add;
        pub mod add_i32;
        pub mod add_u32;
        pub mod even_odd;
        pub mod neg;
        pub mod sub_i32;
        pub mod sub_u32;
    }
    pub mod basic {
        pub mod new_and_default;
    }
    pub mod comparison {
        pub mod eq;
        pub mod ord;
        pub mod partial_eq_i32;
        pub mod partial_eq_natural;
        pub mod partial_eq_u32;
        pub mod partial_ord_i32;
        pub mod partial_ord_natural;
        pub mod partial_ord_u32;
        pub mod sign;
    }
    pub mod conversion {
        pub mod assign_i32;
        pub mod assign_natural;
        pub mod assign_u32;
        pub mod clone_and_assign_integer;
        pub mod from_i32;
        pub mod from_u32;
        pub mod into_natural;
        pub mod to_i32;
        pub mod to_u32;
    }
    pub mod logic {
        pub mod get_bit;
        pub mod significant_bits;
    }
}
pub mod natural {
    pub mod arithmetic {
        pub mod add;
        pub mod add_u32;
        pub mod even_odd;
        pub mod shl_u32;
        pub mod sub;
        pub mod sub_u32;
        pub mod is_power_of_two;
    }
    pub mod basic {
        pub mod new_and_default;
    }
    pub mod comparison {
        pub mod eq;
        pub mod ord;
        pub mod partial_eq_integer;
        pub mod partial_eq_u32;
        pub mod partial_ord_integer;
        pub mod partial_ord_u32;
    }
    pub mod conversion {
        pub mod assign_integer;
        pub mod assign_u32;
        pub mod clone_and_assign_natural;
        pub mod from_u32;
        pub mod from_u64;
        pub mod into_integer;
        pub mod to_u32;
    }
    pub mod logic {
        pub mod assign_limbs_le;
        pub mod get_bit;
        pub mod limb_count;
        pub mod limbs_le;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod trailing_zeros;
    }
    pub mod random {
        pub mod assign_random_bits;
        pub mod assign_random_up_to_bits;
        pub mod random_below;
    }
}
