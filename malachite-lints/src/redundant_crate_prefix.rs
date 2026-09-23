// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use clippy_utils::diagnostics::span_lint_and_sugg;
use rustc_errors::Applicability;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{HirId, Path};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, impl_lint_pass};
use rustc_span::{Span, kw};
use std::collections::HashSet;

declare_lint! {
    /// ### What it does
    ///
    /// Flags a path that reaches another crate through `crate::`, such as `use
    /// crate::malachite_base::num::basic::traits::Zero;`.
    ///
    /// ### Why is this bad?
    ///
    /// The `crate::` is not needed. Such a path compiles only because the crate root declares
    /// `extern crate malachite_base;`, which puts the crate at `crate::malachite_base` as well as
    /// under its own name. House style names another crate the way the rest of the code does,
    /// starting from its own name.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// use crate::malachite_base::num::basic::traits::Zero;
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use malachite_base::num::basic::traits::Zero;
    /// ```
    pub REDUNDANT_CRATE_PREFIX,
    Deny,
    "reaching another crate through a `crate::` prefix that is not needed"
}

// A `use` item with a braced list is lowered to one path per name in the list, and every one of
// them starts with the same `crate::<name>` span. The spans already reported are kept, so that the
// prefix is reported once.
#[derive(Default)]
pub struct RedundantCratePrefix {
    reported: HashSet<Span>,
}

impl_lint_pass!(RedundantCratePrefix => [REDUNDANT_CRATE_PREFIX]);

impl<'tcx> LateLintPass<'tcx> for RedundantCratePrefix {
    fn check_path(&mut self, cx: &LateContext<'tcx>, path: &Path<'tcx>, _: HirId) {
        // Paths written by macros are not the author's to change.
        if path.span.from_expansion() || path.segments.len() < 2 {
            return;
        }
        let [first, second, ..] = path.segments else {
            return;
        };
        if first.ident.name != kw::Crate {
            return;
        }
        // The second segment must be another crate's root, which is what an `extern crate` item in
        // the crate root resolves to.
        let Res::Def(DefKind::Mod, did) = second.res else {
            return;
        };
        if !did.is_crate_root() || did.is_local() {
            return;
        }
        let span = first.ident.span.to(second.ident.span);
        if span.from_expansion() || !self.reported.insert(span) {
            return;
        }
        let name = second.ident;
        span_lint_and_sugg(
            cx,
            REDUNDANT_CRATE_PREFIX,
            span,
            format!("`{name}` is reached through a `crate::` prefix that is not needed"),
            "remove the prefix",
            name.to_string(),
            Applicability::MachineApplicable,
        );
    }
}
