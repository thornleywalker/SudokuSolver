#[cfg(test)]
mod tests
{
    use crate::Board;
    const SOLVED_TEST_1: &'static str = "827154396,965327148,341689752,593468271,472513689,618972435,786235914,154796823,239841567";
    const SOLVED_TEST_2: &'static str = "";
    const SOLVED_TEST_3: &'static str = "";
    const SOLVED_TESTS: [&str; 3] = [SOLVED_TEST_1, SOLVED_TEST_2, SOLVED_TEST_3];

    const BASIC_TEST_1: &'static str = "500407903,002010087,100680004,800300700,026001345,470050,000032409,030008062,009760508";
    const BASIC_TEST_2: &'static str = "060209,000030010,100600009,420500090,005302860,083100024,870906035,340050270,206073001";
    const BASIC_TEST_3: &'static str = "123085400,000034026,006010003,007920,390000062,005473009,072000901,000107040,950342008";
    const BASIC_TESTS: [&str; 3] = [BASIC_TEST_1, BASIC_TEST_2, BASIC_TEST_3];

    const DEEP_SOLVE_TEST_1: &'static str = "000006300,068007002,010008500,00,080050200,040001070,400010003,603000,020090400";
    const DEEP_SOLVE_TEST_2: &'static str = "000600000,006730590,083010760,040107003,000300900,900008000,000000001,009000006,700801040";
    const DEEP_SOLVE_TEST_3: &'static str = "020001084,008600250,005027,050000008,037000400,000304060,070000802,800003100,000210040";
    const DEEP_SOLVE_TESTS: [&str; 3] = [DEEP_SOLVE_TEST_1, DEEP_SOLVE_TEST_2, DEEP_SOLVE_TEST_3];

    const LAST_RESORT_TEST_1: &'static str = "004000057,000000390,000106,1059,940200001,000003,260008,080000700,000500020";
    const LAST_RESORT_TEST_2: &'static str = "010086007,000004905,000100,070050032,00,000090700,006010009,340000001,800030050";
    const LAST_RESORT_TEST_3: &'static str = "000520,000400001,000080900,060000004,070302085,038070,900,050003,007100042";
    const LAST_RESORT_TESTS: [&str; 3] = [LAST_RESORT_TEST_1, LAST_RESORT_TEST_2, LAST_RESORT_TEST_3];

    const UNSOLVABLE_TEST_1: &'static str = "123085400,000034026,006010003,007920,390000062,105473009,072000901,000107040,950342008";
    const UNSOLVABLE_TEST_2: &'static str = "020001084,008600250,005927,050000008,037000400,000304060,070000802,800003100,000210040";
    const UNSOLVABLE_TEST_3: &'static str = "22000382,580002000,305700000,700,500034000,003005002,000000080,60000200,00000001";
    const UNSOLVABLE_TESTS: [&str; 3] = [UNSOLVABLE_TEST_1, UNSOLVABLE_TEST_2, UNSOLVABLE_TEST_3];

    const ERROR_TEST_1: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    const ERROR_TEST_2: &'static str = "408l2003401201501g010600101034&0500610123405";
    const ERROR_TEST_3: &'static str = "";
    const ERROR_TESTS: [&str; 3] = [ERROR_TEST_1, ERROR_TEST_2, ERROR_TEST_3];

    //unit tests
    #[test]
    fn solved_tests(){
        let mut all_solved = true;

        for test in SOLVED_TESTS.iter(){
            let mut curr_board = 
                Board::from_list(
                    String::from(*test)
                );

            if !curr_board.solve() { all_solved = false; }
        }

        assert!(all_solved);
    }
    #[test]
    fn basic_tests(){
        let mut all_solved = true;

        for test in BASIC_TESTS.iter(){
            let mut curr_board = 
                Board::from_list(
                    String::from(*test)
                );

            if !curr_board.solve() { all_solved = false; }
        }

        assert!(all_solved);
    }
    #[test]
    fn deep_solve_test(){
        let mut all_solved = true;

        for test in DEEP_SOLVE_TESTS.iter(){
            let mut curr_board = 
                Board::from_list(
                    String::from(*test)
                );

            if !curr_board.solve() { all_solved = false; }
        }

        assert!(all_solved);
    }
    #[test]
    fn last_resort_test(){
        let mut all_solved = true;

        for test in LAST_RESORT_TESTS.iter(){
            let mut curr_board = 
                Board::from_list(
                    String::from(*test)
                );

            if !curr_board.solve() { all_solved = false; }
        }

        assert!(all_solved);
    }
    #[test]
    fn unsolvable_test(){
        let mut all_failed = true;

        for test in UNSOLVABLE_TESTS.iter(){
            let mut curr_board = 
                Board::from_list(
                    String::from(*test)
                );

            if curr_board.solve() { all_failed = false; }
        }

        assert!(all_failed);
    }
    #[test]
    fn error_test(){
        let mut all_solved = true;

        for test in ERROR_TESTS.iter(){
            let mut curr_board = 
                Board::from_list(
                    String::from(*test)
                );

            if !curr_board.solve() { all_solved = false; }
        }

        assert!(all_solved);
    }
}