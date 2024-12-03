use regex::Regex;

pub fn solve1() -> u32 {
    todo!();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(self::decorrupt_memory(test_input), 161);
    }
}
