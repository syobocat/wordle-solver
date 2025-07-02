use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::Write,
};

const DIC_PATH_READ: &str = "./dictionary_raw.txt";
const DIC_PATH_WRITE: &str = "./dictionary.txt";

fn analyze_freq(words: &Vec<[char; 5]>) -> [HashMap<char, u32>; 5] {
    let mut freq: [HashMap<char, u32>; 5] = std::array::from_fn(|_| HashMap::new());
    for word in words {
        for i in 0..5 {
            freq[i].entry(word[i]).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    freq
}

fn main() {
    // 辞書ファイルのロード
    let contents = fs::read_to_string(DIC_PATH_READ).expect("Dictionary not found");
    let lines: Vec<[char; 5]> = contents
        .lines()
        .filter_map(|line| line.chars().collect::<Vec<char>>().try_into().ok())
        .collect();

    // 頻度分析
    let freq = analyze_freq(&lines);

    // ランク付け
    let mut dict: Vec<(String, u32)> = Vec::new();
    for line in lines {
        let score = line
            .iter()
            .enumerate()
            .fold(0, |score, (i, x)| score + freq[i].get(x).unwrap_or(&0));
        // 同じ文字が含まれるやつのランクを下げる
        let divisor = 6 - line.iter().collect::<HashSet<&char>>().len() as u32;
        let score = score / divisor;
        dict.push((line.iter().collect(), score));
    }

    // ソートして保存
    dict.sort_by(|a, b| b.1.cmp(&a.1));
    let words: Vec<String> = dict.into_iter().map(|word| word.0).collect();
    let new_dict = words.join("\n");
    let mut file = File::create(DIC_PATH_WRITE).expect("Cannot create the new dictionary");
    file.write_all(new_dict.as_bytes())
        .expect("Failed to write to the new dictionary");

    println!("Done.");
}
