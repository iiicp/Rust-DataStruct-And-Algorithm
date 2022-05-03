pub fn select_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut min_ele_idx = i - 1;
        let mut cur_ele = nums[i - 1];
        for j in i..nums.len() {
            if cur_ele > nums[j] {
                min_ele_idx = j;
                cur_ele = nums[j];
            }
        }
        nums.swap(i - 1, min_ele_idx);
    }
}