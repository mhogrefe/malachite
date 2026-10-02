// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::eq_expr_value;
use clippy_utils::source::snippet_opt;
use clippy_utils::visitors::for_each_expr;
use core::ops::ControlFlow;
use rustc_errors::Applicability;
use rustc_hir::def::Res;
use rustc_hir::{
    BindingMode, Block, ByRef, Expr, ExprKind, HirId, Mutability, PatKind, Path, QPath, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::{Span, Symbol};

declare_lint! {
    /// ### What it does
    ///
    /// Flags two or more consecutive `let` statements that each split a chunk of the same length
    /// off the front of a slice with `split_at` or `split_at_mut`, each splitting the remainder
    /// left by the one before, like
    ///
    /// ```rust,ignore
    /// let (x_sum, scratch) = scratch.split_at_mut(c);
    /// let (y_sum, scratch) = scratch.split_at_mut(c);
    /// ```
    ///
    /// ### Why is this bad?
    ///
    /// malachite-base's `split_into_chunks!` and `split_into_chunks_mut!` macros say the same
    /// thing in one line: these are equal-length chunks, followed by whatever is left. The chain of
    /// `split_at`s repeats the length and threads a remainder binding through every line, which
    /// hides that the chunks are uniform.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let (x_sum, scratch) = scratch.split_at_mut(c);
    /// let (y_sum, scratch) = scratch.split_at_mut(c);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// split_into_chunks_mut!(scratch, c, [x_sum, y_sum], scratch);
    /// ```
    pub USE_SPLIT_INTO_CHUNKS,
    Deny,
    "a chain of equal-length `split_at`s that `split_into_chunks!` expresses directly"
}

declare_lint_pass!(UseSplitIntoChunks => [USE_SPLIT_INTO_CHUNKS]);

// One `let (chunk, rest) = receiver.split_at(len);` statement.
struct Split<'tcx> {
    span: Span,
    method: Symbol,
    receiver: &'tcx Expr<'tcx>,
    len: &'tcx Expr<'tcx>,
    chunk: Symbol,
    // The remainder's binding, or `None` if it is discarded with `_`.
    rest: Option<(HirId, Symbol)>,
}

// Recognizes `let (chunk, rest) = receiver.split_at(len);` or the `split_at_mut` form, where the
// receiver is a slice (after auto-deref), `chunk` is a plain immutable binding, and `rest` is a
// plain immutable binding or `_`. The macros bind exactly that way.
fn as_split<'tcx>(cx: &LateContext<'tcx>, stmt_kind: &StmtKind<'tcx>) -> Option<Split<'tcx>> {
    let StmtKind::Let(local) = stmt_kind else {
        return None;
    };
    if local.els.is_some() || local.ty.is_some() {
        return None;
    }
    let init = local.init?;
    let ExprKind::MethodCall(seg, receiver, [len], _) = init.kind else {
        return None;
    };
    let method = seg.ident.name;
    if !matches!(method.as_str(), "split_at" | "split_at_mut") {
        return None;
    }
    if !cx
        .typeck_results()
        .expr_ty_adjusted(receiver)
        .peel_refs()
        .is_slice()
    {
        return None;
    }
    let PatKind::Tuple([chunk_pat, rest_pat], dot_dot) = local.pat.kind else {
        return None;
    };
    if dot_dot.as_opt_usize().is_some() {
        return None;
    }
    let PatKind::Binding(BindingMode(ByRef::No, Mutability::Not), _, chunk, None) = chunk_pat.kind
    else {
        return None;
    };
    let rest = match rest_pat.kind {
        PatKind::Wild => None,
        PatKind::Binding(BindingMode(ByRef::No, Mutability::Not), id, ident, None) => {
            Some((id, ident.name))
        }
        _ => return None,
    };
    Some(Split {
        span: local.span,
        method,
        receiver,
        len,
        chunk: chunk.name,
        rest,
    })
}

// Whether `e` is a path to the whole local `id`.
fn is_local(e: &Expr<'_>, id: HirId) -> bool {
    matches!(
        e.kind,
        ExprKind::Path(QPath::Resolved(
            _,
            Path {
                res: Res::Local(local),
                ..
            },
        )) if *local == id
    )
}

// The number of times the local `id` is used in the statements of `block` from index `from` on,
// and in its trailing expression.
fn uses_after<'tcx>(cx: &LateContext<'tcx>, block: &Block<'tcx>, from: usize, id: HirId) -> usize {
    let mut count = 0;
    let mut inspect = |e: &Expr<'_>| {
        if is_local(e, id) {
            count += 1;
        }
        ControlFlow::<()>::Continue(())
    };
    for stmt in &block.stmts[from..] {
        for_each_expr(cx, stmt, &mut inspect);
    }
    if let Some(e) = block.expr {
        for_each_expr(cx, e, &mut inspect);
    }
    count
}

fn report(cx: &LateContext<'_>, run: &[Split<'_>]) {
    let first = &run[0];
    let last = &run[run.len() - 1];
    let (Some(receiver), Some(len)) = (
        snippet_opt(cx, first.receiver.span),
        snippet_opt(cx, first.len.span),
    ) else {
        return;
    };
    let macro_name = if first.method.as_str() == "split_at_mut" {
        "split_into_chunks_mut"
    } else {
        "split_into_chunks"
    };
    let chunks: Vec<_> = run.iter().map(|s| s.chunk.to_string()).collect();
    let rest = last
        .rest
        .map_or_else(|| "_unused".to_string(), |(_, name)| name.to_string());
    span_lint_and_sugg(
        cx,
        USE_SPLIT_INTO_CHUNKS,
        first.span.to(last.span),
        format!(
            "{} equal-length chunks are split off one after another",
            run.len()
        ),
        format!("use malachite-base's `{macro_name}!`"),
        format!(
            "{macro_name}!({receiver}, {len}, [{}], {rest});",
            chunks.join(", ")
        ),
        Applicability::MaybeIncorrect,
    );
}

impl<'tcx> LateLintPass<'tcx> for UseSplitIntoChunks {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        let mut run: Vec<Split<'tcx>> = Vec::new();
        for (i, stmt) in block.stmts.iter().enumerate() {
            let split = if stmt.span.from_expansion() {
                None
            } else {
                as_split(cx, &stmt.kind)
            };
            // A split continues the run if it splits the previous remainder, which is used nowhere
            // else, with the same method and the same length.
            let continues = split.as_ref().is_some_and(|s| {
                run.last().is_some_and(|prev| {
                    prev.rest.is_some_and(|(id, _)| {
                        is_local(s.receiver, id) && uses_after(cx, block, i, id) == 1
                    }) && s.method == prev.method
                        && eq_expr_value(cx, s.len, prev.len)
                })
            });
            if !continues {
                if run.len() >= 2 {
                    report(cx, &run);
                }
                run.clear();
            }
            if let Some(s) = split {
                run.push(s);
            }
        }
        if run.len() >= 2 {
            report(cx, &run);
        }
    }
}
