extern crate malachite_test;

use malachite_test::integer::arithmetic::abs::*;
use malachite_test::integer::arithmetic::neg::*;
use malachite_test::integer::comparison::eq::*;
use malachite_test::integer::comparison::hash::*;
use malachite_test::integer::comparison::ord::*;
use malachite_test::integer::comparison::ord_abs::*;
use malachite_test::integer::comparison::partial_ord_abs_i32::*;
use malachite_test::integer::comparison::partial_ord_abs_natural::*;
use malachite_test::integer::comparison::partial_ord_abs_u32::*;
use malachite_test::integer::comparison::partial_ord_i32::*;
use malachite_test::integer::comparison::partial_ord_natural::*;
use malachite_test::integer::comparison::partial_ord_u32::*;
use malachite_test::integer::comparison::partial_eq_i32::*;
use malachite_test::integer::comparison::partial_eq_natural::*;
use malachite_test::integer::comparison::partial_eq_u32::*;
use malachite_test::integer::comparison::sign::*;
use malachite_test::integer::conversion::assign_i32::*;
use malachite_test::integer::conversion::assign_i64::*;
use malachite_test::integer::conversion::assign_natural::*;
use malachite_test::integer::conversion::assign_u32::*;
use malachite_test::integer::conversion::assign_u64::*;
use malachite_test::integer::conversion::clone_and_assign::*;
use malachite_test::integer::conversion::from_i32::*;
use malachite_test::integer::conversion::from_i64::*;
use malachite_test::integer::conversion::from_u32::*;
use malachite_test::integer::conversion::from_u64::*;
use malachite_test::integer::conversion::natural_assign_integer::*;
use malachite_test::integer::conversion::to_i32::*;
use malachite_test::integer::conversion::to_i64::*;
use malachite_test::integer::conversion::to_natural::*;
use malachite_test::integer::conversion::to_u32::*;
use malachite_test::integer::conversion::to_u64::*;
use malachite_test::integer::logic::from_sign_and_limbs::*;
use malachite_test::integer::logic::get_bit::*;
use malachite_test::integer::logic::not::*;
use malachite_test::integer::logic::sign_and_limbs::*;
use malachite_test::integer::logic::significant_bits::*;
use malachite_test::natural::arithmetic::add::*;
use malachite_test::natural::arithmetic::add_u32::*;
use malachite_test::natural::arithmetic::even_odd::*;
use malachite_test::natural::arithmetic::is_power_of_two::*;
use malachite_test::natural::arithmetic::neg::*;
use malachite_test::natural::arithmetic::shl_u32::*;
use malachite_test::natural::arithmetic::sub::*;
use malachite_test::natural::arithmetic::sub_u32::*;
use malachite_test::natural::comparison::eq::*;
use malachite_test::natural::comparison::hash::*;
use malachite_test::natural::comparison::ord::*;
use malachite_test::natural::comparison::partial_eq_u32::*;
use malachite_test::natural::comparison::partial_ord_u32::*;
use malachite_test::natural::conversion::assign_u32::*;
use malachite_test::natural::conversion::assign_u64::*;
use malachite_test::natural::conversion::clone_and_assign::*;
use malachite_test::natural::conversion::from_u32::*;
use malachite_test::natural::conversion::from_u64::*;
use malachite_test::natural::conversion::to_integer::*;
use malachite_test::natural::conversion::to_u32::*;
use malachite_test::natural::conversion::to_u64::*;
use malachite_test::natural::logic::assign_bit::*;
use malachite_test::natural::logic::clear_bit::*;
use malachite_test::natural::logic::flip_bit::*;
use malachite_test::natural::logic::from_limbs::*;
use malachite_test::natural::logic::get_bit::*;
use malachite_test::natural::logic::limb_count::*;
use malachite_test::natural::logic::limbs::*;
use malachite_test::natural::logic::not::*;
use malachite_test::natural::logic::set_bit::*;
use malachite_test::natural::logic::significant_bits::*;
use malachite_test::natural::logic::trailing_zeros::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: [demo|bench] [limit] [demo name]");
    }
    let limit = if args.len() == 4 {
        args[2].parse().unwrap()
    } else {
        usize::max_value()
    };
    let item_name = &*args.last().unwrap();
    match args[1].as_ref() {
        "demo" => {
            match item_name.as_ref() {
                "exhaustive_integer_abs_assign" => demo_exhaustive_integer_abs_assign(limit),
                "exhaustive_integer_abs" => demo_exhaustive_integer_abs(limit),
                "exhaustive_integer_abs_ref" => demo_exhaustive_integer_abs_ref(limit),
                "exhaustive_integer_natural_abs" => demo_exhaustive_integer_natural_abs(limit),
                "exhaustive_integer_natural_abs_ref" => {
                    demo_exhaustive_integer_natural_abs_ref(limit)
                }
                "exhaustive_integer_assign" => demo_exhaustive_integer_assign(limit),
                "exhaustive_integer_assign_ref" => demo_exhaustive_integer_assign_ref(limit),
                "exhaustive_integer_assign_i32" => demo_exhaustive_integer_assign_i32(limit),
                "exhaustive_integer_assign_i64" => demo_exhaustive_integer_assign_i64(limit),
                "exhaustive_integer_assign_natural" => {
                    demo_exhaustive_integer_assign_natural(limit)
                }
                "exhaustive_integer_assign_natural_ref" => {
                    demo_exhaustive_integer_assign_natural_ref(limit)
                }
                "exhaustive_integer_assign_u32" => demo_exhaustive_integer_assign_u32(limit),
                "exhaustive_integer_assign_u64" => demo_exhaustive_integer_assign_u64(limit),
                "exhaustive_integer_clone" => demo_exhaustive_integer_clone(limit),
                "exhaustive_integer_clone_from" => demo_exhaustive_integer_clone_from(limit),
                "exhaustive_integer_cmp" => demo_exhaustive_integer_cmp(limit),
                "exhaustive_integer_cmp_abs" => demo_exhaustive_integer_cmp_abs(limit),
                "exhaustive_integer_eq" => demo_exhaustive_integer_eq(limit),
                "exhaustive_integer_from_i32" => demo_exhaustive_integer_from_i32(limit),
                "exhaustive_integer_from_i64" => demo_exhaustive_integer_from_i64(limit),
                "exhaustive_integer_from_sign_and_limbs_le" => {
                    demo_exhaustive_integer_from_sign_and_limbs_le(limit)
                }
                "exhaustive_integer_from_sign_and_limbs_be" => {
                    demo_exhaustive_integer_from_sign_and_limbs_be(limit)
                }
                "exhaustive_integer_from_u32" => demo_exhaustive_integer_from_u32(limit),
                "exhaustive_integer_from_u64" => demo_exhaustive_integer_from_u64(limit),
                "exhaustive_integer_get_bit" => demo_exhaustive_integer_get_bit(limit),
                "exhaustive_integer_hash" => demo_exhaustive_integer_hash(limit),
                "exhaustive_integer_neg_assign" => demo_exhaustive_integer_neg_assign(limit),
                "exhaustive_integer_neg" => demo_exhaustive_integer_neg(limit),
                "exhaustive_integer_neg_ref" => demo_exhaustive_integer_neg_ref(limit),
                "exhaustive_integer_not_assign" => demo_exhaustive_integer_not_assign(limit),
                "exhaustive_integer_not" => demo_exhaustive_integer_not(limit),
                "exhaustive_integer_not_ref" => demo_exhaustive_integer_not_ref(limit),
                "exhaustive_integer_partial_cmp_abs_i32" => {
                    demo_exhaustive_integer_partial_cmp_abs_i32(limit)
                }
                "exhaustive_i32_partial_cmp_abs_integer" => {
                    demo_exhaustive_i32_partial_cmp_abs_integer(limit)
                }
                "exhaustive_integer_partial_cmp_abs_u32" => {
                    demo_exhaustive_integer_partial_cmp_abs_u32(limit)
                }
                "exhaustive_u32_partial_cmp_abs_integer" => {
                    demo_exhaustive_u32_partial_cmp_abs_integer(limit)
                }
                "exhaustive_integer_partial_cmp_abs_natural" => {
                    demo_exhaustive_integer_partial_cmp_abs_natural(limit)
                }
                "exhaustive_integer_partial_cmp_i32" => {
                    demo_exhaustive_integer_partial_cmp_i32(limit)
                }
                "exhaustive_i32_partial_cmp_integer" => {
                    demo_exhaustive_i32_partial_cmp_integer(limit)
                }
                "exhaustive_integer_partial_cmp_u32" => {
                    demo_exhaustive_integer_partial_cmp_u32(limit)
                }
                "exhaustive_u32_partial_cmp_integer" => {
                    demo_exhaustive_u32_partial_cmp_integer(limit)
                }
                "exhaustive_integer_partial_cmp_natural" => {
                    demo_exhaustive_integer_partial_cmp_natural(limit)
                }
                "exhaustive_integer_partial_eq_i32" => {
                    demo_exhaustive_integer_partial_eq_i32(limit)
                }
                "exhaustive_i32_partial_eq_integer" => {
                    demo_exhaustive_i32_partial_eq_integer(limit)
                }
                "exhaustive_integer_partial_eq_u32" => {
                    demo_exhaustive_integer_partial_eq_u32(limit)
                }
                "exhaustive_u32_partial_eq_integer" => {
                    demo_exhaustive_u32_partial_eq_integer(limit)
                }
                "exhaustive_integer_partial_eq_natural" => {
                    demo_exhaustive_integer_partial_eq_natural(limit)
                }
                "exhaustive_integer_sign" => demo_exhaustive_integer_sign(limit),
                "exhaustive_integer_sign_and_limbs_le" => {
                    demo_exhaustive_integer_sign_and_limbs_le(limit)
                }
                "exhaustive_integer_sign_and_limbs_be" => {
                    demo_exhaustive_integer_sign_and_limbs_be(limit)
                }
                "exhaustive_integer_significant_bits" => {
                    demo_exhaustive_integer_significant_bits(limit)
                }
                "exhaustive_integer_to_i32" => demo_exhaustive_integer_to_i32(limit),
                "exhaustive_integer_to_i32_wrapping" => {
                    demo_exhaustive_integer_to_i32_wrapping(limit)
                }
                "exhaustive_integer_to_i64" => demo_exhaustive_integer_to_i64(limit),
                "exhaustive_integer_to_i64_wrapping" => {
                    demo_exhaustive_integer_to_i64_wrapping(limit)
                }
                "exhaustive_integer_into_natural" => demo_exhaustive_integer_into_natural(limit),
                "exhaustive_integer_to_natural" => demo_exhaustive_integer_to_natural(limit),
                "exhaustive_integer_to_u32" => demo_exhaustive_integer_to_u32(limit),
                "exhaustive_integer_to_u32_wrapping" => {
                    demo_exhaustive_integer_to_u32_wrapping(limit)
                }
                "exhaustive_integer_to_u64" => demo_exhaustive_integer_to_u64(limit),
                "exhaustive_integer_to_u64_wrapping" => {
                    demo_exhaustive_integer_to_u64_wrapping(limit)
                }
                "exhaustive_natural_add_assign" => demo_exhaustive_natural_add_assign(limit),
                "exhaustive_natural_add_assign_ref" => {
                    demo_exhaustive_natural_add_assign_ref(limit)
                }
                "exhaustive_natural_add" => demo_exhaustive_natural_add(limit),
                "exhaustive_natural_add_val_ref" => demo_exhaustive_natural_add_val_ref(limit),
                "exhaustive_natural_add_ref_val" => demo_exhaustive_natural_add_ref_val(limit),
                "exhaustive_natural_add_ref_ref" => demo_exhaustive_natural_add_ref_ref(limit),
                "exhaustive_natural_add_assign_u32" => {
                    demo_exhaustive_natural_add_assign_u32(limit)
                }
                "exhaustive_natural_add_u32" => demo_exhaustive_natural_add_u32(limit),
                "exhaustive_natural_add_u32_ref" => demo_exhaustive_natural_add_u32_ref(limit),
                "exhaustive_u32_add_natural" => demo_exhaustive_u32_add_natural(limit),
                "exhaustive_u32_add_natural_ref" => demo_exhaustive_u32_add_natural_ref(limit),
                "exhaustive_natural_assign" => demo_exhaustive_natural_assign(limit),
                "exhaustive_natural_assign_ref" => demo_exhaustive_natural_assign_ref(limit),
                "exhaustive_natural_assign_integer" => {
                    demo_exhaustive_natural_assign_integer(limit)
                }
                "exhaustive_natural_assign_integer_ref" => {
                    demo_exhaustive_natural_assign_integer_ref(limit)
                }
                "exhaustive_natural_assign_u32" => demo_exhaustive_natural_assign_u32(limit),
                "exhaustive_natural_assign_u64" => demo_exhaustive_natural_assign_u64(limit),
                "exhaustive_natural_assign_bit" => demo_exhaustive_natural_assign_bit(limit),
                "exhaustive_natural_clear_bit" => demo_exhaustive_natural_clear_bit(limit),
                "exhaustive_natural_clone" => demo_exhaustive_natural_clone(limit),
                "exhaustive_natural_clone_from" => demo_exhaustive_natural_clone_from(limit),
                "exhaustive_natural_cmp" => demo_exhaustive_natural_cmp(limit),
                "exhaustive_natural_eq" => demo_exhaustive_natural_eq(limit),
                "exhaustive_natural_flip_bit" => demo_exhaustive_natural_flip_bit(limit),
                "exhaustive_natural_from_limbs_le" => demo_exhaustive_natural_from_limbs_le(limit),
                "exhaustive_natural_from_limbs_be" => demo_exhaustive_natural_from_limbs_be(limit),
                "exhaustive_natural_from_u32" => demo_exhaustive_natural_from_u32(limit),
                "exhaustive_natural_from_u64" => demo_exhaustive_natural_from_u64(limit),
                "exhaustive_natural_get_bit" => demo_exhaustive_natural_get_bit(limit),
                "exhaustive_natural_hash" => demo_exhaustive_natural_hash(limit),
                "exhaustive_natural_is_even" => demo_exhaustive_natural_is_even(limit),
                "exhaustive_natural_is_odd" => demo_exhaustive_natural_is_odd(limit),
                "exhaustive_natural_is_power_of_two" => {
                    demo_exhaustive_natural_is_power_of_two(limit)
                }
                "exhaustive_natural_limb_count" => demo_exhaustive_natural_limb_count(limit),
                "exhaustive_natural_limbs_le" => demo_exhaustive_natural_limbs_le(limit),
                "exhaustive_natural_limbs_be" => demo_exhaustive_natural_limbs_be(limit),
                "exhaustive_natural_neg" => demo_exhaustive_natural_neg(limit),
                "exhaustive_natural_neg_ref" => demo_exhaustive_natural_neg_ref(limit),
                "exhaustive_natural_not" => demo_exhaustive_natural_not(limit),
                "exhaustive_natural_not_ref" => demo_exhaustive_natural_not_ref(limit),
                "exhaustive_natural_partial_cmp_abs_integer" => {
                    demo_exhaustive_natural_partial_cmp_abs_integer(limit)
                }
                "exhaustive_natural_partial_cmp_integer" => {
                    demo_exhaustive_natural_partial_cmp_integer(limit)
                }
                "exhaustive_natural_partial_cmp_u32" => {
                    demo_exhaustive_natural_partial_cmp_u32(limit)
                }
                "exhaustive_u32_partial_cmp_natural" => {
                    demo_exhaustive_u32_partial_cmp_natural(limit)
                }
                "exhaustive_natural_partial_eq_integer" => {
                    demo_exhaustive_natural_partial_eq_integer(limit)
                }
                "exhaustive_natural_partial_eq_u32" => {
                    demo_exhaustive_natural_partial_eq_u32(limit)
                }
                "exhaustive_u32_partial_eq_natural" => {
                    demo_exhaustive_u32_partial_eq_natural(limit)
                }
                "exhaustive_natural_set_bit" => demo_exhaustive_natural_set_bit(limit),
                "exhaustive_natural_shl_assign_u32" => {
                    demo_exhaustive_natural_shl_assign_u32(limit)
                }
                "exhaustive_natural_shl_u32" => demo_exhaustive_natural_shl_u32(limit),
                "exhaustive_natural_shl_u32_ref" => demo_exhaustive_natural_shl_u32_ref(limit),
                "exhaustive_natural_significant_bits" => {
                    demo_exhaustive_natural_significant_bits(limit)
                }
                "exhaustive_natural_sub_assign" => demo_exhaustive_natural_sub_assign(limit),
                "exhaustive_natural_sub" => demo_exhaustive_natural_sub(limit),
                "exhaustive_natural_sub_ref_ref" => demo_exhaustive_natural_sub_ref_ref(limit),
                "exhaustive_natural_sub_assign_u32" => {
                    demo_exhaustive_natural_sub_assign_u32(limit)
                }
                "exhaustive_natural_sub_u32" => demo_exhaustive_natural_sub_u32(limit),
                "exhaustive_natural_sub_u32_ref" => demo_exhaustive_natural_sub_u32_ref(limit),
                "exhaustive_u32_sub_natural" => demo_exhaustive_u32_sub_natural(limit),
                "exhaustive_natural_into_integer" => demo_exhaustive_natural_into_integer(limit),
                "exhaustive_natural_to_integer" => demo_exhaustive_natural_to_integer(limit),
                "exhaustive_natural_to_u32" => demo_exhaustive_natural_to_u32(limit),
                "exhaustive_natural_to_u32_wrapping" => {
                    demo_exhaustive_natural_to_u32_wrapping(limit)
                }
                "exhaustive_natural_to_u64" => demo_exhaustive_natural_to_u64(limit),
                "exhaustive_natural_to_u64_wrapping" => {
                    demo_exhaustive_natural_to_u64_wrapping(limit)
                }
                "exhaustive_natural_trailing_zeros" => {
                    demo_exhaustive_natural_trailing_zeros(limit)
                }
                "random_integer_abs_assign" => demo_random_integer_abs_assign(limit),
                "random_integer_abs" => demo_random_integer_abs(limit),
                "random_integer_abs_ref" => demo_random_integer_abs_ref(limit),
                "random_integer_natural_abs" => demo_random_integer_natural_abs(limit),
                "random_integer_natural_abs_ref" => demo_random_integer_natural_abs_ref(limit),
                "random_integer_assign" => demo_random_integer_assign(limit),
                "random_integer_assign_ref" => demo_random_integer_assign_ref(limit),
                "random_integer_assign_i32" => demo_random_integer_assign_i32(limit),
                "random_integer_assign_i64" => demo_random_integer_assign_i64(limit),
                "random_integer_assign_natural" => demo_random_integer_assign_natural(limit),
                "random_integer_assign_natural_ref" => {
                    demo_random_integer_assign_natural_ref(limit)
                }
                "random_integer_assign_u32" => demo_random_integer_assign_u32(limit),
                "random_integer_assign_u64" => demo_random_integer_assign_u64(limit),
                "random_integer_clone" => demo_random_integer_clone(limit),
                "random_integer_clone_from" => demo_random_integer_clone_from(limit),
                "random_integer_cmp" => demo_random_integer_cmp(limit),
                "random_integer_cmp_abs" => demo_random_integer_cmp_abs(limit),
                "random_integer_eq" => demo_random_integer_eq(limit),
                "random_integer_from_i32" => demo_random_integer_from_i32(limit),
                "random_integer_from_i64" => demo_random_integer_from_i64(limit),
                "random_integer_from_sign_and_limbs_le" => {
                    demo_random_integer_from_sign_and_limbs_le(limit)
                }
                "random_integer_from_sign_and_limbs_be" => {
                    demo_random_integer_from_sign_and_limbs_be(limit)
                }
                "random_integer_from_u32" => demo_random_integer_from_u32(limit),
                "random_integer_from_u64" => demo_random_integer_from_u64(limit),
                "random_integer_get_bit" => demo_random_integer_get_bit(limit),
                "random_integer_hash" => demo_random_integer_hash(limit),
                "random_integer_neg_assign" => demo_random_integer_neg_assign(limit),
                "random_integer_neg" => demo_random_integer_neg(limit),
                "random_integer_neg_ref" => demo_random_integer_neg_ref(limit),
                "random_integer_not_assign" => demo_random_integer_not_assign(limit),
                "random_integer_not" => demo_random_integer_not(limit),
                "random_integer_not_ref" => demo_random_integer_not_ref(limit),
                "random_integer_partial_cmp_abs_i32" => {
                    demo_random_integer_partial_cmp_abs_i32(limit)
                }
                "random_i32_partial_cmp_abs_integer" => {
                    demo_random_i32_partial_cmp_abs_integer(limit)
                }
                "random_integer_partial_cmp_abs_u32" => {
                    demo_random_integer_partial_cmp_abs_u32(limit)
                }
                "random_u32_partial_cmp_abs_integer" => {
                    demo_random_u32_partial_cmp_abs_integer(limit)
                }
                "random_integer_partial_cmp_abs_natural" => {
                    demo_random_integer_partial_cmp_abs_natural(limit)
                }
                "random_integer_partial_cmp_i32" => demo_random_integer_partial_cmp_i32(limit),
                "random_i32_partial_cmp_integer" => demo_random_i32_partial_cmp_integer(limit),
                "random_integer_partial_cmp_u32" => demo_random_integer_partial_cmp_u32(limit),
                "random_u32_partial_cmp_integer" => demo_random_u32_partial_cmp_integer(limit),
                "random_integer_partial_cmp_natural" => {
                    demo_random_integer_partial_cmp_natural(limit)
                }
                "random_integer_partial_eq_i32" => demo_random_integer_partial_eq_i32(limit),
                "random_i32_partial_eq_integer" => demo_random_i32_partial_eq_integer(limit),
                "random_integer_partial_eq_u32" => demo_random_integer_partial_eq_u32(limit),
                "random_u32_partial_eq_integer" => demo_random_u32_partial_eq_integer(limit),
                "random_integer_partial_eq_natural" => {
                    demo_random_integer_partial_eq_natural(limit)
                }
                "random_integer_sign" => demo_random_integer_sign(limit),
                "random_integer_sign_and_limbs_le" => demo_random_integer_sign_and_limbs_le(limit),
                "random_integer_sign_and_limbs_be" => demo_random_integer_sign_and_limbs_be(limit),
                "random_integer_significant_bits" => demo_random_integer_significant_bits(limit),
                "random_integer_to_i32" => demo_random_integer_to_i32(limit),
                "random_integer_to_i32_wrapping" => demo_random_integer_to_i32_wrapping(limit),
                "random_integer_to_i64" => demo_random_integer_to_i64(limit),
                "random_integer_to_i64_wrapping" => demo_random_integer_to_i64_wrapping(limit),
                "random_integer_into_natural" => demo_random_integer_into_natural(limit),
                "random_integer_to_natural" => demo_random_integer_to_natural(limit),
                "random_integer_to_u32" => demo_random_integer_to_u32(limit),
                "random_integer_to_u32_wrapping" => demo_random_integer_to_u32_wrapping(limit),
                "random_integer_to_u64" => demo_random_integer_to_u64(limit),
                "random_integer_to_u64_wrapping" => demo_random_integer_to_u64_wrapping(limit),
                "random_natural_add_assign" => demo_random_natural_add_assign(limit),
                "random_natural_add_assign_ref" => demo_random_natural_add_assign_ref(limit),
                "random_natural_add" => demo_random_natural_add(limit),
                "random_natural_add_val_ref" => demo_random_natural_add_val_ref(limit),
                "random_natural_add_ref_val" => demo_random_natural_add_ref_val(limit),
                "random_natural_add_ref_ref" => demo_random_natural_add_ref_ref(limit),
                "random_natural_add_assign_u32" => demo_random_natural_add_assign_u32(limit),
                "random_natural_add_u32" => demo_random_natural_add_u32(limit),
                "random_natural_add_u32_ref" => demo_random_natural_add_u32_ref(limit),
                "random_u32_add_natural" => demo_random_u32_add_natural(limit),
                "random_u32_add_natural_ref" => demo_random_u32_add_natural_ref(limit),
                "random_natural_assign" => demo_random_natural_assign(limit),
                "random_natural_assign_ref" => demo_random_natural_assign_ref(limit),
                "random_natural_assign_integer" => demo_random_natural_assign_integer(limit),
                "random_natural_assign_integer_ref" => {
                    demo_random_natural_assign_integer_ref(limit)
                }
                "random_natural_assign_u32" => demo_random_natural_assign_u32(limit),
                "random_natural_assign_u64" => demo_random_natural_assign_u64(limit),
                "random_natural_assign_bit" => demo_random_natural_assign_bit(limit),
                "random_natural_clear_bit" => demo_random_natural_clear_bit(limit),
                "random_natural_clone" => demo_random_natural_clone(limit),
                "random_natural_clone_from" => demo_random_natural_clone_from(limit),
                "random_natural_cmp" => demo_random_natural_cmp(limit),
                "random_natural_eq" => demo_random_natural_eq(limit),
                "random_natural_flip_bit" => demo_random_natural_flip_bit(limit),
                "random_natural_from_limbs_le" => demo_random_natural_from_limbs_le(limit),
                "random_natural_from_limbs_be" => demo_random_natural_from_limbs_be(limit),
                "random_natural_from_u32" => demo_random_natural_from_u32(limit),
                "random_natural_from_u64" => demo_random_natural_from_u64(limit),
                "random_natural_get_bit" => demo_random_natural_get_bit(limit),
                "random_natural_hash" => demo_random_natural_hash(limit),
                "random_natural_is_even" => demo_random_natural_is_even(limit),
                "random_natural_is_odd" => demo_random_natural_is_odd(limit),
                "random_natural_is_power_of_two" => demo_random_natural_is_power_of_two(limit),
                "random_natural_limb_count" => demo_random_natural_limb_count(limit),
                "random_natural_limbs_le" => demo_random_natural_limbs_le(limit),
                "random_natural_limbs_be" => demo_random_natural_limbs_be(limit),
                "random_natural_neg" => demo_random_natural_neg(limit),
                "random_natural_neg_ref" => demo_random_natural_neg_ref(limit),
                "random_natural_not" => demo_random_natural_not(limit),
                "random_natural_not_ref" => demo_random_natural_not_ref(limit),
                "random_natural_partial_eq_u32" => demo_random_natural_partial_eq_u32(limit),
                "random_u32_partial_eq_natural" => demo_random_u32_partial_eq_natural(limit),
                "random_natural_partial_eq_integer" => {
                    demo_random_natural_partial_eq_integer(limit)
                }
                "random_natural_partial_cmp_abs_integer" => {
                    demo_random_natural_partial_cmp_abs_integer(limit)
                }
                "random_natural_partial_cmp_integer" => {
                    demo_random_natural_partial_cmp_integer(limit)
                }
                "random_natural_partial_cmp_u32" => demo_random_natural_partial_cmp_u32(limit),
                "random_u32_partial_cmp_natural" => demo_random_u32_partial_cmp_natural(limit),
                "random_natural_set_bit" => demo_random_natural_set_bit(limit),
                "random_natural_shl_assign_u32" => demo_random_natural_shl_assign_u32(limit),
                "random_natural_shl_u32" => demo_random_natural_shl_u32(limit),
                "random_natural_shl_u32_ref" => demo_random_natural_shl_u32_ref(limit),
                "random_natural_significant_bits" => demo_random_natural_significant_bits(limit),
                "random_natural_sub_assign" => demo_random_natural_sub_assign(limit),
                "random_natural_sub" => demo_random_natural_sub(limit),
                "random_natural_sub_ref_ref" => demo_random_natural_sub_ref_ref(limit),
                "random_natural_sub_assign_u32" => demo_random_natural_sub_assign_u32(limit),
                "random_natural_sub_u32" => demo_random_natural_sub_u32(limit),
                "random_natural_sub_u32_ref" => demo_random_natural_sub_u32_ref(limit),
                "random_u32_sub_natural" => demo_random_u32_sub_natural(limit),
                "random_natural_into_integer" => demo_random_natural_into_integer(limit),
                "random_natural_to_integer" => demo_random_natural_to_integer(limit),
                "random_natural_to_u32" => demo_random_natural_to_u32(limit),
                "random_natural_to_u32_wrapping" => demo_random_natural_to_u32_wrapping(limit),
                "random_natural_to_u64" => demo_random_natural_to_u64(limit),
                "random_natural_to_u64_wrapping" => demo_random_natural_to_u64_wrapping(limit),
                "random_natural_trailing_zeros" => demo_random_natural_trailing_zeros(limit),

                _ => panic!("Invalid demo name: {}", item_name),
            }
        }
        "bench" => {
            match item_name.as_ref() {
                "exhaustive_integer_abs_assign" => {
                    benchmark_exhaustive_integer_abs_assign(limit, "temp.gp")
                }
                "exhaustive_integer_abs" => benchmark_exhaustive_integer_abs(limit, "temp.gp"),
                "exhaustive_integer_abs_evaluation_strategy" => {
                    benchmark_exhaustive_integer_abs_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_integer_natural_abs" => {
                    benchmark_exhaustive_integer_natural_abs(limit, "temp.gp")
                }
                "exhaustive_integer_natural_abs_evaluation_strategy" => {
                    benchmark_exhaustive_integer_natural_abs_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_integer_assign" => {
                    benchmark_exhaustive_integer_assign(limit, "temp.gp")
                }
                "exhaustive_integer_assign_evaluation_strategy" => {
                    benchmark_exhaustive_integer_assign_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_integer_assign_i32" => {
                    benchmark_exhaustive_integer_assign_i32(limit, "temp.gp")
                }
                "exhaustive_integer_assign_i64" => {
                    benchmark_exhaustive_integer_assign_i64(limit, "temp.gp")
                }
                "exhaustive_integer_assign_natural" => {
                    benchmark_exhaustive_integer_assign_natural(limit, "temp.gp")
                }
                "exhaustive_integer_assign_natural_evaluation_strategy" => {
                    benchmark_exhaustive_integer_assign_natural_evaluation_strategy(limit,
                                                                                    "temp.gp")
                }
                "exhaustive_integer_assign_u32" => {
                    benchmark_exhaustive_integer_assign_u32(limit, "temp.gp")
                }
                "exhaustive_integer_assign_u64" => {
                    benchmark_exhaustive_integer_assign_u64(limit, "temp.gp")
                }
                "exhaustive_integer_clone" => benchmark_exhaustive_integer_clone(limit, "temp.gp"),
                "exhaustive_integer_clone_from" => {
                    benchmark_exhaustive_integer_clone_from(limit, "temp.gp")
                }
                "exhaustive_integer_cmp" => benchmark_exhaustive_integer_cmp(limit, "temp.gp"),
                "exhaustive_integer_cmp_abs" => {
                    benchmark_exhaustive_integer_cmp_abs(limit, "temp.gp")
                }
                "exhaustive_integer_eq" => benchmark_exhaustive_integer_eq(limit, "temp.gp"),
                "exhaustive_integer_from_i32" => {
                    benchmark_exhaustive_integer_from_i32(limit, "temp.gp")
                }
                "exhaustive_integer_from_i64" => {
                    benchmark_exhaustive_integer_from_i64(limit, "temp.gp")
                }
                "exhaustive_integer_from_sign_and_limbs_le" => {
                    benchmark_exhaustive_integer_from_sign_and_limbs_le(limit, "temp.gp")
                }
                "exhaustive_integer_from_sign_and_limbs_be" => {
                    benchmark_exhaustive_integer_from_sign_and_limbs_be(limit, "temp.gp")
                }
                "exhaustive_integer_from_u32" => {
                    benchmark_exhaustive_integer_from_u32(limit, "temp.gp")
                }
                "exhaustive_integer_from_u64" => {
                    benchmark_exhaustive_integer_from_u64(limit, "temp.gp")
                }
                "exhaustive_integer_get_bit" => {
                    benchmark_exhaustive_integer_get_bit(limit, "temp.gp")
                }
                "exhaustive_integer_hash" => benchmark_exhaustive_integer_hash(limit, "temp.gp"),
                "exhaustive_integer_neg_assign" => {
                    benchmark_exhaustive_integer_neg_assign(limit, "temp.gp")
                }
                "exhaustive_integer_neg" => benchmark_exhaustive_integer_neg(limit, "temp.gp"),
                "exhaustive_integer_neg_evaluation_strategy" => {
                    benchmark_exhaustive_integer_neg_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_integer_not" => benchmark_exhaustive_integer_not(limit, "temp.gp"),
                "exhaustive_integer_not_evaluation_strategy" => {
                    benchmark_exhaustive_integer_not_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_integer_partial_cmp_abs_i32" => {
                    benchmark_exhaustive_integer_partial_cmp_abs_i32(limit, "temp.gp")
                }
                "exhaustive_i32_partial_cmp_abs_integer" => {
                    benchmark_exhaustive_i32_partial_cmp_abs_integer(limit, "temp.gp")
                }
                "exhaustive_integer_partial_cmp_abs_u32" => {
                    benchmark_exhaustive_integer_partial_cmp_abs_u32(limit, "temp.gp")
                }
                "exhaustive_u32_partial_cmp_abs_integer" => {
                    benchmark_exhaustive_u32_partial_cmp_abs_integer(limit, "temp.gp")
                }
                "exhaustive_integer_partial_cmp_abs_natural" => {
                    benchmark_exhaustive_integer_partial_cmp_abs_natural(limit, "temp.gp")
                }
                "exhaustive_integer_partial_cmp_i32" => {
                    benchmark_exhaustive_integer_partial_cmp_i32(limit, "temp.gp")
                }
                "exhaustive_i32_partial_cmp_integer" => {
                    benchmark_exhaustive_i32_partial_cmp_integer(limit, "temp.gp")
                }
                "exhaustive_integer_partial_cmp_u32" => {
                    benchmark_exhaustive_integer_partial_cmp_u32(limit, "temp.gp")
                }
                "exhaustive_u32_partial_cmp_integer" => {
                    benchmark_exhaustive_u32_partial_cmp_integer(limit, "temp.gp")
                }
                "exhaustive_integer_partial_cmp_natural" => {
                    benchmark_exhaustive_integer_partial_cmp_natural(limit, "temp.gp")
                }
                "exhaustive_integer_partial_eq_i32" => {
                    benchmark_exhaustive_integer_partial_eq_i32(limit, "temp.gp")
                }
                "exhaustive_i32_partial_eq_integer" => {
                    benchmark_exhaustive_i32_partial_eq_integer(limit, "temp.gp")
                }
                "exhaustive_integer_partial_eq_u32" => {
                    benchmark_exhaustive_integer_partial_eq_u32(limit, "temp.gp")
                }
                "exhaustive_u32_partial_eq_integer" => {
                    benchmark_exhaustive_u32_partial_eq_integer(limit, "temp.gp")
                }
                "exhaustive_integer_partial_eq_natural" => {
                    benchmark_exhaustive_integer_partial_eq_natural(limit, "temp.gp")
                }
                "exhaustive_integer_sign" => benchmark_exhaustive_integer_sign(limit, "temp.gp"),
                "exhaustive_integer_sign_and_limbs_le" => {
                    benchmark_exhaustive_integer_sign_and_limbs_le(limit, "temp.gp")
                }
                "exhaustive_integer_sign_and_limbs_be" => {
                    benchmark_exhaustive_integer_sign_and_limbs_be(limit, "temp.gp")
                }
                "exhaustive_integer_significant_bits" => {
                    benchmark_exhaustive_integer_significant_bits(limit, "temp.gp")
                }
                "exhaustive_integer_to_i32" => {
                    benchmark_exhaustive_integer_to_i32(limit, "temp.gp")
                }
                "exhaustive_integer_to_i32_wrapping" => {
                    benchmark_exhaustive_integer_to_i32_wrapping(limit, "temp.gp")
                }
                "exhaustive_integer_to_i64" => {
                    benchmark_exhaustive_integer_to_i64(limit, "temp.gp")
                }
                "exhaustive_integer_to_i64_wrapping" => {
                    benchmark_exhaustive_integer_to_i64_wrapping(limit, "temp.gp")
                }
                "exhaustive_integer_to_natural" => {
                    benchmark_exhaustive_integer_to_natural(limit, "temp.gp")
                }
                "exhaustive_integer_to_natural_evaluation_strategy" => {
                    benchmark_exhaustive_integer_to_natural_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_integer_to_u32" => {
                    benchmark_exhaustive_integer_to_u32(limit, "temp.gp")
                }
                "exhaustive_integer_to_u32_wrapping" => {
                    benchmark_exhaustive_integer_to_u32_wrapping(limit, "temp.gp")
                }
                "exhaustive_integer_to_u64" => {
                    benchmark_exhaustive_integer_to_u64(limit, "temp.gp")
                }
                "exhaustive_integer_to_u64_wrapping" => {
                    benchmark_exhaustive_integer_to_u64_wrapping(limit, "temp.gp")
                }
                "exhaustive_natural_add_assign" => {
                    benchmark_exhaustive_natural_add_assign(limit, "temp.gp")
                }
                "exhaustive_natural_add_assign_evaluation_strategy" => {
                    benchmark_exhaustive_natural_add_assign_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_natural_add" => benchmark_exhaustive_natural_add(limit, "temp.gp"),
                "exhaustive_natural_add_evaluation_strategy" => {
                    benchmark_exhaustive_natural_add_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_natural_add_assign_u32" => {
                    benchmark_exhaustive_natural_add_assign_u32(limit, "temp.gp")
                }
                "exhaustive_natural_add_u32" => {
                    benchmark_exhaustive_natural_add_u32(limit, "temp.gp")
                }
                "exhaustive_natural_add_u32_ref" => {
                    benchmark_exhaustive_natural_add_u32_ref(limit, "temp.gp")
                }
                "exhaustive_u32_add_natural" => {
                    benchmark_exhaustive_u32_add_natural(limit, "temp.gp")
                }
                "exhaustive_u32_add_natural_ref" => {
                    benchmark_exhaustive_u32_add_natural_ref(limit, "temp.gp")
                }
                "exhaustive_natural_assign" => {
                    benchmark_exhaustive_natural_assign(limit, "temp.gp")
                }
                "exhaustive_natural_assign_evaluation_strategy" => {
                    benchmark_exhaustive_natural_assign_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_natural_assign_integer" => {
                    benchmark_exhaustive_natural_assign_integer(limit, "temp.gp")
                }
                "exhaustive_natural_assign_integer_evaluation_strategy" => {
                    benchmark_exhaustive_natural_assign_integer_evaluation_strategy(limit,
                                                                                    "temp.gp")
                }
                "exhaustive_natural_assign_u32" => {
                    benchmark_exhaustive_natural_assign_u32(limit, "temp.gp")
                }
                "exhaustive_natural_assign_u64" => {
                    benchmark_exhaustive_natural_assign_u64(limit, "temp.gp")
                }
                "exhaustive_natural_assign_bit" => {
                    benchmark_exhaustive_natural_assign_bit(limit, "temp.gp")
                }
                "exhaustive_natural_clear_bit" => {
                    benchmark_exhaustive_natural_clear_bit(limit, "temp.gp")
                }
                "exhaustive_natural_clone" => benchmark_exhaustive_natural_clone(limit, "temp.gp"),
                "exhaustive_natural_clone_from" => {
                    benchmark_exhaustive_natural_clone_from(limit, "temp.gp")
                }
                "exhaustive_natural_cmp" => benchmark_exhaustive_natural_cmp(limit, "temp.gp"),
                "exhaustive_natural_eq" => benchmark_exhaustive_natural_eq(limit, "temp.gp"),
                "exhaustive_natural_flip_bit" => {
                    benchmark_exhaustive_natural_flip_bit(limit, "temp.gp")
                }
                "exhaustive_natural_from_limbs_le" => {
                    benchmark_exhaustive_natural_from_limbs_le(limit, "temp.gp")
                }
                "exhaustive_natural_from_limbs_be" => {
                    benchmark_exhaustive_natural_from_limbs_be(limit, "temp.gp")
                }
                "exhaustive_natural_from_u32" => {
                    benchmark_exhaustive_natural_from_u32(limit, "temp.gp")
                }
                "exhaustive_natural_from_u64" => {
                    benchmark_exhaustive_natural_from_u64(limit, "temp.gp")
                }
                "exhaustive_natural_get_bit" => {
                    benchmark_exhaustive_natural_get_bit(limit, "temp.gp")
                }
                "exhaustive_natural_hash" => benchmark_exhaustive_natural_hash(limit, "temp.gp"),
                "exhaustive_natural_is_even" => {
                    benchmark_exhaustive_natural_is_even(limit, "temp.gp")
                }
                "exhaustive_natural_is_odd" => {
                    benchmark_exhaustive_natural_is_odd(limit, "temp.gp")
                }
                "exhaustive_natural_is_power_of_two" => {
                    benchmark_exhaustive_natural_is_power_of_two(limit, "temp.gp")
                }
                "exhaustive_natural_limb_count" => {
                    benchmark_exhaustive_natural_limb_count(limit, "temp.gp")
                }
                "exhaustive_natural_limbs_le" => {
                    benchmark_exhaustive_natural_limbs_le(limit, "temp.gp")
                }
                "exhaustive_natural_limbs_be" => {
                    benchmark_exhaustive_natural_limbs_be(limit, "temp.gp")
                }
                "exhaustive_natural_neg" => benchmark_exhaustive_natural_neg(limit, "temp.gp"),
                "exhaustive_natural_neg_evaluation_strategy" => {
                    benchmark_exhaustive_natural_neg_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_natural_not" => benchmark_exhaustive_natural_not(limit, "temp.gp"),
                "exhaustive_natural_not_evaluation_strategy" => {
                    benchmark_exhaustive_natural_not_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_natural_partial_cmp_integer" => {
                    benchmark_exhaustive_natural_partial_cmp_integer(limit, "temp.gp")
                }
                "exhaustive_natural_partial_cmp_u32" => {
                    benchmark_exhaustive_natural_partial_cmp_u32(limit, "temp.gp")
                }
                "exhaustive_u32_partial_cmp_natural" => {
                    benchmark_exhaustive_u32_partial_cmp_natural(limit, "temp.gp")
                }
                "exhaustive_natural_partial_cmp_abs_integer" => {
                    benchmark_exhaustive_natural_partial_cmp_abs_integer(limit, "temp.gp")
                }
                "exhaustive_natural_partial_eq_integer" => {
                    benchmark_exhaustive_natural_partial_eq_integer(limit, "temp.gp")
                }
                "exhaustive_natural_partial_eq_u32" => {
                    benchmark_exhaustive_natural_partial_eq_u32(limit, "temp.gp")
                }
                "exhaustive_u32_partial_eq_natural" => {
                    benchmark_exhaustive_u32_partial_eq_natural(limit, "temp.gp")
                }
                "exhaustive_natural_set_bit" => {
                    benchmark_exhaustive_natural_set_bit(limit, "temp.gp")
                }
                "exhaustive_natural_shl_assign_u32" => {
                    benchmark_exhaustive_natural_shl_assign_u32(limit, "temp.gp")
                }
                "exhaustive_natural_shl_u32" => {
                    benchmark_exhaustive_natural_shl_u32(limit, "temp.gp")
                }
                "exhaustive_natural_shl_u32_ref" => {
                    benchmark_exhaustive_natural_shl_u32_ref(limit, "temp.gp")
                }
                "exhaustive_natural_significant_bits" => {
                    benchmark_exhaustive_natural_significant_bits(limit, "temp.gp")
                }
                "exhaustive_natural_sub_assign" => {
                    benchmark_exhaustive_natural_sub_assign(limit, "temp.gp")
                }
                "exhaustive_natural_sub" => benchmark_exhaustive_natural_sub(limit, "temp.gp"),
                "exhaustive_natural_sub_evaluation_strategy" => {
                    benchmark_exhaustive_natural_sub_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_natural_sub_assign_u32" => {
                    benchmark_exhaustive_natural_sub_assign_u32(limit, "temp.gp")
                }
                "exhaustive_natural_sub_u32" => {
                    benchmark_exhaustive_natural_sub_u32(limit, "temp.gp")
                }
                "exhaustive_natural_sub_u32_ref" => {
                    benchmark_exhaustive_natural_sub_u32_ref(limit, "temp.gp")
                }
                "exhaustive_u32_sub_natural" => {
                    benchmark_exhaustive_u32_sub_natural(limit, "temp.gp")
                }
                "exhaustive_natural_to_integer" => {
                    benchmark_exhaustive_natural_to_integer(limit, "temp.gp")
                }
                "exhaustive_natural_to_integer_evaluation_strategy" => {
                    benchmark_exhaustive_natural_to_integer_evaluation_strategy(limit, "temp.gp")
                }
                "exhaustive_natural_to_u32" => {
                    benchmark_exhaustive_natural_to_u32(limit, "temp.gp")
                }
                "exhaustive_natural_to_u32_wrapping" => {
                    benchmark_exhaustive_natural_to_u32_wrapping(limit, "temp.gp")
                }
                "exhaustive_natural_to_u64" => {
                    benchmark_exhaustive_natural_to_u64(limit, "temp.gp")
                }
                "exhaustive_natural_to_u64_wrapping" => {
                    benchmark_exhaustive_natural_to_u64_wrapping(limit, "temp.gp")
                }
                "exhaustive_natural_trailing_zeros" => {
                    benchmark_exhaustive_natural_trailing_zeros(limit, "temp.gp")
                }
                "random_integer_abs_assign" => {
                    benchmark_random_integer_abs_assign(limit, 1024, "temp.gp")
                }
                "random_integer_abs" => benchmark_random_integer_abs(limit, 1024, "temp.gp"),
                "random_integer_abs_evaluation_strategy" => {
                    benchmark_random_integer_abs_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_integer_natural_abs" => {
                    benchmark_random_integer_natural_abs(limit, 1024, "temp.gp")
                }
                "random_integer_natural_abs_evaluation_strategy" => {
                    benchmark_random_integer_natural_abs_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_integer_assign" => benchmark_random_integer_assign(limit, 1024, "temp.gp"),
                "random_integer_assign_evaluation_strategy" => {
                    benchmark_random_integer_assign_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_integer_assign_i32" => {
                    benchmark_random_integer_assign_i32(limit, 1024, "temp.gp")
                }
                "random_integer_assign_i64" => {
                    benchmark_random_integer_assign_i64(limit, 1024, "temp.gp")
                }
                "random_integer_assign_natural" => {
                    benchmark_random_integer_assign_natural(limit, 1024, "temp.gp")
                }
                "random_integer_assign_natural_evaluation_strategy" => {
                    benchmark_random_integer_assign_natural_evaluation_strategy(limit,
                                                                                1024,
                                                                                "temp.gp")
                }
                "random_integer_assign_u32" => {
                    benchmark_random_integer_assign_u32(limit, 1024, "temp.gp")
                }
                "random_integer_assign_u64" => {
                    benchmark_random_integer_assign_u64(limit, 1024, "temp.gp")
                }
                "random_integer_clone" => benchmark_random_integer_clone(limit, 1024, "temp.gp"),
                "random_integer_clone_from" => {
                    benchmark_random_integer_clone_from(limit, 1024, "temp.gp")
                }
                "random_integer_cmp" => benchmark_random_integer_cmp(limit, 1024, "temp.gp"),
                "random_integer_cmp_abs" => {
                    benchmark_random_integer_cmp_abs(limit, 1024, "temp.gp")
                }
                "random_integer_eq" => benchmark_random_integer_eq(limit, 1024, "temp.gp"),
                "random_integer_from_i32" => benchmark_random_integer_from_i32(limit, "temp.gp"),
                "random_integer_from_i64" => benchmark_random_integer_from_i64(limit, "temp.gp"),
                "random_integer_from_sign_and_limbs_le" => {
                    benchmark_random_integer_from_sign_and_limbs_le(limit, 128, "temp.gp")
                }
                "random_integer_from_sign_and_limbs_be" => {
                    benchmark_random_integer_from_sign_and_limbs_be(limit, 128, "temp.gp")
                }
                "random_integer_from_u32" => benchmark_random_integer_from_u32(limit, "temp.gp"),
                "random_integer_from_u64" => benchmark_random_integer_from_u64(limit, "temp.gp"),
                "random_integer_get_bit" => {
                    benchmark_random_integer_get_bit(limit, 1024, "temp.gp")
                }
                "random_integer_hash" => benchmark_random_integer_hash(limit, 1024, "temp.gp"),
                "random_integer_neg_assign" => {
                    benchmark_random_integer_neg_assign(limit, 1024, "temp.gp")
                }
                "random_integer_neg" => benchmark_random_integer_neg(limit, 1024, "temp.gp"),
                "random_integer_neg_evaluation_strategy" => {
                    benchmark_random_integer_neg_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_integer_not_assign" => {
                    benchmark_random_integer_not_assign(limit, 1024, "temp.gp")
                }
                "random_integer_not" => benchmark_random_integer_not(limit, 1024, "temp.gp"),
                "random_integer_not_evaluation_strategy" => {
                    benchmark_random_integer_not_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_integer_partial_cmp_abs_i32" => {
                    benchmark_random_integer_partial_cmp_abs_i32(limit, 1024, "temp.gp")
                }
                "random_i32_partial_cmp_abs_integer" => {
                    benchmark_random_i32_partial_cmp_abs_integer(limit, 1024, "temp.gp")
                }
                "random_integer_partial_cmp_abs_u32" => {
                    benchmark_random_integer_partial_cmp_abs_u32(limit, 1024, "temp.gp")
                }
                "random_u32_partial_cmp_abs_integer" => {
                    benchmark_random_u32_partial_cmp_abs_integer(limit, 1024, "temp.gp")
                }
                "random_integer_partial_cmp_abs_natural" => {
                    benchmark_random_integer_partial_cmp_abs_natural(limit, 1024, "temp.gp")
                }
                "random_integer_partial_cmp_i32" => {
                    benchmark_random_integer_partial_cmp_i32(limit, 1024, "temp.gp")
                }
                "random_i32_partial_cmp_integer" => {
                    benchmark_random_i32_partial_cmp_integer(limit, 1024, "temp.gp")
                }
                "random_integer_partial_cmp_u32" => {
                    benchmark_random_integer_partial_cmp_u32(limit, 1024, "temp.gp")
                }
                "random_u32_partial_cmp_integer" => {
                    benchmark_random_u32_partial_cmp_integer(limit, 1024, "temp.gp")
                }
                "random_integer_partial_cmp_natural" => {
                    benchmark_random_integer_partial_cmp_natural(limit, 1024, "temp.gp")
                }
                "random_integer_partial_eq_i32" => {
                    benchmark_random_integer_partial_eq_i32(limit, 1024, "temp.gp")
                }
                "random_i32_partial_eq_integer" => {
                    benchmark_random_i32_partial_eq_integer(limit, 1024, "temp.gp")
                }
                "random_integer_partial_eq_u32" => {
                    benchmark_random_integer_partial_eq_u32(limit, 1024, "temp.gp")
                }
                "random_u32_partial_eq_integer" => {
                    benchmark_random_u32_partial_eq_integer(limit, 1024, "temp.gp")
                }
                "random_integer_partial_eq_natural" => {
                    benchmark_random_integer_partial_eq_natural(limit, 1024, "temp.gp")
                }
                "random_integer_sign" => benchmark_random_integer_sign(limit, 1024, "temp.gp"),
                "random_integer_sign_and_limbs_le" => {
                    benchmark_random_integer_sign_and_limbs_le(limit, 1024, "temp.gp")
                }
                "random_integer_sign_and_limbs_be" => {
                    benchmark_random_integer_sign_and_limbs_be(limit, 1024, "temp.gp")
                }
                "random_integer_significant_bits" => {
                    benchmark_random_integer_significant_bits(limit, 1024, "temp.gp")
                }
                "random_integer_to_i32" => benchmark_random_integer_to_i32(limit, "temp.gp"),
                "random_integer_to_i32_wrapping" => {
                    benchmark_random_integer_to_i32_wrapping(limit, "temp.gp")
                }
                "random_integer_to_i64" => benchmark_random_integer_to_i64(limit, "temp.gp"),
                "random_integer_to_i64_wrapping" => {
                    benchmark_random_integer_to_i64_wrapping(limit, "temp.gp")
                }
                "random_integer_to_natural" => {
                    benchmark_random_integer_to_natural(limit, 1024, "temp.gp")
                }
                "random_integer_to_natural_evaluation_strategy" => {
                    benchmark_random_integer_to_natural_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_integer_to_u32" => benchmark_random_integer_to_u32(limit, "temp.gp"),
                "random_integer_to_u32_wrapping" => {
                    benchmark_random_integer_to_u32_wrapping(limit, "temp.gp")
                }
                "random_integer_to_u64" => benchmark_random_integer_to_u64(limit, "temp.gp"),
                "random_integer_to_u64_wrapping" => {
                    benchmark_random_integer_to_u64_wrapping(limit, "temp.gp")
                }
                "random_natural_add_assign" => {
                    benchmark_random_natural_add_assign(limit, 1024, "temp.gp")
                }
                "random_natural_add_assign_evaluation_strategy" => {
                    benchmark_random_natural_add_assign_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_natural_add" => benchmark_random_natural_add(limit, 1024, "temp.gp"),
                "random_natural_add_evaluation_strategy" => {
                    benchmark_random_natural_add_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_natural_add_assign_u32" => {
                    benchmark_random_natural_add_assign_u32(limit, 1024, "temp.gp")
                }
                "random_natural_add_u32" => {
                    benchmark_random_natural_add_u32(limit, 1024, "temp.gp")
                }
                "random_natural_add_u32_ref" => {
                    benchmark_random_natural_add_u32_ref(limit, 1024, "temp.gp")
                }
                "random_u32_add_natural" => {
                    benchmark_random_u32_add_natural(limit, 1024, "temp.gp")
                }
                "random_u32_add_natural_ref" => {
                    benchmark_random_u32_add_natural_ref(limit, 1024, "temp.gp")
                }
                "random_natural_assign" => benchmark_random_natural_assign(limit, 1024, "temp.gp"),
                "random_natural_assign_evaluation_strategy" => {
                    benchmark_random_natural_assign_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_natural_assign_integer" => {
                    benchmark_random_natural_assign_integer(limit, 1024, "temp.gp")
                }
                "random_natural_assign_integer_evaluation_strategy" => {
                    benchmark_random_natural_assign_integer_evaluation_strategy(limit,
                                                                                1024,
                                                                                "temp.gp")
                }
                "random_natural_assign_u32" => {
                    benchmark_random_natural_assign_u32(limit, 1024, "temp.gp")
                }
                "random_natural_assign_u64" => {
                    benchmark_random_natural_assign_u64(limit, 1024, "temp.gp")
                }
                "random_natural_assign_bit" => {
                    benchmark_random_natural_assign_bit(limit, 1024, "temp.gp")
                }
                "random_natural_clear_bit" => {
                    benchmark_random_natural_clear_bit(limit, 1024, "temp.gp")
                }
                "random_natural_clone" => benchmark_random_natural_clone(limit, 1024, "temp.gp"),
                "random_natural_clone_from" => {
                    benchmark_random_natural_clone_from(limit, 1024, "temp.gp")
                }
                "random_natural_cmp" => benchmark_random_natural_cmp(limit, 1024, "temp.gp"),
                "random_natural_eq" => benchmark_random_natural_eq(limit, 1024, "temp.gp"),
                "random_natural_flip_bit" => {
                    benchmark_random_natural_flip_bit(limit, 128, "temp.gp")
                }
                "random_natural_from_limbs_le" => {
                    benchmark_random_natural_from_limbs_le(limit, 128, "temp.gp")
                }
                "random_natural_from_limbs_be" => {
                    benchmark_random_natural_from_limbs_be(limit, 128, "temp.gp")
                }
                "random_natural_from_u32" => benchmark_random_natural_from_u32(limit, "temp.gp"),
                "random_natural_from_u64" => benchmark_random_natural_from_u64(limit, "temp.gp"),
                "random_natural_get_bit" => {
                    benchmark_random_natural_get_bit(limit, 1024, "temp.gp")
                }
                "random_natural_hash" => benchmark_random_natural_hash(limit, 1024, "temp.gp"),
                "random_natural_is_even" => {
                    benchmark_random_natural_is_even(limit, 1024, "temp.gp")
                }
                "random_natural_is_odd" => benchmark_random_natural_is_odd(limit, 1024, "temp.gp"),
                "random_natural_is_power_of_two" => {
                    benchmark_random_natural_is_power_of_two(limit, 1024, "temp.gp")
                }
                "random_natural_limb_count" => {
                    benchmark_random_natural_limb_count(limit, 1024, "temp.gp")
                }
                "random_natural_limbs_le" => {
                    benchmark_random_natural_limbs_le(limit, 1024, "temp.gp")
                }
                "random_natural_limbs_be" => {
                    benchmark_random_natural_limbs_be(limit, 1024, "temp.gp")
                }
                "random_natural_neg" => benchmark_random_natural_neg(limit, 1024, "temp.gp"),
                "random_natural_neg_evaluation_strategy" => {
                    benchmark_random_natural_neg_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_natural_not" => benchmark_random_natural_not(limit, 1024, "temp.gp"),
                "random_natural_not_evaluation_strategy" => {
                    benchmark_random_natural_not_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_natural_partial_cmp_abs_integer" => {
                    benchmark_random_natural_partial_cmp_abs_integer(limit, 1024, "temp.gp")
                }
                "random_natural_partial_cmp_integer" => {
                    benchmark_random_natural_partial_cmp_integer(limit, 1024, "temp.gp")
                }
                "random_natural_partial_cmp_u32" => {
                    benchmark_random_natural_partial_cmp_u32(limit, 1024, "temp.gp")
                }
                "random_u32_partial_cmp_natural" => {
                    benchmark_random_u32_partial_cmp_natural(limit, 1024, "temp.gp")
                }
                "random_natural_partial_eq_u32" => {
                    benchmark_random_natural_partial_eq_u32(limit, 1024, "temp.gp")
                }
                "random_u32_partial_eq_natural" => {
                    benchmark_random_u32_partial_eq_natural(limit, 1024, "temp.gp")
                }
                "random_natural_partial_eq_integer" => {
                    benchmark_random_natural_partial_eq_integer(limit, 1024, "temp.gp")
                }
                "random_natural_set_bit" => {
                    benchmark_random_natural_set_bit(limit, 1024, "temp.gp")
                }
                "random_natural_shl_assign_u32" => {
                    benchmark_random_natural_shl_assign_u32(limit, 1024, "temp.gp")
                }
                "random_natural_shl_u32" => {
                    benchmark_random_natural_shl_u32(limit, 1024, "temp.gp")
                }
                "random_natural_shl_u32_ref" => {
                    benchmark_random_natural_shl_u32_ref(limit, 1024, "temp.gp")
                }
                "random_natural_significant_bits" => {
                    benchmark_random_natural_significant_bits(limit, 1024, "temp.gp")
                }
                "random_natural_sub_assign" => {
                    benchmark_random_natural_sub_assign(limit, 1024, "temp.gp")
                }
                "random_natural_sub" => benchmark_random_natural_sub(limit, 1024, "temp.gp"),
                "random_natural_sub_evaluation_strategy" => {
                    benchmark_random_natural_sub_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_natural_sub_assign_u32" => {
                    benchmark_random_natural_sub_assign_u32(limit, 1024, "temp.gp")
                }
                "random_natural_sub_u32" => {
                    benchmark_random_natural_sub_u32(limit, 1024, "temp.gp")
                }
                "random_natural_sub_u32_ref" => {
                    benchmark_random_natural_sub_u32_ref(limit, 1024, "temp.gp")
                }
                "random_u32_sub_natural" => {
                    benchmark_random_u32_sub_natural(limit, 1024, "temp.gp")
                }
                "random_natural_to_integer" => {
                    benchmark_random_natural_to_integer(limit, 1024, "temp.gp")
                }
                "random_natural_to_integer_evaluation_strategy" => {
                    benchmark_random_natural_to_integer_evaluation_strategy(limit, 1024, "temp.gp")
                }
                "random_natural_to_u32" => benchmark_random_natural_to_u32(limit, "temp.gp"),
                "random_natural_to_u32_wrapping" => {
                    benchmark_random_natural_to_u32_wrapping(limit, "temp.gp")
                }
                "random_natural_to_u64" => benchmark_random_natural_to_u64(limit, "temp.gp"),
                "random_natural_to_u64_wrapping" => {
                    benchmark_random_natural_to_u64_wrapping(limit, "temp.gp")
                }
                "random_natural_trailing_zeros" => {
                    benchmark_random_natural_trailing_zeros(limit, 1024, "temp.gp")
                }

                "all" => {
                    benchmark_exhaustive_integer_abs_assign(100000,
                                                            "exhaustive_integer_abs_assign.gp");
                    benchmark_exhaustive_integer_abs(100000, "exhaustive_integer_abs.gp");
                    let s = "exhaustive_integer_abs_evaluation_strategy.gp";
                    benchmark_exhaustive_integer_abs_evaluation_strategy(100000, s);
                    benchmark_exhaustive_integer_natural_abs(100000,
                                                             "exhaustive_integer_natural_abs.gp");
                    let s = "exhaustive_integer_natural_abs_evaluation_strategy.gp";
                    benchmark_exhaustive_integer_natural_abs_evaluation_strategy(100000, s);
                    benchmark_exhaustive_integer_assign(100000, "exhaustive_integer_assign.gp");
                    let s = "exhaustive_integer_assign_evaluation_strategy.gp";
                    benchmark_exhaustive_integer_assign_evaluation_strategy(100000, s);
                    benchmark_exhaustive_integer_assign_i32(100000,
                                                            "exhaustive_integer_assign_i32.gp");
                    benchmark_exhaustive_integer_assign_i64(100000,
                                                            "exhaustive_integer_assign_i64.gp");
                    let s = "exhaustive_integer_assign_natural.gp";
                    benchmark_exhaustive_integer_assign_natural(100000, s);
                    let s = "exhaustive_integer_assign_natural_evaluation_strategy.gp";
                    benchmark_exhaustive_integer_assign_natural_evaluation_strategy(100000, s);
                    benchmark_exhaustive_integer_assign_u32(100000,
                                                            "exhaustive_integer_assign_u32.gp");
                    benchmark_exhaustive_integer_assign_u64(100000,
                                                            "exhaustive_integer_assign_u64.gp");
                    benchmark_exhaustive_integer_clone(100000, "exhaustive_integer_clone.gp");
                    benchmark_exhaustive_integer_clone_from(100000,
                                                            "exhaustive_integer_clone_from.gp");
                    benchmark_exhaustive_integer_cmp(100000, "exhaustive_integer_cmp.gp");
                    benchmark_exhaustive_integer_cmp_abs(100000, "exhaustive_integer_cmp_abs.gp");
                    benchmark_exhaustive_integer_eq(100000, "exhaustive_integer_eq.gp");
                    benchmark_exhaustive_integer_from_i32(100000, "exhaustive_integer_from_i32.gp");
                    benchmark_exhaustive_integer_from_i64(100000, "exhaustive_integer_from_i64.gp");
                    let s = "exhaustive_integer_from_sign_and_limbs_le.gp";
                    benchmark_exhaustive_integer_from_sign_and_limbs_le(100000, s);
                    let s = "exhaustive_integer_from_sign_and_limbs_be.gp";
                    benchmark_exhaustive_integer_from_sign_and_limbs_be(100000, s);
                    benchmark_exhaustive_integer_from_u32(100000, "exhaustive_integer_from_u32.gp");
                    benchmark_exhaustive_integer_from_u64(100000, "exhaustive_integer_from_u64.gp");
                    benchmark_exhaustive_integer_get_bit(100000, "exhaustive_integer_get_bit.gp");
                    benchmark_exhaustive_integer_hash(100000, "exhaustive_integer_hash.gp");
                    benchmark_exhaustive_integer_neg_assign(100000,
                                                            "exhaustive_integer_neg_assign.gp");
                    benchmark_exhaustive_integer_neg(100000, "exhaustive_integer_neg.gp");
                    let s = "exhaustive_integer_neg_evaluation_strategy.gp";
                    benchmark_exhaustive_integer_neg_evaluation_strategy(100000, s);
                    benchmark_exhaustive_integer_not_assign(100000,
                                                            "exhaustive_integer_not_assign.gp");
                    benchmark_exhaustive_integer_not(100000, "exhaustive_integer_not.gp");
                    let s = "exhaustive_integer_not_evaluation_strategy.gp";
                    benchmark_exhaustive_integer_not_evaluation_strategy(100000, s);
                    let s = "exhaustive_integer_partial_cmp_abs_i32.gp";
                    benchmark_exhaustive_integer_partial_cmp_abs_i32(100000, s);
                    let s = "exhaustive_i32_partial_cmp_abs_integer.gp";
                    benchmark_exhaustive_i32_partial_cmp_abs_integer(100000, s);
                    let s = "exhaustive_integer_partial_cmp_abs_u32.gp";
                    benchmark_exhaustive_integer_partial_cmp_abs_u32(100000, s);
                    let s = "exhaustive_u32_partial_cmp_abs_integer.gp";
                    benchmark_exhaustive_u32_partial_cmp_abs_integer(100000, s);
                    let s = "exhaustive_integer_partial_cmp_abs_natural.gp";
                    benchmark_exhaustive_integer_partial_cmp_abs_natural(100000, s);
                    let s = "exhaustive_integer_partial_cmp_i32.gp";
                    benchmark_exhaustive_integer_partial_cmp_i32(100000, s);
                    let s = "exhaustive_i32_partial_cmp_integer.gp";
                    benchmark_exhaustive_i32_partial_cmp_integer(100000, s);
                    let s = "exhaustive_integer_partial_cmp_u32.gp";
                    benchmark_exhaustive_integer_partial_cmp_u32(100000, s);
                    let s = "exhaustive_u32_partial_cmp_integer.gp";
                    benchmark_exhaustive_u32_partial_cmp_integer(100000, s);
                    let s = "exhaustive_integer_partial_cmp_natural.gp";
                    benchmark_exhaustive_integer_partial_cmp_natural(100000, s);
                    let s = "exhaustive_integer_partial_eq_u32.gp";
                    benchmark_exhaustive_integer_partial_eq_u32(100000, s);
                    let s = "exhaustive_u32_partial_eq_integer.gp";
                    benchmark_exhaustive_u32_partial_eq_integer(100000, s);
                    let s = "exhaustive_integer_partial_eq_natural.gp";
                    benchmark_exhaustive_integer_partial_eq_natural(100000, s);
                    benchmark_exhaustive_integer_sign(100000, "exhaustive_integer_sign.gp");
                    let s = "exhaustive_integer_sign_and_limbs_le.gp";
                    benchmark_exhaustive_integer_sign_and_limbs_le(100000, s);
                    let s = "exhaustive_integer_sign_and_limbs_be.gp";
                    benchmark_exhaustive_integer_sign_and_limbs_be(100000, s);
                    let s = "exhaustive_integer_significant_bits.gp";
                    benchmark_exhaustive_integer_significant_bits(100000, s);
                    benchmark_exhaustive_integer_to_i32(100000, "exhaustive_integer_to_i32.gp");
                    let s = "exhaustive_integer_to_i32_wrapping.gp";
                    benchmark_exhaustive_integer_to_i32_wrapping(100000, s);
                    benchmark_exhaustive_integer_to_i64(100000, "exhaustive_integer_to_i64.gp");
                    let s = "exhaustive_integer_to_i64_wrapping.gp";
                    benchmark_exhaustive_integer_to_i64_wrapping(100000, s);
                    benchmark_exhaustive_integer_to_natural(100000,
                                                            "exhaustive_integer_to_natural.gp");
                    let s = "exhaustive_integer_to_natural_evaluation_strategy.gp";
                    benchmark_exhaustive_integer_to_natural_evaluation_strategy(100000, s);
                    benchmark_exhaustive_integer_to_u32(100000, "exhaustive_integer_to_u32.gp");
                    let s = "exhaustive_integer_to_u32_wrapping.gp";
                    benchmark_exhaustive_integer_to_u32_wrapping(100000, s);
                    benchmark_exhaustive_integer_to_u64(100000, "exhaustive_integer_to_u64.gp");
                    let s = "exhaustive_integer_to_u64_wrapping.gp";
                    benchmark_exhaustive_integer_to_u64_wrapping(100000, s);
                    benchmark_exhaustive_natural_add_assign(100000,
                                                            "exhaustive_natural_add_assign.gp");
                    let s = "exhaustive_natural_add_assign_evaluation_strategy.gp";
                    benchmark_exhaustive_natural_add_assign_evaluation_strategy(100000, s);
                    benchmark_exhaustive_natural_add(100000, "exhaustive_natural_add.gp");
                    let s = "exhaustive_natural_add_evaluation_strategy.gp";
                    benchmark_exhaustive_natural_add_evaluation_strategy(100000, s);
                    let s = "exhaustive_natural_add_assign_u32.gp";
                    benchmark_exhaustive_natural_add_assign_u32(100000, s);
                    benchmark_exhaustive_natural_add_u32(100000, "exhaustive_natural_add_u32.gp");
                    benchmark_exhaustive_natural_add_u32_ref(100000,
                                                             "exhaustive_natural_add_u32_ref.gp");
                    benchmark_exhaustive_u32_add_natural(100000, "exhaustive_u32_add_natural.gp");
                    benchmark_exhaustive_u32_add_natural_ref(100000,
                                                             "exhaustive_u32_add_natural_ref.gp");
                    benchmark_exhaustive_natural_assign(100000, "exhaustive_natural_assign.gp");
                    let s = "exhaustive_natural_assign_evaluation_strategy.gp";
                    benchmark_exhaustive_natural_assign_evaluation_strategy(100000, s);
                    let s = "exhaustive_natural_assign_integer.gp";
                    benchmark_exhaustive_natural_assign_integer(100000, s);
                    let s = "exhaustive_natural_assign_integer_evaluation_strategy.gp";
                    benchmark_exhaustive_natural_assign_integer_evaluation_strategy(100000, s);
                    benchmark_exhaustive_natural_assign_u32(100000,
                                                            "exhaustive_natural_assign_u32.gp");
                    benchmark_exhaustive_natural_assign_u64(100000,
                                                            "exhaustive_natural_assign_u64.gp");
                    benchmark_exhaustive_natural_assign_bit(100000,
                                                            "exhaustive_natural_assign_bit.gp");
                    benchmark_exhaustive_natural_clear_bit(100000,
                                                           "exhaustive_natural_clear_bit.gp");
                    benchmark_exhaustive_natural_clone(100000, "exhaustive_natural_clone.gp");
                    benchmark_exhaustive_natural_clone_from(100000,
                                                            "exhaustive_natural_clone_from.gp");
                    benchmark_exhaustive_natural_cmp(100000, "exhaustive_natural_cmp.gp");
                    benchmark_exhaustive_natural_eq(100000, "exhaustive_natural_eq.gp");
                    let s = "exhaustive_natural_flip_bit.gp";
                    benchmark_exhaustive_natural_flip_bit(100000, s);
                    let s = "exhaustive_natural_from_limbs_le.gp";
                    benchmark_exhaustive_natural_from_limbs_le(100000, s);
                    let s = "exhaustive_natural_from_limbs_be.gp";
                    benchmark_exhaustive_natural_from_limbs_be(100000, s);
                    benchmark_exhaustive_natural_from_u32(100000, "exhaustive_natural_from_u32.gp");
                    benchmark_exhaustive_natural_from_u64(100000, "exhaustive_natural_from_u64.gp");
                    benchmark_exhaustive_natural_get_bit(100000, "exhaustive_natural_get_bit.gp");
                    benchmark_exhaustive_natural_hash(100000, "exhaustive_natural_hash.gp");
                    benchmark_exhaustive_natural_is_even(100000, "exhaustive_natural_is_even.gp");
                    benchmark_exhaustive_natural_is_odd(100000, "exhaustive_natural_is_odd.gp");
                    let s = "exhaustive_natural_is_power_of_two.gp";
                    benchmark_exhaustive_natural_is_power_of_two(100000, s);
                    benchmark_exhaustive_natural_limb_count(100000,
                                                            "exhaustive_natural_limb_count.gp");
                    benchmark_exhaustive_natural_limbs_le(100000, "exhaustive_natural_limbs_le.gp");
                    benchmark_exhaustive_natural_limbs_be(100000, "exhaustive_natural_limbs_be.gp");
                    benchmark_exhaustive_natural_neg(100000, "exhaustive_natural_neg.gp");
                    let s = "exhaustive_natural_neg_evaluation_strategy.gp";
                    benchmark_exhaustive_natural_neg_evaluation_strategy(100000, s);
                    benchmark_exhaustive_natural_not(100000, "exhaustive_natural_not.gp");
                    let s = "exhaustive_natural_not_evaluation_strategy.gp";
                    benchmark_exhaustive_natural_not_evaluation_strategy(100000, s);
                    let s = "exhaustive_natural_partial_cmp_abs_integer.gp";
                    benchmark_exhaustive_natural_partial_cmp_abs_integer(100000, s);
                    let s = "exhaustive_natural_partial_cmp_u32.gp";
                    benchmark_exhaustive_natural_partial_cmp_u32(100000, s);
                    let s = "exhaustive_u32_partial_cmp_natural.gp";
                    benchmark_exhaustive_u32_partial_cmp_natural(100000, s);
                    let s = "exhaustive_natural_partial_cmp_integer.gp";
                    benchmark_exhaustive_natural_partial_cmp_integer(100000, s);
                    let s = "exhaustive_natural_partial_eq_integer.gp";
                    benchmark_exhaustive_natural_partial_eq_integer(100000, s);
                    let s = "exhaustive_natural_partial_eq_u32.gp";
                    benchmark_exhaustive_natural_partial_eq_u32(100000, s);
                    let s = "exhaustive_u32_partial_eq_natural.gp";
                    benchmark_exhaustive_u32_partial_eq_natural(100000, s);
                    benchmark_exhaustive_natural_set_bit(100000, "exhaustive_natural_set_bit.gp");
                    let s = "exhaustive_natural_shl_assign_u32.gp";
                    benchmark_exhaustive_natural_shl_assign_u32(100000, s);
                    benchmark_exhaustive_natural_shl_u32(100000, "exhaustive_natural_shl_u32.gp");
                    benchmark_exhaustive_natural_shl_u32_ref(100000,
                                                             "exhaustive_natural_shl_u32_ref.gp");
                    let s = "exhaustive_natural_significant_bits.gp";
                    benchmark_exhaustive_natural_significant_bits(100000, s);
                    benchmark_exhaustive_natural_sub_assign(100000,
                                                            "exhaustive_natural_sub_assign.gp");
                    benchmark_exhaustive_natural_sub(100000, "exhaustive_natural_sub.gp");
                    let s = "exhaustive_natural_sub_evaluation_strategy.gp";
                    benchmark_exhaustive_natural_sub_evaluation_strategy(100000, s);
                    let s = "exhaustive_natural_sub_assign_u32.gp";
                    benchmark_exhaustive_natural_sub_assign_u32(100000, s);
                    benchmark_exhaustive_natural_sub_u32(100000, "exhaustive_natural_sub_u32.gp");
                    benchmark_exhaustive_natural_sub_u32_ref(100000,
                                                             "exhaustive_natural_sub_u32_ref.gp");
                    benchmark_exhaustive_u32_sub_natural(100000, "exhaustive_u32_sub_natural.gp");
                    benchmark_exhaustive_natural_to_integer(100000,
                                                            "exhaustive_natural_to_integer.gp");
                    let s = "exhaustive_natural_to_integer_evaluation_strategy.gp";
                    benchmark_exhaustive_natural_to_integer_evaluation_strategy(100000, s);
                    benchmark_exhaustive_natural_to_u32(100000, "exhaustive_natural_to_u32.gp");
                    let s = "exhaustive_natural_to_u32_wrapping.gp";
                    benchmark_exhaustive_natural_to_u32_wrapping(100000, s);
                    benchmark_exhaustive_natural_to_u64(100000, "exhaustive_natural_to_u64.gp");
                    let s = "exhaustive_natural_to_u64_wrapping.gp";
                    benchmark_exhaustive_natural_to_u64_wrapping(100000, s);
                    let s = "exhaustive_natural_trailing_zeros.gp";
                    benchmark_exhaustive_natural_trailing_zeros(100000, s);
                    benchmark_random_integer_abs_assign(100000,
                                                        1024,
                                                        "random_integer_abs_assign.gp");
                    benchmark_random_integer_abs(100000, 1024, "random_integer_abs.gp");
                    let s = "random_integer_abs_evaluation_strategy.gp";
                    benchmark_random_integer_abs_evaluation_strategy(100000, 1024, s);
                    benchmark_random_integer_natural_abs(100000,
                                                         1024,
                                                         "random_integer_natural_abs.gp");
                    let s = "random_integer_natural_abs_evaluation_strategy.gp";
                    benchmark_random_integer_natural_abs_evaluation_strategy(100000, 1024, s);
                    benchmark_random_integer_assign(100000, 1024, "random_integer_assign.gp");
                    let s = "random_integer_assign_evaluation_strategy.gp";
                    benchmark_random_integer_assign_evaluation_strategy(100000, 1024, s);
                    benchmark_random_integer_assign_i32(100000,
                                                        1024,
                                                        "random_integer_assign_i32.gp");
                    benchmark_random_integer_assign_i64(100000,
                                                        1024,
                                                        "random_integer_assign_i64.gp");
                    benchmark_random_integer_assign_natural(100000,
                                                            1024,
                                                            "random_integer_assign_natural.gp");
                    let s = "random_integer_assign_natural_evaluation_strategy.gp";
                    benchmark_random_integer_assign_natural_evaluation_strategy(100000, 1024, s);
                    benchmark_random_integer_assign_u32(100000,
                                                        1024,
                                                        "random_integer_assign_u32.gp");
                    benchmark_random_integer_assign_u64(100000,
                                                        1024,
                                                        "random_integer_assign_u64.gp");
                    benchmark_random_integer_clone(100000, 1024, "random_integer_clone.gp");
                    benchmark_random_integer_clone_from(100000,
                                                        1024,
                                                        "random_integer_clone_from.gp");
                    benchmark_random_integer_cmp(100000, 1024, "random_integer_cmp.gp");
                    benchmark_random_integer_cmp_abs(100000, 1024, "random_integer_cmp_abs.gp");
                    benchmark_random_integer_eq(100000, 1024, "random_integer_eq.gp");
                    benchmark_random_integer_from_i32(100000, "random_integer_from_i32.gp");
                    benchmark_random_integer_from_i64(100000, "random_integer_from_i64.gp");
                    let s = "random_integer_from_sign_and_limbs_le.gp";
                    benchmark_random_integer_from_sign_and_limbs_le(100000, 128, s);
                    let s = "random_integer_from_sign_and_limbs_be.gp";
                    benchmark_random_integer_from_sign_and_limbs_be(100000, 128, s);
                    benchmark_random_integer_from_u32(100000, "random_integer_from_u32.gp");
                    benchmark_random_integer_from_u64(100000, "random_integer_from_u64.gp");
                    benchmark_random_integer_get_bit(100000, 1024, "random_integer_get_bit.gp");
                    benchmark_random_integer_hash(100000, 1024, "random_integer_hash.gp");
                    benchmark_random_integer_neg_assign(100000,
                                                        1024,
                                                        "random_integer_neg_assign.gp");
                    benchmark_random_integer_neg(100000, 1024, "random_integer_neg.gp");
                    let s = "random_integer_neg_evaluation_strategy.gp";
                    benchmark_random_integer_neg_evaluation_strategy(100000, 1024, s);
                    benchmark_random_integer_not_assign(100000,
                                                        1024,
                                                        "random_integer_not_assign.gp");
                    benchmark_random_integer_not(100000, 1024, "random_integer_not.gp");
                    let s = "random_integer_not_evaluation_strategy.gp";
                    benchmark_random_integer_not_evaluation_strategy(100000, 1024, s);
                    let s = "random_integer_partial_cmp_abs_i32.gp";
                    benchmark_random_integer_partial_cmp_abs_i32(100000, 1024, s);
                    let s = "random_i32_partial_cmp_abs_integer.gp";
                    benchmark_random_i32_partial_cmp_abs_integer(100000, 1024, s);
                    let s = "random_integer_partial_cmp_abs_u32.gp";
                    benchmark_random_integer_partial_cmp_abs_u32(100000, 1024, s);
                    let s = "random_u32_partial_cmp_abs_integer.gp";
                    benchmark_random_u32_partial_cmp_abs_integer(100000, 1024, s);
                    let s = "random_integer_partial_cmp_abs_natural.gp";
                    benchmark_random_integer_partial_cmp_abs_natural(100000, 1024, s);
                    let s = "random_integer_partial_cmp_i32.gp";
                    benchmark_random_integer_partial_cmp_i32(100000, 1024, s);
                    let s = "random_i32_partial_cmp_integer.gp";
                    benchmark_random_i32_partial_cmp_integer(100000, 1024, s);
                    let s = "random_integer_partial_cmp_u32.gp";
                    benchmark_random_integer_partial_cmp_u32(100000, 1024, s);
                    let s = "random_u32_partial_cmp_integer.gp";
                    benchmark_random_u32_partial_cmp_integer(100000, 1024, s);
                    let s = "random_integer_partial_cmp_natural.gp";
                    benchmark_random_integer_partial_cmp_natural(100000, 1024, s);
                    let s = "random_integer_partial_eq_u32.gp";
                    benchmark_random_integer_partial_eq_u32(100000, 1024, s);
                    let s = "random_u32_partial_eq_integer.gp";
                    benchmark_random_u32_partial_eq_integer(100000, 1024, s);
                    let s = "random_integer_partial_eq_natural.gp";
                    benchmark_random_integer_partial_eq_natural(100000, 1024, s);
                    benchmark_random_integer_sign(100000, 1024, "random_integer_sign.gp");
                    let s = "random_integer_sign_and_limbs_le.gp";
                    benchmark_random_integer_sign_and_limbs_le(100000, 1024, s);
                    let s = "random_integer_sign_and_limbs_be.gp";
                    benchmark_random_integer_sign_and_limbs_be(100000, 1024, s);
                    benchmark_random_integer_significant_bits(100000,
                                                              1024,
                                                              "random_integer_significant_bits.gp");
                    benchmark_random_integer_to_i32(100000, "random_integer_to_i32.gp");
                    benchmark_random_integer_to_i32_wrapping(100000,
                                                             "random_integer_to_i32_wrapping.gp");
                    benchmark_random_integer_to_i64(100000, "random_integer_to_i64.gp");
                    benchmark_random_integer_to_i64_wrapping(100000,
                                                             "random_integer_to_i64_wrapping.gp");
                    benchmark_random_integer_to_natural(100000,
                                                        1024,
                                                        "random_integer_to_natural.gp");
                    let s = "random_integer_to_natural_evaluation_strategy.gp";
                    benchmark_random_integer_to_natural_evaluation_strategy(100000, 1024, s);
                    benchmark_random_integer_to_u32(100000, "random_integer_to_u32.gp");
                    benchmark_random_integer_to_u32_wrapping(100000,
                                                             "random_integer_to_u32_wrapping.gp");
                    benchmark_random_integer_to_u64(100000, "random_integer_to_u64.gp");
                    benchmark_random_integer_to_u64_wrapping(100000,
                                                             "random_integer_to_u64_wrapping.gp");
                    benchmark_random_natural_add_assign(100000,
                                                        1024,
                                                        "random_natural_add_assign.gp");
                    let s = "random_natural_add_assign_evaluation_strategy.gp";
                    benchmark_random_natural_add_assign_evaluation_strategy(100000, 1024, s);
                    benchmark_random_natural_add(100000, 1024, "random_natural_add.gp");
                    let s = "random_natural_add_evaluation_strategy.gp";
                    benchmark_random_natural_add_evaluation_strategy(100000, 1024, s);
                    benchmark_random_natural_add_assign_u32(100000,
                                                            1024,
                                                            "random_natural_add_assign_u32.gp");
                    benchmark_random_natural_add_u32(100000, 1024, "random_natural_add_u32.gp");
                    benchmark_random_natural_add_u32_ref(100000,
                                                         1024,
                                                         "random_natural_add_u32_ref.gp");
                    benchmark_random_u32_add_natural(100000, 1024, "random_u32_add_natural.gp");
                    benchmark_random_u32_add_natural_ref(100000,
                                                         1024,
                                                         "random_u32_add_natural_ref.gp");
                    benchmark_random_natural_assign(100000, 1024, "random_natural_assign.gp");
                    benchmark_random_natural_assign_integer(100000,
                                                            1024,
                                                            "random_natural_assign_integer.gp");
                    let s = "random_natural_assign_integer_evaluation_strategy.gp";
                    benchmark_random_natural_assign_integer_evaluation_strategy(100000, 1024, s);
                    benchmark_random_natural_assign_u32(100000,
                                                        1024,
                                                        "random_natural_assign_u32.gp");
                    benchmark_random_natural_assign_u64(100000,
                                                        1024,
                                                        "random_natural_assign_u64.gp");
                    benchmark_random_natural_assign_bit(100000,
                                                        1024,
                                                        "random_natural_assign_bit.gp");
                    benchmark_random_natural_clear_bit(100000, 1024, "random_natural_clear_bit.gp");
                    benchmark_random_natural_clone(100000, 1024, "random_natural_clone.gp");
                    benchmark_random_natural_clone_from(100000,
                                                        1024,
                                                        "random_natural_clone_from.gp");
                    benchmark_random_natural_cmp(100000, 1024, "random_natural_cmp.gp");
                    benchmark_random_natural_eq(100000, 1024, "random_natural_eq.gp");
                    benchmark_random_natural_flip_bit(100000, 128, "random_natural_flip_bit.gp");
                    benchmark_random_natural_from_limbs_le(100000,
                                                           128,
                                                           "random_natural_from_limbs_le.gp");
                    benchmark_random_natural_from_limbs_be(100000,
                                                           128,
                                                           "random_natural_from_limbs_be.gp");
                    benchmark_random_natural_from_u32(100000, "random_natural_from_u32.gp");
                    benchmark_random_natural_from_u64(100000, "random_natural_from_u64.gp");
                    benchmark_random_natural_get_bit(100000, 1024, "random_natural_get_bit.gp");
                    benchmark_random_natural_hash(100000, 1024, "random_natural_hash.gp");
                    benchmark_random_natural_is_even(100000, 1024, "random_natural_is_even.gp");
                    benchmark_random_natural_is_odd(100000, 1024, "random_natural_is_odd.gp");
                    benchmark_random_natural_is_power_of_two(100000,
                                                             1024,
                                                             "random_natural_is_power_of_two.gp");
                    benchmark_random_natural_limb_count(100000,
                                                        1024,
                                                        "random_natural_limb_count.gp");
                    benchmark_random_natural_limbs_le(100000, 1024, "random_natural_limbs_le.gp");
                    benchmark_random_natural_limbs_be(100000, 1024, "random_natural_limbs_be.gp");
                    benchmark_random_natural_neg(100000, 1024, "random_natural_neg.gp");
                    let s = "random_natural_neg_evaluation_strategy.gp";
                    benchmark_random_natural_neg_evaluation_strategy(100000, 1024, s);
                    benchmark_random_natural_not(100000, 1024, "random_natural_not.gp");
                    let s = "random_natural_not_evaluation_strategy.gp";
                    benchmark_random_natural_not_evaluation_strategy(100000, 1024, s);
                    let s = "random_natural_partial_cmp_integer.gp";
                    benchmark_random_natural_partial_cmp_integer(100000, 1024, s);
                    benchmark_random_natural_partial_cmp_u32(100000,
                                                             1024,
                                                             "random_natural_partial_cmp_u32.gp");
                    benchmark_random_u32_partial_cmp_natural(100000,
                                                             1024,
                                                             "random_u32_partial_cmp_natural.gp");
                    let s = "random_natural_partial_eq_integer.gp";
                    benchmark_random_natural_partial_eq_integer(100000, 1024, s);
                    benchmark_random_natural_partial_eq_u32(100000,
                                                            1024,
                                                            "random_natural_partial_eq_u32.gp");
                    benchmark_random_u32_partial_eq_natural(100000,
                                                            1024,
                                                            "random_u32_partial_eq_natural.gp");
                    benchmark_random_natural_set_bit(100000, 1024, "random_natural_set_bit.gp");
                    benchmark_random_natural_shl_assign_u32(100000,
                                                            1024,
                                                            "random_natural_shl_assign_u32.gp");
                    benchmark_random_natural_shl_u32(100000, 1024, "random_natural_shl_u32.gp");
                    benchmark_random_natural_shl_u32_ref(100000,
                                                         1024,
                                                         "random_natural_shl_u32_ref.gp");
                    benchmark_random_natural_significant_bits(100000,
                                                              1024,
                                                              "random_natural_significant_bits.gp");
                    benchmark_random_natural_sub_assign(100000,
                                                        1024,
                                                        "random_natural_sub_assign.gp");
                    benchmark_random_natural_sub(100000, 1024, "random_natural_sub.gp");
                    let s = "random_natural_sub_evaluation_strategy.gp";
                    benchmark_random_natural_sub_evaluation_strategy(100000, 1024, s);
                    benchmark_random_natural_sub_assign_u32(100000,
                                                            1024,
                                                            "random_natural_sub_assign_u32.gp");
                    benchmark_random_natural_sub_u32(100000, 1024, "random_natural_sub_u32.gp");
                    benchmark_random_natural_sub_u32_ref(100000,
                                                         1024,
                                                         "random_natural_sub_u32_ref.gp");
                    benchmark_random_u32_sub_natural(100000, 1024, "random_u32_sub_natural.gp");
                    benchmark_random_natural_to_integer(100000,
                                                        1024,
                                                        "random_natural_to_integer.gp");
                    let s = "random_natural_to_integer_evaluation_strategy.gp";
                    benchmark_random_natural_to_integer_evaluation_strategy(100000, 1024, s);
                    benchmark_random_natural_to_u32(100000, "random_natural_to_u32.gp");
                    benchmark_random_natural_to_u32_wrapping(100000,
                                                             "random_natural_to_u32_wrapping.gp");
                    benchmark_random_natural_to_u64(100000, "random_natural_to_u64.gp");
                    benchmark_random_natural_to_u64_wrapping(100000,
                                                             "random_natural_to_u64_wrapping.gp");
                    benchmark_random_natural_trailing_zeros(100000,
                                                            1024,
                                                            "random_natural_trailing_zeros.gp");
                }

                _ => panic!("Invalid bench name: {}", item_name),
            }
        }
        _ => panic!("Invalid item_type: {}", args[1]),
    }
}
