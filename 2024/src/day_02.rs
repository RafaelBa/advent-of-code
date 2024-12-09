use crate::read_input;

pub fn solve_1() -> u32 {
    let file_content = read_input::day(2);
    return count_safe_reports(file_content.as_str());
}

pub fn solve_2() -> u32 {
    let file_content = read_input::day(2);
    return count_safe_reports_with_dampener(file_content.as_str());
}

pub fn count_safe_reports(reports: &str) -> u32 {
    let lines = reports.split('\n');
    return lines
        .filter(|line| line.len() > 0)
        .map(|line| {
            let levels = get_levels(line);
            let is_safe = check_safety(&levels);
            if is_safe {
                1
            } else {
                0
            }
        })
        .sum();
}

pub fn count_safe_reports_with_dampener(reports: &str) -> u32 {
    let lines = reports.split('\n');
    return lines
        .filter(|line| line.len() > 0)
        .map(|line| {
            let levels = get_levels(line);
            let dampened_level_reports = get_dampened_reports(&levels);
            let is_safe = dampened_level_reports
                .iter()
                .any(|level_version| check_safety(level_version));
            if is_safe {
                1
            } else {
                0
            }
        })
        .sum();
}

fn get_levels(line: &str) -> Vec<u32> {
    let strings: Vec<&str> = line.split(' ').collect();
    let levels: Vec<u32> = strings
        .iter()
        .map(|s| {
            s.parse::<u32>()
                .expect(format!("'{s}' is not a number").as_str())
        })
        .collect();
    return levels;
}

fn check_safety(levels: &Vec<u32>) -> bool {
    let mut has_increase: bool = false;
    let mut has_decrease: bool = false;
    let mut has_invalid_change: bool = false;
    let n = levels.len();
    levels[0..(n - 1)]
        .iter()
        .zip(levels[1..n].iter())
        .for_each(|(current, next)| {
            let diff = abs_diff(current, next);
            if diff < 1 || diff > 3 {
                has_invalid_change = true;
            }
            // Technically this ignores the case if the numbers are equal, but this doesn't matter,
            // since this is an error case anyway
            if current < next {
                has_increase = true;
            } else {
                has_decrease = true;
            }
        });

    return (has_increase ^ has_decrease) && !has_invalid_change;
}

fn abs_diff(left: &u32, right: &u32) -> u32 {
    return if left < right {
        right - left
    } else {
        left - right
    };
}

fn get_dampened_reports(levels: &Vec<u32>) -> Vec<Vec<u32>> {
    let versions: Vec<Vec<u32>> = (0..levels.len())
        .map(|i| {
            let mut new_levels = levels.clone();
            new_levels.remove(i);
            new_levels
        })
        .collect();

    let mut result: Vec<Vec<u32>> = vec![levels.clone()];
    result.extend(versions);
    result.clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_safe_reports() {
        assert_eq!(count_safe_reports(TEST_INPUT), 2);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(), 421);
    }

    #[test]
    fn test_count_safe_reports_with_dampener() {
        assert_eq!(count_safe_reports_with_dampener(TEST_INPUT), 4);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(), 476);
    }

    static TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
