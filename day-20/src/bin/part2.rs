use std::collections::{HashMap, VecDeque};

use nom::{
    bytes::complete,
    character::complete::{alpha1, multispace1, one_of},
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

const BUTTON_PRESSES: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    destinations: Vec<&'a str>,
    kind: ModuleKind<'a>,
}

#[derive(Debug)]
enum ModuleKind<'a> {
    Broadcaster,
    FlipFlop { is_on: bool },
    Conjunction { states: HashMap<&'a str, Pulse> },
}

impl<'a> ModuleKind<'a> {
    fn handle_pulse(&mut self, pulse: Pulse, name: &'a str) -> Option<Pulse> {
        match self {
            ModuleKind::Broadcaster => Some(pulse),
            ModuleKind::FlipFlop { ref mut is_on } => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    *is_on = !*is_on;
                    is_on.then_some(Pulse::High).or(Some(Pulse::Low))
                }
            },
            ModuleKind::Conjunction { ref mut states } => {
                states.insert(name, pulse);
                states
                    .values()
                    .all(|pulse| pulse == &Pulse::High)
                    .then_some(Pulse::Low)
                    .or(Some(Pulse::High))
            }
        }
    }
}

fn process(input: &str) -> String {
    let (_, module_list) = parse(input).unwrap();

    let mut conjuctions: HashMap<_, Vec<&str>> = module_list
        .iter()
        .filter_map(|module| {
            matches!(module.kind, ModuleKind::Conjunction { .. })
                .then_some((module.name, Vec::new()))
        })
        .collect();

    let mut modules: HashMap<&str, Module> = module_list
        .into_iter()
        .map(|module| (module.name, module))
        .collect();

    // Find all connecting inputs for every conjuction module
    for module in modules.values() {
        for destination in &module.destinations {
            if let Some(connections) = conjuctions.get_mut(destination) {
                connections.push(module.name);
            }
        }
    }
    // Default every connection module to Pulse::Low
    for (name, connections) in conjuctions {
        match modules.get_mut(name) {
            Some(Module {
                kind: ModuleKind::Conjunction { ref mut states },
                ..
            }) => {
                states.extend(
                    connections
                        .into_iter()
                        .map(|connection| (connection, Pulse::Low)),
                );
            }
            _ => unreachable!("Conjunctions should still be conjunctions"),
        };
    }
    dbg!(&modules);

    struct QueueItem<'a> {
        to: &'a str,
        from: &'a str,
        pulse: Pulse,
    }

    let mut queue: VecDeque<QueueItem> = VecDeque::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..BUTTON_PRESSES {
        println!("NEW ROUND HERE\n");
        // Button module pressed
        queue.push_back(QueueItem {
            from: "button",
            to: "broadcaster",
            pulse: Pulse::Low,
        });
        // dbg!(&queue, &modules);
        while let Some(QueueItem { to, from, pulse }) = queue.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }
            let Some(module) = modules.get_mut(to) else {
                continue;
            };
            let Some(pulse_to_send) = module.kind.handle_pulse(pulse, from) else {
                continue;
            };
            println!("{to} {pulse_to_send:?} -> {:?}", module.destinations);
            for destination in &module.destinations {
                queue.push_back(QueueItem {
                    from: module.name,
                    to: destination,
                    pulse: pulse_to_send,
                });
            }
        }

        // dbg!(&modules);
    }

    dbg!(low_pulses, high_pulses);

    (low_pulses * high_pulses).to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Module>> {
    separated_list1(
        multispace1,
        separated_pair(
            tuple((opt(one_of("%&")), alpha1)),
            complete::tag(" -> "),
            separated_list1(complete::tag(", "), alpha1),
        )
        .map(|((kind, name), destinations)| {
            let module_kind = match kind {
                Some('%') => ModuleKind::FlipFlop { is_on: false },
                Some('&') => ModuleKind::Conjunction {
                    states: HashMap::new(),
                },
                _ => ModuleKind::Broadcaster,
            };
            Module {
                name,
                kind: module_kind,
                destinations,
            }
        }),
    )
    .parse(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const ANSWER: &str = "32000000";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }

    const EXAMPLE1: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    const ANSWER1: &str = "11687500";

    #[test]
    fn example1() {
        assert_eq!(ANSWER1, process(EXAMPLE1))
    }
}
