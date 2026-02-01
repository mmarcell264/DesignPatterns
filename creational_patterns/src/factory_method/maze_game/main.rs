use super::magic_maze::MagicMaze;
use super::ordinary_maze::OrdinaryMaze;

/// The game runs with different mazes depending on the concrete factory type:
/// it's either an ordinary maze or a magic maze.
///
/// For demonstration purposes, both mazes are used to construct the game.
pub fn maze_main() {
    // Option 1: The game starts with an ordinary maze.
    let ordinary_maze = OrdinaryMaze::new();
    super::game::run(ordinary_maze);

    // Option 2: The game starts with a magic maze.
    let magic_maze = MagicMaze::new();
    super::game::run(magic_maze);
}