// Day 20: Pulse Propagation

use std::collections::{HashMap, VecDeque};
use std::fmt::Write;
use std::str::FromStr;

use crate::framework::Day;
use crate::prelude::*;

fn puzzle_input() -> Result<ModuleConfiguration> {
    include_str!("./day20_input.txt").parse()
}

pub struct Day20;

impl Day for Day20 {
    fn day_number(&self) -> u8 {
        20
    }

    fn part1(&self) -> Option<Result<String>> {
        Some(try_block(move || {
            let (low, high) = puzzle_input()?
                .into_state()
                .tally_low_high_pulses_after_button_pressed(1000)?;
            let result = low * high;
            result.to_string().pipe(Ok)
        }))
    }

    fn part2(&self) -> Option<Result<String>> {
        if cfg!(feature = "slow_solutions") {
            Some(try_block(move || {
                let result = puzzle_input()?
                    .into_state()
                    .find_button_presses_until_target("rx")?;
                result.to_string().pipe(Ok)
            }))
        } else {
            None
        }
    }

    fn run_script(&self, name: &str) -> anyhow::Result<bool> {
        if name == "mermaid_diagram" {
            let configuration = indoc! {"
                broadcaster -> a
                %a -> inv, con
                &inv -> b
                %b -> con
                &con -> output
            "}
            .parse::<ModuleConfiguration>()?;
            println!("{}", configuration.as_mermaid_diagram());
            return Ok(true);
        }

        return Ok(false);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    /// "%"
    FlipFlop,
    /// "&"
    Conjunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleDefinition {
    module_id: String,
    module_type: ModuleType,
    destination_modules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TargetedPulse {
    source_module: String,
    pulse: bool,
    destination_module: String,
}
impl TargetedPulse {
    fn new(
        source_module: impl Into<String>,
        pulse: bool,
        destination_module: impl Into<String>,
    ) -> Self {
        Self {
            source_module: source_module.into(),
            pulse,
            destination_module: destination_module.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleState {
    Broadcaster,
    /// on/off
    FlipFlop(bool),
    /// last pulse for each input
    Conjunction(HashMap<String, bool>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    definition: ModuleDefinition,
    state: ModuleState,
}

impl Module {
    fn from_definition(definition: ModuleDefinition, input_ids: &[String]) -> Self {
        let state = match definition.module_type {
            ModuleType::Broadcaster => ModuleState::Broadcaster,
            ModuleType::FlipFlop => ModuleState::FlipFlop(false),
            ModuleType::Conjunction => ModuleState::Conjunction(
                input_ids.iter().map(|id| (id.to_owned(), false)).collect(),
            ),
        };
        Module { definition, state }
    }

    fn process_pulse(&mut self, input_pulse: TargetedPulse) -> Vec<TargetedPulse> {
        let output_pulse = match &mut self.state {
            ModuleState::Broadcaster => Some(input_pulse.pulse),
            ModuleState::FlipFlop(on_off) => {
                if input_pulse.pulse {
                    None
                } else {
                    *on_off = !*on_off;
                    Some(*on_off)
                }
            }
            ModuleState::Conjunction(last_pulse_for_inputs) => {
                last_pulse_for_inputs.insert(input_pulse.source_module, input_pulse.pulse);
                let remember_high_pulses = last_pulse_for_inputs.values().all(|&it| it == true);
                Some(!remember_high_pulses)
            }
        };
        let output_pulse = match output_pulse {
            Some(it) => it,
            None => return vec![],
        };
        let send_pulses = self
            .definition
            .destination_modules
            .iter()
            .map(|destination_module| TargetedPulse {
                source_module: self.definition.module_id.to_owned(),
                destination_module: destination_module.to_owned(),
                pulse: output_pulse,
            })
            .collect();
        send_pulses
    }
}

impl FromStr for ModuleDefinition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (name, destinations) = s
            .split_once(" -> ")
            .ok_or(anyhow!("Invalid module: {}", s))?;
        let first_char = name.chars().next();
        let module_type = match first_char {
            Some('%') => ModuleType::FlipFlop,
            Some('&') => ModuleType::Conjunction,
            _ => ModuleType::Broadcaster,
        };
        let module_id = if module_type == ModuleType::Broadcaster {
            name.to_string()
        } else {
            name[1..].to_string()
        };
        let destination_modules = destinations
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Ok(ModuleDefinition {
            module_id,
            module_type,
            destination_modules,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleConfiguration {
    module_definitions: HashMap<String, ModuleDefinition>,
}

impl ModuleConfiguration {
    fn into_state(self) -> ModuleConfigurationState {
        let modules = self
            .module_definitions
            .iter()
            .map(|(key, definition)| {
                let inputs = self
                    .module_definitions
                    .values()
                    .filter(|module| module.destination_modules.contains(&key))
                    .map(|module| module.module_id.to_owned())
                    .collect_vec();
                (
                    key.to_owned(),
                    Module::from_definition(definition.to_owned(), &inputs),
                )
            })
            .collect();
        ModuleConfigurationState { modules }
    }

    fn as_mermaid_diagram(&self) -> String {
        let mut result = String::new();
        writeln!(result, "graph TD;").unwrap();
        for module_def in self.module_definitions.values() {
            let module_id = &module_def.module_id;
            let module_name = match module_def.module_type {
                ModuleType::Broadcaster => None,
                ModuleType::FlipFlop => Some(format!("%{}", module_id)),
                ModuleType::Conjunction => Some(format!("&{}", module_id)),
            };
            let module_node = if let Some(module_name) = module_name {
                format!("{}[{}]", module_id, module_name)
            } else {
                module_id.to_owned()
            };
            for destination in &module_def.destination_modules {
                writeln!(result, "  {} --> {}", module_node, destination).unwrap();
            }
        }
        result
    }
}

impl FromStr for ModuleConfiguration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let module_definitions = s
            .lines()
            .map(|line| ModuleDefinition::from_str(line))
            .map_ok(|line| (line.module_id.clone(), line))
            .collect::<Result<_>>()?;
        Ok(ModuleConfiguration { module_definitions })
    }
}

#[derive(Clone)]
struct ModuleConfigurationState {
    modules: HashMap<String, Module>,
}

impl ModuleConfigurationState {
    fn push_button<FPulse>(&mut self, mut on_pulse: FPulse) -> Result<()>
    where
        FPulse: FnMut(&TargetedPulse),
    {
        let mut queue = VecDeque::new();
        queue.push_back(TargetedPulse::new("button", false, "broadcaster"));
        while let Some(input_pulse) = queue.pop_front() {
            on_pulse(&input_pulse);
            let module = self.modules.get_mut(&input_pulse.destination_module);
            if let Some(module) = module {
                // it's OK to send pulses to a module that doesn't exist
                let response_pulses = module.process_pulse(input_pulse);
                for pulse in response_pulses {
                    queue.push_back(pulse);
                }
            }
        }
        Ok(())
    }

    #[cfg(test)]
    fn push_button_record_pulses(&mut self) -> Result<Vec<TargetedPulse>> {
        let mut all_pulses = vec![];
        self.push_button(|pulse| all_pulses.push(pulse.clone()))?;
        Ok(all_pulses)
    }

    fn tally_low_high_pulses_after_button_pressed(
        &mut self,
        button_presses: usize,
    ) -> Result<(usize, usize)> {
        let mut low = 0;
        let mut high = 0;
        for _ in 0..button_presses {
            self.push_button(|pulse| match pulse.pulse {
                false => {
                    low += 1;
                }
                true => {
                    high += 1;
                }
            })?
        }
        Ok((low, high))
    }

    fn find_button_presses_until_target(&mut self, target: &str) -> Result<usize> {
        let mut button_presses = 0;
        let mut found = false;
        while !found {
            button_presses += 1;
            self.push_button(|pulse| {
                if pulse.pulse == false && pulse.destination_module == target {
                    found = true;
                }
            })?
        }
        Ok(button_presses)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            super::Day20.part1().unwrap().unwrap(),
            "787056720".to_string(),
        );
    }

    #[test]
    fn test_parse_module_definition() {
        let input = "%a -> b, c";
        let expected = ModuleDefinition {
            module_id: "a".to_string(),
            module_type: ModuleType::FlipFlop,
            destination_modules: vec!["b".to_string(), "c".to_string()],
        };
        assert_eq!(ModuleDefinition::from_str(input).unwrap(), expected);
    }

    #[test]
    fn test_inverter() {
        let configuration = indoc! {"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "}
        .parse::<ModuleConfiguration>()
        .unwrap();

        let expected = vec![
            TargetedPulse::new("button", false, "broadcaster"),
            TargetedPulse::new("broadcaster", false, "a"),
            TargetedPulse::new("broadcaster", false, "b"),
            TargetedPulse::new("broadcaster", false, "c"),
            TargetedPulse::new("a", true, "b"),
            TargetedPulse::new("b", true, "c"),
            TargetedPulse::new("c", true, "inv"),
            TargetedPulse::new("inv", false, "a"),
            TargetedPulse::new("a", false, "b"),
            TargetedPulse::new("b", false, "c"),
            TargetedPulse::new("c", false, "inv"),
            TargetedPulse::new("inv", true, "a"),
        ];

        let mut state = configuration.clone().into_state();

        let first_sequence = state.push_button_record_pulses().unwrap();

        assert_eq!(first_sequence, expected);

        let (low, high) = configuration
            .clone()
            .into_state()
            .tally_low_high_pulses_after_button_pressed(1000)
            .unwrap();
        assert_eq!(low, 8000);
        assert_eq!(high, 4000);
    }

    #[test]
    fn test_conjunction() {
        let configuration = indoc! {"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "}
        .parse::<ModuleConfiguration>()
        .unwrap();
        let mut state = configuration.clone().into_state();

        fn only_output(pulse: TargetedPulse) -> Option<bool> {
            if pulse.destination_module == "output" {
                Some(pulse.pulse)
            } else {
                None
            }
        }
        let expected_1 = vec![true, false];
        let expected_2 = vec![true];
        let expected_3 = vec![false, true];
        let expected_4 = vec![true];

        let actual_1: Vec<bool> = state
            .push_button_record_pulses()
            .unwrap()
            .into_iter()
            .filter_map(only_output)
            .collect();
        assert_eq!(actual_1, expected_1);

        let actual_2: Vec<bool> = state
            .push_button_record_pulses()
            .unwrap()
            .into_iter()
            .filter_map(only_output)
            .collect();
        assert_eq!(actual_2, expected_2);

        let actual_3: Vec<bool> = state
            .push_button_record_pulses()
            .unwrap()
            .into_iter()
            .filter_map(only_output)
            .collect();
        assert_eq!(actual_3, expected_3);

        let actual_4: Vec<bool> = state
            .push_button_record_pulses()
            .unwrap()
            .into_iter()
            .filter_map(only_output)
            .collect();
        assert_eq!(actual_4, expected_4);

        let actual_5: Vec<bool> = state
            .push_button_record_pulses()
            .unwrap()
            .into_iter()
            .filter_map(only_output)
            .collect();
        assert_eq!(actual_5, expected_1);

        let (low, high) = configuration
            .clone()
            .into_state()
            .tally_low_high_pulses_after_button_pressed(1000)
            .unwrap();
        assert_eq!(low, 4250);
        assert_eq!(high, 2750);
    }
}
