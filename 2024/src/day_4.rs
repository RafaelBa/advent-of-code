pub fn solve_1() -> u32 {
    todo!()
}

pub fn count_xmas(word_serach: &str) -> u32 {
    let matrix: Vec<Vec<char>> = word_serach
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();
    let rows = matrix.len();
    let columns = matrix[0].len();

    let r = matrix
        .iter()
        .enumerate()
        .map(|(i, cs)| {
            return cs
                .iter()
                .enumerate()
                .map(|(j, x)| {
                    if (*x == X) {
                        println!("found an X")
                    }
                    let row = if checkRow() { 1 } else { 0 };
                    let diagonal = if checkDiagonal() { 1 } else { 0 };
                    let column = if checkColumn() { 1 } else { 0 };

                    return row + diagonal + column;
                })
                .sum();
        })
        .sum();
    return r;
}

fn checkRow(
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
    letter: StartingLetter,
    matrix: &Vec<Vec<char>>,
) -> bool {
    // I can potentially do a matrix[i][j] == X.first() && matrix[i][j+1] == X.second() … ||
    // matrix[i][j] == S.first() && matrix[i][j+1] == S.second() …

    if column < max_column {
        return matrix[row][column + 1] == letter.second();
    }
    return false;
}

static X: char = 'X';

enum StartingLetter {
    X,
    S,
}

impl StartingLetter {
    fn second(&self) -> char {
        match *self {
            StartingLetter::X => 'M',
            StartingLetter::S => 'A',
        }
    }

    fn third(&self) -> char {
        match *self {
            StartingLetter::X => 'A',
            StartingLetter::S => 'M',
        }
    }

    fn fourth(&self) -> char {
        match *self {
            StartingLetter::X => 'S',
            StartingLetter::S => 'X',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_xmas(TEST1), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(count_xmas(TEST2), 18);
    }
    static TEST1: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    static TEST2: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
}
