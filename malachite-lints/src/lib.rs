// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod assert_ordering_equal_prefer_exact;
mod assign_then_consumed_once;
mod clone_with_ref_variant;
mod compare_with_power_of_2;
mod compare_with_primitive;
mod let_tuple_underscore_to_field;
mod long_lines;
mod manual_float_from_primitive;
mod manual_from_sign_and_abs;
mod manual_rational_significant_bits;
mod missing_inline_on_delegator;
mod mul_div_by_power_of_2;
mod mul_div_by_power_of_2_literal;
mod redundant_from_in_comparison;
mod redundant_from_in_literal_comparison;
mod redundant_nearest;
mod redundant_prec_round_of_exact_constant;
mod runtime_literal_conversion;
mod shift_of_one;
mod use_assign_variant;
mod use_checked_log_base_2;
mod use_divisible_by;
mod use_exact_from;
mod use_named_constant;
mod use_parity;
mod use_reciprocal;
mod use_round_variant;
mod use_saturating_from;
mod use_square;
mod use_trailing_zeros;
mod use_width_mask;

dylint_linting::dylint_library!();

const BIGNUM_TYPES: [(&[&str], &str); 4] = [
    (&["malachite_nz", "natural", "Natural"], "Natural"),
    (&["malachite_nz", "integer", "Integer"], "Integer"),
    (&["malachite_q", "Rational"], "Rational"),
    (&["malachite_float", "Float"], "Float"),
];

// If `e` (possibly behind `&`) is a call to `T::power_of_2` where `T` is a Malachite bignum type,
// returns `T`'s unqualified name.
fn power_of_2_call<'tcx>(
    cx: &rustc_lint::LateContext<'tcx>,
    e: &'tcx rustc_hir::Expr<'tcx>,
) -> Option<&'static str> {
    use rustc_hir::ExprKind;
    use rustc_hir::def::{DefKind, Res};
    let e = e.peel_borrows();
    let ExprKind::Call(callee, [_]) = e.kind else {
        return None;
    };
    let ExprKind::Path(qpath) = &callee.kind else {
        return None;
    };
    let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
        return None;
    };
    if cx.tcx.item_name(fn_did).as_str() != "power_of_2" {
        return None;
    }
    bignum_name(cx, cx.typeck_results().expr_ty(e))
}

// If `ty` is one of the Malachite bignum types, returns its unqualified name. The def path is
// compared via `get_def_path`, which includes the crate name even for local types --
// `def_path_str` would not, leaving the lints blind inside the very crates that define the
// types.
fn bignum_name<'tcx>(
    cx: &rustc_lint::LateContext<'tcx>,
    ty: rustc_middle::ty::Ty<'tcx>,
) -> Option<&'static str> {
    let rustc_middle::ty::Adt(adt, _) = ty.kind() else {
        return None;
    };
    let path = cx.get_def_path(adt.did());
    BIGNUM_TYPES
        .iter()
        .find(|(p, _)| {
            path.len() == p.len() && path.iter().zip(p.iter()).all(|(a, b)| a.as_str() == *b)
        })
        .map(|&(_, name)| name)
}

// Strips any trailing `_val`/`_ref` variant suffixes, so that all by-value/by-reference variants
// of a function family normalize to the same name.
fn strip_variant_suffixes(mut name: &str) -> &str {
    loop {
        if let Some(stripped) = name
            .strip_suffix("_val")
            .or_else(|| name.strip_suffix("_ref"))
        {
            name = stripped;
        } else {
            return name;
        }
    }
}

// Whether the type `self_ty_did` has an inherent associated function named `name`.
fn has_inherent_fn(
    cx: &rustc_lint::LateContext<'_>,
    self_ty_did: rustc_hir::def_id::DefId,
    name: &str,
) -> bool {
    use rustc_hir::def::DefKind;
    let sym = rustc_span::Symbol::intern(name);
    cx.tcx.inherent_impls(self_ty_did).iter().any(|impl_did| {
        cx.tcx
            .associated_items(*impl_did)
            .filter_by_name_unhygienic(sym)
            .any(|item| matches!(cx.tcx.def_kind(item.def_id), DefKind::AssocFn))
    })
}

