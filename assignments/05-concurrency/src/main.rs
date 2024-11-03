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
// assume the two sorted lists are contiguous in the input array
// writes the return in place to the input array
fn merge_sorted_halves<'a>(arr: &'a mut [i64], left_s: usize, left_len: usize, right_s: usize, right_len: usize)  {
    let mut result = vec![0; left_len + right_len];
    let mut left_iter = 0;
    let mut right_iter = 0;

    // sort with two point approach
    while left_iter < left_len || right_iter < right_len {
        if (right_iter == right_len) || (left_iter != left_len && arr[left_s + left_iter] < arr[right_s + right_iter])   {
            result[left_iter + right_iter] = arr[left_s + left_iter];
            left_iter += 1;
        }
        else {
            result[left_iter + right_iter] = arr[right_s + right_iter];
            right_iter += 1;
        }
    }
    arr.copy_from_slice(&result);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_halves_basic() {
        let mut arr = vec![1, 4, 7, 2, 5, 8, 3, 6, 9];
        let (left, right) = arr.split_at_mut(3);
        left.sort();
        right.sort();
        merge_sorted_halves(&mut arr, 0, 3, 3, 6);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
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

        let first_half_len = split_point;
        let second_half_len = arr.len() - first_half_len;
        merge_sorted_halves(&mut arr, 0, first_half_len, first_half_len, second_half_len);

        arr_copy.sort();
        assert_eq!(arr, arr_copy);
    }   
}
