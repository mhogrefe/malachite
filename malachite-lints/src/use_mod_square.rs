// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::SpanlessEq;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a modular multiplication -- `mod_mul`, `mod_mul_assign`, `mod_mul_precomputed`, or
    /// `mod_mul_precomputed_assign` -- whose two multiplicand operands are the same expression
    /// (possibly through `&` or `.clone()`), where a `mod_square` counterpart exists.
    ///
    /// ### Why is this bad?
    ///
    /// `mod_square(m)` and `mod_square_precomputed(m, &data)` say what is meant, need no cloned or
    /// re-borrowed second operand, and leave one call site to optimize if squaring ever gets a
    /// dedicated kernel.
    ///
    /// ### Known problems
    ///
    /// The precomputed forms are only flagged for `Natural` receivers: for primitive integers,
    /// `mod_square_precomputed` takes the modular-exponentiation data `(inverse, shift)`, so a
    /// call site holding only the bare multiplication inverse cannot be rewritten mechanically.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let y = (&x).mod_mul_precomputed(&x, &m, &data);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let y = (&x).mod_square_precomputed(&m, &data);
    /// ```
    pub USE_MOD_SQUARE,
    Deny,
    "modular multiplication with equal operands instead of modular squaring"
}

declare_lint_pass!(UseModSquare => [USE_MOD_SQUARE]);

// Whether `expr` sits inside the implementation of a squaring operation itself.
fn inside_own_definition<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> bool {
    let did = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
    cx.tcx
        .item_name(did.to_def_id())
        .as_str()
        .contains("mod_square")
}

impl<'tcx> LateLintPass<'tcx> for UseModSquare {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || crate::in_test_code(cx, expr.span)
            || inside_own_definition(cx, expr)
        {
            return;
        }
        let ExprKind::MethodCall(seg, receiver, args, _) = expr.kind else {
            return;
        };
        let (multiplicand, replacement) = match (seg.ident.name.as_str(), args) {
            ("mod_mul", [y, _]) => (y, "mod_square()"),
            ("mod_mul_assign", [y, _]) => (y, "mod_square_assign()"),
            ("mod_mul_precomputed", [y, _, _]) => (y, "mod_square_precomputed()"),
            ("mod_mul_precomputed_assign", [y, _, _]) => (y, "mod_square_precomputed_assign()"),
            _ => return,
        };
        // For primitive integers, `mod_square_precomputed` takes the modular-exponentiation data,
        // not the bare multiplication inverse that `mod_mul_precomputed` call sites hold, so only
        // `Natural` receivers can be rewritten mechanically.
        if args.len() == 3
            && crate::bignum_name(cx, cx.typeck_results().expr_ty(receiver).peel_refs())
                != Some("Natural")
        {
            return;
        }
        let x = crate::peel_clone_and_borrows(receiver);
        let y = crate::peel_clone_and_borrows(multiplicand);
        if SpanlessEq::new(cx).eq_expr(x, y) {
            clippy_utils::diagnostics::span_lint(
                cx,
                USE_MOD_SQUARE,
                expr.span,
                format!(
                    "both multiplicand operands of this modular multiplication are the same; use \
                    `{replacement}`"
                ),
            );
        }
    }
}
