#![feature(let_chains)]

use anyhow::Result;
use rayon::prelude::*;
use std::{
    env, fs,
    io::{self, BufRead},
};

#[allow(clippy::or_fun_call)]
fn main() -> Result<()> {
    let charset = if let Some(set) = env::args().nth(1) && set.chars().count() == 7 { set } else { panic!("Invalid charset!"); };
    let max_length = env::args()
        .nth(2)
        .unwrap_or(String::from("7"))
        .parse::<usize>()
        .unwrap_or(7);
    let tolerance = env::args()
        .nth(3)
        .unwrap_or(String::from("3"))
        .parse::<usize>()
        .unwrap_or(3);

    let wordlist = io::BufReader::new(fs::File::open(
        match env::args().nth(1).unwrap_or(String::from("")).as_str() {
            "--full" | "-f" => {
                println!("Using FULL wordlist");
                "../wordlist.full.txt"
            }
            _ => {
                println!("Using COMMON wordlist");
                "../wordlist.common.txt"
            }
        },
    )?);

    let lines = wordlist.lines();

    let mut scores = lines
        .par_bridge()
        .into_par_iter()
        .map(|line| {
            let line = line.unwrap();
            (
                line.clone(),
                line.chars()
                    .map(|chr| charset.chars().filter(|&c| c == chr).count())
                    .sum(),
            )
        })
        .collect::<Vec<(String, usize)>>();

    scores.par_sort_by(|a, b| a.1.cmp(&b.1));
    let filtered_by_word = scores
        .iter()
        .rev()
        .filter(|(word, _)| word.chars().count() <= max_length)
        .filter(|(word, _)| word.chars().next() == charset.chars().next())
        .filter(|(word, _)| word.chars().all(|c| charset.contains(c)));

    let max_score = &filtered_by_word.clone().next().unwrap().1;

    filtered_by_word
        .filter(|(_, score)| *score >= *max_score - tolerance && *score <= *max_score + tolerance)
        .for_each(|(word, score)| println!("{}: {}", word, score));

    Ok(())
}
