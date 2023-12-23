use std::collections::{HashMap, VecDeque};

use indicatif::ProgressIterator;
use nom::{
    bytes::complete,
    character::complete::{alpha1, multispace1, one_of},
    combinator::opt,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use num::Integer;

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

    let (final_name, mut loops): (&str, HashMap<&str, (usize, Option<usize>)>) = modules
        .values()
        .find_map(|module| match module {
            Module {
                destinations,
                kind: ModuleKind::Conjunction { states },
                ..
            } if destinations.contains(&"rx") => Some((
                module.name,
                states.keys().map(|&name| (name, (0, None))).collect(),
            )),
            _ => None,
        })
        .unwrap();

    struct QueueItem<'a> {
        to: &'a str,
        from: &'a str,
        pulse: Pulse,
    }

    let mut queue: VecDeque<QueueItem> = VecDeque::new();

    for button_presses in 0usize.. {
        if loops.values().all(|(_, loop_end)| loop_end.is_some()) {
            break;
        }

        // println!("NEW ROUND HERE\n");
        // Button module pressed
        queue.push_back(QueueItem {
            from: "button",
            to: "broadcaster",
            pulse: Pulse::Low,
        });
        // dbg!(&queue, &modules);
        while let Some(QueueItem { to, from, pulse }) = queue.pop_front() {
            let Some(module) = modules.get_mut(to) else {
                continue;
            };
            let Some(pulse_to_send) = module.kind.handle_pulse(pulse, from) else {
                continue;
            };

            if to == final_name && pulse == Pulse::High {
                if let ModuleKind::Conjunction { states } = &module.kind {
                    for name in states
                        .iter()
                        .filter_map(|(name, pulse)| (pulse == &Pulse::High).then_some(name))
                    {
                        loops.insert(
                            &name,
                            match loops.get(name).unwrap() {
                                (0, None) => (button_presses, None),
                                (prev, None) | (_, Some(prev)) => {
                                    println!("LOOP LEN FOR {name}: {}", button_presses - prev);
                                    (*prev, Some(button_presses))
                                }
                            },
                        );
                    }
                    dbg!(button_presses, states);
                }
            }

            // println!("{to} {pulse_to_send:?} -> {:?}", module.destinations);
            for destination in &module.destinations {
                queue.push_back(QueueItem {
                    from: module.name,
                    to: destination,
                    pulse: pulse_to_send,
                });
            }
        }

        if let Some(Module {
            kind: ModuleKind::Conjunction { states },
            ..
        }) = modules.get(final_name)
        {
            if states.values().any(|&state| state == Pulse::High) {
                dbg!(button_presses, states);
                for sub in states.keys().map(|name| modules.get(name).unwrap()) {
                    println!("{sub:?}");
                }
            }
        }

        // dbg!(&modules);
    }

    loops
        .values()
        .map(|(start, end)| end.unwrap() - start)
        .reduce(|acc, loop_len| acc.lcm(&loop_len))
        .unwrap()
        .to_string()
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
