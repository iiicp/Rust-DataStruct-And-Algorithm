pub fn get_min_max_in_nums(nums: &[i32]) -> (i32, i32) {
    let mut min_val = i32::MAX;
    let mut max_val = i32::MIN;
    for i in 0..nums.len() {
        if nums[i] > max_val {
            max_val = nums[i];
        } else if nums[i] < min_val {
            min_val = nums[i];
        }
    }
    (min_val, max_val)
}