use crate::read_input;

pub fn solve_1() -> u32 {
    let input = read_input::day(4);
    return count_xmas(input.as_str());
}

pub fn solve_2() -> u32 {
    let input = read_input::day(4);
    return count_x_mas(input.as_str());
}

pub fn count_xmas(word_serach: &str) -> u32 {
    let matrix: Vec<Vec<char>> = matrix_from_string(word_serach);
    let row_count = matrix.len();
    let column_count = matrix[0].len();

    let r = matrix
        .iter()
        .enumerate()
        .map(|(row_index, cs)| {
            cs.iter()
                .enumerate()
                .map(|(column_index, _)| {
                    let row = count_row(row_index, column_index, column_count, &matrix);
                    let diagonal: u32 =
                        count_diagonal(row_index, column_index, row_count, column_count, &matrix);
                    let column: u32 = count_column(row_index, column_index, row_count, &matrix);

                    row + diagonal + column
                })
                .sum::<u32>()
        })
        .sum();
    return r;
}

pub fn count_x_mas(word_search: &str) -> u32 {
    let matrix: Vec<Vec<char>> = matrix_from_string(word_search);
    let row_count = matrix.len();
    let column_count = matrix[0].len();

    let r = matrix
        .iter()
        .enumerate()
        .map(|(row_index, cs)| {
            cs.iter()
                .enumerate()
                .map(|(column_index, _)| {
                    x_mes_count(row_index, column_index, row_count, column_count, &matrix)
                })
                .sum::<u32>()
        })
        .sum();
    return r;
}

fn matrix_from_string(s: &str) -> Vec<Vec<char>> {
    return s
        .split("\n")
        .map(|s| s.chars().collect())
        .filter(|s: &Vec<char>| s.len() > 0)
        .collect();
}

fn count_row(row: usize, column: usize, max_column: usize, matrix: &Vec<Vec<char>>) -> u32 {
    let exists_forward = column + 3 < max_column
        && matrix[row][column] == 'X'
        && matrix[row][column + 1] == 'M'
        && matrix[row][column + 2] == 'A'
        && matrix[row][column + 3] == 'S';

    let exists_back = column >= 3
        && matrix[row][column] == 'X'
        && matrix[row][column - 1] == 'M'
        && matrix[row][column - 2] == 'A'
        && matrix[row][column - 3] == 'S';

    let forward: u32 = if exists_forward { 1 } else { 0 };
    let back: u32 = if exists_back { 1 } else { 0 };
    return forward + back;
}

fn count_diagonal(
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
    matrix: &Vec<Vec<char>>,
) -> u32 {
    let has_space_back = column >= 3;
    let has_space_up = row >= 3;
    let has_space_forward = column + 3 < max_column;
    let has_space_down = row + 3 < max_row;

    let exists_back_up = has_space_back
        && has_space_up
        && matrix[row][column] == 'X'
        && matrix[row - 1][column - 1] == 'M'
        && matrix[row - 2][column - 2] == 'A'
        && matrix[row - 3][column - 3] == 'S';
    let exists_forward_up = has_space_forward
        && has_space_up
        && matrix[row][column] == 'X'
        && matrix[row - 1][column + 1] == 'M'
        && matrix[row - 2][column + 2] == 'A'
        && matrix[row - 3][column + 3] == 'S';

    let exists_back_down = has_space_back
        && has_space_down
        && matrix[row][column] == 'X'
        && matrix[row + 1][column - 1] == 'M'
        && matrix[row + 2][column - 2] == 'A'
        && matrix[row + 3][column - 3] == 'S';

    let exists_forward_down = has_space_forward
        && has_space_down
        && matrix[row][column] == 'X'
        && matrix[row + 1][column + 1] == 'M'
        && matrix[row + 2][column + 2] == 'A'
        && matrix[row + 3][column + 3] == 'S';

    let back_up = if exists_back_up { 1 } else { 0 };
    let forward_up = if exists_forward_up { 1 } else { 0 };
    let back_down = if exists_back_down { 1 } else { 0 };
    let forward_down = if exists_forward_down { 1 } else { 0 };

    return back_up + forward_up + back_down + forward_down;
}

fn count_column(row: usize, column: usize, max_row: usize, matrix: &Vec<Vec<char>>) -> u32 {
    let exists_up = row >= 3
        && matrix[row][column] == 'X'
        && matrix[row - 1][column] == 'M'
        && matrix[row - 2][column] == 'A'
        && matrix[row - 3][column] == 'S';

    let exists_down = row + 3 < max_row
        && matrix[row][column] == 'X'
        && matrix[row + 1][column] == 'M'
        && matrix[row + 2][column] == 'A'
        && matrix[row + 3][column] == 'S';

    let up = if exists_up { 1 } else { 0 };
    let down = if exists_down { 1 } else { 0 };
    return up + down;
}

fn x_mes_count(
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
    matrix: &Vec<Vec<char>>,
) -> u32 {
    let has_space_back = column >= 1;
    let has_space_up = row >= 1;
    let has_space_forward = column + 1 < max_column;
    let has_space_down = row + 1 < max_row;
    let has_space = has_space_down && has_space_forward && has_space_back && has_space_up;

    let exists_backslash = has_space
        && matrix[row][column] == 'A'
        && (matrix[row - 1][column - 1] == 'M' && matrix[row + 1][column + 1] == 'S'
            || matrix[row - 1][column - 1] == 'S' && matrix[row + 1][column + 1] == 'M');

    let exists_slash = has_space
        && matrix[row][column] == 'A'
        && (matrix[row - 1][column + 1] == 'M' && matrix[row + 1][column - 1] == 'S'
            || matrix[row - 1][column + 1] == 'S' && matrix[row + 1][column - 1] == 'M');

    return if exists_backslash && exists_slash {
        1
    } else {
        0
    };
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

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(), 2547);
    }

    #[test]
    fn test_3() {
        assert_eq!(count_x_mas(TEST3), 9);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(), 1939);
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

    static TEST3: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
}
