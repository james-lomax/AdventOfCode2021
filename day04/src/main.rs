use std::collections::HashSet;

const BOARD_WIDTH: usize = 5;

struct BingoCard {
    grid: Vec<Vec<u32>>,
    // List of rows and collumns
    lines: Vec<HashSet<u32>>,
}

impl BingoCard {
    fn parse(contents: &str) -> Self {
        // Parse into 5x5 grid
        let grid = contents
            .split('\n')
            .map(|line| {
                line.split(' ')
                    .map(|s| s.trim())
                    .filter(|s| s.len() > 0)
                    .map(|s| s.parse::<u32>().expect("Failed to parse input integer"))
                    .collect::<Vec<u32>>()
            })
            // Precondition: all rows are correct width
            .filter(|row| row.len() == BOARD_WIDTH)
            .collect::<Vec<Vec<u32>>>();

        // Precondition: all rows are correct width
        if grid.len() != BOARD_WIDTH {
            panic!(
                "Failed precondition: board width/height not {}",
                BOARD_WIDTH
            );
        }

        // Create sets for the rows and columns
        let lines = grid
            .iter()
            .map(|row| HashSet::from_iter(row.iter().cloned()))
            .chain(
                // Take nth item from each row for n=0..5
                (0..BOARD_WIDTH).map(|index| HashSet::from_iter(grid.iter().map(|row| row[index]))),
            )
            // Precondition: no repeated numbers
            .filter(|set| set.len() == BOARD_WIDTH)
            .collect::<Vec<HashSet<u32>>>();

        // Precondition: no repeated numbers
        if lines.len() != 2 * BOARD_WIDTH {
            panic!("Failed precondition: numebrs in board rows/columns not unique");
        }

        Self {
            grid: grid,
            lines: lines,
        }
    }

    fn is_bingo(&self, called: &HashSet<u32>) -> bool {
        for line in self.lines.iter() {
            if line.is_subset(called) {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self, called: &HashSet<u32>) -> u32 {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|n| !called.contains(n)).sum::<u32>())
            .sum()
    }
}

fn parse_input(contents: &str) -> (Vec<u32>, Vec<BingoCard>) {
    let mut blocks = contents
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .collect::<Vec<&str>>();

    let drawn = blocks[0]
        .split(',')
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().expect("Failed to parse input integer"))
        .collect::<Vec<u32>>();

    let bingo_cards = blocks
        .drain(1..)
        .map(BingoCard::parse)
        .collect::<Vec<BingoCard>>();
    (drawn, bingo_cards)
}

fn p1_winning_board_score(contents: &str) -> u32 {
    let (drawn, boards) = parse_input(contents);
    let mut called = HashSet::<u32>::from_iter(drawn.iter().take(BOARD_WIDTH - 1).cloned());
    // called contains first 4 elements
    // so we can start each iteration by adding the next number
    for num in drawn.iter().skip(BOARD_WIDTH - 1) {
        called.insert(*num);

        for board in boards.iter() {
            if board.is_bingo(&called) {
                return board.unmarked_sum(&called) * num;
            }
        }
    }

    panic!("Nobody won. Wtf");

    // For each board, form the set of sets of rows and columns
    // Then iterate over each successively bigger set of input numbers
    // (starting at 5 - the minimum required for a win) and look
    // for the set with a row/column set which is a subset of the input set

    // For the score, the unmarked numbers can be found by a union of all
    // row/column sets intersected with called numbers

    // The above implies uniqueness is a constraint
    // - cant imagine why a bingo board wouldnt be unique
    // Even so, this precondition must be asserted on set creation using the set sizes
}

// Or more precisely: last to win board score
fn p2_losing_board_score(contents: &str) -> u32 {
    let (drawn, mut boards) = parse_input(contents);
    let mut called = HashSet::<u32>::from_iter(drawn.iter().take(BOARD_WIDTH - 1).cloned());

    for num in drawn.iter().skip(BOARD_WIDTH - 1) {
        called.insert(*num);

        // Have to use old fashion iterating
        // in order to remove from list while iterating
        let mut i = 0;
        while i < boards.len() {
            if boards[i].is_bingo(&called) {
                if boards.len() == 1 {
                    return boards[i].unmarked_sum(&called) * num;
                } else {
                    boards.remove(i);
                }
            } else {
                i += 1;
            }
        }
    }

    panic!("Nobody won. Wtf");
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    println!("Part 1 = {}", p1_winning_board_score(&contents));
    println!("Part 2 = {}", p2_losing_board_score(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let sample = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

        let (drawn, boards) = parse_input(sample);
        assert_eq!(drawn[0..3], [7, 4, 9]);
        assert_eq!(boards.len(), 3);
        assert_eq!(boards[0].lines.len(), 10);
        assert_eq!(boards[0].lines[5], HashSet::from([22, 8, 21, 6, 1]));

        assert_eq!(p1_winning_board_score(sample), 4512);
        assert_eq!(p2_losing_board_score(sample), 1924);
    }
}
