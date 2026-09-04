use std::vec;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        if nums.len() < 3 {
            return res;
        }

        for ind in 0..nums.len() {
            for ind2 in (ind + 1)..nums.len() {
                for ind3 in (ind2 + 1)..nums.len() {
                    if nums[ind] + nums[ind2] + nums[ind3] == 0 {
                        let mut new_res_el = vec![nums[ind], nums[ind2], nums[ind3]];
                        new_res_el.sort();

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
