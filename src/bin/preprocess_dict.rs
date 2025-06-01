use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Write,
};

const DIC_PATH_READ: &str = "./dictionary_raw.txt";
const DIC_PATH_WRITE: &str = "./dictionary.txt";

fn main() {
    // 辞書ファイルのロード
    let contents = fs::read_to_string(DIC_PATH_READ).expect("Dictionary not found");
    let lines: Vec<&str> = contents.lines().collect();

    // 頻度分析
    let mut freq: HashMap<char, u32> = HashMap::new();
    for line in lines.iter() {
        for char in line.chars() {
            freq.entry(char).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    // ランク付け
    let mut dict: Vec<(&str, u32)> = Vec::new();
    for line in lines {
        let chars = line.chars().collect::<HashSet<char>>();
        let score = chars
            .iter()
            .fold(0, |score, x| score + freq.get(x).unwrap_or(&0));
        dict.push((line, score));
    }

    // ソートして保存
    dict.sort_by(|a, b| b.1.cmp(&a.1));
    let words: Vec<&str> = dict.into_iter().map(|word| word.0).collect();
    let new_dict = words.join("\n");
    let mut file = File::create(DIC_PATH_WRITE).expect("Cannot create the new dictionary");
    file.write_all(new_dict.as_bytes())
        .expect("Failed to write to the new dictionary");

    println!("Done.");
}
