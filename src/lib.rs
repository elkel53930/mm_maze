pub mod adachi;
pub mod maze;
pub mod path_finder;

#[cfg(test)]
mod tests {
    use path_finder::PathFinder;

    use super::*;

    #[test]
    fn it_works() {
        let mut maze = maze::Maze::new(16, 16);
        maze.init();
        println!("{}", maze);
    }

    #[test]
    fn read() {
        let mut maze = maze::Maze::new(16, 16);
        maze.init();
        match maze.read_maze_file(
            "maze_data/AllJapan_032_2011_classic_exp_fin_16x16.txt",
            16,
            16,
        ) {
            Ok(_) => {
                println!("Read maze file successfully\n{}", maze);
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }

    #[test]
    fn solve() {
        let mut actual_maze = maze::Maze::new(16, 16);
        actual_maze.init();
        match actual_maze.read_maze_file(
            "maze_data/AllJapan_032_2011_classic_exp_fin_16x16.txt",
            16,
            16,
        ) {
            Ok(_) => {
                println!("Read maze file successfully\n{}", actual_maze);
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }

        let maze_text = actual_maze.to_text_data("   ", "---", "???", " ", "|", " ", "+", "   ");
        println!("{}", maze_text);

        let mut solver = adachi::Adachi::new(maze::Maze::new(16, 16));

        let mut limit = 0;

        loop {
            let x = solver.get_location().pos.x;
            let y = solver.get_location().pos.y;
            let d = solver.get_location().dir;

            let front = actual_maze.get(y, x, d.turn(maze::Direction::Forward));
            let left = actual_maze.get(y, x, d.turn(maze::Direction::Left));
            let right = actual_maze.get(y, x, d.turn(maze::Direction::Right));

            let dir = solver.navigate(front, left, right, solver.get_goal());
            assert!(dir.is_ok());

            // println!("{}", solver.display_step_map());

            // Move to the next location
            let dir = dir.unwrap();

            if actual_maze.get(y, x, d.turn(dir)) == maze::Wall::Present {
                println!("Error: Wall is present at {:?}", d.turn(dir));
                println!("Loc:{} Go:{}", solver.get_location(), dir.to_log());
                assert!(false);
            }
            let mut loc = solver.get_location();
            loc.dir = loc.dir.turn(dir);
            loc.forward();

            // Display the current location
            println!(
                "{} {} {}",
                maze::Wall::make_wall_detection_log(left, front, right),
                dir.to_log(),
                loc
            );
            solver.set_location(loc);

            limit += 1;
            if limit > 1000 {
                println!("Limit reached");
                assert!(false);
            }

            // Check if the goal is reached
            if loc.pos == solver.get_goal() {
                println!("Goal reached");
                break;
            }
        }
    }
}
