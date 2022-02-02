use rand::Rng;
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

    let mut confirmed: Vec<char> = "_____".chars().collect();
    let mut with: Vec<char> = Vec::new();
    let mut without: Vec<char> = Vec::new();

    while confirmed.contains(&'_') {
        print!("Current: {}", confirmed.iter().cloned().collect::<String>());
        if with.len() > 0 {
            print!(" / with {}", with.iter().cloned().collect::<String>());
        }
        if without.len() > 0 {
            print!(" / without {}", without.iter().cloned().collect::<String>());
        }
        println!();

        let attempt = choose(&words);
        println!("{}", attempt);
        let chars = attempt.chars().collect::<Vec<char>>();
        let result = read_line();
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
        words = filter(&words, &confirmed, &with, &without);
    }
}

fn filter(
    words: &Vec<String>,
    confirmed: &Vec<char>,
    with: &Vec<char>,
    without: &Vec<char>,
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
        for letter in with {
            if !chars.contains(letter) {
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
        for i in 0..5 {
            if !state[i] {
                continue;
            } else if chars[i] == confirmed[i] {
                continue;
            } else {
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

fn choose(words: &Vec<String>) -> String {
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
