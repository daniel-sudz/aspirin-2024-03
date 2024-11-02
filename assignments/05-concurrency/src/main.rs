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
fn merge_sorted_halves<'a>(arr: &'a mut [i64], left_s: usize, left_len: usize, right_s: usize, right_len: usize)  {
    let mut left_iter = 0;
    let mut right_iter = 0;

    // sort with two point approach
    while left_iter < left_len || right_iter < right_len {
        if (right_iter == right_len) || (left_iter != left_len && arr[left_s + left_iter] < arr[right_s + right_iter])   {
            arr[left_s + left_iter + right_iter] = arr[left_s + left_iter];
            left_iter += 1;
        }
        else {
            arr[left_s + left_iter + right_iter] = arr[right_s + right_iter];
            right_iter += 1;
        }
    }
}

/*
fn merge_sort_parallel<'a>(data: &'a mut [i64], pool: &'a ThreadPool<'a, i64>) -> &'a [i64] {
    match data.len() {
        0 | 1 => return data,
        _ => {
            // split array into two halves with mutable ownership using split_at_mut
            let mid = data.len() / 2;
            let (mut left, mut right) = data.split_at_mut(mid);
            
            // queue the sort operation for each half
            let sort_left = pool.execute(|| merge_sort_parallel(&mut left, pool));
            let sort_right = pool.execute(|| merge_sort_parallel(&mut right, pool));

            // wait for both halves to be sorted
            pool.wait_for_task(sort_left);
            pool.wait_for_task(sort_right);

            // merge the two halves
            merge_sorted_halves(&mut left, &mut right)
        }
    }
}
    */

fn main() -> Result<()> {
    let data = random_vec(10_000_000);
    Ok(())
}
