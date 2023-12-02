use std::io::stdin;
use clap::Parser;
use aoc2023::find_numbers::{find_two_digits, find_two_digits_words};

#[derive(Parser)]
struct Options {
    #[arg(short = 'w',long)]
    pub allow_words: bool,
}

pub fn main() -> anyhow::Result<()> {
    aoc2023::bootstrap();
    let opts = Options::parse();

    let f = if opts.allow_words {
        find_two_digits_words
    } else {
        find_two_digits
    };

    let mut total = 0;
    for line in stdin().lines() {
        let line = line?;

        if let Some(v) = f(&line) {
            total += v;
            tracing::debug!("v={v} total={total}");
        }
    }

    tracing::info!("total={total}");
    Ok(())
}
