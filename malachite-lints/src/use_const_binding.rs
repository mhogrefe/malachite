// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::consts::ConstEvalCtxt;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{BindingMode, ExprKind, LetStmt, PatKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

declare_lint! {
    /// ### What it does
    ///
    /// Flags an immutable `let` whose initializer is a *derived* compile-time constant — arithmetic,
    /// a unary operation, or a cast built from at least one named constant, such as
    /// `let rnd_bit = Limb::WIDTH - 5;`. Suggests declaring it as a `const` instead.
    ///
    /// ### Why is this bad?
    ///
    /// A value that is fixed at compile time reads more clearly as a named `const`: the `const`
    /// announces that it does not depend on any runtime state, is computed once, and can be
    /// referred to by an unambiguous `SCREAMING_SNAKE_CASE` name. It also lets the reader (and other
    /// lints — see `shift_of_one`, which treats a constant shift amount specially) recognize the
    /// operand as a constant without having to trace the `let` back to its initializer.
    ///
    /// Only *derived* constants are flagged: a bare literal (`let n = 5;`) is already as clear as it
    /// gets, and a bare path to an existing constant would just be a rename. Initializers that do
    /// not evaluate at compile time are left alone — which also excludes anything that depends on a
    /// generic parameter, where a `const` item could not name it in the first place.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let rnd_bit = Limb::WIDTH - 5;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// const RND_BIT: u64 = Limb::WIDTH - 5;
    /// ```
    pub USE_CONST_BINDING,
    Deny,
    "binding a compile-time constant with `let` instead of declaring a `const`"
}

declare_lint_pass!(UseConstBinding => [USE_CONST_BINDING]);

impl<'tcx> LateLintPass<'tcx> for UseConstBinding {
    fn check_local(&mut self, cx: &LateContext<'tcx>, local: &'tcx LetStmt<'tcx>) {
        if local.span.from_expansion() || crate::in_test_code(cx, local.span) {
            return;
        }
        // An immutable, by-value binding of a single name: `let x = ..`.
        let PatKind::Binding(BindingMode::NONE, _, ident, None) = local.pat.kind else {
            return;
        };
        let Some(init) = local.init else {
            return;
        };
        // A *computed* initializer: arithmetic, a unary op, or a cast. A bare literal or a bare path
        // to an existing constant is not worth turning into a `const`.
        if !matches!(
            init.kind,
            ExprKind::Binary(..) | ExprKind::Unary(..) | ExprKind::Cast(..)
        ) {
            return;
        }
        // ... that evaluates at compile time. This also rules out anything depending on a generic
        // parameter, where a `const` item could not name it.
        if ConstEvalCtxt::new(cx).eval(init).is_none() {
            return;
        }
        // ... and is built from at least one named constant (so it is a derived constant, not a
        // literal computation).
        if !crate::references_named_const(cx, init) {
            return;
        }
        span_lint_and_help(
            cx,
            USE_CONST_BINDING,
            local.span,
            format!("`{}` is bound to a compile-time constant", ident.name),
            None,
            "declare it as a `const` (with a `SCREAMING_SNAKE_CASE` name and an explicit type)",
        );
    }
}
