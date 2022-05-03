// 构建大顶端
pub fn heap_sort(nums: &mut [i32]) {
    fn parent(i: usize) -> usize {
        (i - 1) >> 1
    }
    fn left_child(i: usize) -> usize {
        2 * i + 1
    }

    fn shift_down(nums: &mut [i32], i: usize) {
        let mut i = i;
        while left_child(i) < nums.len() {
            let mut j = left_child(i);
            if j + 1 < nums.len() && nums[j + 1] > nums[j] {
                j += 1;
            }
            if nums[i] >= nums[j] {
                break;
            }
            nums.swap(i, j);
            i = j;
        }
    }
    fn heapify(nums: &mut [i32]) {
        let max_non_leaf_node = ((nums.len() - 1) - 1) / 2;
        for i in (0..=max_non_leaf_node).rev() {
            shift_down(nums, i);
        }
    }

    if nums.len() <= 2 {
        return;
    }

    heapify(nums);
    let size = nums.len();
    for i in 0..size {
        nums.swap(0, (size - 1) - i);
        shift_down(&mut nums[..((size - 1) - i)], 0);
    }
}