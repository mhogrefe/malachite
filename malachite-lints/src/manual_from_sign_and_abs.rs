// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::path_to_local_with_projections;
use clippy_utils::visitors::for_each_expr;
use core::ops::ControlFlow;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{BindingMode, Block, ByRef, Expr, ExprKind, Mutability, PatKind, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags building a signed [`Integer`] as an absolute value and then conditionally negating it
    /// in place, like
    ///
    /// ```rust,ignore
    /// let mut a = Integer::from(nat);
    /// if negative {
    ///     a.neg_assign();
    /// }
    /// ```
    ///
    /// ### Why is this bad?
    ///
    /// This is exactly `Integer::from_sign_and_abs` (or `from_sign_and_abs_ref`), which builds the
    /// signed value in one step from a sign and a [`Natural`] magnitude. The `mut` binding and the
    /// conditional `neg_assign` collapse into `Integer::from_sign_and_abs(!negative, nat)`.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let mut a = Integer::from(q.numerator_ref());
    /// if *q < 0 {
    ///     a.neg_assign();
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let a = Integer::from_sign_and_abs_ref(*q >= 0, q.numerator_ref());
    /// ```
    pub MANUAL_FROM_SIGN_AND_ABS,
    Deny,
    "building an `Integer` as an absolute value then conditionally negating it, instead of using \
    `Integer::from_sign_and_abs`"
}

declare_lint_pass!(ManualFromSignAndAbs => [MANUAL_FROM_SIGN_AND_ABS]);

// Whether `init` is `Integer::from(<natural>)`.
fn is_integer_from_natural<'tcx>(cx: &LateContext<'tcx>, init: &'tcx Expr<'tcx>) -> bool {
    let ExprKind::Call(callee, [abs]) = init.kind else {
        return false;
    };
    let ExprKind::Path(qpath) = &callee.kind else {
        return false;
    };
    let Res::Def(DefKind::AssocFn, fn_did) = cx.qpath_res(qpath, callee.hir_id) else {
        return false;
    };
    cx.tcx.item_name(fn_did).as_str() == "from"
        && crate::bignum_name(cx, cx.typeck_results().expr_ty(init)) == Some("Integer")
        && crate::bignum_name(cx, cx.typeck_results().expr_ty(abs).peel_refs()) == Some("Natural")
}

impl<'tcx> LateLintPass<'tcx> for ManualFromSignAndAbs {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        for i in 0..block.stmts.len() {
            let s1 = &block.stmts[i];
            let Some(s2) = block.stmts.get(i + 1) else {
                continue;
            };
            if s1.span.from_expansion() {
                continue;
            }
            // s1: `let mut a = Integer::from(<natural>);`
            let StmtKind::Let(local) = s1.kind else {
                continue;
            };
            let PatKind::Binding(BindingMode(ByRef::No, Mutability::Mut), name_hir, _, None) =
                local.pat.kind
            else {
                continue;
            };
            let Some(init) = local.init else {
                continue;
            };
            if local.els.is_some() || !is_integer_from_natural(cx, init) {
                continue;
            }
            // s2: `if <cond> { a.neg_assign(); }` with no `else`.
            let (StmtKind::Semi(e) | StmtKind::Expr(e)) = s2.kind else {
                continue;
            };
            let ExprKind::If(cond, then, None) = e.kind else {
                continue;
            };
            // The rewrite moves the sign decision before the construction, so the condition must
            // not read the freshly built value; `if a.get_bit(0) { a.neg_assign(); }` has no
            // `from_sign_and_abs` form.
            if for_each_expr(cx, cond, |e: &Expr<'tcx>| {
                if path_to_local_with_projections(e) == Some(name_hir) {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            })
            .is_some()
            {
                continue;
            }
            let ExprKind::Block(then_block, _) = then.kind else {
                continue;
            };
            // The `then` block is exactly one effectful `a.neg_assign()`.
            let body = match (then_block.stmts, then_block.expr) {
                ([stmt], None) => {
                    let (StmtKind::Semi(e) | StmtKind::Expr(e)) = stmt.kind else {
                        continue;
                    };
                    e
                }
                ([], Some(e)) => e,
                _ => continue,
            };
            let ExprKind::MethodCall(seg, recv, [], _) = body.kind else {
                continue;
            };
            if seg.ident.name.as_str() != "neg_assign"
                || path_to_local_with_projections(recv) != Some(name_hir)
            {
                continue;
            }
            if crate::in_test_code(cx, s1.span) {
                continue;
            }
            span_lint_and_help(
                cx,
                MANUAL_FROM_SIGN_AND_ABS,
                s1.span.to(s2.span),
                "this builds an `Integer` from a magnitude then conditionally negates it",
                None,
                "use `Integer::from_sign_and_abs` (or `from_sign_and_abs_ref`) with the sign \
                 instead",
            );
        }
    }
}
