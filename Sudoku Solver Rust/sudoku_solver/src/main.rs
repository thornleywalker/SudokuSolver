mod sudoku;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const SOLVED_TEST_1: &'static str = "827154396,965327148,341689752,593468271,472513689,618972435,786235914,154796823,239841567";
const SOLVED_TEST_2: &'static str = "";
const SOLVED_TEST_3: &'static str = "";

const BASIC_TEST_1: &'static str = "500407903,002010087,100680004,800300700,026001345,470050,000032409,030008062,009760508";
const BASIC_TEST_2: &'static str = "060209,000030010,100600009,420500090,005302860,083100024,870906035,340050270,206073001";
const BASIC_TEST_3: &'static str = "123085400,000034026,006010003,007920,390000062,005473009,072000901,000107040,950342008";

const DEEP_SOLVE_TEST_1: &'static str = "000006300,068007002,010008500,00,080050200,040001070,400010003,603000,020090400";
const DEEP_SOLVE_TEST_2: &'static str = "000600000,006730590,083010760,040107003,000300900,900008000,000000001,009000006,700801040";
const DEEP_SOLVE_TEST_3: &'static str = "020001084,008600250,005027,050000008,037000400,000304060,070000802,800003100,000210040";

const LAST_RESORT_TEST_1: &'static str = "004000057,000000390,000106,1059,940200001,000003,260008,080000700,000500020";
const LAST_RESORT_TEST_2: &'static str = "010086007,000004905,000100,070050032,00,000090700,006010009,340000001,800030050";
const LAST_RESORT_TEST_3: &'static str = "000520,000400001,000080900,060000004,070302085,038070,900,050003,007100042";

const UNSOLVABLE_TEST_1: &'static str = "123085400,000034026,006010003,007920,390000062,105473009,072000901,000107040,950342008";
const UNSOLVABLE_TEST_2: &'static str = "020001084,008600250,005927,050000008,037000400,000304060,070000802,800003100,000210040";
const UNSOLVABLE_TEST_3: &'static str = "22000382,580002000,305700000,700,500034000,003005002,000000080,60000200,00000001";

const ERROR_TEST_1: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
const ERROR_TEST_2: &'static str = "408l2003401201501g010600101034&0500610123405";
const ERROR_TEST_3: &'static str = "";

const TEST_BATTERY: [&str; 18] = [  SOLVED_TEST_1,
                                    SOLVED_TEST_2,
                                    SOLVED_TEST_3,
                                    BASIC_TEST_1,
                                    BASIC_TEST_2,
                                    BASIC_TEST_3,
                                    DEEP_SOLVE_TEST_1,
                                    DEEP_SOLVE_TEST_2,
                                    DEEP_SOLVE_TEST_3,
                                    LAST_RESORT_TEST_1,
                                    LAST_RESORT_TEST_2,
                                    LAST_RESORT_TEST_3,
                                    UNSOLVABLE_TEST_1,
                                    UNSOLVABLE_TEST_2,
                                    UNSOLVABLE_TEST_3,
                                    ERROR_TEST_1,
                                    ERROR_TEST_2,
                                    ERROR_TEST_3,
                                    ];

fn main() {

    // println!("beggining tests");

    // for test in TEST_BATTERY.iter(){
    //     let mut curr_board = 
    //     sudoku::Board::from_list(
    //         String::from(*test)
    //     );
    //     println!("initial board");
    //     curr_board.to_display();

    //     curr_board.solve();
    //     println!("board after solve");
    //     curr_board.to_display();
    // }

    //let easy_file = File::open("puzzles/easy.txt");
    //easy_file
    if let Ok(lines) = read_lines("/puzzles/easy.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                println!("{}", puzzle);
            }
        }
    }

    // let mut curr_board = 
    //     sudoku::Board::from_list(
    //         String::from("058030020,402000905,007000680,290054070,500062000,003810250,109003064,865490130,070006000,")
    //     );
    //     println!("initial board");
    //     curr_board.to_display();

    //     curr_board.solve();
    //     println!("board after solve");
    //     curr_board.to_display();
    
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}