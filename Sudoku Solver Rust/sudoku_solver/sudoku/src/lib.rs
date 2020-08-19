use std::{
    char, 
    collections::{HashSet, HashMap}, 
    thread, 
    sync::{Arc, Mutex, mpsc}
};
pub mod tests;

const DEBUG_PRINTOUT: bool = false;

pub struct Board
{
    Blocks: Vec<Vec<Block>>
}
impl Board
{
    //initialize a 3x3 array of Blockes
    pub fn new() -> Board
    {
        Board{Blocks:vec![vec![Block::new(), Block::new(), Block::new()],
                        vec![Block::new(), Block::new(), Block::new()],
                        vec![Block::new(), Block::new(), Block::new()]]}
    }
    //create a board from a string of numbers
    pub fn from_list(in_string: String) -> Board
    {
        let mut return_board = Board::new();
        let mut row = 0;
        let mut col = 0;
        for curr_char in in_string.chars()
        {
            if curr_char == ','
            {
                row += 1;
                col = 0;
            }
            else
            {
                let val = curr_char.to_digit(16);
                match val
                {
                    Some(num) => if num != 0 { return_board.set(row, col, num as i32) },
                    None => {}
                }
                col += 1;
            }
        }
        return_board
    }
    //create a copy of an existing board
    fn copy_from(to_copy: &mut Board) -> Board
    {
        let mut return_board = Board::new();
        
        for row in 0..9 {
            for col in 0..9 {
                let curr_square = to_copy.at(row, col);
                match curr_square{
                    Square::Value(val) => return_board.set(row, col, *val),
                    Square::Possibilities(_) => {}
                }
            }
        }

        return_board
    }

