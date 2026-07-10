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
    /// `NEGATIVE_ONE`, `ONE_HALF`) to a precision, like
    /// `Float::from_float_prec_round(Float::ONE, prec, rm)` or
    /// `Float::from_float_prec(Float::ONE, prec)`.
    ///
    /// ### Why is this bad?
    ///
    /// Each of these constants has a single significant bit, so it is exactly representable at
    /// every precision; rounding it is a no-op. The rounding mode is dead and the ordering is
    /// always `Equal`. Use the dedicated constructor, e.g. `(Float::one_prec(prec), Equal)`, which
    /// says the value is exact and skips the rounding machinery.
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

// If `e` is a path to a named `Float` constant that is exact at every precision, returns the name
// of its dedicated `*_prec` constructor.
fn prec_constructor_for<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<&'static str> {
    // Peel `&` layers so the by-reference spelling `&Float::ONE` is recognized too.
    let e = e.peel_borrows();
    let ExprKind::Path(qpath) = &e.kind else {
        return None;
    };
    let Some(did) = cx.qpath_res(qpath, e.hir_id).opt_def_id() else {
        return None;
    };
    if crate::bignum_name(cx, cx.typeck_results().expr_ty(e).peel_refs()) != Some("Float") {
        return None;
    }
    // Only single-significant-bit constants are exact at every precision; `THREE` etc. are not.
    match cx.tcx.item_name(did).as_str() {
        "ONE" => Some("one_prec"),
        "TWO" => Some("two_prec"),
        "NEGATIVE_ONE" => Some("negative_one_prec"),
        "ONE_HALF" => Some("one_half_prec"),
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for RedundantPrecRoundOfExactConstant {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        let ExprKind::Call(callee, args) = expr.kind else {
            return;
        };
        // `from_float_prec_round(c, prec, rm)` or `from_float_prec(c, prec)`.
        let ([c, prec, _] | [c, prec]) = args else {
            return;
        };
        let ExprKind::Path(qpath) = &callee.kind else {
            return;
        };
        let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
            return;
        };
        let fn_name = cx.tcx.item_name(fn_did);
        if !matches!(
            fn_name.as_str(),
            "from_float_prec_round"
                | "from_float_prec"
                | "from_float_prec_round_ref"
                | "from_float_prec_ref"
        ) {
            return;
        }
        let Some(ctor) = prec_constructor_for(cx, c) else {
            return;
        };
        if crate::in_test_code(cx, expr.span) {
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
                "use `(Float::{ctor}({}), Equal)`; the rounding mode is dead and the ordering is \
                 always `Equal`",
                snippet(cx, prec.span, ".."),
            ),
        );
    }
}
