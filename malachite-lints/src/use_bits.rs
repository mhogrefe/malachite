// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::eq_expr_value;
use clippy_utils::higher::While;
use clippy_utils::visitors::for_each_expr;
use core::ops::ControlFlow;
use rustc_hir::{AssignOpKind, BinOpKind, Expr, ExprKind, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a loop that walks the bits of an unsigned integer from the lowest up by testing the
    /// low bit and shifting right, such as `while x != 0 { if x & 1 == 1 { .. } x >>= 1; }`.
    ///
    /// ### Why is this bad?
    ///
    /// This re-implements `bits()`, which iterates over the bits of the integer from the least
    /// significant to the most significant set bit, and says what the loop is doing.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// while exp != 0 {
    ///     if exp & 1 == 1 {
    ///         result *= &base;
    ///     }
    ///     base.square_assign();
    ///     exp >>= 1;
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// for bit in exp.bits() {
    ///     if bit {
    ///         result *= &base;
    ///     }
    ///     base.square_assign();
    /// }
    /// ```
    pub USE_BITS,
    Deny,
    "a loop that tests the low bit and shifts right instead of iterating over `bits()`"
}

declare_lint_pass!(UseBits => [USE_BITS]);

// Whether `ty` (with references peeled) has `bits()` with the semantics of the loop: an unsigned
// primitive or a `Natural`. A signed primitive is excluded, since an arithmetic right shift never
// takes a negative value to zero.
fn has_bits<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> bool {
    ty.is_integral() && !ty.is_signed() || crate::bignum_name(cx, ty) == Some("Natural")
}

// If `cond` is `x != 0` or `x > 0` (either operand order), returns `x`.
fn nonzero_test_place<'tcx>(cond: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Binary(op, l, r) = cond.kind else {
        return None;
    };
    match op.node {
        BinOpKind::Ne => {
            if crate::literal_value(r) == Some(0) {
                Some(l)
            } else if crate::literal_value(l) == Some(0) {
                Some(r)
            } else {
                None
            }
        }
        BinOpKind::Gt if crate::literal_value(r) == Some(0) => Some(l),
        BinOpKind::Lt if crate::literal_value(l) == Some(0) => Some(r),
        _ => None,
    }
}

// Whether `e` shifts `x` right by one bit: `x >>= 1`, `x /= 2`, or `x = x >> 1`.
fn is_shift_by_one<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>, x: &'tcx Expr<'tcx>) -> bool {
    match e.kind {
        ExprKind::AssignOp(op, lhs, rhs) => {
            eq_expr_value(cx, lhs, x)
                && match op.node {
                    AssignOpKind::ShrAssign => crate::is_int_const(cx, rhs, 1, "ONE"),
                    AssignOpKind::DivAssign => crate::is_int_const(cx, rhs, 2, "TWO"),
                    _ => false,
                }
        }
        ExprKind::Assign(lhs, rhs, _) => {
            eq_expr_value(cx, lhs, x)
                && matches!(rhs.kind, ExprKind::Binary(op, l, r)
                    if op.node == BinOpKind::Shr
                        && eq_expr_value(cx, l, x)
                        && crate::is_int_const(cx, r, 1, "ONE"))
        }
        _ => false,
    }
}

// Whether `e` reads the low bit of `x`: `x & 1`, `x % 2`, `x.odd()`, `x.even()`, or `x.get_bit(0)`.
fn is_low_bit_read<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>, x: &'tcx Expr<'tcx>) -> bool {
    match e.kind {
        ExprKind::Binary(op, l, r) => {
            eq_expr_value(cx, crate::peel_clone_and_borrows(l), x)
                && match op.node {
                    BinOpKind::BitAnd => crate::is_int_const(cx, r, 1, "ONE"),
                    BinOpKind::Rem => crate::is_int_const(cx, r, 2, "TWO"),
                    _ => false,
                }
        }
        ExprKind::MethodCall(seg, recv, args, _) => {
            eq_expr_value(cx, crate::peel_clone_and_borrows(recv), x)
                && match (seg.ident.name.as_str(), args) {
                    ("odd" | "even", []) => true,
                    ("get_bit", [i]) => crate::literal_value(i) == Some(0),
                    _ => false,
                }
        }
        _ => false,
    }
}

impl<'tcx> LateLintPass<'tcx> for UseBits {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let Some(while_loop) = While::hir(expr) else {
            return;
        };
        if while_loop.condition.span.from_expansion()
            || crate::in_cross_check_code(cx, while_loop.condition.span)
        {
            return;
        }
        let Some(x) = nonzero_test_place(while_loop.condition) else {
            return;
        };
        if !has_bits(cx, cx.typeck_results().expr_ty(x).peel_refs()) {
            return;
        }
        let ExprKind::Block(block, _) = while_loop.body.kind else {
            return;
        };
        // The loop must end by shifting `x` right by one bit, with no tail expression after it.
        if block.expr.is_some() {
            return;
        }
        let Some((last, rest)) = block.stmts.split_last() else {
            return;
        };
        let (StmtKind::Semi(shift) | StmtKind::Expr(shift)) = last.kind else {
            return;
        };
        if !is_shift_by_one(cx, shift, x) {
            return;
        }
        // Every other use of `x` in the body must be a read of its low bit, and there must be at
        // least one; otherwise the loop uses `x` for more than walking its bits.
        let mut low_bit_reads = 0;
        let mut other_use = false;
        for stmt in rest {
            let _: Option<()> = for_each_expr(cx, stmt, |e| {
                if is_low_bit_read(cx, e, x) {
                    low_bit_reads += 1;
                    return ControlFlow::Continue(clippy_utils::visitors::Descend::No);
                }
                if eq_expr_value(cx, e, x) {
                    other_use = true;
                    return ControlFlow::Break(());
                }
                ControlFlow::Continue(clippy_utils::visitors::Descend::Yes)
            });
            if other_use {
                return;
            }
        }
        if low_bit_reads == 0 {
            return;
        }
        span_lint_and_help(
            cx,
            USE_BITS,
            while_loop.span,
            "this loop walks the bits of an integer by testing the low bit and shifting right",
            None,
            "iterate over the bits instead, with `for bit in x.bits()` (`BitIterable`), using `bit` \
             where the loop tests the low bit",
        );
    }
}
