use std::convert::TryInto;

use itertools::Itertools;

use crate::{helper::parse, Solution};

pub struct Problem2 {}

impl Problem2 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Problem2 {
    fn run(&mut self, data: &String) {
        let data = parse("\n", data);
        let mut input = vec![];
        for d in data {
            let x = d.parse::<i32>().unwrap();
            input.push(x);
        }
        input.sort();

        let mut sum = 0;
        for perm in UniquePermutations::new(input) {
            let mut p = vec![];
            for c in perm.iter() {
                p.push(c.to_owned());
            }
            if !is_sorted_rare(&p) {
                continue;
            }

            let mut one_jolt_differences = 1;
            let mut three_jolt_differences = 1;

            let mut highest_outlet: u64 = 0;
            let mut last_highest_outlet: u64 = 0;
            for i in 0..p.len() {
                let outlet = p[i];

                // println!("Outlet: {}", outlet);

                let mut higher = 0;
                for _ in 0..3 {
                    let mut found_higher = false;
                    for y in i + 1..p.len() {
                        if p[y] - outlet <= 3 && p[y] - outlet >= 1 {
                            higher = y;
                            found_higher = true;

                            if p[y] - outlet == 1 {
                                one_jolt_differences += 1;
                            } else if p[y] - outlet == 3 {
                                three_jolt_differences += 1;
                            }
                            break;
                        }
                    }

                    if found_higher {
                        break;
                    }
                }

                if higher > highest_outlet.try_into().unwrap() {
                    // highest_outlet = i;
                } else if highest_outlet == last_highest_outlet {
                    highest_outlet = outlet.to_owned().try_into().unwrap();
                    sum += 1;
                    break;
                }

                last_highest_outlet = highest_outlet;
            }

            println!(
                "One {}, Two: {}",
                one_jolt_differences, three_jolt_differences
            );
            println!("Highest outlet found {}", highest_outlet);
            println!("Perm: {:?}", p);
        }

        println!("Concluded that sum of all arrangements to be {}", sum);
    }

    fn test(&mut self) {}
}

fn is_sorted_rare<I>(data: I) -> bool
where
    I: IntoIterator,
    I::Item: Ord + Clone,
{
    data.into_iter().tuple_windows().all(|(a, b)| a <= b)
}

use std::collections::btree_set::{BTreeSet, IntoIter};

enum UniquePermutations {
    Leaf {
        elements: Option<Vec<i32>>,
    },
    Stem {
        elements: Vec<i32>,
        unique_elements: IntoIter<i32>,
        first_element: i32,
        inner: Box<Self>,
    },
}

impl UniquePermutations {
    fn new(elements: Vec<i32>) -> Self {
        if elements.len() == 1 {
            let elements = Some(elements);
            Self::Leaf { elements }
        } else {
            let mut unique_elements = elements
                .clone()
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter();

            let (first_element, inner) = Self::next_level(&mut unique_elements, elements.clone())
                .expect("Must have at least one item");

            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            }
        }
    }

    fn next_level(
        mut unique_elements: impl Iterator<Item = i32>,
        elements: Vec<i32>,
    ) -> Option<(i32, Box<Self>)> {
        let first_element = unique_elements.next()?;

        let mut remaining_elements = elements;

        if let Some(idx) = remaining_elements.iter().position(|&i| i == first_element) {
            remaining_elements.remove(idx);
        }

        let inner = Box::new(Self::new(remaining_elements));

        Some((first_element, inner))
    }
}

impl Iterator for UniquePermutations {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Leaf { elements } => elements.take(),
            Self::Stem {
                elements,
                unique_elements,
                first_element,
                inner,
            } => loop {
                match inner.next() {
                    Some(mut v) => {
                        v.insert(0, *first_element);
                        return Some(v);
                    }
                    None => {
                        let (next_fe, next_i) =
                            Self::next_level(&mut *unique_elements, elements.clone())?;
                        *first_element = next_fe;
                        *inner = next_i;
                    }
                }
            },
        }
    }
}