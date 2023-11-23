use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut worker_count_computed: &usize = &input.len();
    let mut words_count: HashMap<char, usize> = HashMap::new();

    match worker_count {
        0 => worker_count_computed = &1,
        _ => worker_count_computed = &worker_count,
    }

    let mut threads: Vec<JoinHandle<HashMap<char, usize>>> = vec![];

    input.chunks(*worker_count_computed)
        .for_each(|chunk| threads.push(spawn_worker_and_proccess(chunk)));

    for thread in threads {
        let chunk_result = thread.join();

        match chunk_result {
            Ok(words_map) => { words_count.extend(words_map) }
            Err(_) => { println!("Chunk failed!") }
        }
    }

    return words_count;
}

fn spawn_worker_and_proccess(chunk: &[&str]) -> JoinHandle<HashMap<char, usize>> {
    let chunk_owned: Vec<String> = chunk
        .iter()
        .map(|line| line.to_lowercase())
        .collect();

    return thread::spawn(move || {
        let mut chunk_words_count = HashMap::new();

        for line in chunk_owned {
            chunk_words_count.extend(count_chars(&line));
        }

        return chunk_words_count;
    })
}

fn count_chars(input: &String) -> HashMap<char, usize> {
    let mut words_count = HashMap::new();

    input.chars()
        .for_each(|char| match words_count.get(&char) {
            Some(w) => { words_count.insert(char, w + 1); }
            None => { words_count.insert(char, 1); }
        });

    return words_count;
}