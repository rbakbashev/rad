#![allow(
    clippy::collapsible_else_if,
    clippy::many_single_char_names,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default,
    clippy::option_if_let_else,
    clippy::similar_names,
    clippy::uninlined_format_args
)]
#![cfg_attr(
    test,
    allow(
        clippy::bool_assert_comparison,
        clippy::unwrap_used,
        clippy::option_map_unit_fn,
        clippy::explicit_iter_loop,
    )
)]
// remove when Vectors become const stable
#![allow(internal_features)]
#![feature(const_heap, core_intrinsics)]

pub mod algorithms;
pub mod data_structures;
pub mod rand;

#[cfg(test)]
mod tests;
