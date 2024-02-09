use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter = |input: &[&str]| {
        let mut map = HashMap::new();
        for line in input {
            for c in line
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
            {
                *map.entry(c).or_default() += 1;
            }
        }
        map
    };

    match input.len() {
        0 => HashMap::new(),
        n if n < 500 => counter(input),
        _ => thread::scope(|s| {
            let mut handlers = Vec::with_capacity(worker_count);
            for chunk in input.chunks(input.len() / worker_count + 1) {
                handlers.push(s.spawn(|| counter(chunk)))
            }
            let mut map = HashMap::new();

            for res in handlers {
                res.join().unwrap().into_iter().for_each(|(k, v)| {
                    *map.entry(k).or_default() += v;
                })
            }
            map
        }),
    }
}