// The `DefId` of the ADT of `ty`, if `ty` (after peeling references) is one of the bignum types.
fn bignum_adt_did<'tcx>(
    cx: &rustc_lint::LateContext<'tcx>,
    ty: rustc_middle::ty::Ty<'tcx>,
) -> Option<rustc_hir::def_id::DefId> {
    let ty = ty.peel_refs();
    let rustc_middle::ty::Adt(adt, _) = ty.kind() else {
        return None;
    };
    bignum_name(cx, ty).map(|_| adt.did())
}

// Peels `&` and `.clone()` layers off an expression, for comparing against the underlying place.
fn peel_clone_and_borrows<'tcx>(e: &'tcx rustc_hir::Expr<'tcx>) -> &'tcx rustc_hir::Expr<'tcx> {
    use rustc_hir::ExprKind;
    let mut e = e;
    loop {
        match e.kind {
            ExprKind::AddrOf(_, _, inner) => e = inner,
            ExprKind::MethodCall(seg, recv, [], _) if seg.ident.name.as_str() == "clone" => {
                e = recv;
            }
            _ => return e,
        }
    }
}

// Converts a snake_case function-family name to the CamelCase trait-name stem, e.g.
// `exp_x_minus_1` to `ExpXMinus1`.
fn camel_case(name: &str) -> String {
    name.split('_')
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |c| {
                c.to_uppercase().collect::<String>() + chars.as_str()
            })
        })
        .collect()
}

// Whether an `*Assign` companion of the function family `base` exists: either an inherent
// `{base}_assign`/`{base}_assign_ref` on the type, or a `{CamelCase(base)}Assign` trait in
// `malachite_base`. Returns a name to suggest.
fn assign_variant(
    cx: &rustc_lint::LateContext<'_>,
    self_ty_did: rustc_hir::def_id::DefId,
    base: &str,
) -> Option<String> {
    let inherent = format!("{base}_assign");
    if has_inherent_fn(cx, self_ty_did, &inherent) {
        return Some(inherent);
    }
    let inherent_ref = format!("{base}_assign_ref");
    if has_inherent_fn(cx, self_ty_did, &inherent_ref) {
        return Some(inherent_ref);
    }
    let trait_name = format!("{}Assign", camel_case(base));
    for module in ["arithmetic", "logic"] {
        let path = format!("malachite_base::num::{module}::traits::{trait_name}");
        if !clippy_utils::paths::lookup_path_str(cx.tcx, clippy_utils::paths::PathNS::Type, &path)
            .is_empty()
        {
            return Some(format!("{base}_assign"));
        }
    }
    None
}

// The value of an integer literal, negated literal included.
fn literal_value(e: &rustc_hir::Expr<'_>) -> Option<i128> {
    use rustc_ast::LitKind;
    use rustc_hir::{ExprKind, UnOp};
    match &e.kind {
        ExprKind::Lit(lit) => match lit.node {
            LitKind::Int(v, _) => i128::try_from(v.get()).ok(),
            _ => None,
        },
        ExprKind::Unary(UnOp::Neg, inner) => literal_value(inner).map(i128::checked_neg)?,
        _ => None,
    }
}

// Whether `e` is the integer literal `value` or a path to a constant named `name` (e.g. the value
// 2, spelled `2` or `T::TWO`).
fn is_int_const(
    cx: &rustc_lint::LateContext<'_>,
    e: &rustc_hir::Expr<'_>,
    value: i128,
    name: &str,
) -> bool {
    use rustc_hir::ExprKind;
    use rustc_hir::def::DefKind;
    let e = peel_clone_and_borrows(e);
    if literal_value(e) == Some(value) {
        return true;
    }
    let ExprKind::Path(qpath) = &e.kind else {
        return false;
    };
    let Some(did) = cx.qpath_res(qpath, e.hir_id).opt_def_id() else {
        return false;
    };
    matches!(
        cx.tcx.def_kind(did),
        DefKind::Const { .. } | DefKind::AssocConst { .. }
    ) && cx.tcx.item_name(did).as_str() == name
}

