// shell排序是建立在当元素部分有序的情况下，插入排序表现不错
// gap的物理意义是要插入的元素和前一个元素之间的距离
pub fn shell_sort(nums: &mut [i32]) {
    fn insert_sort(nums: &mut [i32], gap: usize) {
        for i in gap..nums.len() {
            let cur_num = nums[i];
            let mut j = i - gap;
            loop {
                if nums[j] > cur_num {
                    nums[j + gap] = nums[j];
                    // 当首元素也移动之后，loop应该要结束
                    // 将当前要插入的元素移动到首元素的位置
                    if j < gap {
                        nums[j] = cur_num;
                        break;
                    }
                } else {
                    nums[j + gap] = cur_num;
                    break;
                }

                j -= gap;
            }
        }
    }
    let mut gap = nums.len() >> 1;
    while gap > 0 {
        insert_sort(nums, gap);
        gap >>= 1;
    }
}