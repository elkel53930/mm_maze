use serde::{Deserialize, Serialize};

/*
    Coordinate system:
    (0,0) is the bottom left corner
    x increases to the right (east)
    y increases upwards (north)
    The robot starts at (0,0) facing north

    Horizontal walls are blocks between (x,y) and (x,y+1)
    Vertical walls are blocks between (x,y) and (x+1,y)

    Vertical walls:
       |     North
     4 +---+---+---+---+
       |               |
 Y   3 +   +   +   +   +
 ^     |               |
West 2 +   +   +   +   + East
       |               |
     1 +   +   +   +   +
       |               |
     0 +---+---+---+---+---Horizontal walls
       0   1   2   3   4
             South >X
*/

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Wall {
    Absent,
    Present,
    Unexplored,
}

impl Wall {
    pub fn make_wall_detection_log(left: Wall, front: Wall, right: Wall) -> String {
        let mut s = String::new();
        s += match left {
            Wall::Absent => " ",
            Wall::Present => "|",
            Wall::Unexplored => "?",
        };
        s += match front {
            Wall::Absent => " ",
            Wall::Present => "-",
            Wall::Unexplored => "?",
        };
        s += match right {
            Wall::Absent => " ",
            Wall::Present => "|",
            Wall::Unexplored => "?",
        };
        s
    }

