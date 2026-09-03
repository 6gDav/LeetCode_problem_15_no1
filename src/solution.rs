use std::vec;

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
            return res;
        }

        let mut dummy_nums = nums.clone();

        for ind in 0..dummy_nums.len() {
            for ind2 in 1..dummy_nums.len() {
                for ind3 in 1..dummy_nums.len() {
                    if dummy_nums[ind] + dummy_nums[ind2] + dummy_nums[ind3] == 0 {
                        let new_res_el = vec![dummy_nums[ind], dummy_nums[ind2], dummy_nums[ind3]];

                        if !res.contains(&new_res_el) {
                            res.push(new_res_el);
                        }
                    }
                }
            }
        }

        res
    }
}
