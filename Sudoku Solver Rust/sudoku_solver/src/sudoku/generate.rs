use {
    reqwest,
    scraper::{Selector, Html},
    crate::sudoku, 
    std::collections::HashMap
};

pub enum Difficulty{
    Easy,
    Medium,
    Hard,
    Expert
}
pub fn generate_board(difficulty: Difficulty) -> Option<sudoku::Board>
{
    let resp = reqwest::blocking::get(
            match difficulty{
            Easy => "https://sudoku.com/easy/",
            Medium => "https://sudoku.com/medium/",
            Hard => "https://sudoku.com/hard/",
            Expert => "https://sudoku.com/expert/",
            }
        ).ok()?;

    

    let body = resp.text().unwrap();
    
    let document = Html::parse_document(&body);
    let selector = Selector::parse("#game").unwrap();

    let something = document.select(&selector);

    for element in something{
        println!("got something!");
        println!("{:?}", element);
    }




    let mut in_string = String::new();

    let mut return_board = sudoku::Board::from_list(in_string);

    Some(return_board)
}