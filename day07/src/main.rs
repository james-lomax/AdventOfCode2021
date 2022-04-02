fn parse_crab_positions(input: &str) -> Vec<i32> {
    input.split(",")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn find_min_fuel_p1(mut crabs: Vec<i32>) -> (i32, i32) {
    // The minimum position must be on the middle crab (median)
    assert!(crabs.len() > 0);
    crabs.sort();
    let min_pos = crabs[crabs.len() / 2];
    (min_pos, crabs.iter().map(|pos| (pos - min_pos).abs()).sum())
}

fn p2_fuel(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs.iter()
        .map(|p| (p - pos).abs())
        .map(|dist| (1..=dist).sum::<i32>())
        .sum()
}

fn find_min_fuel_p2(crabs: Vec<i32>) -> (i32, i32) {
    // Center position (smallest sums of distances squared) i.e. mean
    let min_pos: f64 = crabs.iter().map(|n| *n as f64).sum::<f64>() / (crabs.len() as f64);
    // I'm not sure why rounding doesn't work, but sometimes you have to try both
    let min_pos = min_pos.floor() as i32;
    let a = p2_fuel(&crabs, min_pos);
    let b = p2_fuel(&crabs, min_pos + 1);
    if a < b {
        (min_pos, a)
    } else {
        (min_pos + 1, b)
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    let crabs = parse_crab_positions(&contents);
    println!("Part 1 = {}", find_min_fuel_p1(crabs.clone()).1);
    println!("Part 2 = {}", find_min_fuel_p2(crabs).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample = "16,1,2,0,4,2,7,1,2,14";
        let crabs = parse_crab_positions(sample);
        assert_eq!(find_min_fuel_p1(crabs.clone()), (2, 37));
        assert_eq!(find_min_fuel_p2(crabs), (5, 168));
    }
}
