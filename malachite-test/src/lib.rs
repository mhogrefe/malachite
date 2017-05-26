extern crate malachite_gmp;
extern crate malachite_native;
extern crate num;
extern crate rugint;
extern crate rust_wheels;

pub mod common;
pub mod integer {
    pub mod arithmetic {
        pub mod add_i32;
        pub mod add_u32;
        pub mod sub_i32;
        pub mod sub_u32;
    }
    pub mod conversion {
        pub mod assign_i32;
        pub mod assign_u32;
    }
    pub mod comparison {
        pub mod partial_cmp_i32;
        pub mod partial_cmp_u32;
        pub mod partial_eq_i32;
        pub mod partial_eq_u32;
        pub mod sign;
    }
}
pub mod natural {
    pub mod arithmetic {
        pub mod add;
        pub mod add_u32;
    }
    pub mod comparison {
        pub mod partial_cmp_u32;
        pub mod partial_eq_u32;
    }
    pub mod conversion {
        pub mod assign_u32;
        pub mod from_u32;
        pub mod from_u64;
        pub mod to_u32;
    }
    pub mod logic {
        pub mod get_bit;
        pub mod set_bit;
    }
}
