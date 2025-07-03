use wordle_solver::{Response, WordleSolver};

const DIC_PATH: &str = "./dictionary.txt";
const ANSWERS_PATH: &str = "./possible_answers.txt";

fn main() {
    let mut solver = WordleSolver::new(ANSWERS_PATH, DIC_PATH).unwrap();

    for _ in 0..6 {
        solver.print();
        println!("==========");
        let guess = solver.guess();
        println!("{}", guess.iter().collect::<String>());
        let response = read_line();
        if solver.feedback(guess, response) {
            println!("Finished!");
            return;
        };
    }

    // 6トライで当てられなかった場合、残った単語を表示
    println!("Failed...");
    println!("Remaining words:");
    for word in solver.remaining_answers() {
        println!("  {}", word.iter().collect::<String>());
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
