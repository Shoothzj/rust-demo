use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut arr = vec![0; (n + 1) as usize];
        arr[0] = 0;
        if n == 0 {
            return 0;
        }
        arr[1] = 1;
        if n == 1 {
            return 1;
        }
        let mut result = 0;
        for i in 2..(n + 1) {
            let half_index: usize = (i / 2) as usize;
            let index: usize = i as usize;
            if i % 2 == 0 {
                arr[index] = arr[half_index]
            } else {
                arr[index] = arr[half_index] + arr[half_index + 1]
            }
            result = cmp::max(result, arr[index]);
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let result = Solution::get_maximum_generated(5);
        println!("{}", result)
    }

    #[test]
    fn test_case2() {
        let result = Solution::get_maximum_generated(4);
        println!("{}", result);
        assert_eq!(result, 2);
    }
}