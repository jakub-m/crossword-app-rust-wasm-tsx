use std::io::{self, BufRead};

fn main() {
    let input_words = read_non_blank_lines();
    let words = input_words.iter().map(|w| w.as_str()).collect();
    let layout =
        crossword::generator::generate_crossword(&words, crossword::GeneratorMode::InputOrder);
    println!("Final:\n\n{:>0}", layout);
}

fn read_non_blank_lines() -> Vec<String> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("#"))
        .collect();

    lines
}
