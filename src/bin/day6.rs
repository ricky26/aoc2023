use anyhow::bail;

fn solve(t: i64, d: i64) -> (i64, i64) {
    let tf = t as f64;
    let df = d as f64;
    let right = (tf * tf - 4. * df).sqrt();
    let a = (tf - right) * 0.5;
    let b = (tf + right) * 0.5;
    let min = a.min(b);
    let max = a.max(b);
    (min.floor() as i64 + 1, max.ceil() as i64 - 1)
}

fn parse_number_ignore_spaces(src: &str) -> i64 {
    let mut result = 0;
    for c in src.chars().filter(|c| c.is_ascii_digit()) {
        let v = c.to_digit(10).unwrap() as i64;
        result *= 10;
        result += v;
    }
    result
}

pub fn main() -> anyhow::Result<()> {
    aoc2023::bootstrap();

    let mut times = Vec::new();
    let mut distances = Vec::new();

    let mut big_time = 0;
    let mut big_distance = 0;

    for line in std::io::stdin().lines() {
        let line = line?;
        let line = line.trim();

        if let Some(line) = line.strip_prefix("Time:") {
            big_time = parse_number_ignore_spaces(line);
            times = line.split_whitespace()
                .map(|e| e.parse())
                .collect::<Result<_, _>>()?;
        } else if let Some(line) = line.strip_prefix("Distance:") {
            big_distance = parse_number_ignore_spaces(line);
            distances = line.split_whitespace()
                .map(|e| e.parse())
                .collect::<Result<_, _>>()?;
        }
    }

    if times.len() != distances.len() {
        bail!("must have same number of times & distances");
    }

    let mut total = 1;
    for (t, d) in times.iter().zip(&distances) {
        let (a, b) = solve(*t, *d);
        total *= (b - a) + 1;
    }

    let (big_min, big_max) = solve(big_time, big_distance);
    let big = (big_max - big_min) + 1;
    tracing::info!("total={total} big={big}");
    Ok(())
}
