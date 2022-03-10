use log::info;
use std::io;
use std::io::Write;

mod solver;
mod utils;

fn main() {
    env_logger::init();

    let candidate: Vec<String> = utils::load_vocab("./vocab/wordlist_candidate".to_string());
    let whole = utils::load_vocab("./vocab/wordlist_whole".to_string());
    info!("# of candidate {}", candidate.len());
    info!("# of whole {}", whole.len());

    let mut solver = solver::new_solver(candidate, whole);
    loop {
        let attempt = solver.attempt();
        println!("Attempt: {}", attempt);
        let feedback = read_feedback();
        if feedback == "ggggg" {
            break;
        }
        solver.feedback(attempt, feedback);
    }
}

fn read_feedback() -> String {
    loop {
        let mut line = String::new();
        print!("Input feedback> ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap(); // TODO: 5文字アサート
        let l = line
            .strip_suffix("\n")
            .map(|s| s.to_string())
            .unwrap_or(line.clone());
        if !is_valid_feedback(&l) {
            info!("Invalid feedback");
            continue;
        }
        return l;
    }
}

fn is_valid_feedback(feedback: &String) -> bool {
    if feedback.len() != 5 {
        return false;
    }
    for c in feedback.chars() {
        if c != 'g' && c != 'y' && c != '.' {
            return false;
        }
    }
    return true;
}
