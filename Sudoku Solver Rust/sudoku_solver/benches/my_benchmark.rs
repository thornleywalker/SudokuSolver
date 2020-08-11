use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use sudoku;


fn speed_tests(puzzles:&Vec<String>)
{
    for puzzle in puzzles{
        let mut curr_board = 
            sudoku::Board::from_list(
                String::from(puzzle)
            );
        curr_board.solve();
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut easy_puzzles_list: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("../../puzzle_generator/puzzles/easy.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                easy_puzzles_list.push(puzzle);
            }
        }
    }

    let mut medium_puzzles_list: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("../../puzzle_generator/puzzles/medium.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                medium_puzzles_list.push(puzzle);
            }
        }
    }

    let mut hard_puzzles_list: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("../../puzzle_generator/puzzles/hard.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                hard_puzzles_list.push(puzzle);
            }
        }
    }

    let mut expert_puzzles_list: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("../../puzzle_generator/puzzles/expert.txt"){
        for line in lines{
            if let Ok(puzzle) = line{
                expert_puzzles_list.push(puzzle);
            }
        }
    }

    
    c.bench_function("easy tests", |b| b.iter(||
             speed_tests(black_box(&easy_puzzles_list))
        ));
    c.bench_function("medium tests", |b| b.iter(||
             speed_tests(black_box(&medium_puzzles_list))
        ));
    c.bench_function("hard tests", |b| b.iter(||
             speed_tests(black_box(&hard_puzzles_list))
        ));

    c.bench_function("expert tests", |b| b.iter(||
             speed_tests(black_box(&expert_puzzles_list))
        ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);