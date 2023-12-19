// Day 19: Aplenty

use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<Input> {
    include_str!("./day19_input.txt").parse()
}

pub struct Day19;

impl Day for Day19 {
    fn day_number(&self) -> u8 {
        19
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            puzzle_input()?
                .accepted_parts_rating()
                .map(|it| it.to_string())
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RatingCategory {
    X,
    M,
    A,
    S,
}

impl RatingCategory {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'x' => Ok(Self::X),
            'm' => Ok(Self::M),
            'a' => Ok(Self::A),
            's' => Ok(Self::S),
            _ => Err(anyhow!("Invalid rating category: {}", c)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl Part {
    fn get(&self, category: RatingCategory) -> u32 {
        match category {
            RatingCategory::X => self.x,
            RatingCategory::M => self.m,
            RatingCategory::A => self.a,
            RatingCategory::S => self.s,
        }
    }

    fn rating(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
    }
}

lazy_static! {
    static ref PART_REGEX: Regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = PART_REGEX
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid part: {}", s))?;

        let x = captures[1].parse()?;
        let m = captures[2].parse()?;
        let a = captures[3].parse()?;
        let s = captures[4].parse()?;
        Ok(Self { x, m, a, s })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Outcome {
    Workflow(String),
    Accept,
    Reject,
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            _ => Ok(Self::Workflow(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    category: RatingCategory,
    comparison: Ordering,
    value: u32,
    outcome: Outcome,
}

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^([xmas])([<>=])(\d+):([a-zA-Z]+)$").unwrap();
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = RULE_REGEX
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid rule: {}", s))?;
        let category = RatingCategory::from_char(captures[1].chars().next().unwrap())?;
        let comparison = match &captures[2] {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            "=" => Ordering::Equal,
            _ => return Err(anyhow!("Invalid comparison: {}", &captures[2])),
        };
        let value = captures[3].parse()?;
        let outcome = captures[4].parse()?;
        Ok(Self {
            category,
            comparison,
            value,
            outcome,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: Outcome,
}

impl Workflow {
    fn follow(&self, part: &Part) -> Outcome {
        for rule in &self.rules {
            let value = part.get(rule.category);
            if rule.comparison == value.cmp(&rule.value) {
                return rule.outcome.clone();
            }
        }
        self.fallback.clone()
    }
}

lazy_static! {
    static ref WORKFLOW_REGEX: Regex =
        Regex::new(r"^([a-z]+)\{([a-zA-Z0-9<>=:,]+),([a-zA-Z]+)\}$").unwrap();
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let captures = WORKFLOW_REGEX
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid workflow: {}", s))?;
        let name = captures[1].to_string();
        let rules = captures[2]
            .split(',')
            .map(|rule| rule.parse())
            .collect::<Result<_>>()?;
        let fallback = captures[3].parse()?;
        Ok(Self {
            name,
            rules,
            fallback,
        })
    }
}

#[derive(Clone)]
struct WorkflowSeries {
    workflows: HashMap<String, Workflow>,
}

impl WorkflowSeries {
    fn follow(&self, part: &Part) -> Result<Outcome> {
        let mut outcome = Outcome::Workflow("in".to_string());
        while let Outcome::Workflow(workflow_name) = outcome {
            let workflow = self
                .workflows
                .get(&workflow_name)
                .ok_or(anyhow!("Workflow not found: {}", workflow_name))?;
            outcome = workflow.follow(part);
        }
        Ok(outcome)
    }
}

impl FromStr for WorkflowSeries {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let workflows = s
            .lines()
            .map(|line| Workflow::from_str(line))
            .map_ok(|workflow| (workflow.name.clone(), workflow))
            .collect::<Result<_>>()?;

        Ok(Self { workflows })
    }
}

#[derive(Clone)]
struct Input {
    workflows: WorkflowSeries,
    parts: Vec<Part>,
}

impl Input {
    fn accepted_parts_rating(&self) -> Result<u64> {
        self.parts
            .iter()
            .map(|part| {
                let accepted = self
                    .workflows
                    .follow(part)
                    .map(|outcome| outcome == Outcome::Accept)?;
                Ok((part, accepted))
            })
            .filter_ok(|(_, accepted)| *accepted)
            .map_ok(|(part, _)| part.rating())
            .sum::<Result<u64>>()
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (workflows, parts) = s.split_once("\n\n").unwrap();
        let workflows = workflows.parse()?;
        let parts = parts
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_>>()?;

        Ok(Self { workflows, parts })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            super::Day19.part1().unwrap().unwrap(),
            "402185".to_string(),
        );
    }

    fn sample_input() -> Input {
        indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "}
        .parse()
        .unwrap()
    }

    #[test]
    fn parse_input() {
        let input = sample_input();
        assert_eq!(input.parts.len(), 5);
        assert_eq!(input.workflows.workflows.len(), 11);
    }

    #[test]
    fn test_workflow() {
        let workflow = Workflow::from_str("ex{x>10:one,m<20:two,a>30:R,A}").unwrap();
        assert_eq!(
            workflow.follow(&Part {
                x: 11,
                m: 0,
                a: 0,
                s: 0
            }),
            Outcome::Workflow("one".to_string())
        );
        assert_eq!(
            workflow.follow(&Part {
                x: 0,
                m: 19,
                a: 0,
                s: 0
            }),
            Outcome::Workflow("two".to_string())
        );
        assert_eq!(
            workflow.follow(&Part {
                x: 0,
                m: 21,
                a: 31,
                s: 0
            }),
            Outcome::Reject
        );
        assert_eq!(
            workflow.follow(&Part {
                x: 0,
                m: 21,
                a: 0,
                s: 0
            }),
            Outcome::Accept
        );
    }

    #[test]
    fn test_workflow_series() {
        let input = sample_input();
        assert_eq!(
            input.workflows.follow(&input.parts[0]).unwrap(),
            Outcome::Accept
        );
        assert_eq!(
            input.workflows.follow(&input.parts[1]).unwrap(),
            Outcome::Reject
        );
        assert_eq!(
            input.workflows.follow(&input.parts[2]).unwrap(),
            Outcome::Accept
        );
        assert_eq!(
            input.workflows.follow(&input.parts[3]).unwrap(),
            Outcome::Reject
        );
        assert_eq!(
            input.workflows.follow(&input.parts[4]).unwrap(),
            Outcome::Accept
        );
    }

    #[test]
    fn test_accepted_parts_rating() {
        let input = sample_input();
        assert_eq!(input.accepted_parts_rating().unwrap(), 19114);
    }
}
