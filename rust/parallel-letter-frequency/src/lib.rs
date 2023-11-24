use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let workers_to_spawn: usize;
    let mut words_count: HashMap<char, usize> = HashMap::new();

    if input.len() == 0 { return words_count };

    match worker_count {
        0 => workers_to_spawn = 1,
        x if x > input.len() => workers_to_spawn = input.len(),
        _ => workers_to_spawn = worker_count,
    }

    let mut threads: Vec<JoinHandle<HashMap<char, usize>>> = vec![];

    for chunk in input.chunks(workers_to_spawn) {
        let lines: Vec<String> = chunk
            .iter()
            .filter(|string| !string.is_empty())
            .map(|line| line.to_lowercase())
            .collect();

        let thread = thread::spawn(|| {
            let mut thread_word_count = HashMap::new();

            for line in lines {
                concat_maps(&mut thread_word_count, count_chars(&line));
            }

            return thread_word_count;
        });

        threads.push(thread);
    }

    for thread in threads {
        let chunk_result = thread.join();

        match chunk_result {
            Ok(words_map) => { concat_maps(&mut words_count, words_map) }
            Err(_) => { println!("Chunk failed!") }
        }
    }

    return words_count;
}

fn count_chars(input: &String) -> HashMap<char, usize> {
    let mut words_count = HashMap::new();

    input.chars()
        .filter(|char| char.is_alphabetic())
        .for_each(|char| match words_count.get(&char) {
            Some(w) => { words_count.insert(char, w + 1); }
            None => { words_count.insert(char, 1); }
        });

    return words_count;
}

fn concat_maps(a: &mut HashMap<char, usize>, b: HashMap<char, usize>) {
    b.iter()
        .for_each(|(key, val)| {
            let anchor = a.get_mut(key);

            match anchor {
                Some(anchor_val) => { *anchor_val += val; }
                None => { a.insert(*key, *val); }
            }
        })
}