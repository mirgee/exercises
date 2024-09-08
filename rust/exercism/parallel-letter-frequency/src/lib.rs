use std::{collections::HashMap, thread};

fn frequency_str(input: &String) -> HashMap<char, usize> {
    input
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut acc, c| {
            acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
            acc
        })
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = vec![];
    let mut results = vec![];
    for worker_num in 0..worker_count {
        let input = input
            .iter()
            .map(|s| {
                s.to_string()
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect()
            })
            .collect::<Vec<_>>();
        let handle = thread::spawn(move || {
            input
                .iter()
                .enumerate()
                .filter(|(i, _elem)| i % worker_count == worker_num)
                .map(|(_i, elem)| elem)
                .map(frequency_str)
                .fold(HashMap::<char, usize>::new(), |mut acc, hm| {
                    for (k, v) in hm {
                        acc.entry(k).and_modify(|val| *val += v).or_insert(v);
                    }
                    acc
                })
        });
        handles.push(handle);
    }

    for handle in handles {
        results.push(handle.join().unwrap());
    }

    results
        .into_iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, hm| {
            for (k, v) in hm {
                acc.entry(k).and_modify(|val| *val += v).or_insert(v);
            }
            acc
        })
}
