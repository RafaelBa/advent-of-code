use crate::read_input;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn solve_1() -> u32 {
    let contents = read_input::day(5);
    correct_middle_pages(contents.as_str())
}

pub fn solve_2() -> u32 {
    let contents = read_input::day(5);
    fixed_incorrect_middle_page(contents.as_str())
}

type Instructions<'a> = HashMap<&'a str, Vec<&'a str>>;
type Updates<'a> = Vec<Vec<&'a str>>;

pub fn correct_middle_pages(input: &str) -> u32 {
    let (instructions, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| check_update(update, &instructions))
        .map(|update| find_middle_page(update))
        .map(|page_number| page_number.parse::<u32>().unwrap())
        .sum()
}

pub fn fixed_incorrect_middle_page(input: &str) -> u32 {
    let (instructions, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| !check_update(update, &instructions))
        .map(|update| fix_update(update, &instructions))
        .map(|update| find_middle_page(&update))
        .map(|page_number| page_number.parse::<u32>().unwrap())
        .sum()
}

/// Parse string input to HashMap of instructions (X|Y) and list of updates (1,13,49,12)
/// For the following algorithm the HashMap of instruction is flipped,
/// so it is constructed as Y -> X
fn parse_input<'a>(input: &'a str) -> (Instructions<'a>, Updates<'a>) {
    let mut instructions: Instructions = HashMap::new();
    let mut updates: Updates = Vec::new();

    let instruction_re = Regex::new(r"([\d]+)\|([\d]+)").expect("Regex: instructions");
    let parse_updates = |s: &'a str| -> Vec<&'a str> { s.split(',').collect() };

    input.split('\n').for_each(|line| {
        if line.contains("|") {
            instruction_re
                .captures(line)
                .map(|xy| xy.extract())
                .map(|(_, [x, y])| {
                    let xs_o = instructions.get(y);
                    if xs_o.is_some() {
                        let mut xs = xs_o.unwrap().clone();
                        xs.push(x);
                        instructions.insert(y, xs);
                    } else {
                        instructions.insert(y, vec![x]);
                    }
                });
        }

        if line.contains(",") {
            updates.push(parse_updates(line));
        }
    });

    (instructions, updates)
}

fn check_update(update: &&Vec<&str>, instructions: &Instructions) -> bool {
    let mut invalid_update_list = (0..(update.len() - 1)).map(|i| {
        let v = update[i];
        let instruction_o = instructions.get(v);
        instruction_o
            .map(|inst| update[i..].iter().any(|u| inst.contains(u)))
            .unwrap_or(false)
    });

    // if there is `any` invalid_update in the list `true`, we return `false`, because the check is
    // invalid - not all update entries are valid
    !(invalid_update_list.any(|c| c))
}

fn find_middle_page<'a, 'b>(update: &'a Vec<&'b str>) -> &'b str {
    let pos = (update.len() - 1) / 2;
    update.iter().nth(pos).unwrap()
}

fn fix_update<'a, 'b>(
    update: &'b Vec<&'a str>,
    instructions: &'a Instructions<'a>,
) -> Vec<&'a str> {
    let mut fixed_update = update.clone();

    let are_in_wrong_order = |v1: &&str, v2: &&str| {
        instructions
            .get(v1)
            .map(|vs| vs.contains(v2))
            .unwrap_or(false)
    };

    fixed_update.sort_by(|a, b| {
        if are_in_wrong_order(a, b) {
            Ordering::Greater
        } else if are_in_wrong_order(b, a) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    fixed_update
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_middle_pages() {
        assert_eq!(correct_middle_pages(TEST_INPUT), 143);
    }

    #[test]
    fn test_find_middle_page() {
        let (instructions, updates) = parse_input(TEST_INPUT);

        let middle_pages: Vec<&str> = updates
            .iter()
            .filter(|update| check_update(update, &instructions))
            .map(|update| find_middle_page(update))
            .collect();

        let expected_middle_pages = vec!["61", "53", "29"];

        assert_eq!(middle_pages, expected_middle_pages)
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1(), 5762);
    }

    #[test]
    fn test_fixed_incorrect_middle_page() {
        assert_eq!(fixed_incorrect_middle_page(TEST_INPUT), 123);
    }

    #[test]
    fn test_fixed_incorrect_update_fix() {
        let (instructions, updates) = parse_input(TEST_INPUT);

        let actual: Vec<Vec<&str>> = updates
            .iter()
            .filter(|update| !check_update(update, &instructions))
            .map(|update| fix_update(update, &instructions))
            .collect();

        let expected = vec![
            vec!["97", "75", "47", "61", "53"],
            vec!["61", "29", "13"],
            vec!["97", "75", "47", "29", "13"],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fix_update() {
        let (instructions, _) = parse_input(TEST_INPUT);
        let result: Vec<Vec<&str>> = vec![
            vec!["75", "97", "47", "61", "53"],
            vec!["61", "13", "29"],
            vec!["97", "13", "75", "29", "47"],
        ]
        .iter()
        .map(|u| fix_update(u, &instructions))
        .collect();

        let expected = vec![
            vec!["97", "75", "47", "61", "53"],
            vec!["61", "29", "13"],
            vec!["97", "75", "47", "29", "13"],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(), 4130);
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
