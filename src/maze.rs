use std::alloc::GlobalAlloc;

use serde::{Deserialize, Serialize};
use tenji_draw::tenji_draw;

/*
    Coordinate system:
    (0,0) is the bottom left corner
    x increases to the right (east)
    y increases upwards (north)
    The robot starts at (0,0) facing north

    Horizontal walls are blocks between (x,y) and (x,y+1)
    Vertical walls are blocks between (x,y) and (x+1,y)
*/

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Wall {
    Absent,
    Present,
    Unexplored,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Forward,
    Left,
    Right,
    Backward,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Compass {
    North,
    East,
    South,
    West,
}

impl Compass {
    fn turn(&self, direction: Direction) -> Compass {
        match (self, direction) {
            (Compass::North, Direction::Forward) => Compass::North,
            (Compass::North, Direction::Left) => Compass::West,
            (Compass::North, Direction::Right) => Compass::East,
            (Compass::North, Direction::Backward) => Compass::South,
            (Compass::East, Direction::Forward) => Compass::East,
            (Compass::East, Direction::Left) => Compass::North,
            (Compass::East, Direction::Right) => Compass::South,
            (Compass::East, Direction::Backward) => Compass::West,
            (Compass::South, Direction::Forward) => Compass::South,
            (Compass::South, Direction::Left) => Compass::East,
            (Compass::South, Direction::Right) => Compass::West,
            (Compass::South, Direction::Backward) => Compass::North,
            (Compass::West, Direction::Forward) => Compass::West,
            (Compass::West, Direction::Left) => Compass::South,
            (Compass::West, Direction::Right) => Compass::North,
            (Compass::West, Direction::Backward) => Compass::East,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Maze {
    width: usize,
    height: usize,
    horizontal_walls: Vec<Vec<Wall>>,
    vertical_walls: Vec<Vec<Wall>>,
    goal: (usize, usize),
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Maze {
            width,
            height,
            horizontal_walls: vec![vec![Wall::Unexplored; width]; height + 1],
            vertical_walls: vec![vec![Wall::Unexplored; width + 1]; height],
            goal: (0, 0),
        }
    }

    pub fn init(&mut self) {
        // Set all walls to unexplored
        for y in 0..self.height + 1 {
            for x in 0..self.width {
                self.horizontal_walls[y][x] = Wall::Unexplored;
            }
        }
        for y in 0..self.height {
            for x in 0..self.width + 1 {
                self.vertical_walls[y][x] = Wall::Unexplored;
            }
        }

        // Set the outer walls to present
        for x in 0..self.width {
            self.horizontal_walls[0][x] = Wall::Present;
            self.horizontal_walls[self.height][x] = Wall::Present;
        }
        for y in 0..self.height {
            self.vertical_walls[y][0] = Wall::Present;
            self.vertical_walls[y][self.width] = Wall::Present;
        }

        // Set the right wall of the start cell to present
        self.set(0, 0, Compass::North.turn(Direction::Right), Wall::Present);

        // Set the goal
        self.goal = (7, 7);
    }

    pub fn get(&self, x: usize, y: usize, compass: Compass) -> Wall {
        match compass {
            Compass::North => self.horizontal_walls[y][x],
            Compass::East => self.vertical_walls[y][x + 1],
            Compass::South => self.horizontal_walls[y + 1][x],
            Compass::West => self.vertical_walls[y][x],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, compass: Compass, wall: Wall) {
        match compass {
            Compass::North => self.horizontal_walls[y][x] = wall,
            Compass::East => self.vertical_walls[y][x + 1] = wall,
            Compass::South => self.horizontal_walls[y + 1][x] = wall,
            Compass::West => self.vertical_walls[y][x] = wall,
        }
    }

    pub fn get_goal(&self) -> (usize, usize) {
        self.goal
    }

    pub fn set_goal(&mut self, x: usize, y: usize) {
        self.goal = (x, y);
    }

    /*
    maze file example
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    |             | | | |           |
    +-+ + +-+-+-+-+ + + + +-+-+-+ + +
    |   |     | |                 | |
    + + +-+ + + + + + + + + +-+-+-+ +
    | |   | | |   | | | | | |     | |
    + + + +-+-+ +-+-+-+ +-+-+ +-+ + +
    | | |   |         | |     |   | |
    + +-+-+ + +-+-+-+ + + +-+ + + + +
    |                 |   | | | | | |
    + +-+-+-+-+ +-+-+-+-+-+ +-+ + + +
    |     |     |       |       | | |
    + +-+-+ +-+-+ +-+ + +-+-+-+-+-+ +
    |         |       |   |         |
    + +-+ +-+-+ + +-+-+ + + +-+ +-+ +
    |   |     | |  G  | | |         |
    + + +-+-+-+ +-+ + + + + + +-+ + +
    | |       |   |   | | | |     | |
    + +-+-+-+ +-+ +-+-+ + + +-+ +-+ +
    |     |   |     |     |   |   | |
    + +-+-+ +-+-+-+ +-+-+ +-+ +-+ + +
    | |   |     |     |     |     | |
    + + + +-+-+ +-+-+ + +-+ +-+ +-+ +
    |   |         |             |   |
    + +-+-+ +-+-+ +-+-+ +-+-+ + +-+ +
    |   |     |     |         | |   |
    + + + +-+ +-+-+ +-+-+ +-+ + +-+ +
    | |   |   | |   | | |   |       |
    + + +-+-+ + + +-+ + +-+ +-+ +-+ +
    |   | | | | |   |     |         |
    + + + + + + +-+ + +-+ +-+-+-+-+ +
    | |               | |           |
    +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    "-" and "|" measn wall is present
    " "  means wall is absent
    "G" means goal
      + means pillar
    */
    pub fn read_maze_file(
        &mut self,
        filename: &str,
        width: usize,
        height: usize,
    ) -> Result<(), String> {
        let contents = match std::fs::read_to_string(filename) {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };
        // Split the contents into lines and store them in Vec<String>
        let lines: Vec<&str> = contents.lines().collect();
        // Reverse the lines
        let lines: Vec<&str> = lines.iter().rev().map(|l| *l).collect();
        // Remove "+"
        let lines: Vec<String> = lines.iter().map(|l| l.replace("+", "")).collect();
        // Convert " " to Wall::Absent and "-" to Wall::Present
        for y in 0..height {
            // Horizontal walls
            for x in 0..width {
                let c = lines[y * 2].chars().nth(x).unwrap();
                self.horizontal_walls[y][x] = match c {
                    ' ' => Wall::Absent,
                    '-' => Wall::Present,
                    _ => Wall::Unexplored,
                };
            }
            // Vertical walls (two characters per wall)
            for x in 0..width {
                let c = lines[y * 2 + 1].chars().nth(x * 2).unwrap();
                self.vertical_walls[y][x] = match c {
                    ' ' => Wall::Absent,
                    '|' => Wall::Present,
                    _ => Wall::Unexplored,
                };

                // Goal location
                let c = lines[y * 2 + 1].chars().nth(x * 2 + 1).unwrap();
                if c == 'G' {
                    self.goal = (x, y);
                }
            }
        }
        Ok(())
    }

    pub fn write_maze_file(&self, filename: &str) -> Result<(), String> {
        let mut contents = self.to_text_data(" ", "-", " ", " ", "|", " ", "+", "G");
        match std::fs::write(filename, contents) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn to_text_data(
        &self,
        horizontal_wall_absent: &str,
        horizontal_wall_present: &str,
        horizontal_wall_unexplored: &str,
        vertical_wall_absent: &str,
        vertical_wall_present: &str,
        vertical_wall_unexplored: &str,
        pillar: &str,
        goal: &str,
    ) -> String {
        let mut lines: Vec<String> = Vec::new();
        let mut line = "".to_string();
        let (gx, gy) = self.goal;
        for i in 0..self.height {
            for j in 0..self.width {
                line += &pillar;
                line += match self.horizontal_walls[i][j] {
                    Wall::Absent => &horizontal_wall_absent,
                    Wall::Present => &horizontal_wall_present,
                    Wall::Unexplored => &horizontal_wall_unexplored,
                };
            }
            line += "+";
            lines.push(line);
            line = "".to_string();
            for j in 0..self.width + 1 {
                line += match self.vertical_walls[i][j] {
                    Wall::Absent => &vertical_wall_absent,
                    Wall::Present => &vertical_wall_present,
                    Wall::Unexplored => &vertical_wall_unexplored,
                };
                if j == gx && i == gy {
                    line += &goal;
                } else {
                    // goalと同じ長さになるように空白を追加
                    line += " ".repeat(goal.len()).as_str();
                }
            }
            lines.push(line);
            line = "".to_string();
        }
        for j in 0..self.width {
            line += &pillar;
            line += match self.horizontal_walls[self.height][j] {
                Wall::Absent => &horizontal_wall_absent,
                Wall::Present => &horizontal_wall_present,
                Wall::Unexplored => &horizontal_wall_unexplored,
            };
        }
        line += &pillar;
        lines.push(line);
        // join reversed lines
        lines
            .iter()
            .rev()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.to_text_data("  ", "--", "  ", " ", "|", " ", "+", "GL")
        )?;
        Ok(())
    }
}
