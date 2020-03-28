use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use Direction::*;

#[derive(Copy, Clone)]
struct Table {
    width: usize,
    height: usize,
}

impl Table {
    fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

#[cfg(test)]
mod test_table {
    use super::*;

    #[test]
    fn valid() {
        let t = Table {
            width: 1,
            height: 1,
        };
        assert_eq!(t.is_valid(0, 0), true);
        assert_eq!(t.is_valid(1, 0), false);
        assert_eq!(t.is_valid(0, 1), false);
    }
}

#[derive(Copy, Clone)]
struct Robot {
    x: usize,
    y: usize,
    d: Direction,
}

impl Robot {
    fn rotate(&mut self, delta: i32) {
        self.d = self.d.rotate(delta)
    }

    fn r#move(&mut self) {
        match self.d {
            North => self.y += 1,
            East => self.x += 1,
            South => self.y -= 1,
            West => self.x -= 1,
        }
    }
}

#[derive(Debug, EnumIter, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(self, delta: i32) -> Direction {
        let it = Direction::iter();
        let count = it.len() as i32;
        let index = ((self as i32 + delta + count) % count) as usize;
        it.skip(index).next().unwrap()
    }
}

#[cfg(test)]
mod test_direction {
    use super::Direction::*;

    #[test]
    fn rotation() {
        assert_eq!(North.rotate(1), East);
        assert_eq!(East.rotate(1), South);
        assert_eq!(South.rotate(1), West);
        assert_eq!(West.rotate(1), North);

        assert_eq!(North.rotate(-1), West);
        assert_eq!(East.rotate(-1), North);
        assert_eq!(South.rotate(-1), East);
        assert_eq!(West.rotate(-1), South);

        assert_eq!(North.rotate(4), North);
        assert_eq!(North.rotate(-4), North);
    }
}

struct Game {
    robot: Option<Robot>,
    table: Table,
}

impl Game {
    fn place(&mut self, x: usize, y: usize, d: Direction) -> Result<(), String> {
        match self.table.is_valid(x, y) {
            true => {
                self.robot = Some(Robot { x, y, d });
                Ok(())
            }
            false => Err(format!(
                "Position ({}, {}) is invalid for a {} x {} table",
                x, y, self.table.width, self.table.height
            )),
        }
    }

    fn map_robot<U, F: FnOnce(&mut Robot) -> U>(&mut self, f: F) -> Result<U, String> {
        self.robot
            .as_mut()
            .map(f)
            .ok_or("Robot has not been placed on table".to_string())
    }

    fn r#move(&mut self) -> Result<(), String> {
        self.map_robot(|r| r.r#move())
    }

    fn rotate_right(&mut self) -> Result<(), String> {
        self.rotate(1)
    }

    fn rotate_left(&mut self) -> Result<(), String> {
        self.rotate(-1)
    }

    fn rotate(&mut self, delta: i32) -> Result<(), String> {
        self.map_robot(|r| r.rotate(delta))
    }

    fn report(&mut self) -> Result<String, String> {
        self.map_robot(|r| format!("{}, {}, {:?}", r.x, r.y, r.d))
    }
}

fn main() {
    println!("{}", 3);
}
