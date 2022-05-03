use crate::algo::quick_sort::quick_sort;
use crate::algo::common::get_min_max_in_nums;

pub fn bucket_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }
    // step 1
    let (min_val, max_val) = get_min_max_in_nums(nums);

    // step 2
    // 前面的桶内的元素一定全部小于后面的桶的元素，划分到桶中一定是按照大小序来分的
    // 根据step 1的range范围，获取每个桶的range范围
    let bucket_nums = 5;
    let distance = (max_val - min_val) / (bucket_nums - 1);
    let mut buckets = Vec::with_capacity(bucket_nums as usize);
    for _ in 0..bucket_nums {
        buckets.push(Vec::new());
    }
    for i in 0..nums.len() {
        if nums[i] >= min_val && nums[i] < min_val + distance {
            buckets[0].push(nums[i]);
        } else if nums[i] >= min_val + distance && nums[i] < min_val + distance * 2 {
            buckets[1].push(nums[i]);
        } else if nums[i] >= min_val + distance * 2 && nums[i] < min_val + distance * 3 {
            buckets[2].push(nums[i]);
        } else if nums[i] >= min_val + distance * 3 && nums[i] < min_val + distance * 4 {
            buckets[3].push(nums[i]);
        } else {
            buckets[4].push(nums[i]);
        }
    }

    // step 3
    for i in 0..bucket_nums as usize {
        quick_sort(&mut buckets[i]);
    }

    // step 4
    let mut k = 0;
    for i in 0..bucket_nums as usize {
        for n in buckets[i].iter() {
            nums[k] = *n;
            k += 1;
        }
    }
}