use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use sudoku;


fn easy_speed_tests()
{

    let mut puzz_count = 0;
    let mut solved_count = 0;

    // for puzzle in puzzles_list{
    //     puzz_count += 1;
    //     let mut curr_board = 
    //         sudoku::Board::from_list(
    //             String::from(puzzle)
    //         );
            
    //     if curr_board.solve() {solved_count += 1;}
    // }
    println!("solved {} of {}", solved_count, puzz_count);
}

fn medium_speed_tests()
{
    let mut puzzles_list: Vec<String> = Vec::new();
    
    if let Ok(lines) = read_lines("../../puzzle_generator/puzzles/medium.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                puzzles_list.push(puzzle);
            }
        }
    }

    let mut puzz_count = 0;
    let mut solved_count = 0;

    for puzzle in puzzles_list{
        puzz_count += 1;
        let mut curr_board = 
            sudoku::Board::from_list(
                String::from(puzzle)
            );
            
        if curr_board.solve() {solved_count += 1;}
    }
    println!("solved {} of {}", solved_count, puzz_count);
}

fn hard_speed_tests()
{
    let mut puzzles_list: Vec<String> = Vec::new();
    
    if let Ok(lines) = read_lines("../../puzzle_generator/puzzles/hard.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                puzzles_list.push(puzzle);
            }
        }
    }

    let mut puzz_count = 0;
    let mut solved_count = 0;

    for puzzle in puzzles_list{
        puzz_count += 1;
        let mut curr_board = 
            sudoku::Board::from_list(
                String::from(puzzle)
            );
            
        if curr_board.solve() {solved_count += 1;}
    }
    println!("solved {} of {}", solved_count, puzz_count);
}

fn expert_speed_tests()
{
    let mut puzzles_list: Vec<String> = Vec::new();
    
    if let Ok(lines) = read_lines("../../puzzle_generator/puzzles/expert.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                puzzles_list.push(puzzle);
            }
        }
    }

    let mut puzz_count = 0;
    let mut solved_count = 0;

    for puzzle in puzzles_list{
        puzz_count += 1;
        let mut curr_board = 
            sudoku::Board::from_list(
                String::from(puzzle)
            );
            
        if curr_board.solve() {solved_count += 1;}
    }
    println!("solved {} of {}", solved_count, puzz_count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut puzzles_list: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("../../puzzle_generator/puzzles/easy.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                puzzles_list.push(puzzle);
            }
        }
    }
    c.bench_function("easy tests", |b| b.iter(||
         for puzzle in puzzles_list{
             easy_speed_tests()
         }
        ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);