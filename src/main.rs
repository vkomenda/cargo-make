#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    async_idents,
    const_err,
    dead_code,
    deprecated,
    duplicate_associated_type_bindings,
    duplicate_macro_exports,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incoherent_fundamental_impls,
    intra_doc_link_resolution_failure,
    invalid_type_param_default,
    irrefutable_let_patterns,
    late_bound_lifetime_arguments,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_use_extern_crate,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    private_no_mangle_fns,
    private_no_mangle_statics,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    question_mark_macro_sep,
    safe_extern_statics,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_crate_types,
    unnameable_test_functions,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]
#![cfg_attr(feature = "clippy", feature(plugin))]

//! # cargo-make
//!
//! Rust task runner and build tool.<br>
//! The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
//! A task is a command or a script to execute.<br>
//! Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
//! With a simple toml based configuration file, you can define a multi platform build script that can run build, test,
//! documentation generation, bench tests execution, security validations and more by running a single command.
//!
//! ## Installation
//! In order to install, just run the following command
//!
//! ```sh
//! cargo install cargo-make
//! ```
//!
//! This will install cargo-make in your ~/.cargo/bin.<br>
//! Make sure to add ~/.cargo/bin directory to your PATH variable.
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/simple_redis/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/simple_redis/blob/master/LICENSE) open source license.
//!

extern crate ci_info;
extern crate clap;
extern crate dirs;
extern crate fern;
extern crate glob;
extern crate indexmap;
#[macro_use]
extern crate log;
extern crate rand;
extern crate run_script;
extern crate rust_info;
extern crate semver;
#[macro_use]
extern crate serde_derive;
extern crate shell2batch;
extern crate toml;

// make types public for docs
pub mod types;

mod cache;
mod cli;
mod command;
mod condition;
mod config;
mod descriptor;
mod environment;
mod installer;
mod legacy;
mod logger;
mod runner;
mod scriptengine;
mod storage;
mod version;

#[cfg(test)]
#[path = "./main_test.rs"]
mod main_test;

#[cfg(test)]
#[path = "./test.rs"]
mod test;

fn main() {
    cli::run_cli();
}
