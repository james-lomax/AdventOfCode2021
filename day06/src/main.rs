fn parse_int_list(input: &str) -> Vec<u32> {
    input.split(',')
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn make_fish_counts(fishes: &Vec<u32>) -> [usize; 9] {
    let mut fish_counts = [0; 9];
    for fish in fishes {
        assert!(*fish >= 0 && *fish < 7);
        fish_counts[*fish as usize] += 1;
    }
    fish_counts
}

// Simulates fish described in counts by age:
// fish_counts represents list where index is age (0-8) and value is
// count of fish that age
fn simulate(mut fish_counts: [usize; 9], days: usize) -> [usize; 9] {
    for day_count in 0..days {
        let mut next_counts = [0; 9];
        for idx in 0..8 {
            next_counts[idx] = fish_counts[idx + 1];
        }

        // Each fish spawns a new one when it hits 0.
        next_counts[6] = next_counts[6].checked_add(fish_counts[0]).unwrap();
        next_counts[8] = next_counts[8].checked_add(fish_counts[0]).unwrap();

        fish_counts = next_counts;
    }
    fish_counts
}

fn p1_simulate_count(input: &str, days: usize) -> usize {
    let fishes = parse_int_list(input);
    let fish_counts = make_fish_counts(&fishes);
    simulate(fish_counts, days).iter().sum()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    println!("Part 1 = {}", p1_simulate_count(&contents, 80));
    println!("Part 2 = {}", p1_simulate_count(&contents, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sample = "3,4,3,1,2";
        assert_eq!(p1_simulate_count(sample, 18), 26);
        assert_eq!(p1_simulate_count(sample, 80), 5934);
    }
}