// Whether the expression is inside a const context: the body of a `const` item, a `const` block, a
// `static` initializer, or a `const fn` (whose body is evaluated at compile time whenever its caller
// is).
fn in_const_context(cx: &rustc_lint::LateContext<'_>, e: &rustc_hir::Expr<'_>) -> bool {
    use rustc_hir::{BodyOwnerKind, Constness};
    let owner = cx.tcx.hir_enclosing_body_owner(e.hir_id);
    match cx.tcx.hir_body_owner_kind(owner) {
        BodyOwnerKind::Const { .. } | BodyOwnerKind::Static(_) => true,
        BodyOwnerKind::Fn => cx.tcx.constness(owner) == Constness::Const,
        _ => false,
    }
}

// Whether the span lies in test-oriented code: tests, demos and benches (`bin_util`), test
// utilities, or any code compiled only as part of a test harness (which covers `#[cfg(test)]`
// modules inside `src`). Such code exercises the discouraged spellings on purpose.
fn in_test_code(cx: &rustc_lint::LateContext<'_>, span: rustc_span::Span) -> bool {
    use rustc_lint::LintContext;
    if cx.sess().opts.test {
        return true;
    }
    if let rustc_span::FileName::Real(real) = cx.sess().source_map().span_to_filename(span)
        && let Some(path) = real.local_path()
    {
        let path = path.to_string_lossy().replace('\\', "/");
        path.contains("/tests/") || path.contains("/bin_util/") || path.contains("/test_util/")
    } else {
        false
    }
}

