pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        let mut cont_minus = false;
        let mut count_zero = 0;

        for el in nums.iter() {
            if el < &0 {
                cont_minus = true;
            }
            if el == &0 {
                count_zero = count_zero + 1;
            }
        }

        if count_zero == nums.len() {
            return vec![nums];
        }

        if !cont_minus {
            return  res;
        }  
        res
    }
}