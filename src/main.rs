use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs;

enum Response {
    Black,
    Yellow,
    Green,
}

impl TryFrom<char> for Response {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        match item {
            '0' => Ok(Self::Black),
            '1' => Ok(Self::Yellow),
            '2' => Ok(Self::Green),
            _ => Err(()),
        }
    }
}

const DIC_PATH: &str = "./dictionary.txt";

fn main() {
    // 辞書ファイルのロード
    let contents = fs::read_to_string(DIC_PATH).expect("Dictionary not found");
    let dictionary: Vec<String> = contents.split_whitespace().map(|x| x.to_owned()).collect();

    // 単語リストと探索用リストの生成
    let mut words = dictionary.clone();
    let mut filtering_table = dictionary;

    // 探索用リストからは文字の重複をなくす
    filtering_table.retain(|x| x.chars().collect::<HashSet<char>>().len() == 5);

    // ステータス用変数
    let mut confirmed: Vec<char> = vec!['_'; 5]; // 緑
    let mut with: Vec<char> = Vec::new(); // 黄色
    let mut without: Vec<char> = Vec::new(); // 黒

    for _ in 0..6 {
        // ステータス表示
        println!("\n==Status==");
        print!("Current: {}", confirmed.iter().collect::<String>());
        if !with.is_empty() {
            print!(" / with {}", with.iter().collect::<String>());
        }
        if !without.is_empty() {
            print!(" / without {}", without.iter().collect::<String>());
        }
        println!();
        println!("Words remaining: {}", words.len());

        // 単語生成
        // 候補が2つ以上あり、未探索文字だけで単語が作れる場合はそれを優先
        // 候補が1つだけ、もしくは未探索文字で単語が作れない場合は候補からランダムに
        let attempt = if !filtering_table.is_empty() && words.len() > 1 {
            println!("Too many words remaining. Trying to reduce the number of words...");
            filtering_table.choose(&mut rand::thread_rng()).unwrap()
        } else {
            words.choose(&mut rand::thread_rng()).unwrap()
        };
        let chars: Vec<char> = attempt.chars().collect();

        // ステータス表示終わり
        println!("==========");
        println!("{}", attempt);

        // 結果を入力
        let result = read_line();

        // 全て緑なら終了
        if result.iter().all(|x| matches!(x, Response::Green)) {
            println!("Finished!");
            return;
        }

        // ステータス更新
        for i in 0..5 {
            // 未探索のみ
            if !confirmed.contains(&chars[i])
                && !with.contains(&chars[i])
                && !without.contains(&chars[i])
            {
                match result[i] {
                    Response::Green => {
                        confirmed[i] = chars[i]; // 緑をconfirmedに
                        words.retain(|x| x.chars().nth(i) == Some(chars[i]));
                    }
                    Response::Yellow => {
                        // 黄色をwithに
                        with.push(chars[i]);

                        // 黄色を持たない、もしくは黄色と同じ位置に同じ文字を持つ単語を除外
                        words
                            .retain(|x| x.contains(chars[i]) && x.chars().nth(i) != Some(chars[i]));
                    }
                    Response::Black => {
                        // 黒をwithoutに
                        without.push(chars[i]);

                        // 黒を持つ単語を除外
                        words.retain(|x| !x.contains(chars[i]));
                    }
                };
                filtering_table.retain(|x| !x.contains(chars[i]))
            }
        }

        // 緑になった黄色を削除
        with.retain(|x| !confirmed.contains(x));

        // withとwithoutを整理
        with.sort_unstable();
        without.sort_unstable();
    }

    // 6トライで当てられなかった場合、残った単語を表示
    println!("Failed...");
    println!("Remaining words:");
    for word in words {
        println!("  {}", word);
    }
}

// ユーザー入力取得
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