#[expect(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    dylint_linting::init_config(sess);
    lint_store.register_lints(&[
        assert_ordering_equal_prefer_exact::ASSERT_ORDERING_EQUAL_PREFER_EXACT,
        assign_then_consumed_once::ASSIGN_THEN_CONSUMED_ONCE,
        clone_with_ref_variant::CLONE_WITH_REF_VARIANT,
        compare_with_power_of_2::COMPARE_WITH_POWER_OF_2,
        compare_with_primitive::COMPARE_WITH_PRIMITIVE,
        let_tuple_underscore_to_field::LET_TUPLE_UNDERSCORE_TO_FIELD,
        long_lines::LONG_LINES,
        manual_float_from_primitive::MANUAL_FLOAT_FROM_PRIMITIVE,
        manual_from_sign_and_abs::MANUAL_FROM_SIGN_AND_ABS,
        manual_rational_significant_bits::MANUAL_RATIONAL_SIGNIFICANT_BITS,
        missing_inline_on_delegator::MISSING_INLINE_ON_DELEGATOR,
        mul_div_by_power_of_2::MUL_DIV_BY_POWER_OF_2,
        mul_div_by_power_of_2_literal::MUL_DIV_BY_POWER_OF_2_LITERAL,
        redundant_from_in_comparison::REDUNDANT_FROM_IN_COMPARISON,
        redundant_from_in_literal_comparison::REDUNDANT_FROM_IN_LITERAL_COMPARISON,
        redundant_nearest::REDUNDANT_NEAREST,
        redundant_prec_round_of_exact_constant::REDUNDANT_PREC_ROUND_OF_EXACT_CONSTANT,
        runtime_literal_conversion::RUNTIME_LITERAL_CONVERSION,
        shift_of_one::SHIFT_OF_ONE,
        use_assign_variant::USE_ASSIGN_VARIANT,
        use_checked_log_base_2::USE_CHECKED_LOG_BASE_2,
        use_divisible_by::USE_DIVISIBLE_BY,
        use_exact_from::USE_EXACT_FROM,
        use_named_constant::USE_NAMED_CONSTANT,
        use_parity::USE_PARITY,
        use_reciprocal::USE_RECIPROCAL,
        use_round_variant::USE_ROUND_VARIANT,
        use_saturating_from::USE_SATURATING_FROM,
        use_square::USE_SQUARE,
        use_trailing_zeros::USE_TRAILING_ZEROS,
        use_width_mask::USE_WIDTH_MASK,
    ]);
    lint_store.register_late_pass(|_| {
        Box::new(assert_ordering_equal_prefer_exact::AssertOrderingEqualPreferExact)
    });
    lint_store.register_late_pass(|_| Box::new(assign_then_consumed_once::AssignThenConsumedOnce));
    lint_store.register_late_pass(|_| Box::new(compare_with_power_of_2::CompareWithPowerOf2));
    lint_store.register_late_pass(|_| Box::new(compare_with_primitive::CompareWithPrimitive));
    lint_store
        .register_late_pass(|_| Box::new(let_tuple_underscore_to_field::LetTupleUnderscoreToField));
    lint_store.register_late_pass(|_| Box::new(long_lines::LongLines));
    lint_store
        .register_late_pass(|_| Box::new(manual_float_from_primitive::ManualFloatFromPrimitive));
    lint_store.register_late_pass(|_| Box::new(manual_from_sign_and_abs::ManualFromSignAndAbs));
    lint_store.register_late_pass(|_| {
        Box::new(manual_rational_significant_bits::ManualRationalSignificantBits)
    });
    lint_store
        .register_late_pass(|_| Box::new(missing_inline_on_delegator::MissingInlineOnDelegator));
    lint_store.register_late_pass(|_| Box::new(mul_div_by_power_of_2::MulDivByPowerOf2));
    lint_store
        .register_late_pass(|_| Box::new(mul_div_by_power_of_2_literal::MulDivByPowerOf2Literal));
    lint_store
        .register_late_pass(|_| Box::new(redundant_from_in_comparison::RedundantFromInComparison));
    lint_store.register_late_pass(|_| {
        Box::new(redundant_from_in_literal_comparison::RedundantFromInLiteralComparison)
    });
    lint_store.register_late_pass(|_| Box::new(redundant_nearest::RedundantNearest));
    lint_store.register_late_pass(|_| {
        Box::new(redundant_prec_round_of_exact_constant::RedundantPrecRoundOfExactConstant)
    });
    lint_store
        .register_late_pass(|_| Box::new(runtime_literal_conversion::RuntimeLiteralConversion));
    lint_store.register_late_pass(|_| Box::new(shift_of_one::ShiftOfOne));
    lint_store.register_late_pass(|_| Box::new(clone_with_ref_variant::CloneWithRefVariant));
    lint_store.register_late_pass(|_| Box::new(use_assign_variant::UseAssignVariant));
    lint_store.register_late_pass(|_| Box::new(use_checked_log_base_2::UseCheckedLogBase2));
    lint_store.register_late_pass(|_| Box::new(use_divisible_by::UseDivisibleBy));
    lint_store.register_late_pass(|_| Box::new(use_exact_from::UseExactFrom));
    lint_store.register_late_pass(|_| Box::new(use_named_constant::UseNamedConstant));
    lint_store.register_late_pass(|_| Box::new(use_parity::UseParity));
    lint_store.register_late_pass(|_| Box::new(use_reciprocal::UseReciprocal));
    lint_store.register_late_pass(|_| Box::new(use_round_variant::UseRoundVariant));
    lint_store.register_late_pass(|_| Box::new(use_saturating_from::UseSaturatingFrom));
    lint_store.register_late_pass(|_| Box::new(use_square::UseSquare));
    lint_store.register_late_pass(|_| Box::new(use_trailing_zeros::UseTrailingZeros));
    lint_store.register_late_pass(|_| Box::new(use_width_mask::UseWidthMask));
}

#[test]
fn ui() {
    dylint_testing::ui::Test::src_base(env!("CARGO_PKG_NAME"), "ui")
        .dylint_toml(
            r#"
            [malachite_lints]
            long_lines_exceptions = [
                { file = "main.rs", line = 3 },
                { file = "main.rs", line = 4 },
            ]
            "#,
        )
        .run();
}

#[test]
fn ui_examples() {
    dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
}
