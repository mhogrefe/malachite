// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::LitKind;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags `x.cmp(&0)` on integers, and `x.cmp(&T::ZERO)` on bignum types, where `x.sign()`
    /// says the same thing.
    ///
    /// ### Why is this bad?
    ///
    /// The `Sign` trait exists precisely for comparing a value against zero; `x.sign()` names the
    /// intent directly, while `x.cmp(&0)` makes the reader check which operand is the zero.
    ///
    /// `partial_cmp(&0u32)` is deliberately not flagged: it returns an `Option` whose `None` arm
    /// (NaN) callers match on, which `sign` does not model the same way.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// x.cmp(&0)
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// x.sign()
    /// ```
    pub USE_SIGN,
    Deny,
    "comparing with zero via cmp instead of using the Sign trait"
}

declare_lint_pass!(UseSign => [USE_SIGN]);

// The `Sign` implementations themselves are the one place `cmp(&0)` belongs.
fn in_exempt_fn<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> bool {
    let did = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
    cx.tcx.item_name(did.to_def_id()).as_str() == "sign"
}

// Whether `e` (after peeling `&`) is the integer literal 0.
fn is_zero_literal(e: &Expr<'_>) -> bool {
    let mut e = e;
    while let ExprKind::AddrOf(_, _, inner) = e.kind {
        e = inner;
    }
    if let ExprKind::Lit(lit) = e.kind
        && let LitKind::Int(v, _) = lit.node
    {
        v.get() == 0
    } else {
        false
    }
}

impl<'tcx> LateLintPass<'tcx> for UseSign {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        let ExprKind::MethodCall(seg, receiver, [arg], _) = expr.kind else {
            return;
        };
        if seg.ident.name.as_str() != "cmp" {
            return;
        }
        let receiver_ty = cx.typeck_results().expr_ty(receiver).peel_refs();
        let zero = if receiver_ty.is_integral() {
            is_zero_literal(arg)
        } else if crate::bignum_name(cx, receiver_ty).is_some() {
            crate::is_zero_assoc_const(cx, arg)
        } else {
            false
        };
        if !zero || in_exempt_fn(cx, expr) {
            return;
        }
        span_lint_and_help(
            cx,
            USE_SIGN,
            expr.span,
            "this compares with zero via `cmp`; use `sign` instead",
            None,
            "`x.cmp(&0)` is `x.sign()`",
        );
    }
}
