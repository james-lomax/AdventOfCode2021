use vector2d::Vector2D;

type Vec2i = Vector2D<i32>;


fn instruction_to_vec(line: &str) -> Result<Vec2i, String> {
    let mut split = line.split(" ");
    let direction = split.next().ok_or("Missing direction field".to_string())?;
    let distance = split.next().ok_or("Missing distance field".to_string())?;
    let distance = distance.parse::<i32>().ok().ok_or(format!("Invalid distance '{}' (must be integer)", distance))?;
    match direction {
        "forward" => Ok(Vec2i::new(distance, 0)),
        "down" => Ok(Vec2i::new(0, distance)),
        "up" => Ok(Vec2i::new(0, -distance)),
        _ => Err(format!("Unexpected direction '{}'", direction))
    }
}

fn run_program_p1(program_str: &str) -> i32 {
    let pos: Vec2i = program_str.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|l| instruction_to_vec(l).unwrap())
        .fold(Vec2i::new(0, 0), |acc, x| acc + x);
    pos.x * pos.y
}

fn run_program_p2(program_str: &str) -> i32 {
    let instructions = program_str.split("\n")
        .map(|s| s.trim()).filter(|s| s.len() > 0)
        .map(|l| instruction_to_vec(l).unwrap());
    let mut aim = Vec2i::new(0, 0);
    let mut pos = Vec2i::new(0, 0);

    for instruction in instructions {
        if instruction.y == 0 {
            // Forward
            pos += instruction + aim * instruction.x;
        } else {
            // Aim adjust
            aim += instruction;
        }
    }

    pos.x * pos.y
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file error");
    println!("Part 1 = {}", run_program_p1(&contents));
    println!("Part 2 = {}", run_program_p2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_instruction_parse() {
        assert_eq!(Ok(Vec2i::new(2, 0)), instruction_to_vec("forward 2"));
        assert_eq!(Ok(Vec2i::new(0, 2)), instruction_to_vec("down 2"));
        assert_eq!(Ok(Vec2i::new(0, -3)), instruction_to_vec("up 3"));
        assert_eq!(Err("Unexpected direction 'left'".to_string()), instruction_to_vec("left 5"));
    }

    #[test]
    fn test_program() {
        let sample = "forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2";
        assert_eq!(150, run_program_p1(sample));
        assert_eq!(900, run_program_p2(sample));
    }
}