use std::sync::Arc;

use anyhow::Result;
use rand::Rng;
use thread_pool::ThreadPool;

mod error;
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
        if (right_iter == right_arr.len()) || (left_iter != left_arr.len() && left_arr[left_iter] < right_arr[right_iter])   {
            result[left_iter + right_iter] = left_arr[left_iter];
            left_iter += 1;
        }
        else {
            result[left_iter + right_iter] = right_arr[right_iter];
            right_iter += 1;
        }
    }
    result
}

fn merge_sort_parallel<'a>(arr: &'a [i64], pool: Arc<ThreadPool<'a, Vec<i64>>>) -> Vec<i64> {
    match arr.len() {
        0 | 1 => arr.to_vec(),
        _ => {
            // split array into two halves 
            let mid = arr.len() / 2;
            let (left, right): (&'a [i64], &'a [i64]) = arr.split_at(mid);

            let left_sort_pool = pool.clone();
            let sort_left_id = pool.execute(move || {
                merge_sort_parallel(left, left_sort_pool)
            });

            let right_sort_pool = pool.clone();
            let sort_right_id = pool.execute(move || {
                merge_sort_parallel(right, right_sort_pool) 
            });

            return vec![];

            // wait for both halves to be sorted
            let sort_left = pool.wait_for_task(sort_left_id);
            let sort_right = pool.wait_for_task(sort_right_id);

            // merge the two halves
            merge_sorted_halves(&sort_left, &sort_right)
        }
    }
}

fn main() -> Result<()> {
    let data = random_vec(10_000_000);
    Ok(())
}

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
        let arr = random_vec(10);
        let mut arr_copy = arr.clone();
        let pool = Arc::new(ThreadPool::new(1));
        let result = merge_sort_parallel(&arr, pool);
        arr_copy.sort();
        assert_eq!(result, arr_copy);
    }
}
