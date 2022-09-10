#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::separated_literal_suffix,
    clippy::blanket_clippy_restriction_lints,
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    clippy::as_conversions,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::option_if_let_else,
    clippy::expect_used,
    clippy::integer_arithmetic,
    clippy::separated_literal_suffix,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

mod cli;

fn main() {
    let args = cli::parse_args();
    println!("{:?}", args);
}
