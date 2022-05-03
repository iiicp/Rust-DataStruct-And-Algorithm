//! `merge_sort` 是一种高阶的排序算法，时间复杂度为O(nlog(n)), 空间复杂度为O(n)，是一种稳定排序算法
//!
//! `bubble_sort` 是一种简单的排序算法，时间复杂度为O(n^2), 空间复杂度为O(1)，是一种稳定排序算法
//!
//! `quick_sort` 是一种高阶的排序算法，平均时间复杂度为O(nlog(n)), 空间复杂度为O(1)，不稳定排序算法
///
pub mod merge_sort;
pub mod bubble_sort;
pub mod quick_sort;
pub mod insert_sort;
pub mod shell_sort;
pub mod select_sort;
pub mod heap_sort;
pub mod bucket_sort;
pub mod count_sort;
pub mod common;
pub mod radix_sort;
pub mod binary_search;

#[cfg(test)]
mod test {
    use crate::algo::{
        bubble_sort::bubble_sort,
        insert_sort::insert_sort,
        select_sort::select_sort,
        quick_sort::quick_sort,
        shell_sort::shell_sort,
        merge_sort::merge_sort,
        heap_sort::heap_sort,
        bucket_sort::bucket_sort,
        count_sort::count_sort,
        radix_sort::radix_sort,
    };

    #[test]
    fn test_empty_vec() {
        let origin = [];
        let output = [];
        {
            let mut input = origin.clone();
            bubble_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            quick_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            insert_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            shell_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            select_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            merge_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            heap_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            bucket_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            count_sort(&mut input);
            assert_eq!(input, output);
        }
    }

    #[test]
    fn test_one_element_vec() {
        let origin = [1];
        let output = [1];
        {
            let mut input = origin.clone();
            bubble_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            quick_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            insert_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            shell_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            select_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            merge_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            heap_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            bucket_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            count_sort(&mut input);
            assert_eq!(input, output);
        }
    }

    #[test]
    fn test_multi_element_vec() {
        let origin = [8, 6, 7, 5, 0, 100, -1, 8, 9, 2, 11];
        let output = [-1, 0, 2, 5, 6, 7, 8, 8, 9, 11, 100];
        {
            let mut input = origin.clone();
            bubble_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            quick_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            insert_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            shell_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            select_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            merge_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            heap_sort(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = origin.clone();
            bucket_sort(&mut input);
            assert_eq!(input, output);
        }

        {
            let mut input = [54,32,99,18,75,31,43,56,21,22];
            let output = [18,21,22,31,32,43,54,56,75,99];
            bucket_sort(&mut input);
            assert_eq!(input, output);
        }

        {
            let mut input = origin.clone();
            count_sort(&mut input);
            assert_eq!(input, output);
        }

        {
            let mut input = [54,32,99,18,75,31,43,56,21,22];
            let output = [18,21,22,31,32,43,54,56,75,99];
            count_sort(&mut input);
            assert_eq!(input, output);
        }

        {
            let mut input = [54,32,99,18,75,31,43,56,21,22];
            let output = [18,21,22,31,32,43,54,56,75,99];
            radix_sort(&mut input);
            assert_eq!(input, output);
        }
    }
}
