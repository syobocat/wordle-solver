use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use anyhow::Context;

#[cfg(test)]
mod test;

pub struct WordleSolver {
    answer_list: Vec<[char; 5]>,
    filter_list: Vec<[char; 5]>,
    confirmed: [char; 5],
    with: HashSet<char>,
    without: HashSet<char>,
}

impl WordleSolver {
    pub fn new<P: AsRef<Path>>(answer_dict_path: P, input_dict_path: P) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(input_dict_path).context("Dictionary not found")?;
        let filter_list: Vec<[char; 5]> = contents
            .lines()
            .filter(|l| l.chars().collect::<HashSet<char>>().len() == 5)
            .filter_map(|l| l.chars().collect::<Vec<char>>().try_into().ok())
            .collect();

        let contents = std::fs::read_to_string(answer_dict_path).context("Dictionary not found")?;
        let answer_list: Vec<[char; 5]> = contents
            .lines()
            .filter_map(|l| l.chars().collect::<Vec<char>>().try_into().ok())
            .collect();

        Ok(Self {
            answer_list,
            filter_list,
            confirmed: ['_'; 5],
            with: HashSet::new(),
            without: HashSet::new(),
        })
    }

    pub fn remaining_answers(self) -> Vec<[char; 5]> {
        self.answer_list
    }

    pub fn feedback(&mut self, guess: [char; 5], response: [Response; 5]) -> bool {
        if response.iter().all(|r| r == &Response::Green) {
            return true;
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
        false
    }

    pub fn print(&self) {
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

    pub fn guess(&mut self) -> [char; 5] {
        if self.answer_list.len() <= 2 {
            self.answer_list[0]
        } else {
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

#[derive(PartialEq)]
pub enum Response {
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
