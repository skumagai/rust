// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Lints in the Rust compiler.
//!
//! This currently only contains the definitions and implementations
//! of most of the lints that `rustc` supports directly, it does not
//! contain the infrastructure for defining/registering lints. That is
//! available in `rustc::lint` and `rustc::plugin` respectively.
//!
//! # Note
//!
//! This API is completely unstable and subject to change.

// Do not remove on snapshot creation. Needed for bootstrap. (Issue #22364)
#![cfg_attr(stage0, feature(custom_attribute))]
#![crate_name = "rustc_lint"]
#![unstable(feature = "rustc_private")]
#![staged_api]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
      html_favicon_url = "http://www.rust-lang.org/favicon.ico",
      html_root_url = "http://doc.rust-lang.org/nightly/")]

#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(collections)]
#![feature(core)]
#![feature(int_uint)]
#![feature(quote)]
#![feature(rustc_diagnostic_macros)]
#![feature(rustc_private)]
#![feature(unsafe_destructor)]
#![feature(staged_api)]
#![feature(std_misc)]
#![feature(unicode)]
#![cfg_attr(test, feature(test))]

extern crate syntax;
#[macro_use]
extern crate rustc;
#[macro_use]
extern crate log;

pub use rustc::lint as lint;
pub use rustc::metadata as metadata;
pub use rustc::middle as middle;
pub use rustc::session as session;
pub use rustc::util as util;

use session::Session;
use lint::{LintPassObject, LintId};

mod builtin;

/// Tell the `LintStore` about all the built-in lints (the ones
/// defined in this crate and the ones defined in
/// `rustc::lint::builtin`).
pub fn register_builtins(store: &mut lint::LintStore, sess: Option<&Session>) {
    macro_rules! add_builtin {
        ($sess:ident, $($name:ident),*,) => (
            {$(
                store.register_pass($sess, false, box builtin::$name as LintPassObject);
                )*}
            )
    }

    macro_rules! add_builtin_with_new {
        ($sess:ident, $($name:ident),*,) => (
            {$(
                store.register_pass($sess, false, box builtin::$name::new() as LintPassObject);
                )*}
            )
    }

    macro_rules! add_lint_group {
        ($sess:ident, $name:expr, $($lint:ident),*) => (
            store.register_group($sess, false, $name, vec![$(LintId::of(builtin::$lint)),*]);
            )
    }

    add_builtin!(sess,
                 HardwiredLints,
                 WhileTrue,
                 UnusedCasts,
                 ImproperCTypes,
                 BoxPointers,
                 UnusedAttributes,
                 PathStatements,
                 UnusedResults,
                 NonCamelCaseTypes,
                 NonSnakeCase,
                 NonUpperCaseGlobals,
                 UnusedParens,
                 UnusedImportBraces,
                 NonShorthandFieldPatterns,
                 UnusedUnsafe,
                 UnsafeCode,
                 UnusedMut,
                 UnusedAllocation,
                 MissingCopyImplementations,
                 UnstableFeatures,
                 Stability,
                 UnconditionalRecursion,
                 InvalidNoMangleItems,
                 PluginAsLibrary,
                 );

    add_builtin_with_new!(sess,
                          TypeLimits,
                          RawPointerDerive,
                          MissingDoc,
                          MissingDebugImplementations,
                          );

    add_lint_group!(sess, "bad_style",
                    NON_CAMEL_CASE_TYPES, NON_SNAKE_CASE, NON_UPPER_CASE_GLOBALS);

    add_lint_group!(sess, "unused",
                    UNUSED_IMPORTS, UNUSED_VARIABLES, UNUSED_ASSIGNMENTS, DEAD_CODE,
                    UNUSED_MUT, UNREACHABLE_CODE, UNUSED_MUST_USE,
                    UNUSED_UNSAFE, PATH_STATEMENTS);

    // We have one lint pass defined specially
    store.register_pass(sess, false, box lint::GatherNodeLevels as LintPassObject);

    // Insert temporary renamings for a one-time deprecation
    store.register_renamed("raw_pointer_deriving", "raw_pointer_derive");

    store.register_renamed("unknown_features", "unused_features");
}
