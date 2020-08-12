use sudoku;

fn main() {
    let mut new_board = 
        sudoku::Board::from_list(
            String::from("000520,000400001,000080900,060000004,070302085,038070,900,050003,007100042"));

    if new_board.solve(){
        println!("the new board was solved");
        new_board.to_display();
    }
    else { println!("failure"); }
}
