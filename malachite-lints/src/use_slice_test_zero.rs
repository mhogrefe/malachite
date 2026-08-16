// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use rustc_ast::ast::LitKind;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyKind;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags `xs.iter().any(|&x| x != 0)` and `xs.iter().all(|&x| x == 0)` on slices of
    /// integers, suggesting `!slice_test_zero(xs)` and `slice_test_zero(xs)`.
    ///
    /// ### Why is this bad?
    ///
    /// `slice_test_zero` is the house helper for exactly this test, names the intent, and is
    /// implemented with a fast limb-wise scan.
    ///
    /// ### Known problems
    ///
    /// Only direct `iter()` receivers are recognized. The implementation of
    /// `slice_test_zero` itself is exempt.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if xs[..hi].iter().any(|&x| x != 0) { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if !slice_test_zero(&xs[..hi]) { .. }
    /// ```
    pub USE_SLICE_TEST_ZERO,
    Deny,
    "scanning a slice for a nonzero element instead of using slice_test_zero"
}

declare_lint_pass!(UseSliceTestZero => [USE_SLICE_TEST_ZERO]);

fn in_exempt_fn<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> bool {
    let did = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
    let name = cx.tcx.item_name(did.to_def_id());
    name.as_str().contains("slice_test_zero")
}

fn is_zero_literal(e: &Expr<'_>) -> bool {
    if let ExprKind::Lit(lit) = e.kind
        && let LitKind::Int(v, _) = lit.node
    {
        v.get() == 0
    } else {
        false
    }
}

// Whether the closure body is a comparison of its parameter (possibly dereferenced) with the
// literal 0, using the given operator.
fn closure_compares_param_with_zero<'tcx>(
    cx: &LateContext<'tcx>,
    closure: &Expr<'tcx>,
    op: BinOpKind,
) -> bool {
    let ExprKind::Closure(c) = closure.kind else {
        return false;
    };
    let body = cx.tcx.hir_body(c.body);
    let ExprKind::Binary(bin_op, l, r) = body.value.kind else {
        return false;
    };
    if bin_op.node != op {
        return false;
    }
    let (param, zero) = if is_zero_literal(r) {
        (l, r)
    } else if is_zero_literal(l) {
        (r, l)
    } else {
        return false;
    };
    let _ = zero;
    // the parameter side: a path (from a `|&x|` pattern) or a dereference of one
    let param = match param.kind {
        ExprKind::Unary(rustc_hir::UnOp::Deref, inner) => inner,
        _ => param,
    };
    matches!(param.kind, ExprKind::Path(_))
}

impl<'tcx> LateLintPass<'tcx> for UseSliceTestZero {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || crate::in_test_code(cx, expr.span)
            || in_exempt_fn(cx, expr)
        {
            return;
        }
        let ExprKind::MethodCall(seg, recv, [closure], _) = expr.kind else {
            return;
        };
        let (cmp, suggestion) = match seg.ident.as_str() {
            "any" => (BinOpKind::Ne, "!slice_test_zero(..)"),
            "all" => (BinOpKind::Eq, "slice_test_zero(..)"),
            _ => return,
        };
        let ExprKind::MethodCall(iter_seg, slice, [], _) = recv.kind else {
            return;
        };
        if iter_seg.ident.as_str() != "iter" {
            return;
        }
        // the receiver must be a slice (or reference to one) of integers
        let ty = cx.typeck_results().expr_ty(slice).peel_refs();
        let TyKind::Slice(elem) = ty.kind() else {
            return;
        };
        if !matches!(elem.kind(), TyKind::Uint(_) | TyKind::Int(_)) {
            return;
        }
        if closure_compares_param_with_zero(cx, closure, cmp) {
            clippy_utils::diagnostics::span_lint(
                cx,
                USE_SLICE_TEST_ZERO,
                expr.span,
                format!("this scans a slice for nonzero elements; use `{suggestion}` instead"),
            );
        }
    }
}
