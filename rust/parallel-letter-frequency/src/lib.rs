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

    input.chunks(workers_to_spawn)
        .for_each(|chunk| threads.push(spawn_worker_and_process(chunk)));

    for thread in threads {
        let chunk_result = thread.join();

        match chunk_result {
            Ok(words_map) => { concat_maps(&mut words_count, words_map) }
            Err(_) => { println!("Chunk failed!") }
        }
    }

    return words_count;
}

fn spawn_worker_and_process(chunk: &[&str]) -> JoinHandle<HashMap<char, usize>> {
    let chunk_owned: Vec<String> = chunk
        .iter()
        .filter(|string| !string.is_empty())
        .map(|line| line.to_lowercase())
        .collect();

    return thread::spawn(move || {
        let mut chunk_words_count = HashMap::new();

        chunk_owned
            .iter()
            .for_each(|line| concat_maps(&mut chunk_words_count, count_chars(&line)));

        return chunk_words_count;
    })
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