use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const DIC_PATH: &str = "./dictionary.txt";

fn main() {
    // 辞書ファイルのロード
    let mut f = File::open(DIC_PATH).expect("Dictionary not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the dictionary");
    let dictionary: Vec<String> = contents.split_whitespace().map(|x| x.to_owned()).collect();

    // 単語リストと探索用リストの生成
    let mut words = dictionary.clone();
    let mut filtering_table = dictionary;
    filtering_table = filtering_table
        .into_iter()
        .filter(|x| x.chars().into_iter().collect::<HashSet<char>>().len() == 5)
        .collect(); // 探索用リストからは文字の重複をなくす

    // ステータス用変数
    let mut confirmed: Vec<char> = "_____".chars().collect(); // 緑
    let mut with: Vec<char> = Vec::new(); // 黄色
    let mut without: Vec<char> = Vec::new(); // 黒

    for _ in 0..6 {
        // ステータス表示
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

        // 単語生成
        // 候補が2つ以上あり、未探索文字だけで単語が作れる場合はそれを優先
        // 候補が1つだけ、もしくは未探索文字で単語が作れない場合は候補からランダムに
        let attempt;
        if !filtering_table.is_empty() && words.len() != 1 {
            println!("Too many words remaining. Trying to reduce the number of words...");
            attempt = choose(&filtering_table);
        } else {
            attempt = choose(&words);
        }
        let chars = attempt.chars().collect::<Vec<char>>();

        // ステータス表示終わり
        println!("==========");
        println!("{}", attempt);

        // 結果を入力
        let result = read_line();

        // 全て緑なら終了
        if result == "22222" {
            println!("Finished!");
            return;
        }

        // ステータス更新
        for i in 0..5 {
            match result.chars().collect::<Vec<char>>()[i] {
                '2' => confirmed[i] = chars[i], // 緑をconformedに
                '1' => {
                    // 未探索だった黄色をwithに
                    if !with.contains(&chars[i]) && !confirmed.contains(&chars[i]) {
                        with.push(chars[i]);
                    }

                    // 黄色と同じ位置に同じ文字を持つ単語を除外
                    words = solve_yellow(&words, chars[i], i);
                }
                _ => {
                    // 未探索だった黒をwithoutに
                    // 誤入力でのバグを防ぐためwithやconfirmedに入っている文字も無視
                    if !with.contains(&chars[i])
                        && !without.contains(&chars[i])
                        && !confirmed.contains(&chars[i])
                    {
                        without.push(chars[i]);
                    }
                }
            }
        }

        // 緑になった黄色を削除
        let mut _with: Vec<char> = Vec::new();
        for letter in with {
            if !confirmed.contains(&letter) {
                _with.push(letter);
            }
        }
        with = _with;

        // withとwithoutを整理
        with.sort_unstable();
        without.sort_unstable();

        // 単語リストと探索用リストを更新
        words = update_words(&words, &confirmed, &with, &without);
        filtering_table = update_filtering_table(&filtering_table, &confirmed, &with, &without);
    }

    // 6トライで当てられなかった場合、残った単語を表示
    println!("Failed...");
    println!("Remaining words:");
    for word in words {
        println!("  {}", word);
    }
}

// 単語リスト更新関数
fn update_words(
    words: &[String],
    confirmed: &[char],
    with: &[char],
    without: &[char],
) -> Vec<String> {
    // 新単語リスト初期化
    let mut match_words: Vec<String> = Vec::new();

    // 単語リストを一つずつ見てまわる
    for word in words {
        let chars = word.chars().collect::<Vec<char>>(); // 単語を文字に分解
        let mut is_valid = true; // このフラグが最後まで生きていればOK

        // 緑と合っているか
        for i in 0..5 {
            if confirmed[i] == '_' || chars[i] == confirmed[i] {
                continue;
            } else {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            continue;
        }

        // 黒と合っているか
        for letter in &chars {
            if without.contains(letter) {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            continue;
        }

        // 黄と合っているか
        for letter in with {
            if !chars.contains(letter) {
                is_valid = false;
                break;
            }
        }

        // 合格
        if is_valid {
            match_words.push(word.to_owned());
        }
    }

    match_words
}

// 探索用リスト更新関数
fn update_filtering_table(
    words: &[String],
    confirmed: &[char],
    with: &[char],
    without: &[char],
) -> Vec<String> {
    // 新探索用リスト初期化
    let mut match_words: Vec<String> = Vec::new();

    // 探索用リストを一つずつ見てまわる
    for word in words {
        let chars = word.chars().collect::<Vec<char>>(); // 単語を文字に分解
        let mut is_valid = true; // このフラグが最後まで生きていればOK

        // 探索済みではない(黒,黄,緑のどれにも含まれない)かどうか
        for letter in &chars {
            if without.contains(letter) || with.contains(letter) || confirmed.contains(letter) {
                is_valid = false;
                break;
            }
        }

        // 合格
        if is_valid {
            match_words.push(word.to_owned());
        }
    }

    match_words
}

// 黄色を処理
fn solve_yellow(words: &[String], letter: char, position: usize) -> Vec<String> {
    // 新単語リスト初期化
    let mut match_words: Vec<String> = Vec::new();

    // 単語リストを一つずつ見てまわる
    for word in words {
        // 黄色だった位置に黄色だった文字を含まない単語だけ新単語リストに入れる
        if word.chars().collect::<Vec<char>>()[position] == letter {
            continue;
        } else {
            match_words.push(word.to_owned());
        }
    }

    match_words
}

// 単語リストからランダムに単語を引き出す
fn choose(words: &[String]) -> String {
    let mut rng = rand::thread_rng();
    words[rng.gen_range(0..words.len())].to_owned()
}

// ユーザー入力取得
fn read_line() -> String {
    loop {
        let mut input = "".to_owned();
        std::io::stdin().read_line(&mut input).ok();
        let result = input.trim().to_owned();

        // 0, 1, 2以外の文字が入ったらエラーを出す
        for letter in result.chars() {
            if letter == '0' || letter == '1' || letter == '2' {
                return result;
            } else {
                println!("Invalid Input.");
            }
        }
    }
}
