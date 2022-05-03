pub(self) fn merge(nums: &mut [i32], mid: usize) {
    let mut tmp_vec = Vec::with_capacity(nums.len());
    let mut i = 0usize;
    let mut j = mid;
    for _ in 0..nums.len() {
        if i == mid || j == nums.len() {
            break;
        }
        if nums[i] <= nums[j] {
            tmp_vec.push(nums[i]);
            i += 1;
        } else {
            tmp_vec.push(nums[j]);
            j += 1;
        }
    }

    while i < mid {
        tmp_vec.push(nums[i]);
        i += 1;
    }

    while j < nums.len() {
        tmp_vec.push(nums[j]);
        j += 1;
    }

    for i in 0..tmp_vec.len() {
        nums[i] = tmp_vec[i];
    }
}

pub fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        merge_sort(&mut nums[..mid]);
        merge_sort(&mut nums[mid..]);
        merge(nums, mid);
    }
}