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
    pub fn new(dict: &str) -> anyhow::Result<Self> {
        let dictionary = dict.lines().map(|l| l.chars().collect::<Vec<char>>());
        let answer_list: Vec<[char; 5]> = dictionary.filter_map(|l| l.try_into().ok()).collect();
        let filter_list: Vec<[char; 5]> = answer_list
            .clone()
            .into_iter()
            .filter(|word| word.iter().collect::<HashSet<&char>>().len() == 5)
            .collect();
        Ok(Self {
            answer_list,
            filter_list,
            confirmed: ['_'; 5],
            with: HashSet::new(),
            without: HashSet::new(),
        })
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path).context("Dictionary not found")?;
        Self::new(&contents)
    }

    pub fn remaining_answers(self) -> Vec<[char; 5]> {
        self.answer_list
    }

    pub fn feedback(&mut self, guess: [char; 5], response: &[Response; 5]) -> bool {
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
            self.next().unwrap_or(self.answer_list[0])
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

    fn next(&self) -> Option<[char; 5]> {
        let mut chars: HashSet<char> = HashSet::new();
        for word in &self.filter_list {
            for c in word {
                let _ = chars.insert(*c);
            }
        }

        let mut scores: [HashMap<char, f64>; 5] = std::array::from_fn(|_| HashMap::new());
        for i in 0..5 {
            for c in chars.iter() {
                let mut green = 0;
                let mut yellow = 0;
                let mut gray = 0;
                for word in &self.answer_list {
                    if word.contains(c) {
                        if word[i] == *c {
                            green += 1;
                        } else {
                            yellow += 1;
                        }
                    } else {
                        gray += 1;
                    }
                }

                // 残る単語数の期待値
                let score = f64::from(green).mul_add(
                    f64::from(green),
                    f64::from(yellow).mul_add(f64::from(yellow), f64::from(gray).powi(2)),
                ) / self.answer_list.len() as f64;

                let _ = scores[i].insert(*c, score);
            }
        }

        let mut ranked: Vec<([char; 5], f64)> = Vec::new();
        for word in &self.filter_list {
            let score = word
                .iter()
                .enumerate()
                .fold(0.0, |score, (i, x)| score + scores[i].get(x).unwrap());
            // if score > 0.0 {
            ranked.push((*word, score));
            // }
        }

        ranked
            .iter()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(word, _)| *word)
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
