use std::{sync::Arc, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use rand::Rng;
use thread_pool::ThreadPool;

mod thread_pool;

/// Generate a random vector of size capacity filled with random i64s
fn random_vec(capacity: usize) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}

// merge two sorted lists into one sorted list using standard two pointer approach
fn merge_sorted_halves<'a>(left_arr: &'a [i64], right_arr: &'a [i64]) -> Vec<i64> {
    let mut result = vec![0; left_arr.len() + right_arr.len()];
    let mut left_iter = 0;
    let mut right_iter = 0;

    // sort with two point approach
    while left_iter < left_arr.len() || right_iter < right_arr.len() {
        if (right_iter == right_arr.len())
            || (left_iter != left_arr.len() && left_arr[left_iter] < right_arr[right_iter])
        {
            result[left_iter + right_iter] = left_arr[left_iter];
            left_iter += 1;
        } else {
            result[left_iter + right_iter] = right_arr[right_iter];
            right_iter += 1;
        }
    }
    result
}

/// classical merge sort algorithm
fn merge_sort(arr: &[i64]) -> Vec<i64> {
    match arr.len() {
        0 | 1 => arr.to_vec(),
        _ => {
            let mid = arr.len() / 2;
            let (left, right): (&[i64], &[i64]) = arr.split_at(mid);
            let sorted_left = merge_sort(left);
            let sorted_right = merge_sort(right);
            merge_sorted_halves(&sorted_left, &sorted_right)
        }
    }
}

/// Merge parallel using a thread pool
fn merge_parallel(chunks: Vec<Vec<i64>>, pool: Arc<ThreadPool<'_, Vec<i64>>>) -> Vec<i64> {
    let pool_copy = Arc::clone(&pool);
    let pool_copy_2 = Arc::clone(&pool);
    let pool_copy_3 = Arc::clone(&pool);

    let chunks_len = chunks.len();
    match chunks_len {
        1 => chunks[0].clone(),
        _ => {
            // split chunks into pairs
            let merge_pairs: Vec<Vec<Vec<i64>>> = chunks.chunks(2).map(|c| c.to_vec()).collect();

            // merge chunks in parallel
            let merge_tasks = merge_pairs
                .into_iter()
                .map(move |merge_pair| match merge_pair.len() {
                    1 => pool_copy.execute(move || merge_pair[0].clone()),
                    _ => pool_copy
                        .execute(move || merge_sorted_halves(&merge_pair[0], &merge_pair[1])),
                })
                .collect::<Vec<_>>();

            // wait for all chunks to be merged
            let result = merge_tasks
                .iter()
                .map(move |task| pool_copy_2.wait_for_task(*task))
                .collect::<Vec<_>>();
            merge_parallel(result, pool_copy_3)
        }
    }
}

/// Merge sort parallel using a thread pool
///
/// threads_avail: number of threads available to sort the array
/// pool: thread pool to use for sorting
///
/// WARNING: the threadpool must have at least threads_avail threads available
///          otherwise the function will deadlock
fn merge_sort_parallel<'a>(
    arr: &'a [i64],
    threads_avail: usize,
    pool: Arc<ThreadPool<'a, Vec<i64>>>,
) -> Vec<i64> {
    match arr.len() {
        0 | 1 => arr.to_vec(),
        _ => {
            // guard against empty array
            if arr.is_empty() {
                return vec![];
            }

            // split array into chunks of size threads_avail
            let chunks: Vec<&'a [i64]> = arr
                .chunks(std::cmp::max(1, arr.len() / threads_avail))
                .collect();

            // sort each chunk in parallel
            let sorted_chunks_ids = chunks
                .into_iter()
                .map(|chunk| pool.execute(move || merge_sort(chunk)))
                .collect::<Vec<_>>();

            // wait for all chunks to be sorted
            let sorted_chunks = sorted_chunks_ids
                .iter()
                .map(|id| pool.wait_for_task(*id))
                .collect::<Vec<_>>();

            // merge all sorted chunks
            merge_parallel(sorted_chunks, pool)
        }
    }
}

fn bench_merge_sort_parallel(c: &mut Criterion) {
    let input_sizes = vec![
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
    ];
    let thread_counts = vec![8, 16, 32, 64, 128, 256];

    let mut group = c.benchmark_group("merge_sort_parallel");
    group.sampling_mode(SamplingMode::Flat);
    group.measurement_time(Duration::from_secs(1));
    group.warm_up_time(Duration::from_millis(10));
    group.sample_size(10);

    for size in input_sizes {
        let arr = random_vec(size);

        for threads in &thread_counts {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{}", size), threads),
                threads,
                |b, &threads| {
                    b.iter(|| {
                        let pool = Arc::new(ThreadPool::new(threads));
                        merge_sort_parallel(&arr, threads, pool);
                    })
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_merge_sort_parallel);
criterion_main!(benches);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_halves_basic() {
        let mut arr = vec![1, 4, 7, 2, 5, 8, 3, 6, 9];
        let (mut left, mut right) = arr.split_at_mut(3);
        left.sort();
        right.sort();
        let result = merge_sorted_halves(&mut left, &mut right);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_merge_sorted_halves_random() {
        let mut arr = random_vec(100_000);
        let mut arr_copy = arr.clone();
        let split_point = rand::thread_rng().gen_range(0..arr.len());

        // sort the two halves
        {
            let (left, right) = arr.split_at_mut(split_point);
            left.sort();
            right.sort();
        }

        let (mut left, mut right) = arr.split_at_mut(split_point);
        let result = merge_sorted_halves(&mut left, &mut right);

        arr_copy.sort();
        assert_eq!(result, arr_copy);
    }

    #[test]
    #[ntest_timeout::timeout(1000)]
    fn test_merge_sort_parallel_one_thread() {
        println!("test_merge_sort_parallel_one_thread");
        let arr = random_vec(10_000);
        let mut arr_copy = arr.clone();
        {
            let pool = Arc::new(ThreadPool::new(1));
            let result = merge_sort_parallel(&arr, 1, pool);
            arr_copy.sort();
            assert_eq!(result, arr_copy);
        }
    }

    #[test]
    fn test_merge_sort_parallel_many_thread() {
        for thread_count in 1..100 {
            for input_size in (1..120).step_by(10) {
                let arr = random_vec(input_size);
                let mut arr_copy = arr.clone();
                let pool = Arc::new(ThreadPool::new(thread_count));
                let result = merge_sort_parallel(&arr, thread_count, pool);
                arr_copy.sort();
                assert_eq!(result, arr_copy);
            }
        }
    }
}
