use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use anyhow::Context;

pub struct WordleSolver {
    finished: bool,
    attempt: u8,
    answer_list: Vec<[char; 5]>,
    filter_list: Vec<[char; 5]>,
    confirmed: [char; 5],
    with: HashSet<char>,
    without: HashSet<char>,
}

impl WordleSolver {
    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path).context("Dictionary not found")?;
        let dictionary = contents.lines().map(|l| l.chars().collect::<Vec<char>>());
        let answer_list: Vec<[char; 5]> = dictionary.filter_map(|l| l.try_into().ok()).collect();
        let filter_list: Vec<[char; 5]> = answer_list
            .clone()
            .into_iter()
            .filter(|l| l.iter().collect::<HashSet<&char>>().len() == 5)
            .collect();
        Ok(Self {
            finished: false,
            attempt: 1,
            answer_list,
            filter_list,
            confirmed: ['_'; 5],
            with: HashSet::new(),
            without: HashSet::new(),
        })
    }

    pub fn try_solve(mut self) -> WordleResult {
        loop {
            self.attempt();
            if let Some(success) = self.is_successed() {
                if success {
                    return WordleResult::Done;
                } else {
                    return WordleResult::Failed(self.answer_list);
                }
            }
        }
    }

    fn is_successed(&self) -> Option<bool> {
        if self.finished {
            Some(true)
        } else if self.attempt > 6 {
            Some(false)
        } else {
            None
        }
    }

    fn attempt(&mut self) {
        self.print();
        let guess = self.guess();
        println!("==========");
        println!("{}", guess.iter().collect::<String>());
        let response = read_line();
        if response.iter().all(|r| r == &Response::Green) {
            self.finished = true;
        }
        for i in 0..5 {
            match response[i] {
                Response::Green => {
                    self.confirm(i, guess[i]);
                }
                Response::Yellow => {
                    self.with(i, guess[i]);
                }
                Response::Black => {
                    self.without(guess[i]);
                }
            }
        }
        self.attempt += 1;
    }

    pub fn remaining_words(&self) -> &Vec<[char; 5]> {
        &self.answer_list
    }

    fn print(&self) {
        println!("\n==Status==");
        print!("Current: {}", self.confirmed.iter().collect::<String>());
        if !self.with.is_empty() {
            print!(" / with {}", self.with.iter().collect::<String>());
        }
        if !self.without.is_empty() {
            print!(" / without {}", self.without.iter().collect::<String>());
        }
        println!();
        println!("Words remaining: {}", self.answer_list.len());
    }

    fn guess(&mut self) -> [char; 5] {
        if self.answer_list.len() <= 2 {
            self.answer_list[0]
        } else {
            println!("Too many words remaining. Trying to reduce the number of words...");
            self.rearrange_filter_list();
            self.filter_list[0]
        }
    }

    fn confirm(&mut self, i: usize, c: char) {
        self.confirmed[i] = c;
        self.answer_list.retain(|x| x[i] == c);
    }

    fn with(&mut self, i: usize, c: char) {
        self.with.insert(c);
        self.answer_list.retain(|x| x.contains(&c) && x[i] != c);
    }

    fn without(&mut self, c: char) {
        self.without.insert(c);
        self.answer_list.retain(|x| !x.contains(&c));
    }

    fn rearrange_filter_list(&mut self) {
        let mut freq: HashMap<char, u32> = HashMap::new();
        let mut freq_per_pos: [HashMap<char, u32>; 5] = std::array::from_fn(|_| HashMap::new());
        for i in 0..5 {
            if self.confirmed[i] != '_' {
                continue;
            }
            for word in self.answer_list.iter() {
                freq.entry(word[i]).and_modify(|c| *c += 1).or_insert(1);
                freq_per_pos[i]
                    .entry(word[i])
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }

        let mut ranked: Vec<([char; 5], u32)> = Vec::new();
        for word in self.filter_list.iter() {
            let score = word.iter().enumerate().fold(0, |score, (i, x)| {
                // 確定してる情報は避ける
                if &self.confirmed[i] == x {
                    score
                } else {
                    // 全体の頻度に加え、その位置での頻度を加味する
                    score + freq.get(&x).unwrap_or(&0) + freq_per_pos[i].get(&x).unwrap_or(&0)
                }
            });
            ranked.push((*word, score));
        }

        ranked.retain(|x| x.1 > 0);
        ranked.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        self.filter_list = ranked.into_iter().map(|x| x.0).collect();
    }
}

pub enum WordleResult {
    Done,
    Failed(Vec<[char; 5]>),
}

#[derive(PartialEq)]
enum Response {
    Black,
    Yellow,
    Green,
}

impl TryFrom<char> for Response {
    type Error = ();

    fn try_from(item: char) -> anyhow::Result<Self, Self::Error> {
        match item {
            '0' => Ok(Self::Black),
            '1' => Ok(Self::Yellow),
            '2' => Ok(Self::Green),
            _ => Err(()),
        }
    }
}

fn read_line() -> [Response; 5] {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("IO Error.");
        let res: Vec<_> = input
            .trim()
            .chars()
            .filter_map(|i| i.try_into().ok())
            .collect();

        if let Ok(res) = res.try_into() {
            return res;
        } else {
            println!("Invalid input.");
        }
    }
}
