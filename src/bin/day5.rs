use std::cmp::Ordering;
use std::io::{Read, stdin};

use anyhow::bail;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0, space1};
use nom::character::complete::i64 as parse_i64;
use nom::combinator::map;
use nom::error::ParseError;
use nom::IResult;
use nom::multi::{fold_many1, many0_count, separated_list1};
use nom::sequence::{delimited, terminated, tuple};
use rayon::prelude::*;

use aoc2023::convert_nom_error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub struct ValueRange {
    source_start: i64,
    target_start: i64,
    len: i64,
}

impl Ord for ValueRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_start.cmp(&other.source_start)
            .then(self.len.cmp(&other.len))
            .then(self.target_start.cmp(&other.target_start))
    }
}

impl ValueRange {
    pub fn source_start(&self) -> i64 {
        self.source_start
    }

    pub fn source_end(&self) -> i64 {
        self.source_start + self.len
    }

    pub fn target_start(&self) -> i64 {
        self.target_start
    }

    pub fn target_end(&self) -> i64 {
        self.target_start + self.len
    }

    pub fn len(&self) -> i64 {
        self.len
    }

    pub fn new(source_start: i64, target_start: i64, len: i64) -> ValueRange {
        ValueRange {
            source_start,
            target_start,
            len,
        }
    }

    pub fn parse(src: &str) -> IResult<&str, ValueRange> {
        map(
            tuple((space0, parse_i64, space0, parse_i64, space0, parse_i64, space0)),
            |(_, target_start, _, source_start, _, len, _)| ValueRange {
                source_start,
                target_start,
                len,
            },
        )(src)
    }
}

#[derive(Clone, Default, Debug)]
pub struct ValueMap {
    storage: Vec<ValueRange>,
}

impl ValueMap {
    fn prior(&self, source: i64) -> Option<ValueRange> {
        let index = match self.storage.binary_search_by_key(&source, |s| s.source_start()) {
            Ok(index) => index,
            Err(index) => {
                if index == 0 {
                    return None;
                }
                index - 1
            }
        };
        Some(self.storage[index])
    }

    fn range_of(&self, source: i64) -> Option<ValueRange> {
        self.prior(source)
            .filter(|r| r.source_end() > source)
    }

    pub fn get(&self, source: i64) -> i64 {
        if let Some(range) = self.range_of(source) {
            range.target_start + (source - range.source_start)
        } else {
            source
        }
    }

    pub fn insert(&mut self, range: ValueRange) {
        if let Some(prior) = self.range_of(range.source_start()) {
            panic!("overlapping range ({} -> {} v {})",
                   prior.source_start(), prior.source_end(), range.source_start());
        }

        let index = self.storage.binary_search(&range).unwrap_err();
        self.storage.insert(index, range);
    }

    pub fn parse(src: &str) -> IResult<&str, ValueMap> {
        fold_many1(
            terminated(ValueRange::parse, tag("\n")),
            || ValueMap::default(),
            |mut m, v| {
                m.insert(v);
                m
            },
        )(src)
    }
}

#[derive(Clone, Default, Debug)]
pub struct Problem {
    seeds: Vec<i64>,
    seed_to_soil: ValueMap,
    soil_to_fertilizer: ValueMap,
    fertilizer_to_water: ValueMap,
    water_to_light: ValueMap,
    light_to_temperature: ValueMap,
    temperature_to_humidity: ValueMap,
    humidity_to_location: ValueMap,
}

impl Problem {
    fn parse_field<'a, T, Error: ParseError<&'a str>>(
        prefix: &'a str, parser: impl nom::Parser<&'a str, T, Error>,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, T, Error> {
        map(
            tuple((tag(prefix), delimited(multispace0, parser, multispace0))),
            |(_, v)| v,
        )
    }

    pub fn parse(src: &str) -> anyhow::Result<Problem> {
        let mut problem = Problem::default();

        {
            let any_field = delimited(multispace0, alt((
                map(
                    Self::parse_field("seeds:", separated_list1(space1, nom::character::complete::i64)),
                    |seeds| problem.seeds = seeds,
                ),
                map(
                    Self::parse_field("seed-to-soil map:", ValueMap::parse),
                    |m| problem.seed_to_soil = m,
                ),
                map(
                    Self::parse_field("soil-to-fertilizer map:", ValueMap::parse),
                    |m| problem.soil_to_fertilizer = m,
                ),
                map(
                    Self::parse_field("fertilizer-to-water map:", ValueMap::parse),
                    |m| problem.fertilizer_to_water = m,
                ),
                map(
                    Self::parse_field("water-to-light map:", ValueMap::parse),
                    |m| problem.water_to_light = m,
                ),
                map(
                    Self::parse_field("light-to-temperature map:", ValueMap::parse),
                    |m| problem.light_to_temperature = m,
                ),
                map(
                    Self::parse_field("temperature-to-humidity map:", ValueMap::parse),
                    |m| problem.temperature_to_humidity = m,
                ),
                map(
                    Self::parse_field("humidity-to-location map:", ValueMap::parse),
                    |m| problem.humidity_to_location = m,
                ),
            )), multispace0);

            let mut parser = many0_count(any_field);
            let (rest, _) = parser(src).map_err(convert_nom_error)?;
            if !rest.is_empty() {
                bail!("unexpected trailing text '{rest}'");
            }
        }

        Ok(problem)
    }

    pub fn plant_location(&self, seed: i64) -> i64 {
        let s = self.seed_to_soil.get(seed);
        let f = self.soil_to_fertilizer.get(s);
        let w = self.fertilizer_to_water.get(f);
        let l = self.water_to_light.get(w);
        let t = self.light_to_temperature.get(l);
        let h = self.temperature_to_humidity.get(t);
        self.humidity_to_location.get(h)
    }

    pub fn best_plant_location(&self) -> Option<i64> {
        self.seeds.iter()
            .copied()
            .map(|seed| self.plant_location(seed))
            .next()
    }

    pub fn best_plant_location_paired(&self) -> Option<i64> {
        self.seeds.chunks(2)
            .par_bridge()
            .map(|chunk| {
                let start = chunk[0];
                let end = start + chunk[1];

                (start..end).map(|seed| self.plant_location(seed)).min()
            })
            .min()
            .flatten()
    }
}


pub fn main() -> anyhow::Result<()> {
    aoc2023::bootstrap();

    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let problem = Problem::parse(&input)?;
    tracing::info!("problem={problem:?}");

    let solution = problem.best_plant_location();
    let solution_paired = problem.best_plant_location_paired();
    tracing::info!("solution={solution:?} paired={solution_paired:?}");
    Ok(())
}
