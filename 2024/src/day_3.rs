use regex::Regex;
use std::fs;

pub fn solve_1() -> u32 {
    let contents = get_file_content();
    decorrupt_memory(contents.as_str())
}

pub fn solve_2() -> u32 {
    let contents = get_file_content();
    decorrupt_enabled_memory(contents.as_str())
}

pub fn decorrupt_memory(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap();
    let result: u32 = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [left, right]) = caps.extract();
            left.parse::<u32>()
                .expect(format!("left is not a number: {left}").as_str())
                * right
                    .parse::<u32>()
                    .expect(format!("right is not a number: {right}").as_str())
        })
        .sum();
    return result;
}

pub fn decorrupt_enabled_memory(input: &str) -> u32 {
    let sanitised: String = sanitise_disabled_memory(String::from(input));
    decorrupt_memory(sanitised.as_str())
}

/// Remove all sections that are disabled, ie. between a don't() and do()
fn sanitise_disabled_memory(input: String) -> String {
    // Regex picking up don't()<everything>do() - important here is the .*?, because the ? after
    // the * makes it _lazy_ (or ungreedy), meaning that it stops at the first do(), instead of
    // getting as many do()s as it can
    let re = Regex::new(r"don't\(\).*?do\(\)").expect("Regex: Replace all");
    re.replace_all(input.as_str(), "").into_owned()
}

fn get_file_content() -> String {
    fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file")
        .replace("\n", "")
}

static FILE_PATH: &str = "src/day_3-input.txt";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let test_input = TEST_INPUT_1;
        assert_eq!(self::decorrupt_memory(test_input), 161);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(), 159892596);
    }

    #[test]
    fn test_part_2_example() {
        let test_input = TEST_INPUT_2;
        assert_eq!(self::decorrupt_enabled_memory(test_input), 48);
    }

    #[test]
    fn test_part_2_test_2() {
        let test_input = "mul(2,3)don't()mul(2,3)mul(4,5)do()mul(2,8)do()mul(3,4)don't()";
        assert_eq!(self::decorrupt_enabled_memory(test_input), 34);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(), 92626942);
    }

    static TEST_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
}
