#![allow(
    clippy::many_single_char_names,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_if_let_else,
    clippy::similar_names,
    clippy::uninlined_format_args
)]

pub mod algorithms;
pub mod data_structures;
pub mod rand;

#[cfg(test)]
mod tests;
