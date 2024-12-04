use regex::Regex;
use std::fs;

pub fn solve_1() -> i32 {
    let (vec1, vec2) = read_day_1_input();
    return sum_of_pair_distances(vec1, vec2);
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

fn read_day_1_input() -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
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

// "hard coded" file path, expecting you to run the program from the rust project's root
static FILE_PATH: &str = "src/day_1-input.txt";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_with_test_data() {
        let vec1 = vec![3, 4, 2, 1, 3, 3];
        let vec2 = vec![4, 3, 5, 3, 9, 3];

        let result = sum_of_pair_distances(vec1, vec2);
        assert_eq!(result, 11);
    }

    #[test]
    fn solution_with_data_from_file() {
        let result = solve_1();
        assert_eq!(result, 2769675);
    }
}
