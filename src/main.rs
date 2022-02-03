use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const DIC_PATH: &str = "./dictionary.txt";

fn main() {
    let mut f = File::open(DIC_PATH).expect("Dictionary not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the dictionary");
    let dictionary: Vec<String> = contents.split_whitespace().map(|x| x.to_owned()).collect();
    let mut words = dictionary.clone();
    let mut filtering_table = dictionary;
    filtering_table = update_filtering_table(&filtering_table, &[], &[], &[]);

    let mut confirmed: Vec<char> = "_____".chars().collect();
    let mut with: Vec<char> = Vec::new();
    let mut without: Vec<char> = Vec::new();

    for _ in 0..6 {
        println!("\n==Status==");
        print!("Current: {}", confirmed.iter().cloned().collect::<String>());
        if !with.is_empty() {
            print!(" / with {}", with.iter().cloned().collect::<String>());
        }
        if !without.is_empty() {
            print!(" / without {}", without.iter().cloned().collect::<String>());
        }
        println!();
        println!("Words remaining: {}", words.len());

        let attempt;
        if !filtering_table.is_empty() {
            println!("Too many words remaining. Trying to reduce the number of words...");
            attempt = choose(&filtering_table);
        } else {
            attempt = choose(&words);
        }

        if let Some(x) = words.clone().into_iter().position(|x| x == attempt) {
            words.swap_remove(x);
        }
        if let Some(x) = filtering_table
            .clone()
            .into_iter()
            .position(|x| x == attempt)
        {
            filtering_table.swap_remove(x);
        }

        println!("==========");
        println!("{}", attempt);
        let chars = attempt.chars().collect::<Vec<char>>();
        let result = read_line();
        if result == "22222" {
            println!("Finished!");
            return;
        }
        for i in 0..5 {
            match result.chars().collect::<Vec<char>>()[i] {
                '2' => confirmed[i] = chars[i],
                '1' => {
                    if !with.contains(&chars[i]) && !confirmed.contains(&chars[i]) {
                        with.push(chars[i]);
                    }
                }
                _ => {
                    if !with.contains(&chars[i])
                        && !without.contains(&chars[i])
                        && !confirmed.contains(&chars[i])
                    {
                        without.push(chars[i]);
                    }
                }
            }
        }
        for i in 0..with.len() {
            if confirmed.contains(&with[i]) {
                with.swap_remove(i);
            }
        }
        with.sort_unstable();
        without.sort_unstable();

        words = update_words(&words, &confirmed, &with, &without);
        filtering_table = update_filtering_table(&filtering_table, &confirmed, &with, &without);
    }
    println!("Failed...");
    println!("Remaining words:");
    for word in words {
        println!("{}", word);
    }
}

fn update_words(
    words: &[String],
    confirmed: &[char],
    with: &[char],
    without: &[char],
) -> Vec<String> {
    let state = vec![
        confirmed[0] != '_',
        confirmed[1] != '_',
        confirmed[2] != '_',
        confirmed[3] != '_',
        confirmed[4] != '_',
    ];

    let mut match_words: Vec<String> = Vec::new();
    for word in words {
        let chars = word.chars().collect::<Vec<char>>();
        let mut is_valid = true;
        for i in 0..5 {
            if !state[i] || chars[i] == confirmed[i] {
                continue;
            } else {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            continue;
        }
        for letter in &chars {
            if without.contains(letter) {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            continue;
        }
        for letter in with {
            if !chars.contains(letter) {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            match_words.push(word.to_owned());
        }
    }

    match_words
}

fn update_filtering_table(
    words: &[String],
    confirmed: &[char],
    with: &[char],
    without: &[char],
) -> Vec<String> {
    let mut match_words: Vec<String> = Vec::new();
    for word in words {
        let chars = word.chars().collect::<Vec<char>>();
        let mut is_valid = true;
        for letter in &chars {
            if without.contains(letter) || with.contains(letter) || confirmed.contains(letter) {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            continue;
        }
        if chars.into_iter().collect::<HashSet<char>>().len() < 5 {
            is_valid = false;
        }
        if is_valid {
            match_words.push(word.to_owned());
        }
    }

    match_words
}

fn choose(words: &[String]) -> String {
    let mut rng = rand::thread_rng();
    words[rng.gen_range(0..words.len())].to_owned()
}

fn read_line() -> String {
    loop {
        let mut input = "".to_owned();
        std::io::stdin().read_line(&mut input).ok();
        let result = input.trim().to_owned();
        for letter in result.chars() {
            if letter == '0' || letter == '1' || letter == '2' {
                return result;
            } else {
                println!("Invalid Input.");
            }
        }
    }
}
