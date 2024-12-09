use crate::read_input;
use regex::Regex;
use std::collections::HashMap;

pub fn solve_1() -> i32 {
    let (vec1, vec2) = read_day_1_input();
    return sum_of_pair_distances(vec1, vec2);
}

pub fn solve_2() -> i32 {
    let (vec1, vec2) = read_day_1_input();
    return similarity_score(vec1, vec2);
}

pub fn sum_of_pair_distances(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut vec1 = left.clone();
    let mut vec2 = right.clone();

    vec1.sort();
    vec2.sort();

    return vec1
        .iter()
        .zip(vec2.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
}

pub fn similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut right_count: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|n| {
        let value: Option<&i32> = right_count.get(n);
        match value {
            Some(count) => right_count.insert(*n, count + 1),
            None => right_count.insert(*n, 1),
        };
    });

    return left
        .iter()
        .map(|n| {
            let multiply = right_count.get(n).unwrap_or(&0);
            n * multiply
        })
        .sum();
}

fn read_day_1_input() -> (Vec<i32>, Vec<i32>) {
    let contents = read_input::day(1);
    let lines = contents.split("\n");
    let re = Regex::new(r"([\d]+) +([\d]+)").unwrap();

    let mut vec_left: Vec<i32> = Vec::new();
    let mut vec_right: Vec<i32> = Vec::new();

    lines.for_each(|s| {
        let caps_ops = re.captures(s);

        match caps_ops {
            Some(caps) => {
                let left = &caps[1].to_owned();
                let right = &caps[2];

                let left_number = left
                    .parse::<i32>()
                    .expect(format!("Expected number on left: {left}").as_str());

                let right_number = right
                    .parse::<i32>()
                    .expect(format!("Expected number on right: {right}").as_str());

                vec_left.push(left_number);
                vec_right.push(right_number);
            }
            _ => {}
        }
    });
    return (vec_left, vec_right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_pair_distances_with_test_data() {
        let vec1 = vec![3, 4, 2, 1, 3, 3];
        let vec2 = vec![4, 3, 5, 3, 9, 3];

        let result = sum_of_pair_distances(vec1, vec2);
        assert_eq!(result, 11);
    }

    #[test]
    fn sum_of_pair_distances_with_data_from_file() {
        let result = solve_1();
        assert_eq!(result, 2769675);
    }

    #[test]
    fn test_similarity_test_data() {
        let vec1 = vec![3, 4, 2, 1, 3, 3];
        let vec2 = vec![4, 3, 5, 3, 9, 3];

        let result = similarity_score(vec1, vec2);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_similarity_with_data_from_file() {
        assert_eq!(solve_2(), 24643097);
    }
}
