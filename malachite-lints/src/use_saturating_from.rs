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
    /// Flags converting to an unsigned type with `exact_from` applied to a `max(0)` clamp, like
    /// `u64::exact_from(err.max(0))`.
    ///
    /// ### Why is this bad?
    ///
    /// The `max(0)` already decides that out-of-range-low values clamp to 0 rather than panic, so
    /// pairing it with `exact_from`, which *panics* on out-of-range-high values, is inconsistent.
    /// `saturating_from` clamps both ends: for an unsigned target its low bound is 0, so it matches
    /// the `max(0)` exactly, and it is equivalent whenever the source cannot exceed the target's
    /// maximum (e.g. `i64` into `u64`).
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let n = u64::exact_from(err.max(0));
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let n = u64::saturating_from(err);
    /// ```
    pub USE_SATURATING_FROM,
    Deny,
    "`exact_from` of a `max(0)` clamp into an unsigned type instead of `saturating_from`"
}

declare_lint_pass!(UseSaturatingFrom => [USE_SATURATING_FROM]);

// Whether `e` is the integer literal 0.
fn is_zero(e: &Expr<'_>) -> bool {
    crate::literal_value(e) == Some(0)
}

impl<'tcx> LateLintPass<'tcx> for UseSaturatingFrom {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        // `T::exact_from(arg)`, where `exact_from` is the `ExactFrom` trait method.
        let ExprKind::Call(callee, [arg]) = expr.kind else {
            return;
        };
        let ExprKind::Path(qpath) = &callee.kind else {
            return;
        };
        let Res::Def(DefKind::AssocFn, did) = cx.qpath_res(qpath, callee.hir_id) else {
            return;
        };
        if cx.tcx.item_name(did).as_str() != "exact_from" {
            return;
        }
        // The target must be an unsigned integer, so that `saturating_from`'s lower bound is 0 and
        // hence matches the `max(0)`. (A signed target would clamp low to its minimum instead.)
        let target_ty = cx.typeck_results().expr_ty(expr);
        if !target_ty.is_integral() || target_ty.is_signed() {
            return;
        }
        // The argument is `operand.max(0)` or `0.max(operand)`.
        let ExprKind::MethodCall(seg, recv, [other], _) = arg.kind else {
            return;
        };
        if seg.ident.name.as_str() != "max" {
            return;
        }
        let operand = if is_zero(other) {
            recv
        } else if is_zero(recv) {
            other
        } else {
            return;
        };
        let suggestion = format!(
            "{}({})",
            snippet(cx, callee.span, "..").replace("exact_from", "saturating_from"),
            snippet(cx, operand.span, ".."),
        );
        span_lint_and_help(
            cx,
            USE_SATURATING_FROM,
            expr.span,
            "`exact_from` of a `max(0)` clamp into an unsigned type",
            None,
            format!("use `{suggestion}`, which clamps both ends consistently"),
        );
    }
}
