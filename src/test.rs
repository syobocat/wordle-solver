use super::*;

const ANSWER_PATH: &str = "past_answers.txt";
const DIC_PATH: &str = "dictionary.txt";

fn solve(word: &str) -> Option<u32> {
    let mut solver = WordleSolver::new(DIC_PATH).unwrap();
    for i in 1..=6 {
        let guess = solver.guess();
        let response: [Response; 5] = std::array::from_fn(|i| {
            if word.chars().nth(i) == Some(guess[i]) {
                Response::Green
            } else if word.contains(guess[i]) {
                Response::Yellow
            } else {
                Response::Black
            }
        });
        if solver.feedback(guess, response) {
            println!("{word}: Success in {i} steps");
            return Some(i);
        };
    }
    println!("{word}: Fail");
    None
}

#[test]
fn success_rate() {
    let mut steps = 0u32;
    let mut success = 0u32;
    let mut fail = 0u32;
    let contents = std::fs::read_to_string(ANSWER_PATH).unwrap();
    for word in contents.lines() {
        if let Some(i) = solve(word) {
            success += 1;
            steps += i;
        } else {
            fail += 1;
        }
    }
    println!(
        "Attempts: {} / Success: {success} / Fail: {fail} / Success Rate: {} / Step Rate: {}",
        contents.lines().count(),
        success as f64 / (success + fail) as f64,
        steps as f64 / success as f64
    )
}
