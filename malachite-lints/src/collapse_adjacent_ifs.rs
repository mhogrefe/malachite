// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::SpanlessEq;
use rustc_hir::{Block, Expr, ExprKind, Stmt, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags two or more consecutive `if` statements without `else` branches whose bodies are
    /// identical and diverge (`return`, `break`, or `continue`), which can be a single `if` with
    /// the conditions joined by `||`.
    ///
    /// ### Why is this bad?
    ///
    /// The chain is longer than the single `if` and hides the fact that all the conditions have
    /// the same consequence. Because the shared body diverges and `||` short-circuits, the merged
    /// form evaluates exactly the same conditions in the same order.
    ///
    /// ### Known problems
    ///
    /// Only diverging bodies are flagged: for a non-diverging body, the chain runs the body once
    /// per satisfied condition, so merging would change behavior. Chains whose ifs are separated
    /// by other statements are not seen.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// if m.even() {
    ///     return None;
    /// }
    /// if m.checked_sqrt().is_some() {
    ///     return None;
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// if m.even() || m.checked_sqrt().is_some() {
    ///     return None;
    /// }
    /// ```
    pub COLLAPSE_ADJACENT_IFS,
    Deny,
    "consecutive ifs with identical diverging bodies instead of a single if with ||"
}

declare_lint_pass!(CollapseAdjacentIfs => [COLLAPSE_ADJACENT_IFS]);

// Whether a condition contains a `let`, as in `if let` or a let chain. Such a condition cannot
// be joined to another with `||`.
fn contains_let(e: &Expr<'_>) -> bool {
    match e.kind {
        ExprKind::Let(_) => true,
        ExprKind::Binary(_, l, r) => contains_let(l) || contains_let(r),
        ExprKind::Unary(_, e) | ExprKind::DropTemps(e) => contains_let(e),
        _ => false,
    }
}

// If `stmt` is an `if` without an `else` whose condition has no `let`, returns the condition and
// the body.
fn as_plain_if<'tcx>(stmt: &Stmt<'tcx>) -> Option<(&'tcx Expr<'tcx>, &'tcx Block<'tcx>)> {
    let (StmtKind::Expr(e) | StmtKind::Semi(e)) = stmt.kind else {
        return None;
    };
    let ExprKind::If(cond, then, None) = e.kind else {
        return None;
    };
    if contains_let(cond) {
        return None;
    }
    let ExprKind::Block(block, None) = then.kind else {
        return None;
    };
    Some((cond, block))
}

// Whether the block's final statement or expression diverges via `return`, `break`, or
// `continue`.
fn diverges(block: &Block<'_>) -> bool {
    let last = match (block.expr, block.stmts.last()) {
        (Some(e), _) => e,
        (None, Some(stmt)) => {
            let (StmtKind::Expr(e) | StmtKind::Semi(e)) = stmt.kind else {
                return false;
            };
            e
        }
        (None, None) => return false,
    };
    matches!(
        last.kind,
        ExprKind::Ret(_) | ExprKind::Break(..) | ExprKind::Continue(..)
    )
}

impl<'tcx> LateLintPass<'tcx> for CollapseAdjacentIfs {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        if block.span.from_expansion() || crate::in_test_code(cx, block.span) {
            return;
        }
        let mut i = 0;
        while i < block.stmts.len() {
            let Some((_, first_body)) = as_plain_if(&block.stmts[i]) else {
                i += 1;
                continue;
            };
            if block.stmts[i].span.from_expansion() || !diverges(first_body) {
                i += 1;
                continue;
            }
            let mut run_len = 1;
            while let Some(stmt) = block.stmts.get(i + run_len) {
                let Some((_, body)) = as_plain_if(stmt) else {
                    break;
                };
                if stmt.span.from_expansion() || !SpanlessEq::new(cx).eq_block(first_body, body) {
                    break;
                }
                run_len += 1;
            }
            if run_len > 1 {
                let span = block.stmts[i].span.to(block.stmts[i + run_len - 1].span);
                clippy_utils::diagnostics::span_lint(
                    cx,
                    COLLAPSE_ADJACENT_IFS,
                    span,
                    "these ifs have identical diverging bodies; join the conditions with `||` in \
                    a single `if`",
                );
            }
            i += run_len;
        }
    }
}
