use std::collections::VecDeque;
use std::io::stdin;
use std::process::exit;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1};
use nom::multi::many1;
use nom::sequence::{delimited, separated_pair, tuple};
use aoc2023::convert_nom_error;

fn parse_card(src: &str) -> anyhow::Result<(i32, Vec<i32>, Vec<i32>)> {
    let number_parser = || many1(delimited(space0, nom::character::complete::i32, space0));
    let mut parser = tuple((
        delimited(tuple((tag("Card"), space1)), nom::character::complete::i32, delimited(space0, tag(":"), space0)),
        separated_pair(number_parser(), tag("|"), number_parser()),
    ));
    let (_, (n, (goal, have))) = parser(src)
        .map_err(convert_nom_error)?;
    Ok((n, goal, have))
}


pub fn main() -> anyhow::Result<()> {
    aoc2023::bootstrap();

    let mut total = 0;
    let mut cards = 0;
    let mut queue = VecDeque::new();
    let mut last_id = None;

    for line in stdin().lines() {
        let count = queue.pop_front().unwrap_or(0) + 1;
        let line = line?;
        let (id, goal, have) = parse_card(&line)?;
        let n = have.iter().filter(|n| goal.contains(n)).count();
        tracing::info!("Card {id} => {n}");

        if let Some(last_id) = &last_id {
            if *last_id + 1 != id {
                tracing::error!("bad card ID: {id} - was {last_id}");
                exit(1);
            }
        }
        last_id = Some(id);

        cards += count;

        if n == 0 {
            continue;
        }

        let value = 1 << (n - 1);
        total += value;

        for i in 0..n {
            if let Some(q) = queue.get_mut(i) {
                *q += count;
            } else {
                queue.push_back(count);
            }
        }
    }

    tracing::info!("total={total} cards={cards}");
    Ok(())
}
