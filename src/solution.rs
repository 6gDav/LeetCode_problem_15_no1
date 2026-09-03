use std::vec;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        if nums.len() < 3 {
            return res;
        }
        
        let dummy_nums = nums.clone();

        for ind in 0..dummy_nums.len() {
            for ind2 in (ind + 1)..dummy_nums.len() {
                for ind3 in (ind + 2)..dummy_nums.len() {
                    if dummy_nums[ind] + dummy_nums[ind2] + dummy_nums[ind3] == 0 {
                        let mut new_res_el = vec![dummy_nums[ind], dummy_nums[ind2], dummy_nums[ind3]];
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
