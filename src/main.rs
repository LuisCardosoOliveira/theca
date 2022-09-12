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
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn,
    clippy::exit
)]

use std::process::exit;

mod cli;
mod errors;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = match cli::parse_args() {
        Ok(args) => args,
        Err(e) => {
            tracing::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{},hyper=info,mio=info", args.log_level),
        );
    }
    tracing::info!("Logs args: {:?}", args);
}
