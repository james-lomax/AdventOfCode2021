fn bits_to_uint(bits: &Vec<bool>) -> usize {
    bits.iter().rev().zip(0..)
        .map(|(x, index)| if *x { 1 << index } else { 0 })
        .sum()
}

// Creates vector of binary numbers represented as vector of digits
fn parse_bits(data: &str) -> Vec<Vec<usize>> {
    data.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|s| s.chars().map(|c|
            match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("Unexpected character '{}' (expected 0 or 1)", c)
            }).collect::<Vec<usize>>())
        .collect()
}

fn most_common_bits(rows: &Vec<Vec<usize>>) -> Vec<bool> {
    let remainder = rows.len() % 2;
    let threshold = (rows.len() - remainder) / 2 + remainder;

    // Creates an iterator over vectors of digits, then folds them by adding
    // each digit of each vector, and then thresholding them to
    // output 0/1 depending on which was more common
    rows.iter()
        .fold(Vec::new(), |acc, x| {
            if acc.len() == 0 {
                x.clone()
            } else {
                acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect()
            }
        })
        .iter().map(|v| *v >= threshold)
        .collect()
}

fn power_consumption(data: &str) -> usize {
    let rows = parse_bits(data);
    let bits = most_common_bits(&rows);

    let gamma = bits_to_uint(&bits);
    let epsilon = bits_to_uint(&bits.iter().map(|x| !*x).collect());
    gamma * epsilon
}

// Reduce according to the criteria which I cba to explain https://adventofcode.com/2021/day/3
// flip_bit is used to enable the inverted criteria (least common bit)
fn bit_criteria_reduce(mut rows: Vec<Vec<usize>>, flip_bit: bool) -> usize {
    let mut bit_idx = 0;
    while rows.len() > 1 {
        let bits = most_common_bits(&rows);
        rows = rows.drain(..).filter(|x| x[bit_idx] == (flip_bit ^ bits[bit_idx]) as usize).collect();
        bit_idx += 1;
    }
    let bits = rows.get(0).expect("Expected at least one matching row - got none!")
        .iter().map(|x| *x > 0).collect();
    bits_to_uint(&bits)
}

fn life_support_rating(data: &str) -> usize {
    let rows = parse_bits(data);

    let oxygen_gen_rating = bit_criteria_reduce(rows.clone(), false);
    let co2_scrub_rating = bit_criteria_reduce(rows, true);
    oxygen_gen_rating * co2_scrub_rating
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    println!("Part 1 = {}", power_consumption(&contents));
    println!("Part 2 = {}", life_support_rating(&contents));
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
        assert_eq!(230, life_support_rating(sample));
    }
}