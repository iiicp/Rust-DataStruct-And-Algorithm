pub fn insert_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut j = i - 1;
        let cur_num = nums[i];
        loop {
            if nums[j] > cur_num {
                nums[j + 1] = nums[j];
                // 当首元素也移动之后，loop应该要结束
                // 将当前要插入的元素移动到首元素的位置
                if j == 0 {
                    nums[j] = cur_num;
                    break;
                }
            } else {
                nums[j + 1] = cur_num;
                break;
            }
            j -= 1;
        }
    }
}