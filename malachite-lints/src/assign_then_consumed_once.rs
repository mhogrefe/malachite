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
use rustc_hir::{
    BindingMode, ByRef, Block, Expr, ExprKind, HirId, Mutability, Node, PatKind, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a freshly bound mutable bignum that is immediately mutated in place by a single
    /// `*_assign*` call and then moved exactly once, like
    ///
    /// ```rust,ignore
    /// let mut t = a.ln_prec(p).0;
    /// t.mul_prec_assign_ref(&y, p);
    /// f(t)
    /// ```
    ///
    /// ### Why is this bad?
    ///
    /// This is GMP/FLINT's assembly-like bind-mutate-move shape. In Malachite the value can be
    /// threaded straight through a by-value variant of the operation, keeping it an expression
    /// that chains: `f(a.ln_prec(p).0.mul_prec_val_ref(&y, p).0)`. The `mut` binding and the
    /// separate statement disappear.
    ///
    /// This is the near-inverse of `use_assign_variant`, which prefers the in-place form when a
    /// *persisted* variable is reassigned its own result; the two do not overlap, because here
    /// the binding is fresh and consumed once rather than reassigned.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let mut t = a.ln_prec(p).0;
    /// t.mul_prec_assign_ref(&y, p);
    /// f(t)
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// f(a.ln_prec(p).0.mul_prec_val_ref(&y, p).0)
    /// ```
    pub ASSIGN_THEN_CONSUMED_ONCE,
    Deny,
    "a fresh mutable bignum mutated once in place then moved once, instead of a by-value chain"
}

declare_lint_pass!(AssignThenConsumedOnce => [ASSIGN_THEN_CONSUMED_ONCE]);

impl<'tcx> LateLintPass<'tcx> for AssignThenConsumedOnce {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        for i in 0..block.stmts.len() {
            let s1 = &block.stmts[i];
            let Some(s2) = block.stmts.get(i + 1) else {
                continue;
            };
            if s1.span.from_expansion() || crate::in_test_code(cx, s1.span) {
                continue;
            }
            // s1: `let mut NAME = INIT;` with NAME a by-value mutable binding of a bignum type.
            let StmtKind::Let(local) = s1.kind else {
                continue;
            };
            if local.els.is_some() {
                continue;
            }
            let PatKind::Binding(BindingMode(ByRef::No, Mutability::Mut), name_hir, _, None) =
                local.pat.kind
            else {
                continue;
            };
            let Some(init) = local.init else {
                continue;
            };
            if crate::bignum_adt_did(cx, cx.typeck_results().expr_ty(init)).is_none() {
                continue;
            }
            // s2: `NAME.<something>_assign<..>(..);` — an in-place method whose receiver is exactly
            // NAME (no projection), evaluated for effect.
            let (StmtKind::Semi(e) | StmtKind::Expr(e)) = s2.kind else {
                continue;
            };
            let ExprKind::MethodCall(seg, recv, _, _) = e.kind else {
                continue;
            };
            if !seg.ident.as_str().contains("_assign")
                || path_to_local_with_projections(recv) != Some(name_hir)
            {
                continue;
            }
            // After the assign, NAME must be consumed exactly once, by a plain move (not `&mut`,
            // not another `_assign` receiver, not re-bound). More or fewer uses means the simple
            // chain rewrite does not apply.
            if !consumed_once_by_move(cx, block, i + 1, name_hir) {
                continue;
            }
            span_lint_and_help(
                cx,
                ASSIGN_THEN_CONSUMED_ONCE,
                s1.span.to(s2.span),
                format!(
                    "`{}` is bound, mutated once in place, then moved once",
                    seg.ident,
                ),
                None,
                "thread the value through a by-value variant of the operation in a chain instead \
                 of a separate `mut` binding and `*_assign*` step",
            );
        }
    }
}

// Whether `name_hir` is referenced exactly once in the block's statements after index `after`
// (plus the trailing block expression), and that single reference is a by-value move: not the
// operand of `&mut`, and not the receiver of another `*_assign*` method.
fn consumed_once_by_move<'tcx>(
    cx: &LateContext<'tcx>,
    block: &'tcx Block<'tcx>,
    after: usize,
    name_hir: HirId,
) -> bool {
    let mut count = 0u32;
    let mut ok = true;
    let mut inspect = |e: &'tcx Expr<'tcx>| {
        if path_to_local_with_projections(e) == Some(name_hir)
            && matches!(e.kind, ExprKind::Path(_))
        {
            count += 1;
            match cx.tcx.parent_hir_node(e.hir_id) {
                Node::Expr(parent) => match parent.kind {
                    ExprKind::AddrOf(_, Mutability::Mut, _) => ok = false,
                    ExprKind::MethodCall(pseg, precv, _, _)
                        if precv.hir_id == e.hir_id && pseg.ident.as_str().contains("_assign") =>
                    {
                        ok = false;
                    }
                    _ => {}
                },
                _ => ok = false,
            }
        }
        ControlFlow::<()>::Continue(())
    };
    for stmt in &block.stmts[after + 1..] {
        for_each_expr(cx, stmt, &mut inspect);
    }
    if let Some(e) = block.expr {
        for_each_expr(cx, e, &mut inspect);
    }
    ok && count == 1
}
