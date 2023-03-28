// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::process::exit;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value%4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!()
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Robot {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let d=self.d as usize;
        Robot { d: Direction::from( d+ 1 ), ..self }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let d=self.d as usize;
        Robot { d: Direction::from(d + 3 ), ..self }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction() {
            Direction::North => Robot { y: self.y + 1, ..self },
            Direction::East => Robot { x: self.x + 1, ..self },
            Direction::South => Robot { y: self.y - 1, ..self },
            Direction::West => Robot { x: self.x - 1, ..self },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for i in instructions.chars() {
            match i {
                'R' => robot = robot.turn_right(),
                'L' => robot = robot.turn_left(),
                'A' => robot = robot.advance(),
                _ => {
                    println!("INSTRUCTION NOT VALID: {}", i);
                    exit(1);
                }
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
