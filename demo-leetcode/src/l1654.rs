use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    // forbidden 不允许落脚的地方
    // a 向前走步数
    // b 向后走步数
    // x 目标地址
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let a = a as usize;
        let b = b as usize;
        let x = x as usize;

        let mut visited = vec![0; 6000];
        visited[0] = 2;
        for i in forbidden.iter() {
            visited[(*i) as usize] = 2;
        }
        let mut q = VecDeque::new();
        q.push_back((0, 0, true));
        while let Some((p, c, forward)) = q.pop_front() {
            if p + a == x {
                return c + 1;
            }
            if p + a < 6000 && visited[p + a] != 2 {
                visited[p + a] = 2;
                q.push_back((p + a, c + 1, true))
            }
            if forward && p > b {
                if p - b == x {
                    return c + 1;
                }
                if visited[p - b] == 0 {
                    visited[p - b] = 1;
                    q.push_back((p - b, c + 1, false));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}