use crate::maze;
use anyhow::Result;

pub trait PathFinder {
    fn navigate(
        &mut self,
        front: maze::Wall,
        left: maze::Wall,
        right: maze::Wall,
        goal: maze::Position,
    ) -> Result<maze::Direction>;
    fn get_location(&self) -> maze::Location;
    fn set_location(&mut self, location: maze::Location);
}
