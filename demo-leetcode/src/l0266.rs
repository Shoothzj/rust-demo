pub struct Solution {}

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut array: [i32; 26] = [0; 26];
        let a_index = 'a' as usize;
        for x in s.chars() {
            let aux = x as usize - a_index;
            array[aux] = array[aux] + 1;
        }
        let mut aux = 0;
        for i in 0..26 {
            if array[i] % 2 != 0 {
                aux = aux + 1
            }
        }
        return aux < 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let result = Solution::can_permute_palindrome("aab".to_string());
        println!("{}", result)
    }

    #[test]
    fn test_case2() {
        let result = Solution::can_permute_palindrome("code".to_string());
        println!("{}", result)
    }

    #[test]
    fn test_case3() {
        let is_palindrome = Solution::can_permute_palindrome("carerac".to_string());
        println!("{}", is_palindrome)
    }

    #[test]
    fn test_case4() {
        let is_palindrome = Solution::can_permute_palindrome("aab".to_string());
        println!("{}", is_palindrome)
    }

}