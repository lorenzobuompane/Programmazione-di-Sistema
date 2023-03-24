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
        let mut robot = self;
        match robot.direction() {
            Direction::North => robot = Robot::new(robot.x, robot.y, Direction::East),
            Direction::South => robot = Robot::new(robot.x, robot.y, Direction::West),
            Direction::East => robot = Robot::new(robot.x, robot.y, Direction::South),
            Direction::West => robot = Robot::new(robot.x, robot.y, Direction::North),
        };
        robot
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let mut robot = self;
        match robot.direction() {
            Direction::North => robot = Robot::new(robot.x, robot.y, Direction::West),
            Direction::South => robot = Robot::new(robot.x, robot.y, Direction::East),
            Direction::East => robot = Robot::new(robot.x, robot.y, Direction::North),
            Direction::West => robot = Robot::new(robot.x, robot.y, Direction::South),
        };
        robot
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut robot = self;
        match robot.direction() {
            Direction::North => robot = Robot::new(robot.x, robot.y + 1, Direction::North),
            Direction::South => robot = Robot::new(robot.x, robot.y - 1, Direction::South),
            Direction::East => robot = Robot::new(robot.x + 1, robot.y, Direction::East),
            Direction::West => robot = Robot::new(robot.x - 1, robot.y, Direction::West),
        };
        robot
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
