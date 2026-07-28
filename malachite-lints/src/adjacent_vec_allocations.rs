// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::eq_expr_value;
use clippy_utils::higher::VecArgs;
use rustc_hir::{Block, Expr, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::Span;

declare_lint! {
    /// ### What it does
    ///
    /// Flags two or more consecutive `let` statements that each allocate a `Vec` of the same
    /// element type with the `vec![x; n]` repeat form.
    ///
    /// ### Why is this bad?
    ///
    /// Each `vec![x; n]` is a separate allocation, and back-to-back buffer allocations of the
    /// same type can be a single allocation split into pieces with `split_at_mut`. The right
    /// shape for the fix varies. If all the buffers are scratch space, one `Vec` and two
    /// `split_at_mut` calls suffice. If one buffer must end up owned — passed to
    /// `Natural::from_owned_limbs_asc`, say — make it the parent's prefix, `truncate` the
    /// parent once the other pieces are no longer in use, and `shrink_to_fit` so the escaping
    /// value does not retain the scratch capacity. And occasionally separate allocations
    /// are genuinely right: if two of the buffers each end up owned, or the algorithm swaps the
    /// buffers as it runs, merging would force a copy that costs more than the saved allocation;
    /// such sites should carry an `allow` with a comment saying why.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let mut fs = vec![0; alloc << 1];
    /// let mut xs = vec![0; alloc];
    /// let mut ys = vec![0; alloc];
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let mut out = vec![0; alloc << 2];
    /// let (fs, rest) = out.split_at_mut(alloc << 1);
    /// let (xs, ys) = rest.split_at_mut(alloc);
    /// ```
    pub ADJACENT_VEC_ALLOCATIONS,
    Deny,
    "allocating several `Vec`s of the same type back to back instead of splitting one allocation"
}

declare_lint_pass!(AdjacentVecAllocations => [ADJACENT_VEC_ALLOCATIONS]);

// If the statement is `let .. = vec![x; n];`, returns the initializer's type's representative
// expression data: the whole init expression (for type lookup) and the repeated element.
fn vec_repeat_init<'tcx>(
    cx: &LateContext<'tcx>,
    stmt_kind: &StmtKind<'tcx>,
) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
    let StmtKind::Let(local) = stmt_kind else {
        return None;
    };
    let init = local.init?;
    match VecArgs::hir(cx, init)? {
        VecArgs::Repeat(elem, _) => Some((init, elem)),
        VecArgs::Vec(_) => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for AdjacentVecAllocations {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        // Each run entry is (span of the `let`, init expr, repeated element).
        let mut run: Vec<(Span, &Expr<'_>, &Expr<'_>)> = Vec::new();
        let mut flush = |run: &mut Vec<(Span, &Expr<'_>, &Expr<'_>)>| {
            // in_test_code is comparatively expensive, so it runs after the structural checks.
            if run.len() >= 2 && !crate::in_test_code(cx, run[0].0) {
                span_lint_and_help(
                    cx,
                    ADJACENT_VEC_ALLOCATIONS,
                    run[0].0.to(run[run.len() - 1].0),
                    format!(
                        "{} `Vec`s of the same type are allocated back to back",
                        run.len()
                    ),
                    None,
                    "allocate one `Vec` of the combined length and divide it with `split_at_mut`; \
                    if one buffer must end up owned, make it the parent's prefix, `truncate` the \
                    parent at the end, and `shrink_to_fit` if the value outlives the function",
                );
            }
            run.clear();
        };
        for stmt in block.stmts {
            let candidate = if stmt.span.from_expansion() {
                None
            } else {
                vec_repeat_init(cx, &stmt.kind)
            };
            match candidate {
                Some((init, elem)) => {
                    // Extend the run only if the `Vec` type and the fill element both match.
                    if let Some(&(_, prev_init, prev_elem)) = run.last()
                        && (cx.typeck_results().expr_ty(init)
                            != cx.typeck_results().expr_ty(prev_init)
                            || !eq_expr_value(cx, elem, prev_elem))
                    {
                        flush(&mut run);
                    }
                    run.push((stmt.span, init, elem));
                }
                None => flush(&mut run),
            }
        }
        flush(&mut run);
    }
}
