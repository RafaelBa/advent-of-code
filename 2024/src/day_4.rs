pub fn solve_1() -> u32 {
    count_xmas(todo!());
    todo!()
}

pub fn count_xmas(word_serach: &str) -> u32 {
    let matrix: Vec<Vec<char>> = word_serach
        .split("\n")
        .map(|s| s.chars().collect())
        .collect();
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
