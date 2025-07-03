use wordle_solver::{WordleResult, WordleSolver};

const DIC_PATH: &str = "./dictionary.txt";

fn main() {
    let solver = WordleSolver::new(DIC_PATH).unwrap();

    let result = solver.try_solve();

    // 6トライで当てられなかった場合、残った単語を表示
    if let WordleResult::Failed(words) = result {
        println!("Failed...");
        println!("Remaining words:");
        for word in words {
            println!("  {}", word.iter().collect::<String>());
        }
    } else {
        println!("Finished!")
    }
}
