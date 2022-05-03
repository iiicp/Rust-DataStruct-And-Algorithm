pub fn bubble_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        for j in 0..nums.len() - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}
