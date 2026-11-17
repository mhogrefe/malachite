// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags rounding one of the exactly-representable named [`Float`] constants (`ONE`, `TWO`,
    /// `NEGATIVE_ONE`, `ONE_HALF`) to a precision, like `Float::from_float_prec_round(Float::ONE,
    /// prec, rm)` or `Float::from_float_prec(Float::ONE, prec)`.
    ///
    /// ### Why is this bad?
    ///
    /// Each of these constants has a single significant bit, so it is exactly representable at
    /// every precision; rounding it is a no-op. The rounding mode is dead and the ordering is
    /// always `Equal`. Use the dedicated constructor, e.g. `(Float::one_prec(prec), Equal)`, which
    /// says the value is exact and skips the rounding machinery.
    ///
    /// The same goes for the `BigExponentFloat` constants: converting one to a `Float` with
    /// `Float::from_big_exponent_float_prec_round` is `(Float::one_prec(prec), Equal)`, and
    /// rounding one with `round_to_prec_round` is a no-op, since a `BigExponentFloat` stores no
    /// precision: `(BigExponentFloat::ONE, Equal)`.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// return Float::from_float_prec_round(Float::ONE, prec, rm);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// return (Float::one_prec(prec), Equal);
    /// ```
    pub REDUNDANT_PREC_ROUND_OF_EXACT_CONSTANT,
    Deny,
    "rounding an exactly-representable named `Float` constant to a precision instead of using its \
    dedicated `*_prec` constructor"
}

declare_lint_pass!(RedundantPrecRoundOfExactConstant => [REDUNDANT_PREC_ROUND_OF_EXACT_CONSTANT]);

// If `e` is a path to a named `Float` or `BigExponentFloat` constant that is exact at every
// precision, returns the type's name, the constant's name, and the name of `Float`'s dedicated
// `*_prec` constructor for it.
fn prec_constructor_for<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
) -> Option<(&'static str, &'static str, &'static str)> {
    // Peel `&` layers so the by-reference spelling `&Float::ONE` is recognized too.
    let e = e.peel_borrows();
    let ExprKind::Path(qpath) = &e.kind else {
        return None;
    };
    let Some(did) = cx.qpath_res(qpath, e.hir_id).opt_def_id() else {
        return None;
    };
    let Some(ty_name @ ("Float" | "BigExponentFloat")) =
        crate::bignum_name(cx, cx.typeck_results().expr_ty(e).peel_refs())
    else {
        return None;
    };
    // Only single-significant-bit constants are exact at every precision; `THREE` etc. are not.
    let (constant, ctor) = match cx.tcx.item_name(did).as_str() {
        "ONE" => ("ONE", "one_prec"),
        "TWO" => ("TWO", "two_prec"),
        "NEGATIVE_ONE" => ("NEGATIVE_ONE", "negative_one_prec"),
        "ONE_HALF" => ("ONE_HALF", "one_half_prec"),
        _ => return None,
    };
    Some((ty_name, constant, ctor))
}

impl<'tcx> LateLintPass<'tcx> for RedundantPrecRoundOfExactConstant {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        let (c, replacement) = match expr.kind {
            // `Float::from_float_prec_round(c, prec, rm)` or `Float::from_float_prec(c, prec)`,
            // and likewise `Float::from_big_exponent_float_prec_round` of a `BigExponentFloat`
            // constant. Either way the result is a `Float`.
            ExprKind::Call(callee, [c, prec, _] | [c, prec]) => {
                let ExprKind::Path(qpath) = &callee.kind else {
                    return;
                };
                let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
                    return;
                };
                let source = match cx.tcx.item_name(fn_did).as_str() {
                    "from_float_prec_round"
                    | "from_float_prec"
                    | "from_float_prec_round_ref"
                    | "from_float_prec_ref" => "Float",
                    "from_big_exponent_float_prec_round"
                    | "from_big_exponent_float_prec"
                    | "from_big_exponent_float_prec_round_ref"
                    | "from_big_exponent_float_prec_ref" => "BigExponentFloat",
                    _ => return,
                };
                let Some((ty_name, _, ctor)) = prec_constructor_for(cx, c) else {
                    return;
                };
                if ty_name != source {
                    return;
                }
                (
                    c,
                    format!("(Float::{ctor}({}), Equal)", snippet(cx, prec.span, "..")),
                )
            }
            // `BigExponentFloat::ONE.round_to_prec_round(prec, rm)` or
            // `BigExponentFloat::ONE.round_to_prec(prec)`. A `BigExponentFloat` stores no
            // precision, so the constant itself is the result.
            ExprKind::MethodCall(seg, c, [_, _] | [_], _) => {
                if !matches!(
                    seg.ident.name.as_str(),
                    "round_to_prec_round"
                        | "round_to_prec"
                        | "round_to_prec_round_ref"
                        | "round_to_prec_ref"
                ) {
                    return;
                }
                let Some(("BigExponentFloat", constant, _)) = prec_constructor_for(cx, c) else {
                    return;
                };
                (c, format!("(BigExponentFloat::{constant}, Equal)"))
            }
            _ => return,
        };
        if crate::in_cross_check_code(cx, expr.span) {
            return;
        }
        span_lint_and_help(
            cx,
            REDUNDANT_PREC_ROUND_OF_EXACT_CONSTANT,
            expr.span,
            format!(
                "`{}` rounds a value that is exact at every precision",
                snippet(cx, c.span, ".."),
            ),
            None,
            format!(
                "use `{replacement}`; the rounding mode is dead and the ordering is always `Equal`"
            ),
        );
    }
}
