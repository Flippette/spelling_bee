use anyhow::Result;
use rand::Rng;
use std::{
    env, fs,
    io::{self, BufRead, Write},
};

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() -> Result<()> {
    let wordlist = io::BufReader::new(fs::File::open(
        match env::args().nth(1).unwrap_or_default().as_str() {
            "--full" | "-f" => {
                println!("Using FULL wordlist");
                "../wordlist.full.txt"
            }
            _ => {
                println!("Using COMMON wordlist");
                "../wordlist.common.txt"
            }
        },
    )?)
    .lines()
    .map(|line| line.unwrap())
    .collect::<Vec<String>>();

    let mut charset = vec![];
    let valid_words = loop {
        charset.clear();

        (0..7).for_each(|_| loop {
            let next = ALPHABET
                .chars()
                .nth(rand::thread_rng().gen_range(1..1000) % ALPHABET.chars().count())
                .unwrap();
            if !charset.contains(&next) {
                charset.push(next);
                break;
            }
        });

        let tmp_words = wordlist
            .iter()
            .filter(|line| line.chars().all(|c| charset.contains(&c)))
            .filter(|line| line.contains(|c| &c == charset.first().unwrap()))
            .filter(|line| line.chars().count() >= 4);

        if tmp_words.clone().count() == 0 {
            continue;
        }
        break tmp_words;
    }
    .cloned()
    .collect::<Vec<String>>();

    println!("Charset: {:?}", charset);
    println!("Center character: {}", charset.first().unwrap());
    println!("Possible words: {}", valid_words.len());
    println!("\n{:?}\n", valid_words);

    let mut correct_words_guessed = 0;
    let mut words_guessed = vec![];

    loop {
        print!("Enter your guess: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "i give up" {
            println!("Words: {:?}", valid_words);
            break;
        } else if words_guessed.contains(&input) {
            println!("Word already guessed!");
            continue;
        } else if valid_words.contains(&input.trim().to_string()) {
            println!("Correct word: {}", &input);
            correct_words_guessed += 1;
        } else {
            println!("Guess not in wordlist!");
        }
        words_guessed.push(input);

        if correct_words_guessed == valid_words.len() {
            break;
        }
    }

    Ok(())
}