    //returns a mutable reference to the Square at row, col
    fn at_mut(&mut self, row:usize, col:usize) -> &mut Square
    {
        let block_row = row / 3;
        let block_col = col / 3;

        let square_row = row % 3;
        let square_col = col % 3;

        self.Blocks[block_row][block_col].at_mut(square_row, square_col)
    }
    fn at(&self, row:usize, col:usize) -> &Square
    {
        let block_row = row / 3;
        let block_col = col / 3;

        let square_row = row % 3;
        let square_col = col % 3;

        self.Blocks[block_row][block_col].at(square_row, square_col)
    }
    //changes the Square at row, col to the val,
    //removes val from all Squares in row, column, and Block
    fn set(&mut self, row:usize, col:usize, val:i32)
    {
        if DEBUG_PRINTOUT { println!["setting {},{} to {}", row+1, col+1, val];}
        self.at_mut(row, col).set(val);
        //remove val from possibilities in the same column
        for _row in 0..9
        {
            self.at_mut(_row, col).remove(val);
        }
        //remove val from possibilities in the same row
        for _col in 0..9
        {
            self.at_mut(row, _col).remove(val);
        }
        //remove val from possibilities in the same Block
        let block_row = row / 3;
        let block_col = col / 3;
        self.Blocks[block_row][block_col].remove(val);
    }
    //checks the given square for the following
    // - it only has one possibility
    // - for each possibility in the Square:
    //      - it is the only one of that possibility in the row
    //      - it is the only one of that possibility in the column
    //      - it is the only one of that possibility in the Block
    //if something changes, returns true,
    //otherwise returns false
    fn check_square(&mut self, row:usize, col:usize) -> bool
    {
        let mut changed = false;
        // let mut board = Arc::new(Mutex::new(Board::copy_from(self)));
        match self.at(row, col).get_possibilities()
        {
            Some(values) =>
            {
                //check for a single possibility
                if values.len() == 1
                {
                    //set square to only possibility
                    let mut new_value = 0;
                    for val in values.iter() { new_value = *val; }
                    self.set(row, col, new_value);
                    changed = true;
                }
                else
                {
                    for val in values.iter()
                    {
                        
                        let mut row_unique = true;
                        let mut col_unique = true;
                        let mut block_unique = true;
                        //check for possibility uniqueness in row
                        for _col in 0..9 {
                            if col != _col {
                                //let _board = Arc::clone(&board);
                                if self.at(row, _col).contains(*val) {
                                    row_unique = false; break;
                                }
                            }
                        }
                        //check for possibility uniqueness in column
                        for _row in 0..9 {
                            if row != _row {
                                if self.at(_row, col).contains(*val) {
                                    col_unique = false; break;
                                }
                            }
                        }
                        //check for possibility uniqueness in Block
                        let block_row = row / 3;
                        let block_col = col / 3;
                        if self.Blocks[block_row][block_col].contains(*val) {
                            block_unique = false;
                        }

                        if row_unique || col_unique || block_unique
                        {
                            self.set(row, col, *val);
                            changed = true;
                            break;
                        }
                    }
                }
            }
            None => { }
        }
        changed
    }
    //does basic checking for every square
    //returns tru if board changed, false otherwise
    fn basic_check(&mut self) -> bool
    {
        /*
        goal: run all square checks in parallel
        */
        //let board = Arc::new(Mutex::new(Board::copy_from(self)));
        //let mut handles = vec![];
        //let changed = Arc::new(Mutex::new(false));
        let mut changed = false;
        for row in 0..9{
            for col in 0..9{
                // let _changed = Arc::clone(&changed);
                // let _board = Arc::clone(&board);
                // let handle = thread::spawn(move || {
                //     if _board.lock().unwrap().check_square(row, col){
                //         let mut val = _changed.lock().unwrap();
                //         *val = true;
                //     }
                // });
                // handles.push(handle);
                if self.check_square(row, col){
                    changed = true;
                }
            }
        }

        // for handle in handles {
        //     handle.join().unwrap();
        // }

        // let changed_ret = *changed.lock().unwrap();
        // if changed_ret {
        //     *self = Board::copy_from(&mut *board.lock().unwrap());
        // }
        // changed_ret
        changed
    }
    //performs a deep check:
    // - if a possibility can only be in one row/column of a Block,
    //   removes that possibility from that row/column in adjacent Blockes
    //returns true if board changed, false otherwise
    fn deep_check(&mut self) -> bool
    {
        if DEBUG_PRINTOUT { println!("running deep check"); }
        let mut changed = false;
        let mut deep_rows: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        let mut deep_cols: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

        let row_string = 7;
        let col_string = 8;
        let block_col_string = 9;
        let block_row_string = 10;
        //get values and rows/cols
        for block_row in 0..3
        {
            for block_col in 0..3
            {
                let curr_block = &mut self.Blocks[block_row][block_col];
                let mut temp_rows = HashMap::new();
                let mut temp_cols = HashMap::new();
                for (key, val) in curr_block.deep_row_check() {
                    temp_rows.insert(key, val + 3*(block_row as i32));
                }
                for (key, val) in curr_block.deep_col_check() {
                    temp_cols.insert(key, val + 3*(block_col as i32));
                }
                
                for (key, row) in temp_rows {
                    match deep_rows.get_mut(&key) {
                        Some(map) => {
                            map.insert(row_string, row);
                            map.insert(block_col_string, block_col as i32);
                        },
                        None => {
                            deep_rows.insert(key, HashMap::new());
                            match deep_rows.get_mut(&key) {
                                Some(map) => {
                                    map.insert(row_string, row);
                                    map.insert(block_col_string, block_col as i32);
                                },
                                None => {}
                            }
                        }
                    }
                }
                for (key, col) in temp_cols {
                    match deep_cols.get_mut(&key) {
                        Some(map) => {
                            map.insert(col_string, col);
                            map.insert(block_row_string, block_row as i32);
                        },
                        None => {
                            deep_cols.insert(key, HashMap::new());
                            match deep_cols.get_mut(&key) {
                                Some(map) => {
                                    map.insert(col_string, col);
                                    map.insert(block_row_string, block_row as i32);
                                },
                                None => {}
                            }
                        }
                    }
                }
            }
        }
        //do changing
        for (poss, map) in deep_rows {
            let row:i32 = match map.get(&row_string) {
                Some(val) => *val,
                None => {9}
            };
            let mut update_cols: HashSet<_> = [0,1,2,3,4,5,6,7,8].iter().cloned().collect();
            update_cols.remove(&(map[&block_col_string]*3+0));
            update_cols.remove(&(map[&block_col_string]*3+1));
            update_cols.remove(&(map[&block_col_string]*3+2));
            for col in update_cols {
                if self.at_mut(row as usize, col as usize).remove(poss) {
                    changed = true;
                    if DEBUG_PRINTOUT { println!("deep check did something on rows");}
                }
            }
        }
        for (poss, map) in deep_cols {
            let col:i32 = match map.get(&col_string) {
                Some(val) => *val,
                None => {9}
            };
            let mut update_rows: HashSet<_> = [0,1,2,3,4,5,6,7,8].iter().cloned().collect();
            update_rows.remove(&(map[&block_row_string]*3+0));
            update_rows.remove(&(map[&block_row_string]*3+1));
            update_rows.remove(&(map[&block_row_string]*3+2));
            for row in update_rows {
                if self.at_mut(row as usize, col as usize).remove(poss) {
                    changed = true;
                    if DEBUG_PRINTOUT { println!("deep check did something on cols");}
                }
            }
        }
        changed
    }
    //changes the square with the least possibilities into each of it's possibilities
    //and attempts to solve.
    fn last_resort(&mut self) -> bool
    {
        if DEBUG_PRINTOUT { println!("beggining last resort check"); }

        let mut changed =false;

        //determine square with lowest number of possibilities
        let mut lowest_count = 9;
        let mut lowest_coords: (usize, usize) = (0, 0);
        for row in 0..9{
            for col in 0..9{
                let curr_square = self.at(row, col);
                match curr_square{
                    Square::Possibilities(vals) => {
                        if vals.len() < lowest_count {
                            lowest_count = vals.len();
                            lowest_coords = (row, col);
                        }
                    }
                    Square::Value(_) => {}
                }
            }
        }

        //set value
        match self.at(lowest_coords.0, lowest_coords.1).get_possibilities(){
            Some(values) => {
                for val in values.iter() {
                    if DEBUG_PRINTOUT{println!("copying to new board");}
                    let mut recurse_board = Board::copy_from(self);

                    if DEBUG_PRINTOUT{println!("setting last resort value");}
                    recurse_board.set(lowest_coords.0, lowest_coords.1, *val);
                    if recurse_board.solve() {
                        *self = recurse_board;
                        changed = true;
                        break;
                    }
                }
            },
            None => {}
        }

        if DEBUG_PRINTOUT{println!("finished last resort");}
        changed
    }
    //checks current board for being solved
    //counts number of each possibility, if all are 9, board is solved
    //returns tuple of (solvable, solved)
    fn solve_check(&mut self) -> (bool, bool)
    {
        let mut solved = false;
        let mut solvable = true;

        //initialize count for each possible value
        let mut count: HashMap<i32, usize> = HashMap::new();

        //initialize
        for i in 1..10 { count.insert(i, 0); }
        
        let mut solved_vals: HashSet<i32> = HashSet::new();

        for row in 0..9 {
            for col in 0..9 {
                match self.at(row, col) {
                    Square::Value(val) => match count.get_mut(val){
                        Some(val) => { *val += 1 },
                        None => {}
                    },
                    Square::Possibilities(values) => {
                        if values.len() == 0 {
                            //if a square is not yet a value, but has no possibilities,
                            //the board is not solvable
                            solvable = false;
                        }
                    }
                }
            }
        }

        for (key, val) in count.iter(){
            if *val == 9 {
                solved_vals.insert(*key);
            }
        }
        
        if solved_vals.len() == 9 { solved = true; }

        (solvable, solved)
    }
    //main solve function
    pub fn solve(&mut self) -> bool
    {
        if DEBUG_PRINTOUT { println!("Solving Sudoku Board");}
        let mut board_solved = false;
        let mut board_changed = true;
        let mut loop_count = 0;
        while board_changed
        {
            loop_count += 1;
            if DEBUG_PRINTOUT{println!("loop {}", loop_count);}

            //solve check
            let solve_check_result = self.solve_check();
            let solvable = solve_check_result.0;
            let solved = solve_check_result.1;

            if !solvable{
                board_solved = false;
                if DEBUG_PRINTOUT { println!("board is unsolvable");}
                return board_solved;
            }
            else if solved{
                board_solved = true;
                if DEBUG_PRINTOUT { println!("------------------SOLVED---------------");}
                return board_solved;
            }

            board_changed = false;

            //basic solving
            if self.basic_check(){
                board_changed = true;
            }

            //deep check
            if !board_changed{
                if self.deep_check(){
                    board_changed = true;
                }
            }

            //last resort: brute force recursion
            if !board_changed{
                if self.last_resort(){
                    board_changed = true;
                }
            }
        }
        board_solved
    }
    //prints possibilities/values for every square
    pub fn to_string(&mut self)
    {
        for row in 0..9
        {
            for col in 0..9
            {
                println!("({},{}): {}", row+1, col+1, self.at(row, col).to_string());
            }
        }
    }
    pub fn to_display(&mut self)
    {
        for row in 0..9{
            let mut display_string = String::new();
            for col in 0..9{
                display_string.push( match self.at(row, col){
                    Square::Possibilities(_) => {'_'}
                    Square::Value(val) => {
                        match char::from_digit(*val as u32, 10){
                            Some(val) => val,
                            None =>{'x'}
                        }
                    }
                });
                display_string.push(' ');
            }
            println!("{}", display_string);
        }
    }

}

