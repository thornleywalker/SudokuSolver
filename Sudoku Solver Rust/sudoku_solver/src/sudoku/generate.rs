use {
    crate::sudoku, 
    std::collections::HashMap
};

pub enum Difficulty{
    Easy,
    Medium,
    Hard,
    Expert
}
pub fn generate_board(difficulty: Difficulty) -> sudoku::Board
{
    let mut in_string = String::new();

    let mut return_board = sudoku::Board::from_list(in_string);

    return_board
}