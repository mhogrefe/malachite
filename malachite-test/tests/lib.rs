extern crate malachite_gmp;
extern crate malachite_native;
extern crate malachite_test;
extern crate num;
extern crate rugint;

pub mod integer {
    pub mod basic {
        pub mod new_and_default;
    }
    pub mod comparison {
        pub mod partial_eq_i32;
        pub mod partial_eq_u32;
        pub mod sign;
    }
    pub mod conversion {
        pub mod assign_i32;
        pub mod assign_u32;
        pub mod clone_and_assign_integer;
        pub mod from_i32;
        pub mod from_u32;
    }
    pub mod logic {
        pub mod significant_bits;
    }
}
pub mod natural {
    pub mod basic {
        pub mod new_and_default;
    }
    pub mod comparison {
        pub mod partial_eq_u32;
    }
    pub mod conversion {
        pub mod assign_u32;
        pub mod clone_and_assign_natural;
        pub mod from_u32;
    }
    pub mod logic {
        pub mod limb_count;
        pub mod limbs_le;
        pub mod significant_bits;
    }
}
