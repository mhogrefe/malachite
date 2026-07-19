// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::source::snippet;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::DefId;
use rustc_hir::{ConstItemRhs, Expr, ExprKind, ImplItemKind, ItemKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::Span;
use std::collections::HashMap;

declare_lint! {
    /// ### What it does
    ///
    /// Flags a *derived* compile-time constant that is written out more than once with the same
    /// value — whether as a `const { .. }` block (`const { Limb::WIDTH - 1 }`) or as a `const` item
    /// (`const TWICE_WIDTH: u64 = Limb::WIDTH << 1;`) — across the crate. Such repeats should be
    /// consolidated into a single named constant.
    ///
    /// ### Why is this bad?
    ///
    /// Repeating the same constant computation in many places is error-prone (they can drift apart)
    /// and obscures that they are meant to be the same value. A single named constant — a
    /// `pub(crate)` associated constant on the relevant type, or a standalone `const` — states the
    /// value once and lets every site refer to it.
    ///
    /// Instances are grouped by their source text *and* by the `DefId`s of the constants they
    /// reference, so two `const { UPPER_LIMIT - 1 }` blocks whose `UPPER_LIMIT` is a different
    /// (scope-local) constant are **not** merged — only genuinely identical values are.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// // in one module:
    /// let a = x >> const { Limb::WIDTH - 1 };
    /// // in another:
    /// const WIDTH_MINUS_1: u64 = Limb::WIDTH - 1;
    /// ```
    ///
    /// Use instead: a single `pub(crate) const WIDTH_MINUS_1: u64 = Limb::WIDTH - 1;` referenced from
    /// both places.
    pub DUPLICATE_CONST,
    Deny,
    "the same derived constant written out more than once instead of a single named constant"
}

#[derive(Default)]
pub struct DuplicateConst {
    // (expression text, DefIds of referenced path constants) -> the spans that share it
    groups: HashMap<(String, Vec<DefId>), Vec<Span>>,
}

impl_lint_pass!(DuplicateConst => [DUPLICATE_CONST]);

// The tail expression of a `{ X }` block (a `const { X }` block's body), else `e` itself.
fn peel_block<'tcx>(e: &'tcx Expr<'tcx>) -> &'tcx Expr<'tcx> {
    if let ExprKind::Block(block, _) = e.kind
        && block.stmts.is_empty()
        && let Some(v) = block.expr
    {
        v
    } else {
        e
    }
}

// Collects the `DefId`s of the `const`/associated-const paths referenced by `e` (resolved paths
// only — a type-relative path like `Limb::WIDTH` is unambiguous by text, whereas a bare path like a
// scope-local `UPPER_LIMIT` needs its `DefId` to tell instances apart).
fn collect_const_defids(e: &Expr<'_>, out: &mut Vec<DefId>) {
    match e.kind {
        ExprKind::Path(QPath::Resolved(_, path)) => {
            if let Res::Def(DefKind::Const { .. } | DefKind::AssocConst { .. }, did) = path.res {
                out.push(did);
            }
        }
        ExprKind::Unary(_, inner) | ExprKind::Cast(inner, _) => collect_const_defids(inner, out),
        ExprKind::Binary(_, l, r) => {
            collect_const_defids(l, out);
            collect_const_defids(r, out);
        }
        _ => {}
    }
}

impl DuplicateConst {
    fn record(&mut self, cx: &LateContext<'_>, init: &Expr<'_>, span: Span) {
        if span.from_expansion() || crate::in_test_code(cx, span) {
            return;
        }
        let inner = peel_block(init);
        // Only *derived* scalar constants — arithmetic, a unary op, or a cast. This skips bare
        // literals and paths (renames), and also array/struct/call constants, which are a different
        // (and much larger) sort of thing than a `Limb::WIDTH - 1`.
        if !matches!(
            inner.kind,
            ExprKind::Binary(..) | ExprKind::Unary(..) | ExprKind::Cast(..)
        ) {
            return;
        }
        let mut dids = Vec::new();
        collect_const_defids(inner, &mut dids);
        let text = snippet(cx, inner.span, "..").to_string();
        self.groups.entry((text, dids)).or_default().push(span);
    }
}

impl<'tcx> LateLintPass<'tcx> for DuplicateConst {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::ConstBlock(const_block) = expr.kind {
            self.record(cx, cx.tcx.hir_body(const_block.body).value, expr.span);
        }
    }

    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx rustc_hir::Item<'tcx>) {
        if let ItemKind::Const(.., ConstItemRhs::Body(body)) = item.kind {
            self.record(cx, cx.tcx.hir_body(body).value, item.span);
        }
    }

    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx rustc_hir::ImplItem<'tcx>) {
        if let ImplItemKind::Const(_, ConstItemRhs::Body(body)) = item.kind {
            self.record(cx, cx.tcx.hir_body(body).value, item.span);
        }
    }

    fn check_crate_post(&mut self, cx: &LateContext<'tcx>) {
        for ((text, _), spans) in &self.groups {
            if spans.len() >= 2 {
                for &span in spans {
                    span_lint_and_help(
                        cx,
                        DUPLICATE_CONST,
                        span,
                        format!("`{text}` is written out {} times with the same value", spans.len()),
                        None,
                        "consolidate it into one named constant (a `pub(crate)` associated constant, \
                         or a standalone `const`)",
                    );
                }
            }
        }
    }
}