struct Block
{
    squares: Vec<Vec<Square>>
}
impl Block
{
    //newializes a 3x3 array of Squares with all possibilities in all Squares
    fn new() -> Block
    {
        Block{squares:vec![vec![Square::new(),Square::new(),Square::new()],
                        vec![Square::new(),Square::new(),Square::new()],
                        vec![Square::new(),Square::new(),Square::new()]]}
    }

    //returns a mutable reference to the Square at row, col
    fn at_mut(&mut self, row:usize, col:usize) -> &mut Square
    {
        &mut self.squares[row][col]
    }
    fn at(&self, row:usize, col:usize) -> &Square
    {
        &self.squares[row][col]
    }
    //removes val from possibilities of each Square
    fn remove(&mut self, val:i32)
    {
        // for square_row in &mut self.squares {
        //     for square in square_row {
        //         square.remove(val);
        //     }
        // }
        for row in 0..3
        {
            for col in 0..3
            {
                self.squares[row][col].remove(val);
            }
        }
    }
    //checks each Square in the Block if it contains the given value
    fn contains(&mut self, val:i32) -> bool
    {
        let mut contains = false;
        for square_row in &mut self.squares {
            for square in square_row {
                if square.contains(val) { contains = true; }
            }
        }
        contains
    }
    fn deep_row_check(&mut self) -> HashMap<i32,i32>
    {
        let mut return_map = HashMap::new();
        let mut possibility_rows = HashMap::new();

        //initialize possibility_rows with empty hash sets
        for i in 1..10 {
            possibility_rows.insert(i, HashSet::new());
        }
        //get which rows each possibility can be in
        for square_row in 0..3 {
            for square_col in 0..3 {
                match &self.squares[square_row][square_col] {
                    Square::Possibilities(values) => {
                        for val in values.iter() {
                            match possibility_rows.get_mut(&val) {
                                Some(set) => { set.insert(square_row); },
                                None => {}
                            }
                        }
                    },
                    Square::Value(_) => {}
                }
            }
        }

        //if a possibility can only be in a single row, add to return_map
        for (poss, rows) in possibility_rows {
            if rows.len() == 1 {
                for row in rows.iter() {
                    return_map.insert(poss, *row as i32);
                }
            }
        }

        return_map
    }
    fn deep_col_check(&mut self) -> HashMap<i32, i32>
    {
        let mut return_map = HashMap::new();
        let mut possibility_cols = HashMap::new();

        //initialize possibility_cols with empty hash sets
        for i in 1..10 {
            possibility_cols.insert(i, HashSet::new());
        }
        //get which cols each possibility can be in
        for square_row in 0..3 {
            for square_col in 0..3 {
                match &self.squares[square_row][square_col] {
                    Square::Possibilities(values) => {
                        for val in values.iter() {
                            match possibility_cols.get_mut(&val) {
                                Some(set) => { set.insert(square_col); },
                                None => {}
                            }
                        }
                    },
                    Square::Value(_) => {}
                }
            }
        }

        //if a possibility can only be in a single row, add to return_map
        for (poss, cols) in possibility_cols {
            if cols.len() == 1 {
                for col in cols.iter() {
                    return_map.insert(poss, *col as i32);
                }
            }
        }

        return_map
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
    //returns true if value was removed, false if not present
    //returns false if Square is Value
    fn remove(&mut self, val:i32) -> bool
    {
        match self
        {
            Square::Possibilities(values) => {values.remove(&val)},
            Square::Value(_val) => false
        }
    }
    fn get_possibilities(&self) -> Option<HashSet<i32>>
    {
        match self
        {
            Square::Possibilities(values) => Some(values.clone()),
            Square::Value(_val) => None
        }
    }
    fn to_string(&self) -> String
    {
        let mut return_string = String::new();
        match self
        {
            Square::Possibilities(values) => 
            {
                return_string += "( ";
                for val in values { return_string += &format!("{}, ", val); }
                return_string += ")";
            },
            Square::Value(val) =>
            {
                return_string += &format!("{}", val)
            }
        }
        return_string
    }
}
