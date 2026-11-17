// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use rustc_hir::def::DefKind;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags dividing `ONE` by a `Rational` or a `Float`, like `Float::ONE / x`, and dividing
    /// `ONE` by a `Float` or a `BigExponentFloat` with one of the rounding division functions,
    /// like `BigExponentFloat::ONE.div_prec(x, prec)`.
    ///
    /// ### Why is this bad?
    ///
    /// Taking the reciprocal has a dedicated implementation: use `reciprocal()`,
    /// `reciprocal_assign()`, or (for `Float` and `BigExponentFloat`) the `reciprocal_prec*`
    /// family when a specific output precision is wanted.
    ///
    /// The rounding division functions are not flagged inside a function with `reciprocal` in its
    /// name, which may be defining or checking the reciprocal by dividing.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = Float::ONE / x;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = x.reciprocal();
    /// ```
    pub USE_RECIPROCAL,
    Deny,
    "dividing `ONE` by a value instead of taking its reciprocal"
}

declare_lint_pass!(UseReciprocal => [USE_RECIPROCAL]);

// If `e` (after peeling `&` and `.clone()`) is a path to a constant named `ONE` whose type is one of
// the bignum types, returns the type's name.
fn one_constant<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> Option<&'static str> {
    let e = crate::peel_clone_and_borrows(e);
    let ExprKind::Path(qpath) = &e.kind else {
        return None;
    };
    let did = cx.qpath_res(qpath, e.hir_id).opt_def_id()?;
    if !matches!(
        cx.tcx.def_kind(did),
        DefKind::Const { .. } | DefKind::AssocConst { .. }
    ) || cx.tcx.item_name(did).as_str() != "ONE"
    {
        return None;
    }
    crate::bignum_name(cx, cx.typeck_results().expr_ty(e).peel_refs())
}

impl<'tcx> LateLintPass<'tcx> for UseReciprocal {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        match expr.kind {
            ExprKind::Binary(op, num, _) if op.node == BinOpKind::Div => {
                // Only `Rational`, `Float`, and `GaussianRational` have both `/` and `Reciprocal`.
                let Some(name @ ("Rational" | "Float" | "GaussianRational")) =
                    one_constant(cx, num)
                else {
                    return;
                };
                if crate::in_cross_check_code(cx, expr.span) {
                    return;
                }
                let extra = if name == "Float" {
                    " (or `reciprocal_prec*` for a specific precision)"
                } else {
                    ""
                };
                span_lint(
                    cx,
                    USE_RECIPROCAL,
                    expr.span,
                    format!(
                        "use `reciprocal()`{extra} instead of dividing `{name}::ONE` by a value"
                    ),
                );
            }
            // `Float::ONE.div_prec(x, prec)` and the rest of the rounding division families, on
            // `Float` and `BigExponentFloat`.
            ExprKind::MethodCall(seg, num, [_, _, ..], _) => {
                let name = seg.ident.name.as_str();
                let base = crate::strip_variant_suffixes(name);
                let Some(stem @ ("prec_round" | "prec" | "round")) = base.strip_prefix("div_")
                else {
                    return;
                };
                let Some(ty_name @ ("Float" | "BigExponentFloat")) = one_constant(cx, num) else {
                    return;
                };
                let Some(adt_did) =
                    crate::bignum_adt_did(cx, cx.typeck_results().expr_ty(num).peel_refs())
                else {
                    return;
                };
                let reciprocal = format!("reciprocal_{stem}");
                if !crate::has_inherent_fn(cx, adt_did, &reciprocal)
                    || crate::in_fn_named_for(cx, expr, "reciprocal")
                    || crate::in_cross_check_code(cx, expr.span)
                {
                    return;
                }
                span_lint(
                    cx,
                    USE_RECIPROCAL,
                    expr.span,
                    format!(
                        "use `{reciprocal}` (or `{reciprocal}_ref`) instead of dividing \
                        `{ty_name}::ONE` by a value"
                    ),
                );
            }
            _ => {}
        }
    }
}
