/// Treat bottom left as 0, 0
/// TODO: check if u32 can be optimised
struct Position {
    x: u32,
    y: u32,
}

pub fn solve_1() -> u32 {
    todo!()
}

pub fn solve_2() -> u32 {
    todo!()
}

fn count_visited_positions(input: &str) -> u32 {
    let (start_position, obstacles) = parse_field(input);
    todo!()
}

fn parse_field(input: &str) -> (Position, Vec<Position>) {
    let mut guard: Option<Position> = None;
    let mut obstacles: Vec<Position> = vec![];

    // TODO: I need to make sure that I don't have an extra line at the bottom (or top as a matter
    // of fact) when parsing the file; an extra empty line can screw the perception of borders.
    // TODO: I need to know where the borders are
    for (line_number, line_content) in input.split('\n').enumerate() {
        for (character_position, character) in line_content.chars().enumerate() {
            // TODO: Saving the position like this has the field updside down; I'm scanning top to
            // bottom, first line having index 0, therefore 0 being at the top, not as planned and
            // defined at the bottom
            if character == '#' {
                obstacles.push(Position {
                    x: character_position,
                    y: line_number,
                })
            }
            // TODO: I also need to save the direction the guard's facing
            if character == '^' {
                todo!()
            }
        }
    }

    (guard.expect("Guard was not found!"), obstacles)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {
        let actual = count_visited_positions(TEST_INPUT);
        assert_eq!(actual, 41);
    }

    static TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
}
