use crate::utils;
use log::{debug, error, info, log_enabled, Level};

pub struct Solver {
    candidate: Vec<String>,
    whole: Vec<String>,
}

pub fn new_solver(candidate: Vec<String>, whole: Vec<String>) -> Solver {
    Solver {
        candidate: candidate,
        whole: whole,
    }
}

impl Solver {
    pub fn attempt(&self) -> String {
        if self.candidate.len() == 1 {
            return self.candidate[0].clone();
        }
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let expected_reductions: Vec<_> = alphabet
            .chars()
            .map(|c| calc_expected_reduction(c, 1, &self.candidate))
            .collect();
        let max_expected_reduction_index = crate::utils::max_index(&expected_reductions);
        let max_expected_reduction = alphabet.chars().nth(max_expected_reduction_index).unwrap();
        info!("期待削減量: {:?}", expected_reductions);
        info!("最大削減文字: {:?}", max_expected_reduction);

        let aa: Vec<_> = self
            .candidate
            .iter()
            .filter(|s| s.contains(max_expected_reduction))
            .collect();
        return aa[0].to_string();
    }
    pub fn feedback(&mut self, attempt_word: String, feedback: String) {
        let n_before = self.candidate.len();
        for (i, c) in feedback.chars().enumerate() {
            if i > 4 || c == '\n' {
                break;
            }
            let attempt_char = attempt_word.chars().nth(i).unwrap();
            if c == 'g' {
                self.candidate = calc_green_words(attempt_char, i, &self.candidate);
            } else if c == 'y' {
                self.candidate = calc_yellow_words(attempt_char, i, &self.candidate);
            } else if c == '.' {
                if !check_exceptive_gray(&attempt_word, &feedback, i) {
                    self.candidate = calc_gray_words(attempt_char, &self.candidate);
                }
            } else {
                panic!("Invalid feedback {}", c);
            }
        }
        let n_after = self.candidate.len();
        for s in self.candidate.iter() {
            debug!("候補: {}", s);
        }
        debug!("候補数 {} -> {}", n_before, n_after);
    }
}

fn calc_expected_reduction(c: char, index: usize, vocab: &Vec<String>) -> f64 {
    let g = calc_green_words(c, index, &vocab);
    let y = calc_yellow_words(c, index, &vocab);
    let gray = calc_gray_words(c, &vocab);

    let n = vocab.len() as f64;
    let n_g = g.len() as f64;
    let n_y = y.len() as f64;
    let n_gray = gray.len() as f64;

    let g_prob = n_g / n;
    let y_prob = n_y / n;
    let gray_prob = n_gray / n;

    let expected_reduction =
        g_prob * (n_y + n_gray) + y_prob * (n_g + n_gray) + gray_prob * (n_g + n_y);
    return expected_reduction;
}

fn calc_green_words(c: char, index: usize, vocab: &Vec<String>) -> Vec<String> {
    vocab
        .iter()
        .filter(|s| s.chars().nth(index as usize).unwrap() == c)
        .map(|s| s.clone())
        .collect()
}

fn calc_yellow_words(c: char, index: usize, vocab: &Vec<String>) -> Vec<String> {
    vocab
        .iter()
        .filter(|s| {
            if s.chars().nth(index as usize).unwrap() == c {
                return false;
            }
            s.contains(&c.to_string())
        })
        .map(|s| s.clone())
        .collect()
}

fn calc_gray_words(c: char, vocab: &Vec<String>) -> Vec<String> {
    vocab
        .iter()
        .filter(|s| !s.contains(&c.to_string()))
        .map(|s| s.clone())
        .collect()
}

#[test]
fn calc_words_works_correctly() {
    let candidate = crate::utils::load_vocab("./vocab/wordlist_candidate".to_string());

    let g = calc_green_words('a', 1, &candidate);
    let y = calc_yellow_words('a', 1, &candidate);
    let gray = calc_gray_words('a', &candidate);
    assert_eq!(candidate.len(), g.len() + y.len() + gray.len());
}

// 例えば文字aが1文字目でgreenのとき、
// 3文字目にもaを指定してattemptした場合、例外的に3文字目はyではなく.になる。
// (yにはならない)
// そのような例外的状況であるかどうかを判定する
fn check_exceptive_gray(attempt_word: &String, feedback: &String, current_i: usize) -> bool {
    let current_char = attempt_word.chars().nth(current_i).unwrap();
    for (i, c) in feedback.chars().enumerate() {
        if current_i == i {
            continue;
        }
        if feedback.chars().nth(i).unwrap() != 'g' {
            continue;
        }
        let attempt_char = attempt_word.chars().nth(i).unwrap();
        if attempt_char == current_char {
            return true;
        }
    }
    return false;
}
