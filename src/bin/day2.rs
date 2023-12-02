use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use clap::Parser;
use nom::Finish;

#[derive(Debug, Clone, Default)]
pub struct Cubes {
    pub seen: HashMap<String, u32>,
}

impl Cubes {
    pub fn power(&self) -> u32 {
        self.seen.values().fold(1, |acc, v| acc * *v)
    }

    pub fn includes(&self, other: &Cubes) -> bool {
        other.seen.iter()
            .all(|(name, needed)|
                self.seen.get(name)
                    .map_or(false, |have| *have >= *needed))
    }

    pub fn max(&mut self, other: &Cubes) {
        for (name, needed) in &other.seen {
            match self.seen.get_mut(name) {
                Some(have) => *have = (*have).max(*needed),
                None => {
                    self.seen.insert(name.to_string(), *needed);
                }
            }
        }
    }
}

impl FromStr for Cubes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::cubes(s)
            .finish()
            .map(|(_, v)| v)
            .map_err(|e| anyhow!("{e}"))
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Cubes>,
}

impl Game {
    pub fn is_valid_for_cubes(&self, cubes: &Cubes) -> bool {
        self.rounds.iter()
            .all(|r| cubes.includes(r))
    }

    pub fn max_cubes(&self) -> Cubes {
        let mut c = Cubes::default();
        for r in &self.rounds {
            c.max(r);
        }
        c
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::game(s)
            .finish()
            .map(|(_, v)| v)
            .map_err(|e| anyhow!("{e}"))
    }
}

mod parse {
    use std::collections::HashMap;

    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, space0, space1, u32};
    use nom::combinator::map;
    use nom::IResult;
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, separated_pair, tuple};

    use crate::{Cubes, Game};

    pub fn cubes(input: &str) -> IResult<&str, Cubes> {
        map(
            separated_list1(
                tag(","),
                map(
                    delimited(space0, separated_pair(u32, space1, alpha1), space0),
                    |(n, name): (u32, &str)| (name.to_string(), n),
                ),
            ),
            |entries| Cubes {
                seen: HashMap::from_iter(entries.into_iter()),
            },
        )(input)
    }

    pub fn game(input: &str) -> IResult<&str, Game> {
        map(
            tuple((
                tag("Game"),
                space1,
                u32,
                tag(":"),
                space0,
                separated_list1(tag(";"), cubes),
            )),
            |(_, _, id, _, _, rounds)| Game { id, rounds },
        )(input)
    }
}

#[derive(Parser)]
struct Options {
    #[arg(default_value = "12 red, 13 green, 14 blue")]
    pub have: Cubes,
}

pub fn main() -> anyhow::Result<()> {
    aoc2023::bootstrap();
    let opts = Options::parse();

    tracing::debug!("have {:?}", &opts.have);

    let mut total = 0;
    let mut total_power = 0;
    for line in stdin().lines() {
        let line = line?;
        let game = line.parse::<Game>()
            .with_context(|| anyhow!("parsing '{line}'"))?;
        let power = game.max_cubes().power();
        total_power += power;

        tracing::info!("{game:?} power={power}");

        if game.is_valid_for_cubes(&opts.have) {
            total += game.id;
            tracing::debug!("have enough for {}", game.id);
        }
    }

    tracing::info!("total={total} power={total_power}");
    Ok(())
}
