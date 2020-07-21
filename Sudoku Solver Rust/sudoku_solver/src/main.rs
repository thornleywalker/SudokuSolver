use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let mut curr_board = Board::from_list(String::from("testString"));
    
    curr_board.solve();
    curr_board.to_string();
}

struct Board
{
    boxes: Vec<Vec<Box>>
}
impl Board
{
    //newialize a 3x3 array of Boxes
    fn new() -> Board
    {
        Board{boxes:vec![vec![Box::new(), Box::new(), Box::new()],
                         vec![Box::new(), Box::new(), Box::new()],
                         vec![Box::new(), Box::new(), Box::new()]]}
    }
    //create a board from a string of numbers
    // FIXME
    fn from_list(in_string: String) -> Board
    {
        let return_board = Board::new();

        return_board
    }
    //returns a mutable reference to the Square at row, col
    fn at(&mut self, row:usize, col:usize) -> &mut Square
    {
        let box_row = row % 3;
        let box_col = col % 3;

        let square_row = row / 3;
        let square_col = col / 3;

        self.boxes[box_row][box_col].at(square_row, square_col)
    }
    //changes the Square at row, col to the val,
    //removes val from all Squares in row, column, and Box
    fn set(&mut self, row:usize, col:usize, val:i32)
    {
        self.at(row, col).set(val);
        //remove val from possibilities in the same column
        for _row in 0..9
        {
            self.at(_row, col).remove(val);
        }
        //remove val from possibilities in the same row
        for _col in 0..9
        {
            self.at(row, _col).remove(val);
        }
        //remove val from possibilities in the same Box
        let box_row = row % 3;
        let box_col = col % 3;
        self.boxes[box_row][box_col].remove(val);
    }
    //checks the given square for the following
    // - it only has one possibility
    // - for each possibility in the Square:
    //      - it is the only one of that possibility in the row
    //      - it is the only one of that possibility in the column
    //      - it is the only one of that possibility in the Box
    //if something changes, returns true,
    //otherwise returns false
    fn check_square(&mut self, row:usize, col:usize) -> bool
    {
        let mut changed = false;
        match self.at(row, col)
        {
            Square::Possibilities(values) =>
            {
                //check for a single possibility
                if values.len() == 1
                {
                    for val in values.iter()
                    {
                        //self.set(row, col, *val);
                        
                    }
                    
                    changed = true;
                    return changed
                };
                //check each possibility for uniqueness in the row
                for poss in values.iter()
                {

                }
                //check each possibility for uniqueness in the column

                //check each possibility for uniqueness in the Box
            },
            Square::Value(_val) => {}
        }
        changed
    }
    //performs a deep check:
    // - if a possibility can only be in one row/column of a Box,
    //   removes that possibility from that row/column in adjacent Boxes
    //returns true if board changed, false otherwise
    fn deep_check(&mut self) -> bool
    {
        let mut changed = false;

        changed
    }
    fn solve_check(&mut self) -> bool
    {
        let solved = false;
        solved
    }
    //main solve function
    fn solve(&mut self) -> bool
    {
        let mut solved = false;
        let mut board_changed = true;
        while board_changed
        {
            //solve check
            if self.solve_check(){
                solved = true;
                return solved;
            }
            board_changed = false;
            //basic solving
            for row in 0..9{
                for col in 0..9{
                    if self.check_square(row, col){
                        board_changed = true;
                    }
                }
            }
            //deep check
            if !board_changed{
                if self.deep_check(){
                    board_changed = true;
                }
            }
            //last resort: brute force recursion
        }
        println!("Solving Sudoku Board");
        solved
    }
    //prints possibilities/values for every square
    fn to_string(&mut self)
    {
        for row in 0..9
        {
            for col in 0..9
            {
                println!("({},{}): {}", row+1, col+1, self.at(row, col).to_string());
            }
        }
    }

}

struct Box
{
    squares: Vec<Vec<Square>>
}
impl Box
{
    //newializes a 3x3 array of Squares with all possibilities in all Squares
    fn new() -> Box
    {
        Box{squares:vec![vec![Square::new(),Square::new(),Square::new(),],
                         vec![Square::new(),Square::new(),Square::new(),],
                         vec![Square::new(),Square::new(),Square::new(),]]}
    }
    //returns a mutable reference to the Square at row, col
    fn at(&mut self, row:usize, col:usize) -> &mut Square
    {
        &mut self.squares[row][col]
    }
    //removes val from possibilities of each Square
    fn remove(&mut self, val:i32)
    {
        for square_row in &mut self.squares
        {
            for square in square_row
            {
                square.remove(val);
            }
        }
    }
}

enum Square
{
    Possibilities(HashSet<i32>),
    Value(i32)
}
impl Square
{
    //creates a Square with all possibilities
    fn new() -> Square
    {
        let new_set: HashSet<i32> = 
            [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();
        Square::Possibilities(new_set)
    }
    //sets Square to Value with val
    fn set(&mut self, val:i32)
    {
        *self = Square::Value(val);
    }
    //returns true if val is in set of possibilities, false if not
    //returns false if Square is Value
    fn contains(&self, val:i32) -> bool
    {
        match self
        {
            Square::Possibilities(values) => values.contains(&val),
            Square::Value(_val) => false
        }
    }
    //removes val from set of possibilities
    //returns true if value was removes, false if not present
    //returns false if Square is Value
    fn remove(&mut self, val:i32) -> bool
    {
        match self
        {
            Square::Possibilities(values) => {values.remove(&val)},
            Square::Value(_val) => false
        }
    }
    //checks if Square has a single possibility, changes to Value if true
    //returns true if only one possibility
    //returns false if Square is Value
    fn single_possibility_check(&mut self) -> bool
    {
        match self
        {
            Square::Possibilities(values) => 
            {
                if values.len() == 1
                {
                    let mut new_val = 0;
                    for val in values.drain()
                    {
                        new_val = val;
                    }
                    self.set(new_val);
                    true
                }
                else {false}
            }
            Square::Value(_val) => false
        }
    }
    fn to_string(&self) -> String
    {
        let mut return_string = String::new();
        match self
        {
            Square::Possibilities(values) => 
            {
                for val in values { return_string += &format!("{}, ", val); }
            },
            Square::Value(val) =>
            {
                return_string += &format!("{}", val)
            }
        }
        return_string
    }
}