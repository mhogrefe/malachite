// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::attrs::InlineAttr;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, Expr, ExprKind, FnDecl};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::Span;
use rustc_span::def_id::LocalDefId;

declare_lint! {
    /// ### What it does
    ///
    /// Flags a public function whose entire body is a single call that forwards to another
    /// function or method (a trivial delegator), but which is not marked `#[inline]`.
    ///
    /// ### Why is this bad?
    ///
    /// The public API is full of thin forwarding wrappers -- the by-value/by-reference variants of
    /// a family (`foo_val_ref`, `foo_ref_val`, ...), the `*_assign` companions, and the operator
    /// trait impls -- that exist only to call the one real implementation. Without `#[inline]`,
    /// such a wrapper is not inlined into a downstream crate, so every caller pays a real function
    /// call to reach the delegate. Marking it `#[inline]` lets it disappear at the call site across
    /// crate boundaries, which is the whole point of a forwarding wrapper.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// pub fn pow_prec_round_ref_val(&self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
    ///     self.pow_prec_round_ref_ref(&other, prec, rm)
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// #[inline]
    /// pub fn pow_prec_round_ref_val(&self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
    ///     self.pow_prec_round_ref_ref(&other, prec, rm)
    /// }
    /// ```
    pub MISSING_INLINE_ON_DELEGATOR,
    Deny,
    "a public function that only delegates to another function is not marked `#[inline]`"
}

declare_lint_pass!(MissingInlineOnDelegator => [MISSING_INLINE_ON_DELEGATOR]);

// Whether `callee` resolves to a tuple-struct or enum-variant constructor (or `Self(..)`). Such a
// call constructs a value rather than delegating to another function, so it is not flagged.
fn is_constructor(cx: &LateContext<'_>, callee: &Expr<'_>) -> bool {
    let ExprKind::Path(qpath) = &callee.kind else {
        return false;
    };
    matches!(
        cx.qpath_res(qpath, callee.hir_id),
        Res::Def(DefKind::Ctor(..), _) | Res::SelfCtor(_)
    )
}

impl<'tcx> LateLintPass<'tcx> for MissingInlineOnDelegator {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        _decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        id: LocalDefId,
    ) {
        // Only named functions and methods; a closure cannot carry `#[inline]`.
        let name_span = match kind {
            FnKind::ItemFn(ident, ..) | FnKind::Method(ident, ..) => ident.span,
            FnKind::Closure => return,
        };
        if span.from_expansion() || crate::in_test_code(cx, span) {
            return;
        }
        // Only a public function benefits from an explicit `#[inline]`: cross-crate inlining is
        // opt-in, whereas the compiler already inlines small same-crate functions on its own.
        if !cx.tcx.visibility(id).is_public() {
            return;
        }
        // Already carries some form of `#[inline]` (hint, always, or never -- respect an explicit
        // `never`).
        if !matches!(
            cx.tcx.codegen_fn_attrs(id.to_def_id()).inline,
            InlineAttr::None
        ) {
            return;
        }
        // The body must be a single delegating call and nothing else: a block with no statements
        // whose trailing expression forwards to another function or method.
        let ExprKind::Block(block, _) = body.value.kind else {
            return;
        };
        if !block.stmts.is_empty() {
            return;
        }
        let Some(tail) = block.expr else {
            return;
        };
        if tail.span.from_expansion() {
            return;
        }
        let delegates = match tail.kind {
            ExprKind::MethodCall(..) => true,
            ExprKind::Call(callee, _) => !is_constructor(cx, callee),
            _ => false,
        };
        if !delegates {
            return;
        }
        span_lint_and_help(
            cx,
            MISSING_INLINE_ON_DELEGATOR,
            name_span,
            "this public function only delegates to another function",
            None,
            "mark it `#[inline]` so it can be inlined across crate boundaries",
        );
    }
}
