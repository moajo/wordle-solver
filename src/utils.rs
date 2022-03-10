use std::fs;

pub fn load_vocab(fname: String) -> Vec<String> {
    fs::read_to_string(fname)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| *s != "")
        .collect()
}

pub fn max_index(list: &Vec<f64>) -> usize {
    let (max_index, max) = list.iter().enumerate().fold(
        (0, f64::MIN),
        |(old_index, old_value), (current_index, &current_value)| {
            if current_value > old_value {
                (current_index, current_value)
            } else {
                (old_index, old_value)
            }
        },
    );
    return max_index;
}