    pub fn from_bool(b: bool) -> Wall{
        if b {Wall::Present} else {Wall::Absent}
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Forward,
    Left,
    Right,
    Backward,
}

impl Direction {
    pub fn to_log(&self) -> &str {
        match self {
            Direction::Forward => "F^",
            Direction::Left => "L<",
            Direction::Right => "R>",
            Direction::Backward => "Bv",
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Forward,
            Direction::Left,
            Direction::Right,
            Direction::Backward,
        ]
        .iter()
        .copied()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Compass {
    North,
    East,
    South,
    West,
}

impl Compass {
    pub fn turn(&self, direction: Direction) -> Compass {
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

    pub fn to_log(&self) -> &str {
        match self {
            Compass::North => "N",
            Compass::East => "E",
            Compass::South => "S",
            Compass::West => "W",
        }
    }

    // Return the Direction to face the given compass from the current compass
    pub fn get_direction_to(&self, target: Compass) -> Direction {
        match (self, target) {
            (Compass::North, Compass::North) => Direction::Forward,
            (Compass::North, Compass::East) => Direction::Right,
            (Compass::North, Compass::South) => Direction::Backward,
            (Compass::North, Compass::West) => Direction::Left,
            (Compass::East, Compass::North) => Direction::Left,
            (Compass::East, Compass::East) => Direction::Forward,
            (Compass::East, Compass::South) => Direction::Right,
            (Compass::East, Compass::West) => Direction::Backward,
            (Compass::South, Compass::North) => Direction::Backward,
            (Compass::South, Compass::East) => Direction::Left,
            (Compass::South, Compass::South) => Direction::Forward,
            (Compass::South, Compass::West) => Direction::Right,
            (Compass::West, Compass::North) => Direction::Right,
            (Compass::West, Compass::East) => Direction::Backward,
            (Compass::West, Compass::South) => Direction::Left,
            (Compass::West, Compass::West) => Direction::Forward,
        }
    }

    pub fn iter() -> impl Iterator<Item = Compass> {
        [Compass::North, Compass::East, Compass::South, Compass::West]
            .iter()
            .copied()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub pos: Position,
    pub dir: Compass,
}

impl Location {
    pub fn new() -> Self {
        Location {
            pos: Position { x: 0, y: 0 },
            dir: Compass::North,
        }
    }

    pub fn turn(&mut self, dir: Direction) {
        self.dir = self.dir.turn(dir);
    }

    pub fn forward(&mut self) {
        match self.dir {
            Compass::North => self.pos.y += 1,
            Compass::East => self.pos.x += 1,
            Compass::South => self.pos.y -= 1,
            Compass::West => self.pos.x -= 1,
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Y:{:2}, X:{:2}, Dir:", self.pos.y, self.pos.x)?;
        match self.dir {
            Compass::North => write!(f, "N"),
            Compass::East => write!(f, "E"),
            Compass::South => write!(f, "S"),
            Compass::West => write!(f, "W"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Maze {
    width: usize,
    height: usize,
    horizontal_walls: Vec<Vec<Wall>>,
    vertical_walls: Vec<Vec<Wall>>,
    goal: Position,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let mut maze = Maze {
            width,
            height,
            horizontal_walls: vec![vec![Wall::Unexplored; width]; height + 1],
            vertical_walls: vec![vec![Wall::Unexplored; width + 1]; height],
            goal: Position { x: 0, y: 0 },
        };
        maze.init();
        maze
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
        self.goal = Position {
            x: self.width / 2,
            y: self.height / 2,
        };
    }

    pub fn get(&self, y: usize, x: usize, compass: Compass) -> Wall {
        match compass {
            Compass::North => self.horizontal_walls[y + 1][x],
            Compass::East => self.vertical_walls[y][x + 1],
            Compass::South => self.horizontal_walls[y][x],
            Compass::West => self.vertical_walls[y][x],
        }
    }

    pub fn set(&mut self, y: usize, x: usize, compass: Compass, wall: Wall) {
        // Check outer walls
        if (y == 0 && compass == Compass::South && wall != Wall::Present)
            || (y == self.height && compass == Compass::North && wall != Wall::Present)
            || (x == 0 && compass == Compass::West && wall != Wall::Present)
            || (x == self.width && compass == Compass::East && wall != Wall::Present)
        {
            // Cannot remove the outer wall
            log::warn!(
                "Cannot remove the outer wall. Operation is ignored. Y: {}, X: {}, compass: {:?}",
                y,
                x,
                compass
            );
            return;
        }

        match compass {
            Compass::North => self.horizontal_walls[y + 1][x] = wall,
            Compass::East => self.vertical_walls[y][x + 1] = wall,
            Compass::South => self.horizontal_walls[y][x] = wall,
            Compass::West => self.vertical_walls[y][x] = wall,
        }
    }

    pub fn get_goal(&self) -> Position {
        self.goal
    }

    pub fn set_goal(&mut self, pos: Position) {
        self.goal = pos;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
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
                    self.goal = Position { x, y };
                }
            }
        }
        Ok(())
    }

    pub fn write_maze_file(&self, filename: &str) -> Result<(), String> {
        let contents = self.to_text_data(" ", "-", " ", " ", "|", " ", "+", "G");
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
        for i in 0..self.height {
            // y
            for j in 0..self.width {
                // x
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
                if j == self.goal.x && i == self.goal.y {
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

    /*
       This function returns the coordinates of the cell that is adjacent to the cell at (x, y)
       When the the cell is at the edge of the maze, None is returned
    */
    pub fn get_neighbor_cell(
        &self,
        y: usize,
        x: usize,
        compass: Compass,
    ) -> Option<(usize, usize)> {
        match compass {
            Compass::North => {
                if y == self.height - 1 {
                    None
                } else {
                    Some((y + 1, x))
                }
            }
            Compass::East => {
                if x == self.width - 1 {
                    None
                } else {
                    Some((y, x + 1))
                }
            }
            Compass::South => {
                if y == 0 {
                    None
                } else {
                    Some((y - 1, x))
                }
            }
            Compass::West => {
                if x == 0 {
                    None
                } else {
                    Some((y, x - 1))
                }
            }
        }
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

impl Default for Maze{
    fn default() -> Self {
        let width = 16;
        let height = 16;
        let mut maze = Maze {
        width,
        height,
        horizontal_walls: vec![vec![Wall::Unexplored; width]; height + 1],
        vertical_walls: vec![vec![Wall::Unexplored; width + 1]; height],
        goal: Position { x: 0, y: 0 },
    };
    maze.init();
    maze
    }
}
