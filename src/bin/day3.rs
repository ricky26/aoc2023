use std::collections::{HashMap, HashSet};
use std::io::Read;

use glam::IVec2;
use nom::character::is_digit;

use aoc2023::ascii_grid::{AsciiGrid, Direction};

fn is_number(b: u8) -> bool {
    b.is_ascii_digit()
}

fn is_symbol(b: u8) -> bool {
    !is_number(b) && b != b'.'
}

fn find_number_start(grid: &AsciiGrid, mut q: IVec2) -> Option<IVec2> {
    let Some(c) = grid.get(q) else {
        return None;
    };

    if !is_number(c) {
        return None;
    }

    while q.x > 0 && grid.get(q - IVec2::X).map_or(false, is_number) {
        q.x -= 1;
    }

    return Some(q);
}

fn read_number(grid: &AsciiGrid, mut p: IVec2) -> i32 {
    let mut v = 0;

    while let Some(c) = grid.get(p).filter(|b| is_digit(*b)) {
        v *= 10;
        v += (c - b'0') as i32;
        p.x += 1;
    }

    v
}

pub fn main() -> anyhow::Result<()> {
    aoc2023::bootstrap();

    let mut stdin = String::new();
    std::io::stdin().read_to_string(&mut stdin)?;
    let grid = AsciiGrid::try_from(stdin.as_str())?;

    let mut seen = HashSet::new();
    let mut gear_locations = HashMap::new();

    for x in 0..grid.width() as i32 {
        for y in 0..grid.height() as i32 {
            let p = IVec2::new(x, y);
            let b = grid.get(p).unwrap();
            if !is_symbol(b) {
                continue;
            }

            for d in Direction::all() {
                let q = p + d.delta();
                let Some(q) = find_number_start(&grid, q) else {
                    continue;
                };

                if b == b'*' {
                    let entries = gear_locations.entry(p)
                        .or_insert(Vec::with_capacity(2));
                    if !entries.contains(&q) {
                        entries.push(q);
                    }
                }

                seen.insert(q);
            }
        }
    }

    let mut total = 0;
    for p in seen.iter().copied() {
        total += read_number(&grid, p);
    }

    let mut gear_total = 0;
    for entries in gear_locations.values() {
        if entries.len() != 2 {
            continue;
        }

        let a = read_number(&grid, entries[0]);
        let b = read_number(&grid, entries[1]);
        gear_total += a * b;
    }

    tracing::info!("total={total} gear-total={gear_total}");
    Ok(())
}
