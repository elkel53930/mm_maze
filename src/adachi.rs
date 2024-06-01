use std::str::Chars;

use crate::maze::{Compass, Direction, Location, Maze, Position, Wall};
use crate::path_finder::PathFinder;
use log;

// Adachi method

#[derive(Clone, Copy, PartialEq)]
pub enum StepMapMode {
    UnexploredAsAbsent,  // Search
    UnexploredAsPresent, // Shortest path
}

pub struct Adachi {
    location: Location,
    maze: Maze,
    step_map: Vec<Vec<u16>>,
    mode: StepMapMode,
}

impl Adachi {
    const NONE: u16 = std::u16::MAX - 1;
    pub fn new(maze: Maze) -> Self {
        Adachi {
            location: Location {
                pos: Position { x: 0, y: 0 },
                dir: Compass::North,
            },
            maze: maze,
            step_map: vec![],
            mode: StepMapMode::UnexploredAsAbsent,
        }
    }

    pub fn set_mode(&mut self, mode: StepMapMode) {
        self.mode = mode;
    }

    pub fn get_goal(&self) -> Position {
        self.maze.get_goal()
    }

    pub fn calc_step_map(&mut self, goal: Position) {
        let mut no_cell_updated: bool;
        no_cell_updated = false;

        // step_mapのサイズとmazeのサイズが異なる場合はstep_mapを再確保
        if self.step_map.is_empty() {
            self.step_map = vec![vec![Adachi::NONE; self.maze.get_width()]; self.maze.get_height()];
        } else if self.step_map.len() != self.maze.get_height()
            && self.step_map[0].len() != self.maze.get_width()
        {
            self.step_map = vec![vec![Adachi::NONE; self.maze.get_width()]; self.maze.get_height()];
        }

        let is_wall = match self.mode {
            StepMapMode::UnexploredAsAbsent => {
                |wall| wall == Wall::Absent || wall == Wall::Unexplored
            }
            StepMapMode::UnexploredAsPresent => |wall| wall == Wall::Absent,
        };

        // Initialize step_map
        for v in self.step_map.iter_mut() {
            for x in v.iter_mut() {
                *x = Adachi::NONE;
            }
        }

        self.step_map[goal.y][goal.x] = 0;

        // calculate step_map
        while !no_cell_updated {
            no_cell_updated = true;
            for i in 0..self.maze.get_height() {
                // y
                for j in 0..self.maze.get_width() {
                    // x
                    for compass in Compass::iter() {
                        match self.maze.get_neighbor_cell(i, j, compass) {
                            Some((y, x)) => {
                                let neighbor = self.step_map[y][x];
                                let current = self.step_map[i][j];
                                if is_wall(self.maze.get(i, j, compass)) {
                                    if current > neighbor + 1 {
                                        self.step_map[i][j] = neighbor + 1;
                                        no_cell_updated = false;
                                    }
                                }
                            }
                            None => (),
                        }
                    }
                }
            }
        }
    }

    pub fn get_step(&self, x: usize, y: usize) -> u16 {
        self.step_map[y][x]
    }

    pub fn display_step_map(&self) -> String {
        let maze_text = self
            .maze
            .to_text_data("   ", "---", "???", " ", "|", "?", "+", "   ");
        let lines = maze_text.lines().collect::<Vec<&str>>();

        let mut result: Vec<String> = vec![];

        let mut index = 0;
        for i in (0..self.maze.get_height()).rev() {
            result.push(lines[index].to_string()); // horizontal wall
            index += 1;
            let chars = lines[index].to_string().chars().collect::<Vec<char>>(); // vertical wall
            index += 1;
            let mut vline = String::new();
            for j in 0..self.maze.get_width() {
                let step = self.step_map[i][j];
                let step_str = if step == Adachi::NONE {
                    "   ".to_string()
                } else {
                    format!("{:3}", step)
                };

                // lineにcharsのj*4文字目を追加
                vline.push(chars[j * 4]);
                // step_strを追加
                vline.push_str(&step_str);
            }
            vline.push_str("| "); // Outwall is always present
            vline.push_str(i.to_string().as_str()); // y-axis
            result.push(vline);
        }
        result.push(lines[0].to_string()); // bottom line
        let mut line = "".to_string();
        for i in 0..self.maze.get_width() {
            line.push_str(format!(" {:3}", i).as_str());
        }
        result.push(line); // x-axis

        result.join("\n")
    }
}

impl PathFinder for Adachi {
    fn navigate(
        &mut self,
        front: Wall,
        left: Wall,
        right: Wall,
        goal: Position,
    ) -> anyhow::Result<Direction> {
        if self.maze.get_goal() == self.location.pos {
            log::info!("Goal reached");
            return Err(anyhow::anyhow!("Goal reached"));
        }

        // Set wall info
        let cur_x = self.location.pos.x;
        let cur_y = self.location.pos.y;
        let cur_d = self.location.dir;
        self.maze
            .set(cur_y, cur_x, cur_d.turn(Direction::Forward), front);
        self.maze
            .set(cur_y, cur_x, cur_d.turn(Direction::Left), left);
        self.maze
            .set(cur_y, cur_x, cur_d.turn(Direction::Right), right);

        // Update step_map
        self.calc_step_map(goal);

        // 壁がなく、かつステップマップの値が一番小さい方向へ進む
        let mut min_step = std::u16::MAX;
        let mut result = None;

        if self.maze.get(cur_y, cur_x, Compass::North) == Wall::Absent {
            if self.step_map[cur_y + 1][cur_x] < min_step {
                min_step = self.step_map[cur_y + 1][cur_x];
                result = Some(Compass::North);
            }
        }
        if self.maze.get(cur_y, cur_x, Compass::East) == Wall::Absent {
            if self.step_map[cur_y][cur_x + 1] < min_step {
                min_step = self.step_map[cur_y][cur_x + 1];
                result = Some(Compass::East);
            }
        }
        if self.maze.get(cur_y, cur_x, Compass::South) == Wall::Absent {
            if self.step_map[cur_y - 1][cur_x] < min_step {
                min_step = self.step_map[cur_y - 1][cur_x];
                result = Some(Compass::South);
            }
        }
        if self.maze.get(cur_y, cur_x, Compass::West) == Wall::Absent {
            if self.step_map[cur_y][cur_x - 1] < min_step {
                min_step = self.step_map[cur_y][cur_x - 1];
                result = Some(Compass::West);
            }
        }

        if result.is_none() {
            log::error!("No path to go");
            return Err(anyhow::anyhow!("No path to go"));
        }

        let result = cur_d.get_direction_to(result.unwrap());

        log::info!(
            "{}, Wall:{}, Go:{}",
            self.location,
            Wall::make_wall_detection_log(left, front, right),
            result.to_log()
        );
        Ok(result)
    }

    fn get_location(&self) -> Location {
        self.location
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }
}
