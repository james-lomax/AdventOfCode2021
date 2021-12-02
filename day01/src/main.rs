use itertools::zip;

fn parse_depths(input: &str) -> Vec<u32> {
    input.split('\n')
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn count_increases<I>(depths: I) -> usize 
    where I: Iterator<Item = u32> + Clone
{
    zip(depths.clone(), depths.skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

fn count_window_increases(depths: &Vec<u32>) -> usize {
    count_increases(
        zip(zip(depths, &depths[1..]), &depths[2..])
            .map(|((a, b), c)| a + b + c))
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    let depths = parse_depths(&contents);
    println!("Part 1 = {}", count_increases(depths.iter().cloned()));
    println!("Part 2 = {}", count_window_increases(&depths));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let sample = "199
            200
            208
            210
            200
            207
            240
            269
            260
            263";
        let depths = parse_depths(sample);
        assert_eq!(7, count_increases(depths.iter().cloned()));
        assert_eq!(5, count_window_increases(&depths));
    }
}