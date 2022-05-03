use crate::algo::common::get_min_max_in_nums;

pub fn count_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }
    // step 1
    let (min_val, max_val) = get_min_max_in_nums(nums);

    // step 2
    let bucket_nums = max_val - min_val + 1;
    let mut buckets = vec![0; bucket_nums as usize];
    for i in 0..nums.len() {
        buckets[(nums[i] - min_val) as usize] += 1;
    }

    // step 3
    let mut j = 0;
    for i in 0..bucket_nums as usize {
        while buckets[i] > 0 {
            nums[j] = i as i32 + min_val;
            j += 1;
            buckets[i] -= 1;
        }
    }
}