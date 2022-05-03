pub(self) fn partition(nums: &mut [i32]) -> usize {
    let t = nums[0];
    let mut i = 0usize;
    let mut j = nums.len() - 1;
    while i < j {
        while i < j && nums[j] >= t {
            j -= 1;
        }
        nums[i] = nums[j];
        while i < j && nums[i] < t {
            i += 1;
        }
        nums[j] = nums[i];
    }
    nums[i] = t;
    return i;
}

pub fn quick_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let p = partition(nums);
        quick_sort(&mut nums[0..p]);
        quick_sort(&mut nums[(p + 1)..]);
    }
}