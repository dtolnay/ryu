#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        cast_lossless,
        cyclomatic_complexity,
        many_single_char_names,
        unreadable_literal,
    )
)]

mod common;
mod d2s;
mod d2s_full_table;
mod digit_table;
mod f2s;
mod mulshift128;

pub use d2s::d2s_buffered_n;
pub use f2s::f2s_buffered_n;
