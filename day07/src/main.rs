fn parse_crab_positions(input: &str) -> Vec<i32> {
    input.split(",")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn fuel_usage(crabs: &Vec<i32>, min_fuel_pos: i32) -> i32 {
    crabs.iter().map(|pos| (pos - min_fuel_pos).abs()).sum()
}

fn find_min_fuel_pos(mut crabs: Vec<i32>) -> (i32, i32) {
    // The minimum position must be on the middle crab
    assert!(crabs.len() > 0);
    crabs.sort();
    let min_pos = crabs[crabs.len() / 2];
    (min_pos, fuel_usage(&crabs, min_pos))
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    let crabs = parse_crab_positions(&contents);
    println!("Part 1 = {}", find_min_fuel_pos(crabs).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "16,1,2,0,4,2,7,1,2,14";
        let crabs = parse_crab_positions(sample);
        assert_eq!(fuel_usage(&crabs, 2), 37);
        assert_eq!(find_min_fuel_pos(crabs), (2, 37));
    }
}
