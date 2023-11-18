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
    let chunks = input.chunks(*worker_count_computed);

    for chunk in chunks {
        let thread = thread::spawn(move || {
            println!("Created thread to compute chunk");

            //TODO This can be done functional without allocation of intermidiate map
            let mut threads_words_count: HashMap<char, usize> = HashMap::new();

            chunk.iter()
                .map(|chunk| count_chars(chunk))
                .for_each(|map| threads_words_count.extend(map));

            return threads_words_count;
        });

        threads.push(thread);
    }

    for thread in threads {
        let result = thread.join();

        match result {
            Ok(subMap) => { words_count.extend(subMap)}
            Err(_) => { eprintln!("Thread failed"); }
        }
    }

    return words_count;
}

fn count_chars(input: &&str) -> HashMap<char, usize> {
    let mut words_count = HashMap::new();

    input.chars()
        .for_each(|char| match words_count.get(&char) {
            Some(w) => { words_count.insert(char, w + 1); }
            None => { words_count.insert(char, 1); }
        });

    return words_count;
}