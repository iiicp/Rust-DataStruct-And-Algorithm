/// `binary_search` 在排序的数组中进行二分查找
///
/// # Examples
///
/// ```
/// let input = vec![1, 2, 3, 4, 30, 35, 88, 100];
/// for i in 0..input.len() {
///     assert_eq!(Some(i), binary_search(&input, input[i]));
/// }
/// ```
pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.len() == 0 {
        return None;
    }
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i <= j {
        let mid = i + (j - i) / 2;
        if nums[mid] == target {
            return Some(mid);
        } else if nums[mid] < target {
            i = mid + 1;
        } else {
            if mid < 1 {
                break;
            }
            j = mid - 1;
        }
    }
    None
}

/// `binary_search_recursive` 在排序的数组中进行二分查找，递归实现版本
///
/// # Examples
///
/// ```
/// let input = vec![1, 2, 3, 4, 30, 35, 88, 100];
/// for i in 0..input.len() {
///     assert_eq!(true, binary_search_recursive(&input, input[i]));
/// }
/// ```
pub fn binary_search_recursive(nums: &[i32], target: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mid = nums.len() / 2;
    return if nums[mid] == target {
        true
    } else if nums[mid] < target {
        binary_search_recursive(&nums[mid + 1..], target)
    } else {
        binary_search_recursive(&nums[0..mid], target)
    }
}

#[test]
fn test_binary_search() {
    {
        let input = vec![];
        assert_eq!(None, binary_search(&input, 1));
        assert_eq!(false, binary_search_recursive(&input, 1));
    }
    {
        let input = vec![1];
        assert_eq!(None, binary_search(&input, 2));
        assert_eq!(false, binary_search_recursive(&input, 2));
    }
    {
        let input = vec![1, 2, 3, 4, 30, 35, 88, 100];
        for i in 0..input.len() {
            assert_eq!(Some(i), binary_search(&input, input[i]));
            assert_eq!(true, binary_search_recursive(&input, input[i]));
        }
    }
    {
        let input = vec![1, 2, 3, 4, 30, 35, 88, 100];
        assert_eq!(None, binary_search(&input, 101));
        assert_eq!(false, binary_search_recursive(&input, 101));
    }
    {
        let input = vec![1, 2, 3, 4, 30, 35, 88, 100];
        assert_eq!(None, binary_search(&input, 0));
        assert_eq!(false, binary_search_recursive(&input, 0));
    }
    {
        let input = vec![1, 2, 3, 4, 30, 35, 88, 100];
        assert_eq!(None, binary_search(&input, 33));
        assert_eq!(false, binary_search_recursive(&input, 33));
    }
}