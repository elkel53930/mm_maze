mod maze;

#[cfg(test)]
mod tests {
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
}
