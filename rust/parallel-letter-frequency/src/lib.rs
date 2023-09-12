use rayon::prelude::*;
use std::cmp::min;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let real_worker_count = min(input.len(), worker_count);

    match input.len() {
        0 => HashMap::new(),
        n if n < 500 || real_worker_count == 1 => compute_sequentially(input),
        _ => compute_parallel(input, real_worker_count),
    }
}

fn compute_sequentially(input: &[&str]) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    input
        .iter()
        .flat_map(|line| line.chars())
        .filter(|c| c.is_alphabetic())
        .filter_map(|c| c.to_lowercase().next())
        .for_each(|c| *result.entry(c).or_insert(0) += 1);
    result
}

fn compute_parallel(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunk_size = input.len() / worker_count + 1;

    let p_results: Vec<HashMap<char, usize>> = input
        .par_chunks(chunk_size)
        .map(compute_sequentially)
        .collect();

    let mut result = HashMap::new();
    p_results.iter().for_each(|p_result| {
        p_result
            .iter()
            .for_each(|(&k, &v)| *result.entry(k).or_insert(0) += v);
    });
    result
}
