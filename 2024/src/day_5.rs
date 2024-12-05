use std::collections::HashMap;

pub fn solve_1() -> u32 {
    todo!()
}

pub fn solve_2() -> u32 {
    todo!()
}

type Instructions<'a> = HashMap<&'a str, &'a str>;
type Updates<'a> = Vec<Vec<&'a str>>;

pub fn correct_middle_pages(input: &str) -> u32 {
    let (instructions, updates) = parse_input(input);
    // TODO: create a partially applied function of `check_update(_, instructions)`
    // TODO: check of I can pass .map(find_middle_page) and .map(_.parse::<u32>().unwrap())
    updates
        .iter()
        .filter(|update| check_update(update, instructions))
        .map(|update| find_middle_page(update))
        .map(|page_number| page_number.parse::<u32>().unwrap())
        .sum()
}

fn parse_input(input: &str) -> (Instructions, Updates) {
    // TODO: instructions as HashMap of left and right page
    // see `check_update` for the reasons, but the HashMap should be Y -> X, so flipped around
    // updates should be Vec<Vec<&str>>
    // there's no need for parsing to numbers, since we don't do any calculations
    todo!()
}

fn check_update(update: Vec<&str>, instructions: HashMap<&str, &str>) -> bool {
    // TODO: check the update list front to back - since "page X must be before page Y _iff_ it exists"
    // it means that X must not be after Y - this is way easier to check
    // So the HashMap should be Y -> X and the check would be
    // values.map(|v, rest| !rest.contains(instructions.get(v)))
    todo!()
}

fn find_middle_page(update: Vec<&str>) -> &str {
    let pos = (update.len() - 1) / 2;
    update.iter().nth(pos).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_middle_pages() {
        assert_eq!(correct_middle_pages(TEST_INPUT), 143);
    }

    static TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
}
