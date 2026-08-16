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
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags `x & power_of_2(k) == 0` and `x & power_of_2(k) != 0` (in any operand order),
    /// suggesting `get_bit(k)`, and the same comparisons against a constant named
    /// `LIMB_HIGH_BIT` or ending in `HIGH_BIT`, suggesting `get_highest_bit()`.
    ///
    /// ### Why is this bad?
    ///
    /// The comparison spells out a single-bit test with a mask; `x.get_bit(k)` and
    /// `x.get_highest_bit()` say directly which bit is being read.
    ///
    /// ### Known problems
    ///
    /// Only the compared forms are flagged: a bare `x & power_of_2(k)` whose value is used
    /// as a number is not a bit test. Functions implementing `get_bit` or `get_highest_bit`
    /// themselves are exempt.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if x & Limb::power_of_2(sh) == 0 { .. }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if !x.get_bit(sh) { .. }
    /// ```
    pub USE_GET_BIT,
    Deny,
    "testing a single bit by masking with a power of 2 instead of using get_bit"
}

declare_lint_pass!(UseGetBit => [USE_GET_BIT]);

fn in_exempt_fn<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'tcx>) -> bool {
    let did = cx.tcx.hir_get_parent_item(expr.hir_id).def_id;
    let name = cx.tcx.item_name(did.to_def_id());
    let name = name.as_str();
    name.contains("get_bit") || name.contains("get_highest_bit")
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

// If `e` is a mask expression this lint recognizes, returns the suggested replacement method.
fn mask_suggestion(e: &Expr<'_>) -> Option<&'static str> {
    match &e.kind {
        ExprKind::Call(callee, [_]) => {
            if let ExprKind::Path(qpath) = &callee.kind
                && crate::qpath_last_segment_name(qpath) == Some("power_of_2")
            {
                Some("get_bit")
            } else {
                None
            }
        }
        ExprKind::Path(qpath) => {
            let name = crate::qpath_last_segment_name(qpath)?;
            if name.ends_with("HIGH_BIT") {
                Some("get_highest_bit")
            } else {
                None
            }
        }
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for UseGetBit {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion()
            || crate::in_test_code(cx, expr.span)
            || in_exempt_fn(cx, expr)
        {
            return;
        }
        let ExprKind::Binary(op, cmp_l, cmp_r) = expr.kind else {
            return;
        };
        let negated = match op.node {
            BinOpKind::Eq => true,
            BinOpKind::Ne => false,
            _ => return,
        };
        // one side is the AND, the other the zero literal
        let and = if is_zero_literal(cmp_r) {
            cmp_l
        } else if is_zero_literal(cmp_l) {
            cmp_r
        } else {
            return;
        };
        let ExprKind::Binary(and_op, l, r) = and.kind else {
            return;
        };
        if and_op.node != BinOpKind::BitAnd {
            return;
        }
        for (mask, _value) in [(r, l), (l, r)] {
            if let Some(method) = mask_suggestion(crate::peel_clone_and_borrows(mask)) {
                let bang = if negated { "!" } else { "" };
                let call = if method == "get_bit" {
                    format!("{bang}x.get_bit(k)")
                } else {
                    format!("{bang}x.get_highest_bit()")
                };
                clippy_utils::diagnostics::span_lint(
                    cx,
                    USE_GET_BIT,
                    expr.span,
                    format!("this compares a single masked bit with 0; use `{call}` instead"),
                );
                return;
            }
        }
    }
}
