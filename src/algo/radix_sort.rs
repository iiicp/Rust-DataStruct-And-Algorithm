fn get_max_digits(nums: &[i32]) -> usize {
    let mut max_val = i32::MIN;
    for i in 0..nums.len() {
        if nums[i] > max_val {
            max_val = nums[i];
        }
    }
    if max_val < 10 {
        1
    } else {
        let mut digits = 0usize;
        while max_val > 0 {
            max_val /= 10;
            digits += 1;
        }
        digits
    }
}

// 所有元素必须是非负数
pub fn radix_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    // setup buckets 10*n
    let mut buckets = Vec::with_capacity(10);
    for _ in 0..10 {
        buckets.push(Vec::new());
    }

    // setup 1 找到最大的数，求出其有多少位
    let max_digits = get_max_digits(nums);

    // setup 2 从最低位开始，依次进行循环
    for i in 1..=max_digits {
        let base = 10i32.pow((i - 1) as u32);
        let nums_size = nums.len();

        // 从个位开始，通过个位的数字找到对应的桶，将原数放入到桶中
        for j in 0..nums_size {
            let idx = (nums[j] / base % 10) as usize;
            buckets[idx].push(nums[j]);
        }

        // 依次从桶中取数，放入原数组中，一轮排序结束
        let mut k = 0usize;
        for i in 0..10 {
            for j in 0..buckets[i].len() {
                let bucket = &buckets[i];
                nums[k] = bucket[j];
                k += 1;
            }
        }

        // 清空桶中的数据，便于下一次迭代
        for i in 0..10 {
            buckets[i].clear();
        }
    }
}