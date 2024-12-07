use std::collections::HashMap;

use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

type Rules = HashMap<i32, Vec<i32>>;
type Update = Vec<i32>;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (rules, updates)) =
        parse_input(input).map_err(|e| miette!("failed to parse input {}", e))?;

    let valid_updates = updates
        .iter()
        .filter(|&update| is_valid_update(update, &rules))
        .collect::<Vec<_>>();

    Ok(valid_updates
        .iter()
        .map(|&u| u[u.len() / 2])
        .sum::<i32>()
        .to_string())
}

fn is_valid_update(update: &Update, rules: &Rules) -> bool {
    let mut index = 0;
    let mut current_page;
    let mut pages_before;
    while index < update.len() {
        current_page = update[index];
        pages_before = &update[0..index];
        if let Some(pages_that_must_be_after) = rules.get(&current_page) {
            if pages_that_must_be_after
                .iter()
                .any(|page| pages_before.contains(page))
            {
                return false;
            }
        }
        index += 1;
    }
    true
}

fn parse_input(input: &str) -> IResult<&str, (Rules, Vec<Update>)> {
    separated_pair(parse_rules, tag("\n"), parse_updates)(input)
}

fn parse_rules(input: &str) -> IResult<&str, Rules> {
    fold_many1(
        terminated(
            separated_pair(complete::i32, tag("|"), complete::i32),
            line_ending,
        ),
        HashMap::default,
        |mut acc: Rules, (page, after)| {
            acc.entry(page)
                .and_modify(|afters| {
                    afters.push(after);
                })
                .or_insert(vec![after]);
            acc
        },
    )(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
    separated_list1(line_ending, separated_list1(tag(","), complete::i32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
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
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
