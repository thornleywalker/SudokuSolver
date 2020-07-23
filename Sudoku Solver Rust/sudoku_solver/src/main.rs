use std::{char, collections::{HashSet, HashMap}};


fn main() {
    let mut curr_board = 
        // Board::from_list(
        //     String::from(
        //         "004300080,\
        //          000600009,\
        //          061900000,\
        //          020490000,\
        //          503000900,\
        //          000062003,\
        //          300004568,\
        //          780000040,\
        //          000000000"
        //     ));
        Board::from_list(
            //String::from("000006300,068007002,010008500,00,080050200,040001070,400010003,603000,020090400")   //basic solving test
            String::from("000006300,068007002,010008500,00,080050200,040001070,400010003,603000,020090400") //deep check test
            //String::from("000006300,068007002,010008500,00,080050200,040001070,400010003,603000,020090400")
        );
    println!("initial board");
    curr_board.to_display();

    curr_board.solve();
    println!("board after solve");
    curr_board.to_display();
}

struct Board
{
    boxes: Vec<Vec<Box>>
}
impl Board
{
    //initialize a 3x3 array of Boxes
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
    //returns a mutable reference to the Square at row, col
    fn at(&mut self, row:usize, col:usize) -> &mut Square
    {
        let box_row = row / 3;
        let box_col = col / 3;

        let square_row = row % 3;
        let square_col = col % 3;

        self.boxes[box_row][box_col].at(square_row, square_col)
    }
    //changes the Square at row, col to the val,
    //removes val from all Squares in row, column, and Box
    fn set(&mut self, row:usize, col:usize, val:i32)
    {
        println!["setting {},{} to {}", row+1, col+1, val];
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
        let box_row = row / 3;
        let box_col = col / 3;
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
                        let mut box_unique = true;
                        //check for possibility uniqueness in row
                        for _col in 0..9 {
                            if col != _col{
                                if self.at(row, _col).contains(*val) { row_unique = false; break; }
                            }
                        }
                        //check for possibility uniqueness in column
                        for _row in 0..9 {
                            if row != _row{
                                if self.at(_row, col).contains(*val) { col_unique = false; break; }
                            }
                        }
                        //check for possibility uniqueness in Box
                        let box_row = row / 3;
                        let box_col = col / 3;
                        if self.boxes[box_row][box_col].contains(*val) { box_unique = false; }

                        if row_unique || col_unique || box_unique
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
    //performs a deep check:
    // - if a possibility can only be in one row/column of a Box,
    //   removes that possibility from that row/column in adjacent Boxes
    //returns true if board changed, false otherwise
    fn deep_check(&mut self) -> bool
    {
        println!("running deep check");
        let mut changed = false;
        let mut deep_rows: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        let mut deep_cols: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

        let row_string = 7;
        let col_string = 8;
        let box_col_string = 9;
        let box_row_string = 10;
        //get values and rows/cols
        for box_row in 0..3
        {
            for box_col in 0..3
            {
                let curr_box = &mut self.boxes[box_row][box_col];
                let mut temp_rows = HashMap::new();
                let mut temp_cols = HashMap::new();
                for (key, val) in curr_box.deep_row_check() {
                    temp_rows.insert(key, val + 3*(box_row as i32));
                }
                for (key, val) in curr_box.deep_col_check() {
                    temp_cols.insert(key, val + 3*(box_col as i32));
                }
                
                for (key, row) in temp_rows {
                    match deep_rows.get_mut(&key) {
                        Some(map) => {
                            map.insert(row_string, row);
                            map.insert(box_col_string, box_col as i32);
                        },
                        None => {
                            deep_rows.insert(key, HashMap::new());
                            match deep_rows.get_mut(&key) {
                                Some(map) => {
                                    map.insert(row_string, row);
                                    map.insert(box_col_string, box_col as i32);
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
                            map.insert(box_row_string, box_row as i32);
                        },
                        None => {
                            deep_cols.insert(key, HashMap::new());
                            match deep_cols.get_mut(&key) {
                                Some(map) => {
                                    map.insert(col_string, col);
                                    map.insert(box_row_string, box_row as i32);
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
            update_cols.remove(&(map[&box_col_string]*3+0));
            update_cols.remove(&(map[&box_col_string]*3+1));
            update_cols.remove(&(map[&box_col_string]*3+2));
            for col in update_cols {
                if self.at(row as usize, col as usize).remove(poss) {
                    changed = true;
                    println!("deep check did something on rows");
                }
            }
        }
        for (poss, map) in deep_cols {
            let col:i32 = match map.get(&col_string) {
                Some(val) => *val,
                None => {9}
            };
            let mut update_rows: HashSet<_> = [0,1,2,3,4,5,6,7,8].iter().cloned().collect();
            update_rows.remove(&(map[&box_row_string]*3+0));
            update_rows.remove(&(map[&box_row_string]*3+1));
            update_rows.remove(&(map[&box_row_string]*3+2));
            for row in update_rows {
                if self.at(row as usize, col as usize).remove(poss) {
                    changed = true;
                    println!("deep check did something on cols");
                }
            }
        }
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
        println!("Solving Sudoku Board");
        let mut solved = false;
        let mut board_changed = true;
        let mut loop_count = 0;
        while board_changed
        {
            loop_count += 1;
            println!("loop {}", loop_count);
            //solve check
            if self.solve_check(){
                solved = true;
                println!("solved");
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
                self.to_display();
            }
            //last resort: brute force recursion
        }
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
    fn to_display(&mut self)
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

struct Box
{
    squares: Vec<Vec<Square>>
}
impl Box
{
    //newializes a 3x3 array of Squares with all possibilities in all Squares
    fn new() -> Box
    {
        Box{squares:vec![vec![Square::new(),Square::new(),Square::new()],
                         vec![Square::new(),Square::new(),Square::new()],
                         vec![Square::new(),Square::new(),Square::new()]]}
    }
    //returns a mutable reference to the Square at row, col
    fn at(&mut self, row:usize, col:usize) -> &mut Square
    {
        &mut self.squares[row][col]
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
    //checks each Square in the Box if it contains the given value
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
    //checks if Square has a single possibility
    //returns true and the value if only one possibility
    //returns false and None if Square is Value
    fn single_possibility_check(&mut self) -> Option<i32>
    {
        match self
        {
            Square::Possibilities(values) => 
            {
                if values.len() == 1
                {
                    let mut new_val = 0;
                    for val in values.iter() { new_val = *val; }
                    Some(new_val)
                }
                else {None}
            }
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