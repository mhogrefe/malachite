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
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags a numeric conversion of a `const { .. }` block — either an `as` cast
    /// (`const { (A - B) << 1 } as f64`) or a `from`/`exact_from`/`wrapping_from` call
    /// (`u64::exact_from(const { Self::MAX_EXPONENT - 1 })`). The whole expression is a compile-time
    /// constant, so the conversion should be an `as` cast *inside* the block:
    /// `const { ((A - B) << 1) as f64 }` / `const { (Self::MAX_EXPONENT - 1) as u64 }`.
    ///
    /// ### Why is this bad?
    ///
    /// The value is already known at compile time (that is what the `const { .. }` block says), but
    /// the conversion is still a runtime call or cast. Folding it into the block as an `as` cast
    /// evaluates the whole thing once, at compile time. For a value representable in the target type
    /// — which a working conversion guarantees — the cast produces the same result.
    ///
    /// This complements `use_const_block`, which wraps the constant argument in the first place. Once
    /// the argument is a `const { .. }` block, the conversion belongs inside it; `use_const_block`
    /// leaves it out because `ConstEvalCtxt` does not fold `as` casts, so the block stops at the
    /// integer operand.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let x = u64::exact_from(const { Self::MAX_EXPONENT - 1 });
    /// let y = const { (A - B) << 1 } as f64;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// let x = const { (Self::MAX_EXPONENT - 1) as u64 };
    /// let y = const { ((A - B) << 1) as f64 };
    /// ```
    pub USE_CONST_CAST,
    Deny,
    "converting a `const` block at runtime instead of an `as` cast inside it"
}

declare_lint_pass!(UseConstCast => [USE_CONST_CAST]);

const CONVERSIONS: [&str; 3] = ["from", "exact_from", "wrapping_from"];

// If `expr` converts a single operand to a numeric type — an `as` cast, or a
// `from`/`exact_from`/`wrapping_from` call — returns the operand and the target type's name.
fn conversion<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> Option<(&'tcx Expr<'tcx>, String)> {
    if !cx.typeck_results().expr_ty(expr).is_numeric() {
        return None;
    }
    match expr.kind {
        // `operand as T`
        ExprKind::Cast(operand, cast_ty) => {
            Some((operand, snippet(cx, cast_ty.span, "..").to_string()))
        }
        // `T::from`/`exact_from`/`wrapping_from(operand)`
        ExprKind::Call(callee, [operand]) => {
            let ExprKind::Path(qpath) = &callee.kind else {
                return None;
            };
            let Res::Def(DefKind::AssocFn, did) = cx.qpath_res(qpath, callee.hir_id) else {
                return None;
            };
            if !CONVERSIONS.contains(&cx.tcx.item_name(did).as_str()) {
                return None;
            }
            // The target type: the callee's path prefix (preserving aliases like `Limb`), otherwise
            // the resolved type.
            let callee_snip = snippet(cx, callee.span, "");
            let target = callee_snip.rsplit_once("::").map_or_else(
                || cx.typeck_results().expr_ty(expr).to_string(),
                |(prefix, _)| prefix.to_string(),
            );
            Some((operand, target))
        }
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for UseConstCast {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() || crate::in_test_code(cx, expr.span) {
            return;
        }
        let Some((operand, target)) = conversion(cx, expr) else {
            return;
        };
        // The operand is a `const { .. }` block (as `use_const_block` produces for a constant
        // island), whose inner value we can lift the cast onto.
        let ExprKind::ConstBlock(const_block) = operand.kind else {
            return;
        };
        // The const block's body is a `{ X }` block; take its tail expression `X`.
        let body = cx.tcx.hir_body(const_block.body).value;
        let inner = match body.kind {
            ExprKind::Block(block, _) if block.stmts.is_empty() => block.expr.unwrap_or(body),
            _ => body,
        };
        span_lint_and_help(
            cx,
            USE_CONST_CAST,
            expr.span,
            "this converts a compile-time constant at runtime",
            None,
            format!(
                "do the conversion at compile time: `const {{ ({}) as {target} }}`",
                snippet(cx, inner.span, "..")
            ),
        );
    }
}
