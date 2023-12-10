use std::fmt::{Debug, Formatter, Write};
use std::ops::Deref;
use anyhow::{anyhow, bail};
use glam::IVec2;

pub struct AsciiGrid {
    contents: Vec<u8>,
    width: usize,
    height: usize,
}

impl AsciiGrid {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, at: IVec2) -> Option<u8> {
        if at.x >= 0 && (at.x as usize) < self.width && at.y >= 0 && (at.y as usize) < self.height {
            Some(self.contents[at.x as usize + at.y as usize * self.width])
        } else {
            None
        }
    }
}

impl Deref for AsciiGrid {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl Debug for AsciiGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() as i32 {
            for x in 0..self.width() as i32 {
                f.write_char(self.get(IVec2::new(x, y)).unwrap() as char)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl TryFrom<&str> for AsciiGrid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());
        let first_line = lines.next()
            .ok_or_else(|| anyhow!("empty string"))?;
        let width = first_line.len();
        let mut height = 1;
        let mut contents = Vec::with_capacity(width * lines.size_hint().0);
        contents.extend_from_slice(first_line.as_bytes());

        for line in lines {
            if line.len() != width {
                bail!("mismatched line length");
            }

            contents.extend_from_slice(line.as_bytes());
            height += 1;
        }

        contents.shrink_to_fit();
        Ok(AsciiGrid {
            contents,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Direction {
    Right = 0,
    RightDown = 1,
    Down = 2,
    LeftDown = 3,
    Left = 4,
    LeftUp = 5,
    Up = 6,
    RightUp = 7,
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        struct All(u8);

        impl Iterator for All {
            type Item = Direction;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0 >= 8 {
                    return None
                }

                let value = match self.0 {
                    0 => Direction::Right,
                    1 => Direction::RightDown,
                    2 => Direction::Down,
                    3 => Direction::LeftDown,
                    4 => Direction::Left,
                    5 => Direction::LeftUp,
                    6 => Direction::Up,
                    7 => Direction::RightUp,
                    _ => unreachable!(),
                };
                self.0 += 1;
                Some(value)
            }
        }

        All(0)
    }

    pub fn delta(self) -> IVec2 {
        match self {
            Direction::Right => IVec2::new(1, 0),
            Direction::RightDown => IVec2::new(1, 1),
            Direction::Down => IVec2::new(0, 1),
            Direction::LeftDown => IVec2::new(-1, 1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::LeftUp => IVec2::new(-1, -1),
            Direction::Up => IVec2::new(0, -1),
            Direction::RightUp => IVec2::new(1, -1),
        }
    }
}
