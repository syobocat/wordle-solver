use clap::{Parser, ValueEnum};
use wordle_solver::{Response, WordleSolver};

const WORDLE_DICT: &str = include_str!("../dictionaries/wordle.txt");
const UNLIMITED_DICT: &str = include_str!("../dictionaries/wordle_unlimited.txt");
const POKEMON_DICT: &str = include_str!("../dictionaries/pokemon.txt");

#[derive(Clone, ValueEnum)]
enum Mode {
    Wordle,
    Unlimited,
    Pokemon,
}

#[derive(Parser)]
struct App {
    #[arg(value_enum, default_value_t = Mode::Wordle)]
    mode: Mode,
}

fn main() {
    let app = App::parse();
    let mut solver = match app.mode {
        Mode::Wordle => WordleSolver::new(WORDLE_DICT).unwrap(),
        Mode::Unlimited => WordleSolver::new(UNLIMITED_DICT).unwrap(),
        Mode::Pokemon => WordleSolver::new(POKEMON_DICT).unwrap(),
    };

    for _ in 0..6 {
        solver.print();
        println!("==========");
        let guess = solver.guess();
        println!("{}", guess.iter().collect::<String>());
        let response = read_line();
        if solver.feedback(guess, &response) {
            println!("Finished!");
            return;
        }
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
        }
        println!("Invalid input.");
    }
}
