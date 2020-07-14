fn main() {
    println!("Hello, world!");
    let curr_board = Board::init();
    curr_board.solve();
}

struct Board
{
    boxes: Vec<Vec<Box>>
}
impl Board
{
    fn init() -> Board
    {
        Board{boxes:vec![vec![Box::init(), Box::init(), Box::init()],
                         vec![Box::init(), Box::init(), Box::init()],
                         vec![Box::init(), Box::init(), Box::init()]]}
    }
    fn fromList() -> Board
    {
        let return_board = Board::init();

        return_board
    }
    fn at(&self, row:usize, col:usize) -> Square
    {
        let box_row = row % 3;
        let box_col = col % 3;

        let square_row = row / 3;
        let square_col = col / 3;

        self.boxes[box_row][box_col].at(square_row, square_col)
    }
    fn set(&self, row:usize, col:usize, val:i32)
    {
        let box_row = row % 3;
        let box_col = col % 3;

        let square_row = row / 3;
        let square_col = col / 3;

        self.boxes[box_row][box_col].set(square_row, square_col, val);
    }
    fn solve(&self)
    {
        println!("Solving Sudoku Board");
    }
}

struct Box
{
    squares: Vec<Vec<Square>>
}
impl Box
{
    fn init() -> Box
    {
        Box{squares:vec![vec![Square::init(),Square::init(),Square::init(),],
                         vec![Square::init(),Square::init(),Square::init(),],
                         vec![Square::init(),Square::init(),Square::init(),]]}
    }
    fn at(&self, row:usize, col:usize) -> Square
    {
        self.squares[row][col]
    }
    fn set(&self, row:usize, col:usize, val:i32)
    {
        self.squares[row][col] = Square::Value(val);
    }
}

enum Square
{
    Possibilities(Vec<i32>),// = vec![1,2,3,4,5,6,7,8,9],
    Value(i32)
}
impl Square
{
    fn init() -> Square
    {
        Square::Possibilities(vec![1,2,3,4,5,6,7,8,9])
    }
}