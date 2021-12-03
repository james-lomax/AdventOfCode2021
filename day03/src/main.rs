fn bits_to_uint(bits: &Vec<bool>) -> usize {
    bits.iter().rev().zip(0..)
        .map(|(x, index)| if *x { 1 << index } else { 0 })
        .sum()
}

fn power_consumption(data: &str) -> usize {
    let rows: Vec<&str> = data.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .collect();
    let threshold = rows.len() / 2;

    // Creates an iterator over vectors of digits, then folds them by adding
    // each digit of each vector, and then thresholding them to
    // output 0/1 depending on which was more common
    let bits: Vec<bool> = rows.iter().map(|s| s.chars().map(|c|
            match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("Unexpected character '{}' (expected 0 or 1)", c)
            }).collect::<Vec<usize>>())
        .fold(Vec::new(), |acc, x| {
            if acc.len() == 0 {
                x
            } else {
                acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect()
            }
        })
        .iter().map(|v| *v > threshold)
        .collect();

    let gamma = bits_to_uint(&bits);
    let epsilon = bits_to_uint(&bits.iter().map(|x| !*x).collect());
    gamma * epsilon
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    println!("Part 1 = {}", power_consumption(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample = "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010";
        assert_eq!(198, power_consumption(sample));
    }
}