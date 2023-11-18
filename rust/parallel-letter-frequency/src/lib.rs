use std::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let worker_count_computed: &usize;
    let mut words_count: HashMap<char, usize> = HashMap::new();

    match worker_count {
        w if w > input.len() => worker_count_computed = &input.len(),
        0 => worker_count_computed = &1,
        _ => worker_count_computed = &worker_count,
    }

    let threads = input.chunks(*worker_count_computed)
        .map(|chunk| thread::spawn(|| {
            println!("Created thread to compute chunk");

            //TODO This can be done functional without allocation of intermidiate map
            let mut threads_words_count: HashMap<char, usize> = HashMap::new();

            chunk.iter()
                .map(|chunk| countChars(chunk))
                .for_each(|map| threads_words_count.extend(map));

            return threads_words_count;
        }))
        .map(|thread| thread.join())//FIXME Is it triggered sequentially?
        .collect::<Vec<_>>();

    threads.iter()
        .for_each(|chunkResult| match chunkResult {
            Ok(subResult) => words_count.extend(subResult),
            Err(err) => eprintln!("Thread failed"),
        });

    return words_count;
}

fn countChars(input: &&str) -> HashMap<char, usize> {

}