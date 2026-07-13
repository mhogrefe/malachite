// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint;
use clippy_utils::eq_expr_value;
use rustc_hir::{Expr, ExprKind, LetStmt, PatKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags rebinding a bignum to the result of a method on itself when an in-place `*_assign*`
    /// companion exists — whether by reassignment, like `x = x.add_prec(y, p).0` or
    /// `x = (&x).abs()`, or by a shadowing `let`, like `let x = x.exp_prec(p).0` or
    /// `let (x, o) = x.div_prec(y, p)`.
    ///
    /// ### Why is this bad?
    ///
    /// The `*_assign*` variants work in place, avoiding a needless move (and, if the receiver was
    /// cloned, a needless copy of a potentially huge value). Operator forms (`x = &x * &y`) are
    /// covered by clippy's `assign_op_pattern`; this lint covers the house method families.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// t = t.sub_prec(Float::ONE, p).0;
    /// let t = t.exp_prec(p).0;
    /// let (t, o) = t.div_prec(k, p);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// t.sub_prec_assign(Float::ONE, p);
    /// t.exp_prec_assign(p);
    /// let o = t.div_prec_assign(k, p);
    /// ```
    pub USE_ASSIGN_VARIANT,
    Deny,
    "rebinding the result of a method for which an in-place `*_assign*` variant exists"
}

declare_lint_pass!(UseAssignVariant => [USE_ASSIGN_VARIANT]);

impl<'tcx> LateLintPass<'tcx> for UseAssignVariant {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, expr.span) {
            return;
        }
        let ExprKind::Assign(lhs, rhs, _) = expr.kind else {
            return;
        };
        // The reassigned result is either the method call itself or the `.0` of its returned
        // tuple (the house `(value, Ordering)` shape).
        let call = match rhs.kind {
            ExprKind::Field(base, ident) if ident.as_str() == "0" => base,
            _ => rhs,
        };
        let ExprKind::MethodCall(seg, recv, _, _) = call.kind else {
            return;
        };
        let name = seg.ident.name.as_str();
        if name.contains("_assign") || name == "clone" {
            return;
        }
        // The receiver, behind any `&` or `.clone()`, must be the assigned place itself.
        if !eq_expr_value(cx, lhs, crate::peel_clone_and_borrows(recv)) {
            return;
        }
        let Some(adt_did) = crate::bignum_adt_did(cx, cx.typeck_results().expr_ty(lhs)) else {
            return;
        };
        let base = crate::strip_variant_suffixes(name);
        let Some(suggestion) = crate::assign_variant(cx, adt_did, base) else {
            return;
        };
        // The assign variant's own defining delegation (`fn foo_assign(&mut self, ..) {
        // *self = (&*self).foo(..); }`) is exempt.
        let owner_did = cx.tcx.hir_get_parent_item(expr.hir_id).to_def_id();
        if matches!(
            cx.tcx.def_kind(owner_did),
            rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::AssocFn
        ) && cx.tcx.item_name(owner_did).as_str().contains("_assign")
        {
            return;
        }
        span_lint(
            cx,
            USE_ASSIGN_VARIANT,
            expr.span,
            format!("use `{suggestion}` (in place) instead of assigning the result of `{name}`"),
        );
    }

    fn check_local(&mut self, cx: &LateContext<'tcx>, local: &'tcx LetStmt<'tcx>) {
        if local.span.from_expansion() {
            return;
        }
        if crate::in_test_code(cx, local.span) {
            return;
        }
        let Some(init) = local.init else {
            return;
        };
        // The rebound name: either the whole pattern (`let x = ...`) or the first element of a
        // tuple pattern (`let (x, o) = ...`, the house `(value, Ordering)` shape).
        let (bound, tuple) = match local.pat.kind {
            PatKind::Binding(_, _, ident, None) => (ident, false),
            PatKind::Tuple([first, ..], _) => {
                let PatKind::Binding(_, _, ident, None) = first.kind else {
                    return;
                };
                (ident, true)
            }
            _ => return,
        };
        // The bound result is the method call itself, or (for a non-tuple binding) the `.0` of
        // its returned tuple.
        let call = match init.kind {
            ExprKind::Field(base, ident) if !tuple && ident.as_str() == "0" => base,
            _ => init,
        };
        let ExprKind::MethodCall(seg, recv, _, _) = call.kind else {
            return;
        };
        let name = seg.ident.name.as_str();
        if name.contains("_assign") || name == "clone" {
            return;
        }
        // The receiver, behind any `&` or `.clone()`, must be a bare path with the same name as
        // the new binding: a shadowing rebind.
        let recv_peeled = crate::peel_clone_and_borrows(recv);
        let ExprKind::Path(QPath::Resolved(None, path)) = recv_peeled.kind else {
            return;
        };
        let [segment] = path.segments else {
            return;
        };
        if segment.ident.name != bound.name {
            return;
        }
        // A reference-typed receiver (e.g. a `&Rational` parameter shadowed by an owned result)
        // has no in-place option; the rebind is a legitimate conversion to an owned value.
        let recv_ty = cx.typeck_results().expr_ty(recv_peeled);
        if recv_ty.is_ref() {
            return;
        }
        let Some(adt_did) = crate::bignum_adt_did(cx, recv_ty) else {
            return;
        };
        let base = crate::strip_variant_suffixes(name);
        let Some(suggestion) = crate::assign_variant(cx, adt_did, base) else {
            return;
        };
        // The assign variant's own defining delegation is exempt, as in `check_expr`.
        let owner_did = cx.tcx.hir_get_parent_item(local.hir_id).to_def_id();
        if matches!(
            cx.tcx.def_kind(owner_did),
            rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::AssocFn
        ) && cx.tcx.item_name(owner_did).as_str().contains("_assign")
        {
            return;
        }
        span_lint(
            cx,
            USE_ASSIGN_VARIANT,
            local.span,
            format!(
                "use `{suggestion}` (in place) instead of shadowing `{bound}` with the result of \
                 `{name}`"
            ),
        );
    }
}
