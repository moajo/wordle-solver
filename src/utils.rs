use std::fs;

pub fn load_vocab(fname: String) -> Vec<String> {
    fs::read_to_string(fname)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| *s != "")
        .collect()
}

pub fn max<T: Copy>(list: &Vec<T>, extract_value_fn: impl Fn(&T) -> f64) -> T {
    let (max_index, _) = list.iter().enumerate().fold(
        (0, f64::MIN),
        |(old_index, old_value), (current_index, current_value)| {
            let v = extract_value_fn(&current_value);
            if v > old_value {
                (current_index, v)
            } else {
                (old_index, old_value)
            }
        },
    );
    return list[max_index];
}
